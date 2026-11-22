// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml_string;
use super::refuse_java_script_call_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_property_query_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceStringGet(source_input, source_key, source_prototype) => {
            let source_input = *source_input;
            let source_key = *source_key;
            let source_prototype = *source_prototype;
            {
                let input = self::lower_java_script_call_source_expression(source_input.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let held = self::allocate_java_script_call_source_register(match input.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9113, _) => {
                            let source_ml_source2_data_property_value9113 = *source_ml_source2_data_property_value9113;
                            source_ml_source2_data_property_value9113.clone()
                        }
                    });
                    {
                        let saved = self::append_java_script_call_source_instruction(match held.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9114, _) => {
                                let source_ml_source2_data_property_value9114 = *source_ml_source2_data_property_value9114;
                                source_ml_source2_data_property_value9114.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9115) => {
                                source_ml_source2_data_property_value9115
                            }
                        }, match input.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9116) => {
                                source_ml_source2_data_property_value9116
                            }
                        }));
                        {
                            let key = self::lower_java_script_call_source_expression(source_key.clone(), bindings.clone(), saved.clone(), (depth + 1f64));
                            {
                                let held_key = self::allocate_java_script_call_source_register(match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9117, _) => {
                                        let source_ml_source2_data_property_value9117 = *source_ml_source2_data_property_value9117;
                                        source_ml_source2_data_property_value9117.clone()
                                    }
                                });
                                {
                                    let saved_key = self::append_java_script_call_source_instruction(match held_key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9118, _) => {
                                            let source_ml_source2_data_property_value9118 = *source_ml_source2_data_property_value9118;
                                            source_ml_source2_data_property_value9118.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Copy(match held_key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9119) => {
                                            source_ml_source2_data_property_value9119
                                        }
                                    }, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9120) => {
                                            source_ml_source2_data_property_value9120
                                        }
                                    }));
                                    {
                                        let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), saved_key.clone(), (depth + 1f64));
                                        {
                                            let condition = self::allocate_java_script_call_source_register(match prototype.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9121, _) => {
                                                    let source_ml_source2_data_property_value9121 = *source_ml_source2_data_property_value9121;
                                                    source_ml_source2_data_property_value9121.clone()
                                                }
                                            });
                                            {
                                                let result = self::allocate_java_script_call_source_register(match condition.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9122, _) => {
                                                        let source_ml_source2_data_property_value9122 = *source_ml_source2_data_property_value9122;
                                                        source_ml_source2_data_property_value9122.clone()
                                                    }
                                                });
                                                {
                                                    let tested = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9123, _) => {
                                                            let source_ml_source2_data_property_value9123 = *source_ml_source2_data_property_value9123;
                                                            source_ml_source2_data_property_value9123.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Primitive(match condition.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9124) => {
                                                            source_ml_source2_data_property_value9124
                                                        }
                                                    }, self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 107, 101, 121]), std::rc::Rc::new(std::cell::RefCell::new(vec![match held.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9125) => {
                                                            source_ml_source2_data_property_value9125
                                                        }
                                                    }, match held_key.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9126) => {
                                                            source_ml_source2_data_property_value9126
                                                        }
                                                    }]))));
                                                    {
                                                        let branch = { let ml_reference_array73 = match tested.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9127, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value9127.clone()
                                                            }
                                                        }; let ml_reference_projection73 = ml_reference_array73.borrow().len() as f64; ml_reference_projection73 };
                                                        {
                                                            let selected = self::append_java_script_call_source_instruction(tested.clone(), self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9128) => {
                                                                    source_ml_source2_data_property_value9128
                                                                }
                                                            }, (branch + 1f64), 0f64));
                                                            {
                                                                let own = self::append_java_script_call_source_instruction(selected.clone(), self::JavaScriptCallInstruction::Primitive(match result.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9129) => {
                                                                        source_ml_source2_data_property_value9129
                                                                    }
                                                                }, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 107, 101, 121]), std::rc::Rc::new(std::cell::RefCell::new(vec![match held.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9130) => {
                                                                        source_ml_source2_data_property_value9130
                                                                    }
                                                                }, match held_key.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9131) => {
                                                                        source_ml_source2_data_property_value9131
                                                                    }
                                                                }]))));
                                                                {
                                                                    let jump = { let ml_reference_array74 = match own.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9132, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value9132.clone()
                                                                        }
                                                                    }; let ml_reference_projection74 = ml_reference_array74.borrow().len() as f64; ml_reference_projection74 };
                                                                    {
                                                                        let skipped = self::append_java_script_call_source_instruction(own.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                                        {
                                                                            let get = { let ml_reference_array75 = match skipped.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9133, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value9133.clone()
                                                                                }
                                                                            }; let ml_reference_projection75 = ml_reference_array75.borrow().len() as f64; ml_reference_projection75 };
                                                                            {
                                                                                let got = self::append_java_script_call_source_instruction(skipped.clone(), self::JavaScriptCallInstruction::OrdinaryGet(match result.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9134) => {
                                                                                        source_ml_source2_data_property_value9134
                                                                                    }
                                                                                }, match prototype.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9135) => {
                                                                                        source_ml_source2_data_property_value9135
                                                                                    }
                                                                                }, match held_key.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9136) => {
                                                                                        source_ml_source2_data_property_value9136
                                                                                    }
                                                                                }, match held.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9137) => {
                                                                                        source_ml_source2_data_property_value9137
                                                                                    }
                                                                                }));
                                                                                {
                                                                                    let end = { let ml_reference_array76 = match got.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9138, _, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value9138.clone()
                                                                                        }
                                                                                    }; let ml_reference_projection76 = ml_reference_array76.borrow().len() as f64; ml_reference_projection76 };
                                                                                    {
                                                                                        let source_ml_source2_constructor_field9140 = match result.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9139) => {
                                                                                                source_ml_source2_data_property_value9139
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_constructor_field9165 = {
                                                                                                let source_ml_source2_nominal_spread9143 = got.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9144 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9145, _, _, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value9145.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9146 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value9147, _, _, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value9147
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9148 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value9149, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value9149
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9150 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value9151, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value9151
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9152 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value9153, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value9153
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9154 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value9155, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value9155.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9156 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value9157, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value9157.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field9158 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value9159, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value9159.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field9160 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value9161, _) => {
                                                                                                                                            source_ml_source2_data_property_value9161.clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field9162 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9163) => {
                                                                                                                                                source_ml_source2_data_property_value9163.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override9164 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values77 = { let ml_reference_value78 = (match got.clone() {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9141, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                    source_ml_source2_data_property_value9141.clone()
                                                                                                                                                }
                                                                                                                                            }).borrow().clone(); ml_reference_value78 }; let ml_callback77 = { let branch = branch; let condition = condition.clone(); let get = get; let jump = jump; let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, at: f64| -> self::JavaScriptCallInstruction { if (at == branch) {
                                                                                                                                                self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9142) => {
                                                                                                                                                        source_ml_source2_data_property_value9142
                                                                                                                                                    }
                                                                                                                                                }, (branch + 1f64), get)
                                                                                                                                            } else {
                                                                                                                                                if (at == jump) {
                                                                                                                                                    self::JavaScriptCallInstruction::Jump(end)
                                                                                                                                                } else {
                                                                                                                                                    instruction.clone()
                                                                                                                                                }
                                                                                                                                            } }) }; ml_values77.iter().cloned().enumerate().map(|(ml_index77, ml_value77)| ml_callback77(ml_value77, (ml_index77 as f64))).collect::<Vec<_>>() }));
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override9164.clone(), source_ml_source2_nominal_field9146, source_ml_source2_nominal_field9148, source_ml_source2_nominal_field9150, source_ml_source2_nominal_field9152, source_ml_source2_nominal_field9154.clone(), source_ml_source2_nominal_field9156.clone(), source_ml_source2_nominal_field9158.clone(), source_ml_source2_nominal_field9160.clone(), source_ml_source2_nominal_field9162.clone())
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
                                                                                            };
                                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field9165.clone()), source_ml_source2_constructor_field9140)
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
        self::JavaScriptCallSource::SourceGet(source_object, source_key_2, source_receiver) => {
            let source_object = *source_object;
            let source_key_2 = *source_key_2;
            {
                let object = self::lower_java_script_call_source_expression(source_object.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let key_2 = self::lower_java_script_call_source_expression(source_key_2.clone(), bindings.clone(), match object.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9166, _) => {
                            let source_ml_source2_data_property_value9166 = *source_ml_source2_data_property_value9166;
                            source_ml_source2_data_property_value9166.clone()
                        }
                    }, (depth + 1f64));
                    {
                        let supplied_receiver = ((source_receiver.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSource>,))).0).clone();
                        {
                            let receiver = if match (supplied_receiver.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                {
                                    let source_ml_source2_constructor_field9168 = match object.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9167) => {
                                            source_ml_source2_data_property_value9167
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field9170 = match key_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9169, _) => {
                                                let source_ml_source2_data_property_value9169 = *source_ml_source2_data_property_value9169;
                                                source_ml_source2_data_property_value9169.clone()
                                            }
                                        };
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field9170.clone()), source_ml_source2_constructor_field9168)
                                    }
                                }
                            } else {
                                self::lower_java_script_call_source_expression(supplied_receiver.clone().expect("a proven present value"), bindings.clone(), match key_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9171, _) => {
                                        let source_ml_source2_data_property_value9171 = *source_ml_source2_data_property_value9171;
                                        source_ml_source2_data_property_value9171.clone()
                                    }
                                }, (depth + 1f64))
                            };
                            {
                                let result_2 = self::allocate_java_script_call_source_register(match receiver.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9172, _) => {
                                        let source_ml_source2_data_property_value9172 = *source_ml_source2_data_property_value9172;
                                        source_ml_source2_data_property_value9172.clone()
                                    }
                                });
                                {
                                    let source_ml_source2_nominal_field9179 = match result_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9180, _) => {
                                            let source_ml_source2_data_property_value9180 = *source_ml_source2_data_property_value9180;
                                            source_ml_source2_data_property_value9180.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field9181 = match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9182) => {
                                                source_ml_source2_data_property_value9182
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override9183 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9173, _) => {
                                                    let source_ml_source2_data_property_value9173 = *source_ml_source2_data_property_value9173;
                                                    source_ml_source2_data_property_value9173.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinaryGet(match result_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9174) => {
                                                    source_ml_source2_data_property_value9174
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9175) => {
                                                    source_ml_source2_data_property_value9175
                                                }
                                            }, match key_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9176) => {
                                                    source_ml_source2_data_property_value9176
                                                }
                                            }, match receiver.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9177) => {
                                                    source_ml_source2_data_property_value9177
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override9183.clone()), source_ml_source2_nominal_field9181)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceDelete(source_object_2, source_key_3) => {
            let source_object_2 = *source_object_2;
            let source_key_3 = *source_key_3;
            {
                let raw_ = self::lower_java_script_call_source_expression(source_object_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let object_2 = self::allocate_java_script_call_source_register(match raw_.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9184, _) => {
                            let source_ml_source2_data_property_value9184 = *source_ml_source2_data_property_value9184;
                            source_ml_source2_data_property_value9184.clone()
                        }
                    });
                    {
                        let saved_2 = self::append_java_script_call_source_instruction(match object_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9185, _) => {
                                let source_ml_source2_data_property_value9185 = *source_ml_source2_data_property_value9185;
                                source_ml_source2_data_property_value9185.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match object_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9186) => {
                                source_ml_source2_data_property_value9186
                            }
                        }, match raw_.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9187) => {
                                source_ml_source2_data_property_value9187
                            }
                        }));
                        {
                            let key_3 = self::lower_java_script_call_source_expression(source_key_3.clone(), bindings.clone(), saved_2.clone(), (depth + 1f64));
                            {
                                let result_3 = self::allocate_java_script_call_source_register(match key_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9188, _) => {
                                        let source_ml_source2_data_property_value9188 = *source_ml_source2_data_property_value9188;
                                        source_ml_source2_data_property_value9188.clone()
                                    }
                                });
                                {
                                    let current = self::append_java_script_call_source_instruction(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9189, _) => {
                                            let source_ml_source2_data_property_value9189 = *source_ml_source2_data_property_value9189;
                                            source_ml_source2_data_property_value9189.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::OrdinaryDelete(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9374) => {
                                            source_ml_source2_data_property_value9374
                                        }
                                    }, match object_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9375) => {
                                            source_ml_source2_data_property_value9375
                                        }
                                    }, match key_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9376) => {
                                            source_ml_source2_data_property_value9376
                                        }
                                    }));
                                    {
                                        let current_3 = {
                                            let checked = self::allocate_java_script_call_source_register(current.clone());
                                            {
                                                let current_2 = self::append_java_script_call_source_instruction(match checked.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9564, _) => {
                                                        let source_ml_source2_data_property_value9564 = *source_ml_source2_data_property_value9564;
                                                        source_ml_source2_data_property_value9564.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Primitive(match checked.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9565) => {
                                                        source_ml_source2_data_property_value9565
                                                    }
                                                }, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 100, 101, 108, 101, 116, 101, 32, 115, 117, 99, 99, 101, 115, 115]), std::rc::Rc::new(std::cell::RefCell::new(vec![match result_3.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9566) => {
                                                        source_ml_source2_data_property_value9566
                                                    }
                                                }]))));
                                                current_2.clone()
                                            }
                                        };
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(current_3.clone()), match result_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9567) => {
                                                source_ml_source2_data_property_value9567
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
        self::JavaScriptCallSource::SourceHas(source_object_3, source_key_4) => {
            let source_object_3 = *source_object_3;
            let source_key_4 = *source_key_4;
            {
                let raw_2 = self::lower_java_script_call_source_expression(source_object_3.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let object_3 = self::allocate_java_script_call_source_register(match raw_2.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9568, _) => {
                            let source_ml_source2_data_property_value9568 = *source_ml_source2_data_property_value9568;
                            source_ml_source2_data_property_value9568.clone()
                        }
                    });
                    {
                        let saved_3 = self::append_java_script_call_source_instruction(match object_3.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9569, _) => {
                                let source_ml_source2_data_property_value9569 = *source_ml_source2_data_property_value9569;
                                source_ml_source2_data_property_value9569.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match object_3.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9570) => {
                                source_ml_source2_data_property_value9570
                            }
                        }, match raw_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9571) => {
                                source_ml_source2_data_property_value9571
                            }
                        }));
                        {
                            let key_4 = self::lower_java_script_call_source_expression(source_key_4.clone(), bindings.clone(), saved_3.clone(), (depth + 1f64));
                            {
                                let result_4 = self::allocate_java_script_call_source_register(match key_4.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9572, _) => {
                                        let source_ml_source2_data_property_value9572 = *source_ml_source2_data_property_value9572;
                                        source_ml_source2_data_property_value9572.clone()
                                    }
                                });
                                {
                                    let current_4 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9573, _) => {
                                            let source_ml_source2_data_property_value9573 = *source_ml_source2_data_property_value9573;
                                            source_ml_source2_data_property_value9573.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::OrdinaryHas(match result_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9761) => {
                                            source_ml_source2_data_property_value9761
                                        }
                                    }, match object_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9762) => {
                                            source_ml_source2_data_property_value9762
                                        }
                                    }, match key_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9763) => {
                                            source_ml_source2_data_property_value9763
                                        }
                                    }));
                                    {
                                        let current_5 = current_4.clone();
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(current_5.clone()), match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9948) => {
                                                source_ml_source2_data_property_value9948
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
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}
