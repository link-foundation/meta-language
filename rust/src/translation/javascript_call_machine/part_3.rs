// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCallSourceUnit;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_string;

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallInstruction {
    Constant(f64, Box<self::JavaScriptSemanticValue>),
    Copy(f64, f64),
    Argument(f64, f64),
    ArgumentCount(f64),
    ArgumentAt(f64, f64),
    FunctionBind(f64),
    BoundFunctionCall(f64),
    FunctionToString(f64),
    PushFinally(f64),
    FinishFinally,
    CompleteJump(f64, f64),
    Jump(f64),
    Branch(f64, f64, f64),
    Call(f64, f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    ArgumentListOpen(f64),
    ArgumentListAppend(f64, f64),
    ArgumentListCall(f64, f64, f64, f64, self::ml_string::Text),
    Construct(f64, f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    ConstructorCheck(f64, f64, bool),
    ReadNewTarget(f64),
    ArrayJoinEnter(f64, f64),
    ArrayJoinLeave(f64),
    ConstructErrorClass(f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    CallClassConstructor(f64, f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    ConstructCollection(f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    CollectionAllocate(f64, f64, self::ml_string::Text),
    CollectionIteratorAllocate(
        f64,
        f64,
        self::ml_string::Text,
        self::ml_string::Text,
        Option<(Option<f64>,)>,
    ),
    ArgumentRest(f64, f64, f64, self::ml::Big),
    IteratorProtocol(
        f64,
        f64,
        f64,
        f64,
        f64,
        self::ml_string::Text,
        self::ml::Big,
        Option<(Option<bool>,)>,
    ),
    CollectionIteratorResult(f64, self::ml_string::Text, f64, f64),
    CollectionOperation(f64, self::ml_string::Text),
    PrimitiveMethod(f64, self::ml_string::Text),
    CollectionForEach(f64, self::ml_string::Text),
    ArrayConcatenation(
        f64,
        Option<(Option<f64>,)>,
        f64,
        Box<self::JavaScriptArraySpeciesDirectory>,
    ),
    ArraySlice(
        f64,
        Option<(Option<f64>,)>,
        Box<self::JavaScriptArraySpeciesDirectory>,
    ),
    ArrayCreation(f64, self::ml_string::Text, f64, f64),
    ArrayCallback(f64, self::ml_string::Text, Option<(Option<f64>,)>),
    ArraySearch(f64, self::ml_string::Text, Option<(Option<f64>,)>),
    ArrayTransform(
        f64,
        self::ml_string::Text,
        Box<self::JavaScriptArraySpeciesDirectory>,
        Option<(Option<f64>,)>,
    ),
    ObjectEnumeration(f64, self::ml_string::Text, f64),
    PropertyDefinitions(f64, self::ml_string::Text, f64, f64),
    PropertyReflection(f64, self::ml_string::Text, f64, f64, f64, f64),
    InstanceOf(f64, f64, f64, f64, self::ml_string::Text),
    ObjectObservation(f64, self::ml_string::Text, f64, f64),
    CollectionCopy(f64, f64, f64, f64, f64, f64, f64, f64),
    ConstructErrorValue(
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        self::ml::Big,
        self::ml_string::Text,
        bool,
    ),
    ReadDerivedReceiver(f64),
    InvokeErrorIntrinsic(f64),
    InvokeHostCapability(f64),
    Return(f64),
    Throw(f64),
    PushHandler(f64, f64),
    PopHandler,
    NumberBinary(f64, f64, f64, self::ml_string::Text),
    SameValue(f64, f64, f64),
    StrictEqual(f64, f64, f64),
    RequireWrite(f64),
    Primitive(
        f64,
        self::ml_string::Text,
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ),
    OrdinaryAllocate(f64, f64),
    LiteralPrototype(f64, f64, f64),
    LiteralAccessor(f64, f64, f64, f64, self::ml_string::Text, self::ml::Big),
    ObjectFromEntries(f64, f64, f64, f64),
    PrimitiveWrapper(f64, f64, f64),
    SymbolAllocate(f64, f64),
    SymbolDescription(f64, f64),
    SymbolRegistry(f64, f64, self::ml_string::Text),
    FunctionName(f64, f64, f64, self::ml_string::Text, self::ml::Big),
    ArrayAllocate(f64, f64, f64),
    ClosureAllocate(
        f64,
        self::ml::Big,
        f64,
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
        f64,
    ),
    ArraySpread(f64, f64, f64, f64, f64, self::ml::Big),
    ObjectSpread(
        f64,
        f64,
        f64,
        self::ml::Big,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>,
        Option<(Option<self::ml_string::Text>,)>,
    ),
    OrdinaryDataProperty(f64, f64, f64, f64, self::ml::Big),
    OrdinaryGet(f64, f64, f64, f64),
    OrdinaryDelete(f64, f64, f64),
    OrdinaryHas(f64, f64, f64),
    OrdinarySet(f64, f64, f64, f64, f64),
    OrdinaryDefine(
        f64,
        f64,
        f64,
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
        self::ml::Big,
    ),
    PropertyKey(f64, f64, self::ml::Big),
    PropertyDefinition(
        f64,
        f64,
        f64,
        f64,
        self::ml::Big,
        Option<(Option<self::ml_string::Text>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallFunction {
    CallFunction(
        self::ml::Big,
        self::ml::Big,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        f64,
        f64,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptCallSourceSpan>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallTemplate {
    CallTemplate(
        self::ml::Big,
        f64,
        self::ml::Big,
        f64,
        f64,
        f64,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        f64,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::JavaScriptCallSourceSpan>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallProgram {
    CallProgram(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,)>,
        Option<(Option<self::JavaScriptArraySpeciesDirectory>,)>,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySpeciesCall {
    ArraySpeciesSelectedCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySpeciesState {
    ArraySpeciesState(
        Box<self::JavaScriptSemanticValue>,
        f64,
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
        Option<(Option<self::JavaScriptArraySpeciesCall>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayConstructorClassification {
    ArrayConstructorNotConstructable,
    ArrayConstructorKnown(Box<self::JavaScriptArrayConstructorFact>),
    ArrayConstructorUnknown(Option<(Option<self::JavaScriptSemanticValue>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySpeciesTransition {
    ArraySpeciesContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySpeciesState>,
    ),
    ArraySpeciesCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySpeciesState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ArraySpeciesNeedAuthority(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySpeciesState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArraySpeciesAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySpeciesState>,
        Box<self::JavaScriptSemanticValue>,
        f64,
    ),
    ArraySpeciesConstruct(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySpeciesState>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        Box<self::JavaScriptArrayConstructorFact>,
    ),
    ArraySpeciesThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArraySpeciesError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ArraySpeciesInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptDescriptorConversionState {
    DescriptorConversion(
        Box<self::JavaScriptSemanticValue>,
        f64,
        std::rc::Rc<
            std::cell::RefCell<(
                (
                    bool,
                    bool,
                    bool,
                    bool,
                    self::JavaScriptSemanticValue,
                    bool,
                    self::JavaScriptSemanticValue,
                    bool,
                    self::JavaScriptSemanticValue,
                    bool,
                    bool,
                    bool,
                ),
                Vec<f64>,
            )>,
        >,
        bool,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSemanticCompletion {
    NormalCompletion(Box<self::JavaScriptSemanticValue>),
    ThrowCompletion(Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptDescriptorConversionTransition {
    DescriptorContinuation(Box<self::JavaScriptDescriptorConversionState>),
    DescriptorCall(
        Box<self::JavaScriptDescriptorConversionState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    DescriptorCompleted(
        std::rc::Rc<
            std::cell::RefCell<(
                (
                    bool,
                    bool,
                    bool,
                    bool,
                    self::JavaScriptSemanticValue,
                    bool,
                    self::JavaScriptSemanticValue,
                    bool,
                    self::JavaScriptSemanticValue,
                    bool,
                    bool,
                    bool,
                ),
                Vec<f64>,
            )>,
        >,
    ),
    DescriptorThrow(Box<self::JavaScriptSemanticValue>),
    DescriptorIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    DescriptorInvalid(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayTransformCall {
    ArrayTransformSelectedCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayTransformState {
    ArrayTransformState(
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        f64,
        f64,
        f64,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
        Option<(Option<self::JavaScriptPrimitiveConversionState>,)>,
        Option<(Option<self::JavaScriptArraySpeciesState>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptArrayConstructorFact>,)>,
        Option<(Option<self::JavaScriptArrayTransformCall>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayTransformTransition {
    ArrayTransformContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayTransformState>,
    ),
    ArrayTransformCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayTransformState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ArrayTransformAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayTransformState>,
        Box<self::JavaScriptSemanticValue>,
        f64,
    ),
    ArrayTransformConstruct(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayTransformState>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        Box<self::JavaScriptArrayConstructorFact>,
    ),
    ArrayTransformNeedAuthority(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayTransformState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayTransformCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayTransformThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayTransformError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ArrayTransformInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ArrayTransformResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayTransformState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptIndexedPropertyCreation {
    IndexedPropertyCreated(Box<self::JavaScriptOrdinaryHeap>),
    IndexedPropertyError(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    IndexedPropertyInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    IndexedPropertyResourceLimit(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyReflectionState {
    PropertyReflection(
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        f64,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        f64,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyReflectionTransition {
    PropertyReflectionContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptPropertyReflectionState>,
    ),
    PropertyReflectionAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptPropertyReflectionState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    PropertyReflectionCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    PropertyReflectionError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    PropertyReflectionInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    PropertyReflectionResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptPropertyReflectionState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptFunctionBindCall {
    FunctionBindCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptFunctionBindState {
    FunctionBindState(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        f64,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptFunctionBindCall>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptFunctionBindTransition {
    FunctionBindContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptFunctionBindState>,
    ),
    FunctionBindAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptFunctionBindState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    FunctionBindCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptFunctionBindState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    FunctionBindCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    FunctionBindThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    FunctionBindError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    FunctionBindInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    FunctionBindResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptFunctionBindState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayAssignmentState {
    ArrayAssignmentDefinition(
        Box<self::JavaScriptArrayDefinitionState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayAssignmentNumber(
        Box<self::JavaScriptArrayDefinitionState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptPrimitiveConversionState>,
    ),
    ArrayAssignmentPrimitive(
        Box<self::JavaScriptArrayDefinitionState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayAssignmentTransition {
    ArrayAssignmentContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayAssignmentState>,
    ),
    ArrayAssignmentCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayAssignmentState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ArrayAssignmentCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        bool,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>>,)>,
    ),
    ArrayAssignmentThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayAssignmentError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ArrayAssignmentInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ArrayAssignmentResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayAssignmentState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptObjectCopyCall {
    ObjectCopyCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptObjectSpreadState {
    ObjectSpreadState(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        f64,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptArrayAssignmentState>,)>,
        Option<(Option<self::JavaScriptObjectCopyCall>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptObjectSpreadTransition {
    ObjectSpreadContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectSpreadState>,
    ),
    ObjectSpreadCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectSpreadState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ObjectSpreadCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ObjectSpreadThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ObjectSpreadError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        Option<(Option<self::ml_string::Text>,)>,
    ),
    ObjectSpreadInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ObjectSpreadResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectSpreadState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayConcatenationCall {
    ArrayConcatenationCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayConcatenationState {
    ArrayConcatenationState(
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        f64,
        Box<self::JavaScriptSemanticValue>,
        f64,
        f64,
        f64,
        f64,
        bool,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
        Option<(Option<self::JavaScriptPrimitiveConversionState>,)>,
        Option<(Option<self::JavaScriptArraySpeciesState>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptArrayConstructorFact>,)>,
        Option<(Option<self::JavaScriptObjectSpreadState>,)>,
        Option<(Option<self::JavaScriptArrayConcatenationCall>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayConcatenationTransition {
    ArrayConcatenationContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayConcatenationState>,
    ),
    ArrayConcatenationCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayConcatenationState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ArrayConcatenationAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayConcatenationState>,
        Box<self::JavaScriptSemanticValue>,
        f64,
    ),
    ArrayConcatenationConstruct(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayConcatenationState>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        Box<self::JavaScriptArrayConstructorFact>,
    ),
    ArrayConcatenationNeedAuthority(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayConcatenationState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayConcatenationCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayConcatenationThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayConcatenationError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ArrayConcatenationInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ArrayConcatenationResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayConcatenationState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySliceCall {
    ArraySliceCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySliceState {
    ArraySliceState(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        bool,
        Box<self::JavaScriptSemanticValue>,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
        self::ml_string::Text,
        Option<(Option<self::JavaScriptPrimitiveConversionState>,)>,
        Option<(Option<self::JavaScriptArraySpeciesState>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptArrayConstructorFact>,)>,
        Option<(Option<self::JavaScriptObjectSpreadState>,)>,
        Option<(Option<self::JavaScriptArraySliceCall>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySliceTransition {
    ArraySliceContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySliceState>,
    ),
    ArraySliceCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySliceState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ArraySliceAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySliceState>,
        Box<self::JavaScriptSemanticValue>,
        f64,
    ),
    ArraySliceConstruct(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySliceState>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        Box<self::JavaScriptArrayConstructorFact>,
    ),
    ArraySliceNeedAuthority(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySliceState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArraySliceCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArraySliceThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArraySliceError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ArraySliceInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ArraySliceResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySliceState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSynchronousIteratorRecord {
    SynchronousIteratorRecord(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        bool,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptIteratorCompletion {
    IteratorCompletion(
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        f64,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptIteratorSelectedCall {
    IteratorSelectedCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptIteratorProtocolState {
    IteratorProtocol(
        self::ml_string::Text,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSynchronousIteratorRecord>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptIteratorCompletion>,
        Option<(Option<bool>,)>,
        Option<(Option<self::JavaScriptIteratorSelectedCall>,)>,
    ),
}
