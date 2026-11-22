// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceControlScope;
use super::JavaScriptCallSourceControlTransfer;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result;
use super::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop58;
use super::source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop59;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_branch(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceIf(source_condition, source_yes, source_no) => {
            let source_condition = *source_condition;
            let source_yes = *source_yes;
            let source_no = *source_no;
            {
                let condition = self::lower_java_script_call_source_expression(source_condition.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
                {
                    let result = self::allocate_java_script_call_source_register(match condition.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12523, _) => {
                            let source_ml_source2_data_property_value12523 = *source_ml_source2_data_property_value12523;
                            source_ml_source2_data_property_value12523.clone()
                        }
                    });
                    {
                        let branch = { let ml_reference_array119 = match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12524, _) => {
                                let source_ml_source2_data_property_value12524 = *source_ml_source2_data_property_value12524;
                                source_ml_source2_data_property_value12524.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12525, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value12525.clone()
                            }
                        }; let ml_reference_projection119 = ml_reference_array119.borrow().len() as f64; ml_reference_projection119 };
                        {
                            let start = self::append_java_script_call_source_instruction(match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12526, _) => {
                                    let source_ml_source2_data_property_value12526 = *source_ml_source2_data_property_value12526;
                                    source_ml_source2_data_property_value12526.clone()
                                }
                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12527) => {
                                    source_ml_source2_data_property_value12527
                                }
                            }, 0f64, 0f64));
                            {
                                let yes_position = { let ml_reference_array120 = match start.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12528, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12528.clone()
                                    }
                                }; let ml_reference_projection120 = ml_reference_array120.borrow().len() as f64; ml_reference_projection120 };
                                {
                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                    {
                                        let copied_yes = self::append_java_script_call_source_instruction(match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12529, _) => {
                                                let source_ml_source2_data_property_value12529 = *source_ml_source2_data_property_value12529;
                                                source_ml_source2_data_property_value12529.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12530) => {
                                                source_ml_source2_data_property_value12530
                                            }
                                        }, match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12531) => {
                                                source_ml_source2_data_property_value12531
                                            }
                                        }));
                                        {
                                            let jump = { let ml_reference_array121 = match copied_yes.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12532, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value12532.clone()
                                                }
                                            }; let ml_reference_projection121 = ml_reference_array121.borrow().len() as f64; ml_reference_projection121 };
                                            {
                                                let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                {
                                                    let no_position = { let ml_reference_array122 = match alternative.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12533, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12533.clone()
                                                        }
                                                    }; let ml_reference_projection122 = ml_reference_array122.borrow().len() as f64; ml_reference_projection122 };
                                                    {
                                                        let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings.clone(), if match (match alternative.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12534, _, _) => {
                                                                ((source_ml_source2_data_property_value12534.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                            }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            alternative.clone()
                                                        } else {
                                                            {
                                                                let source_ml_source2_nominal_field12538 = match alternative.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12539, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12539.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12540 = match alternative.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12541, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12541
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12542 = match alternative.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12543, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12543
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12544 = match alternative.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12545, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12545
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12546 = match alternative.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12547, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12547
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12548 = match alternative.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12549, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12549.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12550 = match alternative.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12551, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12551.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12552 = match alternative.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12553, _, _) => {
                                                                                                    source_ml_source2_data_property_value12553.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12554 = match alternative.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12555, _) => {
                                                                                                        source_ml_source2_data_property_value12555.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12556 = match alternative.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12557) => {
                                                                                                            source_ml_source2_data_property_value12557.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override12558 = (match match condition.clone() {
                                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12535, _) => {
                                                                                                                let source_ml_source2_data_property_value12535 = *source_ml_source2_data_property_value12535;
                                                                                                                source_ml_source2_data_property_value12535.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12536, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value12536.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12538.clone(), source_ml_source2_nominal_field12540, source_ml_source2_nominal_field12542, source_ml_source2_nominal_field12544, source_ml_source2_nominal_field12546, source_ml_source2_nominal_field12548.clone(), source_ml_source2_nominal_field12550.clone(), Some((Some(source_ml_source2_nominal_override12558.clone()),)), source_ml_source2_nominal_field12554.clone(), source_ml_source2_nominal_field12556.clone())
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
                                                        }, (depth + 1f64));
                                                        {
                                                            let copied_no = self::append_java_script_call_source_instruction(match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12559, _) => {
                                                                    let source_ml_source2_data_property_value12559 = *source_ml_source2_data_property_value12559;
                                                                    source_ml_source2_data_property_value12559.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12560) => {
                                                                    source_ml_source2_data_property_value12560
                                                                }
                                                            }, match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12561) => {
                                                                    source_ml_source2_data_property_value12561
                                                                }
                                                            }));
                                                            {
                                                                let end = { let ml_reference_array123 = match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12562, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12562.clone()
                                                                    }
                                                                }; let ml_reference_projection123 = ml_reference_array123.borrow().len() as f64; ml_reference_projection123 };
                                                                if ((match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12564, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12564.clone()
                                                                    }
                                                                }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                } else {
                                                                    {
                                                                        let instructions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values124 = { let ml_reference_value125 = (match copied_no.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12565, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12565.clone()
                                                                            }
                                                                        }).borrow().clone(); ml_reference_value125 }; let ml_callback124 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                            self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12566) => {
                                                                                    source_ml_source2_data_property_value12566
                                                                                }
                                                                            }, yes_position, no_position)
                                                                        } else {
                                                                            if (index == jump) {
                                                                                self::JavaScriptCallInstruction::Jump(end)
                                                                            } else {
                                                                                entry.clone()
                                                                            }
                                                                        } }) }; ml_values124.iter().cloned().enumerate().map(|(ml_index124, ml_value124)| ml_callback124(ml_value124, (ml_index124 as f64))).collect::<Vec<_>>() }));
                                                                        {
                                                                            let updated = {
                                                                                let source_ml_source2_nominal_field12568 = match copied_no.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12569, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12569.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12570 = match copied_no.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12571, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12571
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12572 = match copied_no.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12573, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12573
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12574 = match copied_no.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12575, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12575
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12576 = match copied_no.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12577, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value12577
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12578 = match copied_no.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12579, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value12579.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12580 = match copied_no.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12581, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value12581.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12582 = match copied_no.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12583, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12583.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12584 = match copied_no.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12585, _) => {
                                                                                                                        source_ml_source2_data_property_value12585.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12586 = match copied_no.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12587) => {
                                                                                                                            source_ml_source2_data_property_value12587.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(instructions.clone(), source_ml_source2_nominal_field12570, source_ml_source2_nominal_field12572, source_ml_source2_nominal_field12574, source_ml_source2_nominal_field12576, source_ml_source2_nominal_field12578.clone(), source_ml_source2_nominal_field12580.clone(), source_ml_source2_nominal_field12582.clone(), source_ml_source2_nominal_field12584.clone(), source_ml_source2_nominal_field12586.clone())
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
                                                                                let continuation = if (match (match match yes.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12589, _) => {
                                                                                        let source_ml_source2_data_property_value12589 = *source_ml_source2_data_property_value12589;
                                                                                        source_ml_source2_data_property_value12589.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12590, _, _) => {
                                                                                        ((source_ml_source2_data_property_value12590.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && match (match match no.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12591, _) => {
                                                                                        let source_ml_source2_data_property_value12591 = *source_ml_source2_data_property_value12591;
                                                                                        source_ml_source2_data_property_value12591.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12592, _, _) => {
                                                                                        ((source_ml_source2_data_property_value12592.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                    updated.clone()
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source2_nominal_spread12599 = updated.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12600 = match source_ml_source2_nominal_spread12599.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12601, _, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12601.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12602 = match source_ml_source2_nominal_spread12599.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12603, _, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value12603
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12604 = match source_ml_source2_nominal_spread12599.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12605, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value12605
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12606 = match source_ml_source2_nominal_spread12599.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12607, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value12607
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12608 = match source_ml_source2_nominal_spread12599.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12609, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12609
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12610 = match source_ml_source2_nominal_spread12599.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12611, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value12611.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12612 = match source_ml_source2_nominal_spread12599.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12613, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value12613.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12614 = match source_ml_source2_nominal_spread12599.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12615, _, _) => {
                                                                                                                                source_ml_source2_data_property_value12615.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12616 = match source_ml_source2_nominal_spread12599.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12617, _) => {
                                                                                                                                    source_ml_source2_data_property_value12617.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12618 = match source_ml_source2_nominal_spread12599.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12619) => {
                                                                                                                                        source_ml_source2_data_property_value12619.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override12620 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values126 = { let ml_reference_value127 = ((match match yes.clone() {
                                                                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12593, _) => {
                                                                                                                                            let source_ml_source2_data_property_value12593 = *source_ml_source2_data_property_value12593;
                                                                                                                                            source_ml_source2_data_property_value12593.clone()
                                                                                                                                        }
                                                                                                                                    } {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12594, _, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value12594.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value127 }; let ml_callback126 = { let no = no.clone(); std::rc::Rc::new(move |identity: f64| -> bool { self::ml_array::includes(&{ let ml_reference_value128 = ((match match no.clone() {
                                                                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12597, _) => {
                                                                                                                                            let source_ml_source2_data_property_value12597 = *source_ml_source2_data_property_value12597;
                                                                                                                                            source_ml_source2_data_property_value12597.clone()
                                                                                                                                        }
                                                                                                                                    } {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12598, _, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value12598.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value128 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) }) }; ml_values126.iter().cloned().enumerate().filter_map(|(ml_index126, ml_value126)| if ml_callback126(ml_value126.clone()) { Some(ml_value126) } else { None }).collect::<Vec<_>>() }));
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12600.clone(), source_ml_source2_nominal_field12602, source_ml_source2_nominal_field12604, source_ml_source2_nominal_field12606, source_ml_source2_nominal_field12608, source_ml_source2_nominal_field12610.clone(), source_ml_source2_nominal_field12612.clone(), Some((Some(source_ml_source2_nominal_override12620.clone()),)), source_ml_source2_nominal_field12616.clone(), source_ml_source2_nominal_field12618.clone())
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
                                                                                };
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(continuation.clone()), match result.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12621) => {
                                                                                        source_ml_source2_data_property_value12621
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
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source(source: self::JavaScriptCallSource, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::JavaScriptCallSourceCompilation {
    self::compile_java_script_call_source_with_captures(source.clone(), parameters.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source_with_captures(source: self::JavaScriptCallSource, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, captures: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::JavaScriptCallSourceCompilation {
    {
        let first = self::compile_java_script_call_source_binding_reads(source.clone(), parameters.clone(), captures.clone(), false);
        if ((match first.clone() {
            self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value12623) => {
                source_ml_source2_data_property_value12623.clone()
            }
        }) == (self::ml_string::Text::from_units(vec![109, 117, 116, 97, 98, 108, 101, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 99, 111, 109, 112, 105, 108, 97, 116, 105, 111, 110, 32, 112, 97, 115, 115]))) {
            self::compile_java_script_call_source_binding_reads(source.clone(), parameters.clone(), captures.clone(), true)
        } else {
            first.clone()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source_binding_reads(source: self::JavaScriptCallSource, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, captures: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, snapshot_mutable_bindings: bool) -> self::JavaScriptCallSourceCompilation {
    if ((({ let ml_reference_array129 = parameters.clone(); let ml_reference_projection129 = ml_reference_array129.borrow().len() as f64; ml_reference_projection129 } + { let ml_reference_array130 = captures.clone(); let ml_reference_projection130 = ml_reference_array130.borrow().len() as f64; ml_reference_projection130 })) >= (4096f64)) {
        self::JavaScriptCallSourceCompilation::SourceCompiled(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new())), 0f64, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 98, 117, 100, 103, 101, 116]))
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s12624 = self::source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop58(parameters.clone(), index);
                match source_ml_source2_s12624.clone() {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Done => {
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s12625 = self::source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop59(parameters.clone(), captures.clone(), index_2);
                                match source_ml_source2_s12625.clone() {
                                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Done => {
                                        {
                                            let bindings = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values131 = { let ml_reference_value132 = (std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value133 = parameters.borrow().clone(); ml_reference_value133 }), { let ml_reference_value134 = captures.borrow().clone(); ml_reference_value134 })))).borrow().clone(); ml_reference_value132 }; let ml_callback131 = { let parameters = parameters.clone(); std::rc::Rc::new(move |identity: f64, index_3: f64| -> self::JavaScriptCallSourceBinding { {
                                                let source_ml_source2_constructor_field12628 = identity;
                                                {
                                                    let source_ml_source2_constructor_field12629 = (index_3 + 1f64);
                                                    {
                                                        let source_ml_source2_constructor_field12630 = (index_3 < ({ let ml_reference_array135 = parameters.clone(); let ml_reference_projection135 = ml_reference_array135.borrow().len() as f64; ml_reference_projection135 }));
                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_constructor_field12628, source_ml_source2_constructor_field12629, source_ml_source2_constructor_field12630, None::<(Option<bool>,)>)
                                                    }
                                                }
                                            } }) }; ml_values131.iter().cloned().enumerate().map(|(ml_index131, ml_value131)| ml_callback131(ml_value131, (ml_index131 as f64))).collect::<Vec<_>>() }));
                                            {
                                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings.clone(), {
                                                    let source_ml_source1_temporary_constructor_field52 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()));
                                                    {
                                                        let source_ml_source1_temporary_constructor_field53 = (({ let ml_reference_array136 = parameters.clone(); let ml_reference_projection136 = ml_reference_array136.borrow().len() as f64; ml_reference_projection136 } + { let ml_reference_array137 = captures.clone(); let ml_reference_projection137 = ml_reference_array137.borrow().len() as f64; ml_reference_projection137 }) + 1f64);
                                                        {
                                                            let source_ml_source1_temporary_constructor_field54 = 0f64;
                                                            {
                                                                let source_ml_source1_temporary_constructor_field55 = { let ml_reference_array138 = parameters.clone(); let ml_reference_projection138 = ml_reference_array138.borrow().len() as f64; ml_reference_projection138 };
                                                                {
                                                                    let source_ml_source1_temporary_constructor_field56 = 0f64;
                                                                    if snapshot_mutable_bindings {
                                                                        {
                                                                            let source_ml_source2_constructor_field12637 = Some(true);
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_temporary_constructor_field52.clone(), source_ml_source1_temporary_constructor_field53, source_ml_source1_temporary_constructor_field54, source_ml_source1_temporary_constructor_field55, source_ml_source1_temporary_constructor_field56, self::ml_string::Text::from_units(vec![]), Some((source_ml_source2_constructor_field12637.clone(),)), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,)>)
                                                                        }
                                                                    } else {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_temporary_constructor_field52.clone(), source_ml_source1_temporary_constructor_field53, source_ml_source1_temporary_constructor_field54, source_ml_source1_temporary_constructor_field55, source_ml_source1_temporary_constructor_field56, self::ml_string::Text::from_units(vec![]), None::<(Option<bool>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,)>)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }, 0f64);
                                                {
                                                    let builder = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12644, _) => {
                                                            let source_ml_source2_data_property_value12644 = *source_ml_source2_data_property_value12644;
                                                            source_ml_source2_data_property_value12644.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Return(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12645) => {
                                                            source_ml_source2_data_property_value12645
                                                        }
                                                    }));
                                                    {
                                                        let builder_3 = if { let ml_value = { let ml_reference_array140 = (match builder.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12646) => {
                                                                ((source_ml_source2_data_property_value12646.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,))).0).clone()
                                                            }
                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlTransfer>::new()))); let ml_reference_projection140 = ml_reference_array140.borrow().len() as f64; ml_reference_projection140 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                            {
                                                                let builder_2 = {
                                                                    let source_ml_source2_nominal_field12648 = match builder.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12649, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12649.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12650 = match builder.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12651, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12651
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12652 = match builder.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12653, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12653
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12654 = match builder.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12655, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12655
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12656 = match builder.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12657, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12657
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12658 = match builder.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12659, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12659.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12660 = match builder.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12661, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12661.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12662 = match builder.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12663, _, _) => {
                                                                                                        source_ml_source2_data_property_value12663.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12664 = match builder.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12665, _) => {
                                                                                                            source_ml_source2_data_property_value12665.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12666 = match builder.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12667) => {
                                                                                                                source_ml_source2_data_property_value12667.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12648.clone(), source_ml_source2_nominal_field12650, source_ml_source2_nominal_field12652, source_ml_source2_nominal_field12654, source_ml_source2_nominal_field12656, self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 105, 108, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 114, 101, 115, 111, 108, 118, 101, 100, 32, 99, 111, 110, 116, 114, 111, 108, 32, 116, 114, 97, 110, 115, 102, 101, 114, 115]), source_ml_source2_nominal_field12660.clone(), source_ml_source2_nominal_field12662.clone(), source_ml_source2_nominal_field12664.clone(), source_ml_source2_nominal_field12666.clone())
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
                                                                builder_2.clone()
                                                            }
                                                        } else {
                                                            builder.clone()
                                                        };
                                                        self::JavaScriptCallSourceCompilation::SourceCompiled(if ((match builder_3.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12670, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12670.clone()
                                                            }
                                                        }) == (self::ml_string::Text::from_units(vec![]))) {
                                                            match builder_3.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12671, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12671.clone()
                                                                }
                                                            }
                                                        } else {
                                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()))
                                                        }, if ((match builder_3.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12673, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12673.clone()
                                                            }
                                                        }) == (self::ml_string::Text::from_units(vec![]))) {
                                                            match builder_3.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12674, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12674
                                                                }
                                                            }
                                                        } else {
                                                            0f64
                                                        }, match builder_3.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12675, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12675.clone()
                                                            }
                                                        })
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Return(source_ml_source1_temporary_result59) => {
                                        let source_ml_source1_temporary_result59 = *source_ml_source1_temporary_result59;
                                        source_ml_source1_temporary_result59.clone()
                                    }
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Return(source_ml_source1_temporary_result59_2) => {
                        let source_ml_source1_temporary_result59_2 = *source_ml_source1_temporary_result59_2;
                        source_ml_source1_temporary_result59_2.clone()
                    }
                }
            }
        }
    }
}
