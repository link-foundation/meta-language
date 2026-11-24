// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Result;
use super::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::source_ml_source1_temporary_order_java_script_call_code_functions_loop8;
use super::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1;

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSource {
    SourceConstant(Box<self::JavaScriptSemanticValue>),
    SourceNewTarget,
    SourceReceiver,
    SourceArgument(f64),
    SourceDerivedReceiver,
    SourceErrorIntrinsic,
    SourceConstruct(Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceConstructErrorClass(Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceClassConstructorCall(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceConstructCollection(Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceVariadicCall(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, self::ml_string::Text, bool),
    SourceArgumentSpread(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, self::ml::Big),
    SourceConstructErrorValue(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, self::ml::Big, self::ml_string::Text, bool),
    SourceThrow(Box<self::JavaScriptCallSource>),
    SourceReturn(Box<self::JavaScriptCallSource>),
    SourceAssignBinding(f64, Box<self::JavaScriptCallSource>),
    SourceImmutableBindingWrite(Box<self::JavaScriptCallSource>),
    SourceWhile(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Option<(Option<f64>,)>, Option<(Option<self::JavaScriptCallSource>,)>, Option<(Option<bool>,)>, Option<(Option<self::JavaScriptCallSource>,)>),
    SourceCloneBindingCell(f64, self::ml::Big, Option<(Option<bool>,)>),
    SourceControlJump(f64, bool),
    SourceControlBlock(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceSwitch(f64, Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>, Box<self::JavaScriptCallSource>),
    SourceBindingScope(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceLexicalScope(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, self::ml::Big, Box<self::JavaScriptCallSource>),
    SourceWriteExisting(f64, bool, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceInitializeCell(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceInitializeBinding(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceIteratorBinding(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, self::ml::Big),
    SourceIteratorStep(f64, self::ml::Big, bool),
    SourceIteratorRemainder(f64, Box<self::JavaScriptCallSource>, self::ml::Big),
    SourceForOf(f64, f64, f64, bool, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, self::ml::Big, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>),
    SourceIteratorLoop(f64, f64, f64, bool, Box<self::JavaScriptCallSource>, self::ml::Big, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>),
    SourceIteratorClose(f64, self::ml::Big),
    SourceFinally(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceTry(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceBinding(f64),
    SourceLet(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceLocalLet(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, bool),
    SourceArgumentRest(f64, Box<self::JavaScriptCallSource>, self::ml::Big),
    SourceParameterRest(f64, f64, Box<self::JavaScriptCallSource>, self::ml::Big, Box<self::JavaScriptCallSource>),
    SourceParameterDefault(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceIf(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceNumberBinary(self::ml_string::Text, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceSameValue(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceStrictEqual(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceSnapshotStrictEqual(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourcePrimitive(self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceRecur(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceAllocate(Box<self::JavaScriptCallSource>),
    SourceClosure(self::ml::Big, f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, Box<self::JavaScriptCallSource>),
    SourceObject(Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceProperty>>>, self::ml::Big),
    SourceArrayAccumulation(Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArrayPart>>>, Box<self::JavaScriptCallSource>, self::ml::Big),
    SourceArray(Box<self::JavaScriptCallSource>, f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceArrayElement>>>, self::ml::Big),
    SourcePropertyKey(Box<self::JavaScriptCallSource>, self::ml::Big),
    SourcePropertyDefinition(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, self::ml::Big),
    SourceSet(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceSnapshotSet(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceStringGet(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceGet(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Option<(Option<self::JavaScriptCallSource>,)>),
    SourceInstanceOf(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceDelete(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceHas(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceCall(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceSnapshotCall(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceProperty {
    SourceObjectProperty(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Option<(Option<bool>,)>),
    SourceObjectAccessor(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, self::ml_string::Text, Option<(Option<bool>,)>),
    SourceLiteralPrototype(Box<self::JavaScriptCallSource>),
    SourceObjectSpread(Box<self::JavaScriptCallSource>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallArrayPart {
    SourceArrayValue(Box<self::JavaScriptCallSource>),
    SourceArraySpread(Box<self::JavaScriptCallSource>),
    SourceArrayHole,
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceArrayElement {
    SourceArrayElement(f64, Box<self::JavaScriptCallSource>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceSwitchClause {
    SourceSwitchCase(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceSwitchDefault(Box<self::JavaScriptCallSource>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceControlScope {
    SourceControlScope(f64, self::ml_string::Text, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceControlTransfer {
    SourceControlTransfer(f64, f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallCellInitializer {
    SourceCellInitializer(f64, Box<self::JavaScriptCallSource>, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallBindingDeclaration {
    SourceBindingDeclaration(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceBinding {
    SourceBindingRegister(f64, f64, bool, Option<(Option<bool>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceBuilder {
    SourceBuilder(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, f64, f64, f64, f64, self::ml_string::Text, Option<(Option<bool>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceResult {
    SourceResult(Box<self::JavaScriptCallSourceBuilder>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceCompilation {
    SourceCompiled(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, f64, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorExpression {
    ErrorConstant(Box<self::JavaScriptSemanticValue>),
    ErrorClosure(f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, Option<(Option<self::ml_string::Text>,)>),
    ErrorArgumentSpread(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>),
    ErrorInstanceOf(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>),
    ErrorDelete(Box<self::JavaScriptErrorExpression>),
    ErrorIn(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>),
    ErrorBinding(f64, Option<(Option<bool>,)>),
    ErrorModuleBinding(f64),
    ErrorThis,
    ErrorConditional(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>),
    ErrorGet(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>),
    ErrorComputedGet(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>),
    ErrorComputedSet(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>),
    ErrorComputedMethodCall(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>),
    ErrorOptionalGet(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>),
    ErrorSet(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>),
    ErrorModuleCell(f64),
    ErrorModuleCellSet(f64, Box<self::JavaScriptErrorExpression>),
    ErrorUnary(self::ml_string::Text, Box<self::JavaScriptErrorExpression>),
    ErrorUpdate(self::ml_string::Text, Box<self::JavaScriptErrorExpression>, bool),
    ErrorCompoundAssignment(self::ml_string::Text, Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>),
    ErrorBinary(self::ml_string::Text, Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>, self::ml_string::Text),
    ErrorArray(self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>),
    ErrorArrayAccumulation(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayPart>>>, Box<self::JavaScriptErrorExpression>),
    ErrorArrayLiteral(f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayElement>>>),
    ErrorObject(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorObjectProperty>>>),
    ErrorTemplate(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorTemplatePart>>>),
    ErrorCall(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>, bool),
    ErrorMethodCall(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>),
    ErrorFunctionReference(f64),
    ErrorNativeReference(self::ml_string::Text),
    ErrorNewTarget,
    ErrorGenericConstruction(Box<self::JavaScriptErrorExpression>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>),
    ErrorReceiver,
    ErrorOrdinaryConstruction(f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>),
    ErrorCollectionConstruction(self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>),
    ErrorBindingSet(f64, Box<self::JavaScriptErrorExpression>, Option<(Option<bool>,)>),
    ErrorSpecialNumber(self::ml_string::Text),
    ErrorStrictEqual(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorObjectProperty {
    ErrorObjectProperty(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, Box<self::JavaScriptErrorExpression>),
    ErrorObjectAccessor(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, Box<self::JavaScriptErrorExpression>, self::ml_string::Text),
    ErrorLiteralPrototype(Box<self::JavaScriptErrorExpression>),
    ErrorComputedObjectProperty(Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>, Option<(Option<bool>,)>, Option<(Option<self::ml_string::Text>,)>),
    ErrorObjectSpread(Box<self::JavaScriptErrorExpression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorArrayPart {
    ErrorArrayValue(Box<self::JavaScriptErrorExpression>),
    ErrorArraySpread(Box<self::JavaScriptErrorExpression>),
    ErrorArrayHole,
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorArrayElement {
    ErrorArrayElement(f64, Box<self::JavaScriptErrorExpression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorTemplatePart {
    ErrorTemplateText(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    ErrorTemplateValue(Box<self::JavaScriptErrorExpression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSourceBindingTarget {
    SourceCatchBindingTarget(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorParameterField {
    ErrorParameterField(f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bool, Box<self::JavaScriptErrorExpression>, Option<(Option<self::JavaScriptErrorExpression>,)>, Option<(Option<self::JavaScriptErrorParameter>,)>, Option<(Option<self::JavaScriptSourceBindingTarget>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorParameter {
    ErrorRestParameter(f64, Option<(Option<self::JavaScriptErrorParameter>,)>, Option<(Option<bool>,)>),
    ErrorRequiredParameter(f64, Option<(Option<bool>,)>),
    ErrorDefaultParameter(f64, Box<self::JavaScriptErrorExpression>, Option<(Option<bool>,)>),
    ErrorObjectParameter(f64, bool, Box<self::JavaScriptErrorExpression>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>, Option<(Option<f64>,)>, Option<(Option<self::JavaScriptSourceBindingTarget>,)>),
    ErrorArrayParameter(f64, bool, Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayBindingPart>>>, Option<(Option<self::JavaScriptErrorArrayBindingElement>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorArrayBindingElement {
    ErrorArrayBindingElement(f64, bool, Box<self::JavaScriptErrorExpression>, Option<(Option<self::JavaScriptErrorParameter>,)>, Option<(Option<self::JavaScriptSourceBindingTarget>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorArrayBindingPart {
    ErrorArrayBindingElement(f64, bool, Box<self::JavaScriptErrorExpression>, Option<(Option<self::JavaScriptErrorParameter>,)>, Option<(Option<self::JavaScriptSourceBindingTarget>,)>),
    ErrorArrayElision,
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSourceStatement {
    SourceForOfStatement(f64, f64, bool, Option<(Option<bool>,)>, Option<(Option<self::JavaScriptSourceBindingTarget>,)>, Box<self::JavaScriptErrorExpression>, Box<self::JavaScriptErrorExpression>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, Option<(Option<self::JavaScriptErrorParameter>,)>),
    SourceFinallyStatement(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>),
    SourceTryStatement(f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, Option<(Option<self::JavaScriptErrorParameter>,)>),
    SourceArrayBindingStatement(Box<self::JavaScriptErrorParameter>, Box<self::JavaScriptErrorExpression>, bool, Option<(Option<bool>,)>),
    SourceObjectBindingStatement(Box<self::JavaScriptErrorParameter>, Box<self::JavaScriptErrorExpression>, bool, Option<(Option<bool>,)>),
    SourceVariableStatement(f64, Box<self::JavaScriptErrorExpression>, bool, Option<(Option<self::JavaScriptSourceBindingTarget>,)>),
    SourceConstStatement(f64, Box<self::JavaScriptErrorExpression>),
    SourceMutableStatement(f64, Box<self::JavaScriptErrorExpression>, Option<(Option<bool>,)>),
    SourceExpressionStatement(Box<self::JavaScriptErrorExpression>),
    SourceReturnStatement(Box<self::JavaScriptErrorExpression>),
    SourceThrowStatement(Box<self::JavaScriptErrorExpression>),
    SourceIfStatement(Box<self::JavaScriptErrorExpression>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>),
    SourceBlockStatement(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>),
    SourceDeclarationGroupStatement(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>),
    SourceWhileStatement(Box<self::JavaScriptErrorExpression>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, Option<(Option<f64>,)>, Option<(Option<self::JavaScriptErrorExpression>,)>, Option<(Option<bool>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>),
    SourceControlBlockStatement(f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>),
    SourceControlJumpStatement(f64, bool),
    SourceSwitchStatement(f64, Box<self::JavaScriptErrorExpression>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceSwitchClause>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSourceSwitchClause {
    SourceCaseStatement(Box<self::JavaScriptErrorExpression>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>),
    SourceDefaultStatement(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorFieldAssignment {
    ErrorFieldAssignment(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, Box<self::JavaScriptErrorExpression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorFunctionPlan {
    ErrorConstructorPlan(f64, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameter>>>, Box<self::JavaScriptErrorExpression>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorFieldAssignment>>>),
    ErrorFunctionPlan(f64, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameter>>>, Box<self::JavaScriptErrorExpression>, bool),
    ErrorOrdinaryConstructorPlan(f64, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameter>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>),
    ErrorMethodFunctionPlan(f64, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameter>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>),
    ErrorMutableStatementFunctionPlan(f64, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameter>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>),
    ErrorStatementFunctionPlan(f64, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameter>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorArrowTemplatePlan {
    ErrorArrowTemplatePlan(f64, f64, self::ml_string::Text, f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameter>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, Option<(Option<self::ml_string::Text>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSourceLexicalBinding {
    SourceLexicalBinding(f64, self::ml_string::Text, bool, Option<(Option<bool>,)>, Option<(Option<f64>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorModulePlan {
    ErrorModulePlan(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorFunctionPlan>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, f64, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrowTemplatePlan>>>>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorFunctionOwner {
    ErrorConstructorOwner(f64, self::ml::Big, self::ml::Big, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, self::ml::Big),
    OrdinaryConstructorOwner(f64, self::ml::Big, self::ml::Big, Box<self::JavaScriptSemanticValue>),
    ErrorFunctionOwner(f64, self::ml::Big, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorRealmOperations {
    ErrorRealmOperations(self::ml::Big, Box<self::JavaScriptSemanticValue>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaScriptErrorIntrinsicOwner {
    ErrorIntrinsicOwner(self::ml::Big, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorModuleRegistry {
    ErrorModuleRegistry(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorFunctionOwner>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorRealmOperations>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorIntrinsicOwner>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaScriptSourceCapabilityOwner {
    SourceCapabilityOwner(self::ml_string::Text, self::ml::Big, self::ml::Big, self::ml_string::Text, self::ml_string::Text, bool, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorSourceContext {
    ErrorSourceContext(Box<self::JavaScriptErrorModuleRegistry>, Box<self::JavaScriptErrorRealmOperations>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceCapabilityOwner>>>, f64, f64, self::ml_string::Text, bool, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorSourceExpression {
    ErrorSourceExpression(Box<self::JavaScriptCallSource>, Box<self::JavaScriptErrorSourceContext>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorModuleCompilation {
    ErrorModuleCompilation(Box<self::JavaScriptCallProgram>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptErrorModuleBinding {
    ErrorModuleBindingValue(f64, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSourceReferenceCapture {
    SourceReferenceCapture(f64, Box<self::JavaScriptCallSource>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSourceReference {
    SourceReference(self::ml_string::Text, f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceReferenceCapture>>>, Box<self::JavaScriptErrorSourceContext>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPatternBindingDeclarations {
    BindingDeclarations(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_text_profile_error(profile: &self::ml_string::Text, length: f64) -> self::ml_string::Text {
    if ((*profile) != (self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]))) {
        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101])
    } else {
        if (((!(self::ml_math::is_safe_integer(length)) || (length < (0f64))) || (length > (16777216f64))) || ((length == (0f64)) && (((1f64 / length)) < (0f64)))) {
            self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 49, 54, 55, 55, 55, 50, 49, 54, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_value_error(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Text(units) => {
            if (({ let ml_reference_array1 = units.clone(); let ml_reference_projection1 = ml_reference_array1.borrow().len() as f64; ml_reference_projection1 }) > (1048576f64)) {
                self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s5 = self::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(units.clone(), at);
                        match source_ml_source2_s5.clone() {
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(profile, text) => {
            self::read_java_script_source_text_profile_error(&(profile), text.length())
        }
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if ((identity > (self::ml::Big::from_i128(0))) && (identity <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, _) => {
            if ((identity_2 > (self::ml::Big::from_i128(0))) && (identity_2 <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
        _ => {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_coordinate(value: f64, limit: f64) -> bool {
    (((self::ml_math::is_safe_integer(value) && (value >= (0f64))) && (value < limit)) && ((value != (0f64)) || (((1f64 / value)) > (0f64))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_code_coordinate(value: f64, limit: f64) -> bool {
    self::test_java_script_source_coordinate(value, limit)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn order_java_script_call_code_functions(functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>> {
    {
        let ordered = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value2 = functions.borrow().clone(); ml_reference_value2 }, 0f64, f64::INFINITY)));
        {
            let width = 1f64;
            {
                let source_ml_source2_s402 = self::source_ml_source1_temporary_order_java_script_call_code_functions_loop8(functions.clone(), ordered.clone(), width);
                match source_ml_source2_s402.clone() {
                    self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Done(ordered_2) => {
                        ordered_2.clone()
                    }
                    self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Return(source_ml_source1_temporary_result8) => {
                        source_ml_source1_temporary_result8.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_constant_error(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    self::read_java_script_semantic_value_error(value.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_exception_class(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114])
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114]))) {
            self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) {
                self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
            } else {
                if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114]))) {
                    self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
                } else {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]))) {
                        self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 85, 82, 73, 69, 114, 114, 111, 114]))) {
                            self::ml_string::Text::from_units(vec![85, 82, 73, 69, 114, 114, 111, 114])
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 118, 97, 108, 69, 114, 114, 111, 114]))) {
                                self::ml_string::Text::from_units(vec![69, 118, 97, 108, 69, 114, 114, 111, 114])
                            } else {
                                self::ml_string::Text::from_units(vec![])
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_arity(operation: &self::ml_string::Text) -> f64 {
    if ((((*operation) == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 105, 110, 116, 101, 103, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115])))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
        1f64
    } else {
        if self::ml_array::includes(&{ let ml_reference_value3 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 97, 100, 105, 120, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![119, 114, 97, 112, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 97, 112, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116]), self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100])]))).borrow().clone(); ml_reference_value3 }, operation, 0f64, false, |a, b| a == b) {
            2f64
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_numeric_parsing_arity(operation: &self::ml_string::Text) -> f64 {
    if ((*operation) == (self::ml_string::Text::from_units(vec![112, 97, 114, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 116, 101, 120, 116, 32, 112, 114, 101, 102, 105, 120]))) {
        2f64
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![112, 97, 114, 115, 101, 32, 102, 108, 111, 97, 116, 32, 116, 101, 120, 116, 32, 112, 114, 101, 102, 105, 120]))) {
            1f64
        } else {
            (-1f64)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_method_primitive_arity(operation: &self::ml_string::Text) -> f64 {
    if self::ml_array::includes(&{ let ml_reference_value4 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![99, 111, 112, 121, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114, 32, 99, 111, 100, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116, 32, 116, 101, 120, 116])]))).borrow().clone(); ml_reference_value4 }, operation, 0f64, false, |a, b| a == b) {
        1f64
    } else {
        if self::ml_array::includes(&{ let ml_reference_value5 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![109, 97, 112, 32, 108, 111, 119, 101, 114, 32, 83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101]), self::ml_string::Text::from_units(vec![109, 97, 112, 32, 117, 112, 112, 101, 114, 32, 83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 83, 116, 114, 105, 110, 103, 32, 114, 101, 99, 101, 105, 118, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 83, 116, 114, 105, 110, 103, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 107, 101, 121]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 107, 101, 121])]))).borrow().clone(); ml_reference_value5 }, operation, 0f64, false, |a, b| a == b) {
            2f64
        } else {
            if self::ml_array::includes(&{ let ml_reference_value6 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 105, 110, 103, 32, 83, 116, 114, 105, 110, 103, 32, 117, 110, 105, 116, 115]), self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 32, 83, 116, 114, 105, 110, 103, 32, 117, 110, 105, 116, 115]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 97, 116]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 115, 117, 102, 102, 105, 120, 32, 97, 116]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110, 32, 97, 116])]))).borrow().clone(); ml_reference_value6 }, operation, 0f64, false, |a, b| a == b) {
                3f64
            } else {
                (-1f64)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_unary_number_operation(operation: &self::ml_string::Text) -> bool {
    self::ml_array::includes(&{ let ml_reference_value7 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 78, 97, 78, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 102, 105, 110, 105, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 101, 105, 108, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 98, 101, 114, 32, 115, 105, 103, 110]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 113, 117, 97, 114, 101, 32, 114, 111, 111, 116]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114])]))).borrow().clone(); ml_reference_value7 }, operation, 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_primitive_arity_error(operation: &self::ml_string::Text, count: f64) -> self::ml_string::Text {
    {
        let parsing_arity = self::read_java_script_numeric_parsing_arity(operation);
        if (parsing_arity >= (0f64)) {
            if (count == parsing_arity) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 97, 114, 115, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 118, 101, 114, 121, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 111, 112, 101, 114, 97, 110, 100])
            }
        } else {
            {
                let string_arity = self::read_java_script_string_method_primitive_arity(operation);
                if (string_arity >= (0f64)) {
                    if (count == string_arity) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 111, 112, 101, 114, 97, 110, 100])
                    }
                } else {
                    if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || (count > (4096f64))) {
                        self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 97, 108, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 98, 117, 100, 103, 101, 116])
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119, 32, 114, 101, 115, 116, 114, 105, 99, 116, 101, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 97, 99, 99, 101, 115, 115]))) {
                            if (count == (0f64)) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                self::ml_string::Text::from_units(vec![114, 101, 115, 116, 114, 105, 99, 116, 101, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 97, 99, 99, 101, 115, 115, 32, 105, 103, 110, 111, 114, 101, 115, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115])
                            }
                        } else {
                            if ((self::read_java_script_source_exception_class(operation)) != (self::ml_string::Text::from_units(vec![]))) {
                                if (count == (2f64)) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 109, 101, 115, 115, 97, 103, 101, 32, 97, 110, 100, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101])
                                }
                            } else {
                                {
                                    let integer_arity = self::read_java_script_call_integer_arity(operation);
                                    if (integer_arity != (0f64)) {
                                        if (count == integer_arity) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                                        }
                                    } else {
                                        if ((((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 116, 101, 120, 116, 32, 117, 110, 105, 116, 115])))) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            if self::test_java_script_call_unary_number_operation(operation) {
                                                if (count == (1f64)) {
                                                    self::ml_string::Text::from_units(vec![])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                                                }
                                            } else {
                                                {
                                                    let unary = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 86, 56, 32, 97, 114, 114, 97, 121, 32, 106, 111, 105, 110, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 118, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 98, 105, 110, 97, 114, 121, 51, 50, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 117, 110, 116, 32, 108, 101, 97, 100, 105, 110, 103, 32, 122, 101, 114, 111, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 105, 110, 100, 101, 120]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 97, 114, 114, 97, 121, 32, 108, 105, 107, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 79, 98, 106, 101, 99, 116, 32, 97, 115, 115, 105, 103, 110, 32, 116, 97, 114, 103, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 110, 111, 110, 110, 117, 108, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 110, 112, 117, 116]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101, 99, 116, 32, 102, 111, 114, 32, 105, 110]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 97, 114, 114, 97, 121, 32, 109, 101, 116, 104, 111, 100, 32, 111, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 114, 114, 97, 121, 32, 108, 105, 107, 101, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 100, 101, 108, 101, 116, 101, 32, 115, 117, 99, 99, 101, 115, 115]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 100, 100, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 77, 97, 112, 32, 101, 110, 116, 114, 121, 32, 111, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101, 99, 116, 32, 99, 111, 101, 114, 99, 105, 98, 108, 101]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 111, 98, 106, 101, 99, 116, 32, 118, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 101, 110, 100]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 110, 117, 109, 98, 101, 114, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![100, 101, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 101, 103, 101, 114])]));
                                                    {
                                                        let binary = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 114, 114, 97, 121, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 99, 111, 117, 110, 116]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 99, 104, 101, 99, 107, 101, 100, 32, 97, 114, 114, 97, 121, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 105, 110, 100, 101, 120]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 97, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 99, 104, 101, 99, 107, 101, 100, 32, 97, 114, 114, 97, 121, 32, 97, 112, 112, 101, 110, 100, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 114, 101, 108, 97, 116, 105, 111, 110, 97, 108, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 112, 101, 97, 116, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 112, 114, 101, 102, 105, 120]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 115, 117, 102, 102, 105, 120]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![97, 100, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101, 115])]));
                                                        {
                                                            let ternary = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116]), self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 101, 110, 100]), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 108, 97, 115, 116, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 97, 108, 108, 32, 116, 101, 120, 116])]));
                                                            {
                                                                let arity = if self::ml_array::includes(&{ let ml_reference_value8 = unary.borrow().clone(); ml_reference_value8 }, operation, 0f64, false, |a, b| a == b) {
                                                                    1f64
                                                                } else {
                                                                    if self::ml_array::includes(&{ let ml_reference_value9 = binary.borrow().clone(); ml_reference_value9 }, operation, 0f64, false, |a, b| a == b) {
                                                                        2f64
                                                                    } else {
                                                                        if self::ml_array::includes(&{ let ml_reference_value10 = ternary.borrow().clone(); ml_reference_value10 }, operation, 0f64, false, |a, b| a == b) {
                                                                            3f64
                                                                        } else {
                                                                            (-1f64)
                                                                        }
                                                                    }
                                                                };
                                                                if (arity < (0f64)) {
                                                                    self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
                                                                } else {
                                                                    if (count != arity) {
                                                                        self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![])
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_source_accessor_kind(property: self::JavaScriptCallSourceProperty) -> self::ml_string::Text {
    match property.clone() {
        self::JavaScriptCallSourceProperty::SourceObjectAccessor(_, _, kind, _) => {
            kind.clone()
        }
        _ => {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_call_source(builder: self::JavaScriptCallSourceBuilder, error: &self::ml_string::Text) -> self::JavaScriptCallSourceResult {
    self::JavaScriptCallSourceResult::SourceResult(Box::new({
        let source_ml_source2_nominal_field721 = match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value722, _, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value722.clone()
            }
        };
        {
            let source_ml_source2_nominal_field723 = match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value724, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value724
                }
            };
            {
                let source_ml_source2_nominal_field725 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value726, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value726
                    }
                };
                {
                    let source_ml_source2_nominal_field727 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value728, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value728
                        }
                    };
                    {
                        let source_ml_source2_nominal_field729 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value730, _, _, _, _, _) => {
                                source_ml_source2_data_property_value730
                            }
                        };
                        {
                            let source_ml_source2_nominal_field731 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value732, _, _, _, _) => {
                                    source_ml_source2_data_property_value732.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field733 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value734, _, _, _) => {
                                        source_ml_source2_data_property_value734.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field735 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value736, _, _) => {
                                            source_ml_source2_data_property_value736.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field737 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value738, _) => {
                                                source_ml_source2_data_property_value738.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field739 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value740) => {
                                                    source_ml_source2_data_property_value740.clone()
                                                }
                                            };
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field721.clone(), source_ml_source2_nominal_field723, source_ml_source2_nominal_field725, source_ml_source2_nominal_field727, source_ml_source2_nominal_field729, error.clone(), source_ml_source2_nominal_field733.clone(), source_ml_source2_nominal_field735.clone(), source_ml_source2_nominal_field737.clone(), source_ml_source2_nominal_field739.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }), 0f64)
}
