// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArgumentRestState;
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayConcatenationState;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArrayCreationFallback;
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayIteratorState;
use super::JavaScriptArraySliceState;
use super::JavaScriptArrayTransformState;
use super::JavaScriptCallProgram;
use super::JavaScriptCollectionIteratorResultState;
use super::JavaScriptConstructionState;
use super::JavaScriptFunctionBindState;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptObjectEnumerationState;
use super::JavaScriptObjectFromEntriesState;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveWrapperState;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionsState;
use super::JavaScriptPropertyReflectionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::ml;
use super::ml_string;

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayIterationTransition {
    ArrayIteratorAllocated(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayIteratorContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayIteratorState>,
    ),
    ArrayIteratorCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayIteratorState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ArrayIteratorComponents(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
        bool,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        bool,
    ),
    ArrayIteratorAllocationResourceLimit(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ArrayIteratorNeedBoxing(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayIteratorThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayIteratorError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ArrayIteratorInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ArrayIteratorResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayIteratorState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSpeciesConstructionCall {
    SpeciesConstructionCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSpeciesConstructionState {
    SpeciesConstruction(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptArrayConstructorFact>,
        f64,
        self::ml_string::Text,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptSpeciesConstructionCall>,)>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::JavaScriptConstructionState>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSpeciesConstructionTransition {
    SpeciesConstructionContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSpeciesConstructionState>,
    ),
    SpeciesConstructionCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSpeciesConstructionState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        bool,
    ),
    SpeciesConstructionAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSpeciesConstructionState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    SpeciesConstructionNeedAuthority(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSpeciesConstructionState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    SpeciesConstructionNeedImplementation(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSpeciesConstructionState>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,)>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::ml_string::Text>,)>,
    ),
    SpeciesConstructionCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    SpeciesConstructionThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    SpeciesConstructionError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
        Option<(Option<self::ml::Big>,)>,
    ),
    SpeciesConstructionInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    SpeciesConstructionResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSpeciesConstructionState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptFunctionTextResult {
    FunctionTextValue(Box<self::JavaScriptSemanticValue>),
    FunctionTextNeedAuthority(self::ml_string::Text),
    FunctionTextError(self::ml_string::Text, self::ml_string::Text),
    FunctionTextInvalid(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSymbolRegistryTransition {
    SymbolRegistryValue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    SymbolRegistryError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    SymbolRegistryInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    SymbolRegistryResourceLimit(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    SymbolRegistryAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionForEachCall {
    CollectionForEachSelectedCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionForEachState {
    CollectionForEachState(
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        f64,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptCollectionForEachCall>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionForEachTransition {
    CollectionForEachContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptCollectionForEachState>,
    ),
    CollectionForEachCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptCollectionForEachState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    CollectionForEachCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    CollectionForEachThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    CollectionForEachError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    CollectionForEachInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptInstanceOfCall {
    InstanceOfCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptInstanceOfState {
    InstanceOfState(
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
        Option<(Option<self::JavaScriptInstanceOfCall>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptInstanceOfTransition {
    InstanceOfContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptInstanceOfState>,
    ),
    InstanceOfCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptInstanceOfState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    InstanceOfCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    InstanceOfThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    InstanceOfError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    InstanceOfInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    InstanceOfResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptInstanceOfState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptObjectIntegrityState {
    ObjectIntegrity(
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        f64,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptObjectIntegrityTransition {
    ObjectIntegrityContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectIntegrityState>,
    ),
    ObjectPrototypeMetadata(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ObjectIntegrityCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ObjectIntegrityError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ObjectIntegrityInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ObjectIntegrityResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectIntegrityState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptFunctionNameResult {
    FunctionNameValue(Box<self::JavaScriptSemanticValue>),
    FunctionNameInvalid(self::ml_string::Text),
    FunctionNameResourceLimit(self::ml_string::Text),
    FunctionNameSymbolDescription(Box<self::JavaScriptSemanticValue>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallIteratorRegion {
    CallIteratorRegion(f64, Box<self::JavaScriptSynchronousIteratorRecord>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptIteratorRegionsUpdate {
    IteratorRegionsUpdated(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>,
    ),
    IteratorRegionsInvalid(self::ml_string::Text),
    IteratorRegionsResourceLimit(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallArgumentList {
    CallArgumentList(
        f64,
        f64,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallArgumentListsUpdate {
    ArgumentListsUpdated(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>),
    ArgumentListsInvalid(self::ml_string::Text),
    ArgumentListsResourceLimit(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySpreadState {
    ArraySpreadState(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        f64,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptIteratorProtocolState>,)>,
        Option<(Option<self::JavaScriptSynchronousIteratorRecord>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySpreadTransition {
    ArraySpreadContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySpreadState>,
    ),
    ArraySpreadCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySpreadState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ArraySpreadPrimitiveGet(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySpreadState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArraySpreadCompleted(Box<self::JavaScriptOrdinaryHeap>, f64),
    ArraySpreadThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArraySpreadError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ArraySpreadInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ArraySpreadResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySpreadState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySearchCall {
    ArraySearchSelectedCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySearchState {
    ArraySearchState(
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        bool,
        Box<self::JavaScriptSemanticValue>,
        f64,
        f64,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptPrimitiveConversionState>,)>,
        Option<(Option<self::JavaScriptArraySearchCall>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySearchTransition {
    ArraySearchContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySearchState>,
    ),
    ArraySearchCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySearchState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ArraySearchCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArraySearchThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArraySearchError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ArraySearchInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ArraySearchResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArraySearchState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayCallbackCall {
    ArrayCallbackSelectedCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayCallbackState {
    ArrayCallbackState(
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        f64,
        f64,
        self::ml_string::Text,
        bool,
        bool,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptPrimitiveConversionState>,)>,
        Option<(Option<self::JavaScriptArrayCallbackCall>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayCallbackTransition {
    ArrayCallbackContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayCallbackState>,
    ),
    ArrayCallbackCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayCallbackState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ArrayCallbackCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayCallbackThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayCallbackError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ArrayCallbackInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ArrayCallbackResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayCallbackState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaScriptErrorAllocationIdentity {
    ErrorAllocationIdentity(self::ml::Big, self::ml_string::Text, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaScriptErrorAllocationDirectory {
    ErrorAllocationDirectory(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorAllocationIdentity>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptTextCallState {
    TextProtocolState(Box<self::JavaScriptPrimitiveConversionState>),
    TextPrimitiveValue(Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptNumberCallState {
    NumberProtocolState(Box<self::JavaScriptPrimitiveConversionState>),
    NumberPrimitiveValue(Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayAuthorityUpdate {
    ArrayAuthorityUpdate(
        Box<self::JavaScriptCallProgram>,
        Box<self::JavaScriptCallMachine>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallCompletion {
    CallCompletion(
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        f64,
        f64,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallIteratorOperation {
    IteratorOperation(f64, f64, f64, self::ml_string::Text, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallHandler {
    CallHandler(f64, f64),
    CallFinallyHandler(f64),
    CallDeferredCompletion(Box<self::JavaScriptCallCompletion>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallPending {
    CallIdle,
    WaitCall(f64),
    WaitGet(f64),
    WaitSet(f64),
    WaitHostSet(f64),
    WaitArgumentRest(f64, self::ml::Big, Box<self::JavaScriptArgumentRestState>),
    WaitIteratorProtocol(
        f64,
        f64,
        f64,
        self::ml_string::Text,
        self::ml::Big,
        Box<self::JavaScriptIteratorProtocolState>,
    ),
    WaitArraySpread(f64, self::ml::Big, Box<self::JavaScriptArraySpreadState>),
    WaitArrayIteratorNext(f64, self::ml::Big, Box<self::JavaScriptArrayIteratorState>),
    WaitCollectionIteratorResult(
        f64,
        self::ml::Big,
        Box<self::JavaScriptCollectionIteratorResultState>,
    ),
    WaitArrayConcatenation(
        f64,
        self::ml::Big,
        Box<self::JavaScriptArrayConcatenationState>,
        Option<(Option<self::JavaScriptSpeciesConstructionState>,)>,
    ),
    WaitArraySlice(
        f64,
        self::ml::Big,
        Box<self::JavaScriptArraySliceState>,
        Option<(Option<self::JavaScriptSpeciesConstructionState>,)>,
    ),
    WaitArrayTransform(
        f64,
        self::ml::Big,
        Box<self::JavaScriptArrayTransformState>,
        Option<(Option<self::JavaScriptSpeciesConstructionState>,)>,
    ),
    WaitArraySearch(f64, self::ml::Big, Box<self::JavaScriptArraySearchState>),
    WaitCollectionForEach(
        f64,
        self::ml::Big,
        Box<self::JavaScriptCollectionForEachState>,
    ),
    WaitArrayCallback(f64, self::ml::Big, Box<self::JavaScriptArrayCallbackState>),
    WaitObjectSpread(f64, self::ml::Big, Box<self::JavaScriptObjectSpreadState>),
    WaitObjectEnumeration(
        f64,
        self::ml::Big,
        Box<self::JavaScriptObjectEnumerationState>,
    ),
    WaitPropertyDefinitions(
        f64,
        self::ml::Big,
        Box<self::JavaScriptPropertyDefinitionsState>,
    ),
    WaitPropertyReflection(
        f64,
        self::ml::Big,
        Box<self::JavaScriptPropertyReflectionState>,
    ),
    WaitInstanceOf(f64, self::ml::Big, Box<self::JavaScriptInstanceOfState>),
    WaitObjectIntegrity(
        f64,
        self::ml::Big,
        Box<self::JavaScriptObjectIntegrityState>,
    ),
    WaitObjectPrototype(f64, self::ml::Big, Box<self::JavaScriptSemanticValue>),
    WaitArrayAssignment(
        f64,
        self::ml::Big,
        Box<self::JavaScriptArrayAssignmentState>,
    ),
    WaitKey(
        f64,
        self::ml::Big,
        Box<self::JavaScriptPrimitiveConversionState>,
    ),
    WaitDefinition(
        f64,
        self::ml::Big,
        Box<self::JavaScriptPropertyDefinitionState>,
    ),
    WaitNumber(f64, self::ml::Big, Box<self::JavaScriptNumberCallState>),
    WaitText(f64, self::ml::Big, Box<self::JavaScriptTextCallState>),
    WaitArrayCreation(
        f64,
        self::ml::Big,
        Box<self::JavaScriptArrayCreationState>,
        Option<(Option<self::JavaScriptConstructionState>,)>,
        Option<(Option<self::JavaScriptArrayCreationFallback>,)>,
        Option<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>,
    ),
    WaitConstruction(
        f64,
        self::ml::Big,
        Box<self::JavaScriptConstructionState>,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,)>,
    ),
    WaitFunctionBind(f64, self::ml::Big, Box<self::JavaScriptFunctionBindState>),
    WaitFunctionText(
        f64,
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
    ),
    WaitConstructorCheck(f64, Box<self::JavaScriptSemanticValue>, bool),
    WaitObjectFromEntries(
        f64,
        self::ml::Big,
        Box<self::JavaScriptObjectFromEntriesState>,
        Option<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>,
    ),
    WaitPrimitiveWrapper(f64, Box<self::JavaScriptPrimitiveWrapperState>),
    WaitSymbolAllocation(
        f64,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
    ),
    WaitSymbolDescription(f64, Box<self::JavaScriptSemanticValue>),
    WaitFunctionNameSymbol(
        f64,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
        self::ml::Big,
    ),
    WaitSourceError(
        self::ml::Big,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        bool,
    ),
    WaitErrorValue(
        f64,
        self::ml::Big,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        Box<self::JavaScriptErrorAllocationDirectory>,
    ),
    WaitErrorIntrinsic(f64),
    WaitHostCapability(f64, Box<self::JavaScriptErrorAllocationDirectory>),
    WaitError(self::ml::Big, self::ml_string::Text, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaScriptObjectFromEntriesErrorRequest {
    ObjectFromEntriesErrorRequest(self::ml::Big, self::ml_string::Text, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallFrame {
    CallFrame(
        self::ml::Big,
        f64,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallHandler>>>,
        Box<self::JavaScriptCallPending>,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,)>,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallMachine {
    CallMachine(
        Box<self::JavaScriptOrdinaryHeap>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFrame>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallMachineTransition {
    MachineContinue(Box<self::JavaScriptCallMachine>),
    MachineNeedObjectPrototype(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
        self::ml::Big,
    ),
    MachineAllocateSymbol(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
    ),
    MachineNeedSymbolDescription(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
    ),
    MachineNeedFunctionNameSymbolDescription(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
    ),
    MachineNeedFunctionText(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
        self::ml::Big,
    ),
    MachineCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    MachineThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    MachineAllocateObject(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
    ),
    MachineAllocateError(
        Box<self::JavaScriptCallMachine>,
        self::ml::Big,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    MachineAllocateSourceError(
        Box<self::JavaScriptCallMachine>,
        self::ml::Big,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        bool,
    ),
    MachineAllocateErrorValue(
        Box<self::JavaScriptCallMachine>,
        self::ml::Big,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        bool,
    ),
    MachineInvokeErrorIntrinsic(
        Box<self::JavaScriptCallMachine>,
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    MachineInvokeHostCapability(
        Box<self::JavaScriptCallMachine>,
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    MachineConstruct(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    MachineConstructArraySpecies(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        Box<self::JavaScriptArrayConstructorFact>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::ml_string::Text>,)>,
    ),
    MachineNeedPrimitiveIteratorGet(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
    MachineNeedArrayIteratorBoxing(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
    ),
    MachineNeedConstructorAuthority(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
    ),
    MachineNeedCollectionIterable(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
    MachineInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    MachineResourceLimit(Box<self::JavaScriptCallMachine>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallCompletionTransition {
    MachineContinue(Box<self::JavaScriptCallMachine>),
    MachineThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    MachineInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    MachineResourceLimit(Box<self::JavaScriptCallMachine>, self::ml_string::Text),
    CallReturn(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallExecution {
    CallExecution(Box<self::JavaScriptCallMachineTransition>, f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptIteratorThrowTransition {
    MachineInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    MachineContinue(Box<self::JavaScriptCallMachine>),
}
