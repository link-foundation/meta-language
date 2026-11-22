// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_string;
use super::refuse_java_script_call_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_property_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceAllocate(source_prototype) => {
            let source_prototype = *source_prototype;
            {
                let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let result = self::allocate_java_script_call_source_register(match prototype.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8102, _) => {
                            let source_ml_source2_data_property_value8102 = *source_ml_source2_data_property_value8102;
                            source_ml_source2_data_property_value8102.clone()
                        }
                    });
                    {
                        let source_ml_source2_nominal_field8107 = match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8108, _) => {
                                let source_ml_source2_data_property_value8108 = *source_ml_source2_data_property_value8108;
                                source_ml_source2_data_property_value8108.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field8109 = match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8110) => {
                                    source_ml_source2_data_property_value8110
                                }
                            };
                            {
                                let source_ml_source2_nominal_override8111 = self::append_java_script_call_source_instruction(match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8103, _) => {
                                        let source_ml_source2_data_property_value8103 = *source_ml_source2_data_property_value8103;
                                        source_ml_source2_data_property_value8103.clone()
                                    }
                                }, self::JavaScriptCallInstruction::OrdinaryAllocate(match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8104) => {
                                        source_ml_source2_data_property_value8104
                                    }
                                }, match prototype.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8105) => {
                                        source_ml_source2_data_property_value8105
                                    }
                                }));
                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override8111.clone()), source_ml_source2_nominal_field8109)
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePropertyKey(source_input, realm) => {
            let source_input = *source_input;
            if ((realm <= (self::ml::Big::from_i128(0))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 114, 101, 97, 108, 109, 32, 105, 100, 101, 110, 116, 105, 116, 121])))
            } else {
                {
                    let input = self::lower_java_script_call_source_expression(source_input.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let result_2 = self::allocate_java_script_call_source_register(match input.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8112, _) => {
                                let source_ml_source2_data_property_value8112 = *source_ml_source2_data_property_value8112;
                                source_ml_source2_data_property_value8112.clone()
                            }
                        });
                        {
                            let source_ml_source2_nominal_field8117 = match result_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8118, _) => {
                                    let source_ml_source2_data_property_value8118 = *source_ml_source2_data_property_value8118;
                                    source_ml_source2_data_property_value8118.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field8119 = match result_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8120) => {
                                        source_ml_source2_data_property_value8120
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_override8121 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8113, _) => {
                                            let source_ml_source2_data_property_value8113 = *source_ml_source2_data_property_value8113;
                                            source_ml_source2_data_property_value8113.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::PropertyKey(match result_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8114) => {
                                            source_ml_source2_data_property_value8114
                                        }
                                    }, match input.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8115) => {
                                            source_ml_source2_data_property_value8115
                                        }
                                    }, realm.clone()));
                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override8121.clone()), source_ml_source2_nominal_field8119)
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(source_target, source_key, source_attributes, realm_2) => {
            let source_target = *source_target;
            let source_key = *source_key;
            let source_attributes = *source_attributes;
            if ((realm_2 <= (self::ml::Big::from_i128(0))) || (realm_2 > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 114, 101, 97, 108, 109, 32, 105, 100, 101, 110, 116, 105, 116, 121])))
            } else {
                {
                    let target = self::lower_java_script_call_source_expression(source_target.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let key = self::lower_java_script_call_source_expression(source_key.clone(), bindings.clone(), match target.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8122, _) => {
                                let source_ml_source2_data_property_value8122 = *source_ml_source2_data_property_value8122;
                                source_ml_source2_data_property_value8122.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let attributes = self::lower_java_script_call_source_expression(source_attributes.clone(), bindings.clone(), match key.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8123, _) => {
                                    let source_ml_source2_data_property_value8123 = *source_ml_source2_data_property_value8123;
                                    source_ml_source2_data_property_value8123.clone()
                                }
                            }, (depth + 1f64));
                            {
                                let result_3 = self::allocate_java_script_call_source_register(match attributes.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8124, _) => {
                                        let source_ml_source2_data_property_value8124 = *source_ml_source2_data_property_value8124;
                                        source_ml_source2_data_property_value8124.clone()
                                    }
                                });
                                {
                                    let source_ml_source2_nominal_field8136 = match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8137, _) => {
                                            let source_ml_source2_data_property_value8137 = *source_ml_source2_data_property_value8137;
                                            source_ml_source2_data_property_value8137.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field8138 = match result_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8139) => {
                                                source_ml_source2_data_property_value8139
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override8140 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8125, _) => {
                                                    let source_ml_source2_data_property_value8125 = *source_ml_source2_data_property_value8125;
                                                    source_ml_source2_data_property_value8125.clone()
                                                }
                                            }, {
                                                let source_ml_source2_constructor_field8127 = match result_3.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8126) => {
                                                        source_ml_source2_data_property_value8126
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_constructor_field8129 = match target.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8128) => {
                                                            source_ml_source2_data_property_value8128
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_constructor_field8131 = match key.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8130) => {
                                                                source_ml_source2_data_property_value8130
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field8133 = match attributes.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8132) => {
                                                                    source_ml_source2_data_property_value8132
                                                                }
                                                            };
                                                            self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source2_constructor_field8127, source_ml_source2_constructor_field8129, source_ml_source2_constructor_field8131, source_ml_source2_constructor_field8133, realm_2.clone(), None::<(Option<self::ml_string::Text>,)>)
                                                        }
                                                    }
                                                }
                                            });
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override8140.clone()), source_ml_source2_nominal_field8138)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceSet(source_object, source_key_2, source_value) => {
            let source_object = *source_object;
            let source_key_2 = *source_key_2;
            let source_value = *source_value;
            {
                let object = self::lower_java_script_call_source_expression(source_object.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let object_2 = object.clone();
                    {
                        let key_2 = self::lower_java_script_call_source_expression(source_key_2.clone(), bindings.clone(), match object_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8325, _) => {
                                let source_ml_source2_data_property_value8325 = *source_ml_source2_data_property_value8325;
                                source_ml_source2_data_property_value8325.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let key_3 = key_2.clone();
                            {
                                let value = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), match key_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8510, _) => {
                                        let source_ml_source2_data_property_value8510 = *source_ml_source2_data_property_value8510;
                                        source_ml_source2_data_property_value8510.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8511, _) => {
                                            let source_ml_source2_data_property_value8511 = *source_ml_source2_data_property_value8511;
                                            source_ml_source2_data_property_value8511.clone()
                                        }
                                    });
                                    {
                                        let write = self::append_java_script_call_source_instruction(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8512, _) => {
                                                let source_ml_source2_data_property_value8512 = *source_ml_source2_data_property_value8512;
                                                source_ml_source2_data_property_value8512.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::OrdinarySet(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8513) => {
                                                source_ml_source2_data_property_value8513
                                            }
                                        }, match object_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8514) => {
                                                source_ml_source2_data_property_value8514
                                            }
                                        }, match key_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8515) => {
                                                source_ml_source2_data_property_value8515
                                            }
                                        }, match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8516) => {
                                                source_ml_source2_data_property_value8516
                                            }
                                        }, match object_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8517) => {
                                                source_ml_source2_data_property_value8517
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8518) => {
                                                source_ml_source2_data_property_value8518
                                            }
                                        }))), match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8519) => {
                                                source_ml_source2_data_property_value8519
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
        self::JavaScriptCallSource::SourceSnapshotSet(source_object_2, source_key_3, source_value_2) => {
            let source_object_2 = *source_object_2;
            let source_key_3 = *source_key_3;
            let source_value_2 = *source_value_2;
            {
                let object_3 = self::lower_java_script_call_source_expression(source_object_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let object_5 = {
                        let saved = self::allocate_java_script_call_source_register(match object_3.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8704, _) => {
                                let source_ml_source2_data_property_value8704 = *source_ml_source2_data_property_value8704;
                                source_ml_source2_data_property_value8704.clone()
                            }
                        });
                        {
                            let object_4 = {
                                let source_ml_source2_nominal_field8709 = match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8710, _) => {
                                        let source_ml_source2_data_property_value8710 = *source_ml_source2_data_property_value8710;
                                        source_ml_source2_data_property_value8710.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field8711 = match saved.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8712) => {
                                            source_ml_source2_data_property_value8712
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override8713 = self::append_java_script_call_source_instruction(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8705, _) => {
                                                let source_ml_source2_data_property_value8705 = *source_ml_source2_data_property_value8705;
                                                source_ml_source2_data_property_value8705.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8706) => {
                                                source_ml_source2_data_property_value8706
                                            }
                                        }, match object_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8707) => {
                                                source_ml_source2_data_property_value8707
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override8713.clone()), source_ml_source2_nominal_field8711)
                                    }
                                }
                            };
                            object_4.clone()
                        }
                    };
                    {
                        let key_4 = self::lower_java_script_call_source_expression(source_key_3.clone(), bindings.clone(), match object_5.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8714, _) => {
                                let source_ml_source2_data_property_value8714 = *source_ml_source2_data_property_value8714;
                                source_ml_source2_data_property_value8714.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let key_6 = {
                                let saved_2 = self::allocate_java_script_call_source_register(match key_4.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8899, _) => {
                                        let source_ml_source2_data_property_value8899 = *source_ml_source2_data_property_value8899;
                                        source_ml_source2_data_property_value8899.clone()
                                    }
                                });
                                {
                                    let key_5 = {
                                        let source_ml_source2_nominal_field8904 = match saved_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8905, _) => {
                                                let source_ml_source2_data_property_value8905 = *source_ml_source2_data_property_value8905;
                                                source_ml_source2_data_property_value8905.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field8906 = match saved_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8907) => {
                                                    source_ml_source2_data_property_value8907
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override8908 = self::append_java_script_call_source_instruction(match saved_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8900, _) => {
                                                        let source_ml_source2_data_property_value8900 = *source_ml_source2_data_property_value8900;
                                                        source_ml_source2_data_property_value8900.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Copy(match saved_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8901) => {
                                                        source_ml_source2_data_property_value8901
                                                    }
                                                }, match key_4.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8902) => {
                                                        source_ml_source2_data_property_value8902
                                                    }
                                                }));
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override8908.clone()), source_ml_source2_nominal_field8906)
                                            }
                                        }
                                    };
                                    key_5.clone()
                                }
                            };
                            {
                                let value_2 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), match key_6.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8909, _) => {
                                        let source_ml_source2_data_property_value8909 = *source_ml_source2_data_property_value8909;
                                        source_ml_source2_data_property_value8909.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_5 = self::allocate_java_script_call_source_register(match value_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8910, _) => {
                                            let source_ml_source2_data_property_value8910 = *source_ml_source2_data_property_value8910;
                                            source_ml_source2_data_property_value8910.clone()
                                        }
                                    });
                                    {
                                        let write_2 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8911, _) => {
                                                let source_ml_source2_data_property_value8911 = *source_ml_source2_data_property_value8911;
                                                source_ml_source2_data_property_value8911.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::OrdinarySet(match result_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8912) => {
                                                source_ml_source2_data_property_value8912
                                            }
                                        }, match object_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8913) => {
                                                source_ml_source2_data_property_value8913
                                            }
                                        }, match key_6.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8914) => {
                                                source_ml_source2_data_property_value8914
                                            }
                                        }, match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8915) => {
                                                source_ml_source2_data_property_value8915
                                            }
                                        }, match object_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8916) => {
                                                source_ml_source2_data_property_value8916
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write_2.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8917) => {
                                                source_ml_source2_data_property_value8917
                                            }
                                        }))), match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8918) => {
                                                source_ml_source2_data_property_value8918
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
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInstanceOf(source_left, source_right, source_key_4) => {
            let source_left = *source_left;
            let source_right = *source_right;
            let source_key_4 = *source_key_4;
            {
                let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let held_left = self::allocate_java_script_call_source_register(match left.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8919, _) => {
                            let source_ml_source2_data_property_value8919 = *source_ml_source2_data_property_value8919;
                            source_ml_source2_data_property_value8919.clone()
                        }
                    });
                    {
                        let saved_left = self::append_java_script_call_source_instruction(match held_left.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8920, _) => {
                                let source_ml_source2_data_property_value8920 = *source_ml_source2_data_property_value8920;
                                source_ml_source2_data_property_value8920.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match held_left.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8921) => {
                                source_ml_source2_data_property_value8921
                            }
                        }, match left.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8922) => {
                                source_ml_source2_data_property_value8922
                            }
                        }));
                        {
                            let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings.clone(), saved_left.clone(), (depth + 1f64));
                            {
                                let held_right = self::allocate_java_script_call_source_register(match right.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8923, _) => {
                                        let source_ml_source2_data_property_value8923 = *source_ml_source2_data_property_value8923;
                                        source_ml_source2_data_property_value8923.clone()
                                    }
                                });
                                {
                                    let saved_right = self::append_java_script_call_source_instruction(match held_right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8924, _) => {
                                            let source_ml_source2_data_property_value8924 = *source_ml_source2_data_property_value8924;
                                            source_ml_source2_data_property_value8924.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Copy(match held_right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8925) => {
                                            source_ml_source2_data_property_value8925
                                        }
                                    }, match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8926) => {
                                            source_ml_source2_data_property_value8926
                                        }
                                    }));
                                    {
                                        let key_7 = self::lower_java_script_call_source_expression(source_key_4.clone(), bindings.clone(), saved_right.clone(), (depth + 1f64));
                                        {
                                            let result_6 = self::allocate_java_script_call_source_register(match key_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8927, _) => {
                                                    let source_ml_source2_data_property_value8927 = *source_ml_source2_data_property_value8927;
                                                    source_ml_source2_data_property_value8927.clone()
                                                }
                                            });
                                            {
                                                let source_ml_source2_nominal_field8934 = match result_6.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8935, _) => {
                                                        let source_ml_source2_data_property_value8935 = *source_ml_source2_data_property_value8935;
                                                        source_ml_source2_data_property_value8935.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field8936 = match result_6.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8937) => {
                                                            source_ml_source2_data_property_value8937
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override8938 = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8928, _) => {
                                                                let source_ml_source2_data_property_value8928 = *source_ml_source2_data_property_value8928;
                                                                source_ml_source2_data_property_value8928.clone()
                                                            }
                                                        }, self::JavaScriptCallInstruction::InstanceOf(match result_6.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8929) => {
                                                                source_ml_source2_data_property_value8929
                                                            }
                                                        }, match held_left.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8930) => {
                                                                source_ml_source2_data_property_value8930
                                                            }
                                                        }, match held_right.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8931) => {
                                                                source_ml_source2_data_property_value8931
                                                            }
                                                        }, match key_7.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8932) => {
                                                                source_ml_source2_data_property_value8932
                                                            }
                                                        }, self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102])));
                                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override8938.clone()), source_ml_source2_nominal_field8936)
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
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}
