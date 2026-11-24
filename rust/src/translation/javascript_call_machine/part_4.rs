// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallProgram;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::ml;
use super::ml_string;

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptIteratorProtocolTransition {
    IteratorProtocolContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptIteratorProtocolState>,
    ),
    IteratorProtocolCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptIteratorProtocolState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    IteratorProtocolPrimitiveGet(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptIteratorProtocolState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
    IteratorProtocolAcquired(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSynchronousIteratorRecord>,
    ),
    IteratorProtocolStep(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSynchronousIteratorRecord>,
        bool,
        Box<self::JavaScriptSemanticValue>,
    ),
    IteratorProtocolClosed(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSynchronousIteratorRecord>,
        Box<self::JavaScriptIteratorCompletion>,
    ),
    IteratorProtocolThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptIteratorProtocolState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    IteratorProtocolError(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptIteratorProtocolState>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    IteratorProtocolInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPrimitiveWrapperState {
    PrimitiveWrapperState(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPrimitiveWrapperTransition {
    PrimitiveWrapperAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptPrimitiveWrapperState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    PrimitiveWrapperCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    PrimitiveWrapperInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    PrimitiveWrapperResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptPrimitiveWrapperState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayCreationCall {
    ArrayCreationCallRecord(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayCreationFallback {
    ArrayCreationArrayFallback(Box<self::JavaScriptSemanticValue>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayCreationState {
    ArrayCreationState(
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        self::ml_string::Text,
        f64,
        f64,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSynchronousIteratorRecord>,)>,
        Option<(Option<self::JavaScriptIteratorProtocolState>,)>,
        Option<(Option<self::JavaScriptPrimitiveConversionState>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptPrimitiveWrapperState>,)>,
        Option<(Option<self::JavaScriptArrayAssignmentState>,)>,
        Option<(Option<self::JavaScriptArrayCreationCall>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::ml_string::Text>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayCreationTransition {
    ArrayCreationContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayCreationState>,
    ),
    ArrayCreationConstruct(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayCreationState>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        Box<self::JavaScriptArrayCreationFallback>,
    ),
    ArrayCreationCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayCreationState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ArrayCreationAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayCreationState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayCreationPrimitiveGet(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayCreationState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayCreationNeedError(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayCreationState>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ArrayCreationCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayCreationThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayCreationError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ArrayCreationInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ArrayCreationResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayCreationState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionCompletion {
    CollectionNormal(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    CollectionInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    CollectionTypeError(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    CollectionResourceLimit(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    CollectionNeedIterable(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPrimitiveWrapperRoots {
    PrimitiveWrapperRoots(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionIteratorMethodOwner {
    CollectionIteratorMethodOwner(
        self::ml::Big,
        self::ml::Big,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionIteratorCallableDirectory {
    CollectionIteratorCallableDirectory(
        std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionIteratorMethodOwner>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionIteratorCallCompilation {
    CollectionIteratorCallCompilation(Box<self::JavaScriptCallProgram>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaScriptCollectionMethodOwner {
    CollectionMethodOwner(self::ml::Big, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionConstructorOwner {
    CollectionConstructorOwner(
        self::ml_string::Text,
        self::ml::Big,
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionMethodOwner>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPrimitiveMethodOwner {
    PrimitiveMethodOwner(
        self::ml::Big,
        self::ml::Big,
        self::ml_string::Text,
        Option<(Option<self::JavaScriptArraySpeciesDirectory>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<bool>,)>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::JavaScriptPrimitiveWrapperRoots>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionCallableDirectory {
    CollectionCallableDirectory(
        std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionConstructorOwner>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptPrimitiveMethodOwner>>>,
        Option<(
            Option<
                std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionIteratorMethodOwner>>>,
            >,
        )>,
        Option<(Option<f64>,)>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::JavaScriptArraySpeciesDirectory>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionCallCompilation {
    CollectionCallCompilation(Box<self::JavaScriptCallProgram>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaScriptCollectionMethodCompilation {
    CollectionMethodCompilation(bool, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptClosureAllocation {
    ClosureAllocationNormal(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ClosureAllocationInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ClosureAllocationResourceLimit(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptConstructionCall {
    ConstructionCallRecord(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        bool,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptConstructionState {
    ConstructionState(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        self::ml_string::Text,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        f64,
        self::ml_string::Text,
        Option<(Option<self::JavaScriptPrimitiveConversionState>,)>,
        Option<(Option<self::JavaScriptConstructionCall>,)>,
        Option<(Option<self::JavaScriptConstructionState>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptConstructionTransition {
    ConstructionContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptConstructionState>,
    ),
    ConstructionCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptConstructionState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        bool,
        Box<self::JavaScriptSemanticValue>,
    ),
    ConstructionAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptConstructionState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ConstructionNeedAuthority(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptConstructionState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ConstructionNeedImplementation(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptConstructionState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ConstructionCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ConstructionThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ConstructionError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
        Option<(Option<self::ml::Big>,)>,
    ),
    ConstructionInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ConstructionResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptConstructionState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyKeyConversionTransition {
    PropertyKeyContinuation(Box<self::JavaScriptPrimitiveConversionState>),
    PropertyKeyCall(
        Box<self::JavaScriptPrimitiveConversionState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    PropertyKeyCompleted(Box<self::JavaScriptSemanticValue>),
    PropertyKeyThrow(Box<self::JavaScriptSemanticValue>),
    PropertyKeyIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    PropertyKeyInvalid(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptObjectFromEntriesCall {
    ObjectFromEntriesCallRecord(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptObjectFromEntriesState {
    ObjectFromEntriesState(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptSynchronousIteratorRecord>,)>,
        Option<(Option<self::JavaScriptIteratorProtocolState>,)>,
        Option<(Option<self::JavaScriptPrimitiveConversionState>,)>,
        Option<(Option<self::JavaScriptObjectFromEntriesCall>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::ml_string::Text>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptObjectFromEntriesTransition {
    ObjectFromEntriesContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectFromEntriesState>,
    ),
    ObjectFromEntriesCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectFromEntriesState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ObjectFromEntriesAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectFromEntriesState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ObjectFromEntriesPrimitiveGet(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectFromEntriesState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ObjectFromEntriesNeedError(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectFromEntriesState>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ObjectFromEntriesCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ObjectFromEntriesThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ObjectFromEntriesError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ObjectFromEntriesInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ObjectFromEntriesResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectFromEntriesState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyDefinitionCall {
    PropertyDefinitionCallRecord(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyDefinitionState {
    PropertyDefinitionKey(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptPrimitiveConversionState>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::JavaScriptPropertyDefinitionCall>,)>,
    ),
    PropertyDefinitionDescriptor(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptDescriptorConversionState>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::JavaScriptPropertyDefinitionCall>,)>,
    ),
    PropertyDefinitionArray(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptArrayAssignmentState>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::JavaScriptPropertyDefinitionCall>,)>,
    ),
    PropertyDefinitionReady(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
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
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::JavaScriptPropertyDefinitionCall>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyDefinitionTransition {
    DefinitionContinuation(
        Box<self::JavaScriptPropertyDefinitionState>,
        Option<(Option<self::JavaScriptOrdinaryHeap>,)>,
    ),
    DefinitionCall(
        Box<self::JavaScriptPropertyDefinitionState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        Option<(Option<self::JavaScriptOrdinaryHeap>,)>,
    ),
    DefinitionCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
    ),
    DefinitionThrow(
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptOrdinaryHeap>,)>,
    ),
    DefinitionIntrinsicError(
        self::ml_string::Text,
        self::ml_string::Text,
        Option<(Option<self::JavaScriptOrdinaryHeap>,)>,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>>,)>,
    ),
    DefinitionInvalid(
        self::ml_string::Text,
        Option<(Option<self::JavaScriptOrdinaryHeap>,)>,
    ),
    DefinitionResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptPropertyDefinitionState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyDefinitionsCall {
    PropertyDefinitionsCallRecord(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyDefinitionsState {
    PropertyDefinitions(
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
        Option<(Option<self::JavaScriptDescriptorConversionState>,)>,
        Option<(Option<self::JavaScriptPropertyDefinitionState>,)>,
        Option<(Option<self::JavaScriptPropertyDefinitionsCall>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyDefinitionsTransition {
    PropertyDefinitionsContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptPropertyDefinitionsState>,
    ),
    PropertyDefinitionsCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptPropertyDefinitionsState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    PropertyDefinitionsAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptPropertyDefinitionsState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    PropertyDefinitionsCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    PropertyDefinitionsThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    PropertyDefinitionsError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    PropertyDefinitionsInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    PropertyDefinitionsResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptPropertyDefinitionsState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptObjectEnumerationState {
    ObjectEnumeration(
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        f64,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        f64,
        Box<self::JavaScriptSemanticValue>,
        f64,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptObjectEnumerationTransition {
    ObjectEnumerationContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectEnumerationState>,
    ),
    ObjectEnumerationCall(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectEnumerationState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ObjectEnumerationAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectEnumerationState>,
        Box<self::JavaScriptSemanticValue>,
        f64,
    ),
    ObjectEnumerationCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ObjectEnumerationThrow(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ObjectEnumerationError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ObjectEnumerationInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ObjectEnumerationResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptObjectEnumerationState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArgumentRestState {
    ArgumentRestState(
        Box<self::JavaScriptSemanticValue>,
        f64,
        f64,
        f64,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArgumentRestTransition {
    ArgumentRestContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArgumentRestState>,
    ),
    ArgumentRestAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArgumentRestState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArgumentRestCompleted(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArgumentRestInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ArgumentRestResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArgumentRestState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionIteration {
    CollectionIteration(
        Box<self::JavaScriptOrdinaryHeap>,
        bool,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        bool,
    ),
    CollectionIterationInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    CollectionIterationTypeError(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    CollectionIterationResourceLimit(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    CollectionIteratorAllocated(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionIteratorResultState {
    CollectionIteratorResultState(
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        bool,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionIteratorResultCompletion {
    IteratorResultContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptCollectionIteratorResultState>,
    ),
    IteratorResultAllocate(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptCollectionIteratorResultState>,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
    ),
    IteratorResultNormal(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    IteratorResultInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    IteratorResultTypeError(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    IteratorResultResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptCollectionIteratorResultState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptStringIterationTransition {
    StringIteratorAllocated(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    StringIteratorComponents(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
        bool,
        Box<self::JavaScriptSemanticValue>,
    ),
    StringIteratorTypeError(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    StringIteratorInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    StringIteratorResourceLimit(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayIteratorCall {
    ArrayIteratorCall(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayIteratorState {
    ArrayIteratorState(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        f64,
        self::ml_string::Text,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptPrimitiveConversionState>,)>,
        Option<(Option<self::JavaScriptArrayIteratorCall>,)>,
    ),
}
