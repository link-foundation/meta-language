// Generated from complete js/src/translation/javascript-collection-iteration.js. Do not edit by hand.
use super::JavaScriptCollectionIteration;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::advance_java_script_collection_iterator;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_unit;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn advance_java_script_branded_collection_iterator(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
    collection_kind: &self::ml_string::Text,
) -> self::JavaScriptCollectionIteration {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_3 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_3 {
                self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else if (self::ml_array::includes(
                &{
                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        self::ml_string::Text::from_units(vec![77, 97, 112]),
                        self::ml_string::Text::from_units(vec![83, 101, 116]),
                    ])))
                    .borrow()
                    .clone()
                },
                collection_kind,
                0f64,
                false,
                |a, b| a == b,
            )) {
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                    {
                        let ml_library_inequality_2 =
                            invalid != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_2 {
                            self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                                Box::new(heap.clone()),
                                invalid.clone(),
                            )
                        } else {
                            match receiver.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                {
                                    let iterator = {
                                        let source_ml_source1_array_read_value18229 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value18227, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18227.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_array_read_index18230 = self::find_java_script_ordinary_object(match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value18228, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value18228.clone()
                                                }
                                            }, identity.clone());
                                            { let ml_reference_array96 = source_ml_source1_array_read_value18229; let ml_reference_position96 = source_ml_source1_array_read_index18230; self::ml_array::optional_at(&*ml_reference_array96.borrow(), self::ml_array::number_index(ml_reference_position96)) }
                                        }
                                    };
                                    if match (iterator.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                    } else {
                                        match iterator.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, iterator_collection_kind, _, _) => {
                                                {
let ml_library_inequality_1 = iterator_collection_kind != (*collection_kind) ;
if ml_library_inequality_1 {
                                                    self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                                } else {
                                                    self::advance_java_script_collection_iterator(heap.clone(), receiver.clone())
                                                }
}
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        }
                    }
                }
            } else {
                self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110,
                        101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                        114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114,
                        97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100,
                    ]),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_weak_key_eligibility_loop2(
    mut value: self::JavaScriptSemanticValue,
    mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>,
    mut low: f64,
    mut high: f64,
) -> f64 {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let row = {
                        let ml_reference_array97 = facts.clone();
                        let ml_reference_position97 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array97.borrow(),
                            self::ml_array::number_index(ml_reference_position97),
                        )
                    };
                    if !(match (row.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && {
                        let source_ml_source1_comparison_operand18330 =
                            match row.clone().expect("a proven present value") {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    source_ml_source1_data_property_value18327,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value18327.clone(),
                            };
                        {
                            let source_ml_source1_comparison_operand18331 = match value.clone() {
                                self::JavaScriptSemanticValue::Undefined => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Null => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Number(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Integer(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Text(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    None::<self::ml::Big>
                                }
                                self::JavaScriptSemanticValue::Symbol(
                                    source_ml_source1_data_property_value18328,
                                ) => Some(source_ml_source1_data_property_value18328.clone()),
                                self::JavaScriptSemanticValue::Object(
                                    source_ml_source1_data_property_value18329,
                                    _,
                                ) => Some(source_ml_source1_data_property_value18329.clone()),
                            };
                            if match (
                                source_ml_source1_comparison_operand18331.clone(),
                                None::<self::ml::Big>,
                            ) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                {
                                    let source_ml_source1_comparison_order18332 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source1_comparison_operand18330.clone());
                                    (source_ml_source1_comparison_order18332 == 1f64)
                                }
                            } else {
                                source_ml_source1_comparison_operand18330
                                    < (source_ml_source1_comparison_operand18331
                                        .clone()
                                        .expect("a proven present value"))
                            }
                        }
                    } {
                        {
                            let low_2 = middle + 1f64;
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
        } else {
            low
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop3Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop3Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop3(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result {
    loop {
        return if at
            < ({
                let ml_reference_array98 = units.clone();
                ml_reference_array98.borrow().len() as f64
            }) {
            {
                let unit = ({
                    let ml_reference_array99 = units.clone();
                    let ml_reference_position99 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array99.borrow(),
                        self::ml_array::number_index(ml_reference_position99),
                    )
                })
                .clone()
                .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop4Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop4(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result {
    loop {
        return if at < length {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Text(units) => ({
                        let ml_reference_array100 = units.clone();
                        let ml_reference_position100 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array100.borrow(),
                            self::ml_array::number_index(ml_reference_position100),
                        )
                    })
                    .clone()
                    .unwrap_or_else(|| -1f64),
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_f18339) => {
                        let text = source_ml_source1_f18339.clone();
                        {
                            let source_ml_source1_string_unit_operand18393 = text;
                            {
                                let source_ml_source1_string_unit_operand18394 = at;
                                (std::rc::Rc::new(
                                    move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                        {
                                            let index = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if (index < 0f64) || (index >= (text_2.length())) {
                                                f64::NAN
                                            } else {
                                                ({
                                                let source_ml_source1_string_unit_operand18400 = {
                                                    let source_ml_source1_string_range_operand18395 = text_2.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand18396 = index;
                                                        {
                                                            let source_ml_source1_string_range_operand18397 = index + 1f64;
                                                            {
                                                                let source_ml_source1_string_range_position18398 = source_ml_source1_string_range_operand18396;
                                                                {
                                                                    let source_ml_source1_string_range_position18399 = source_ml_source1_string_range_operand18397;
                                                                    source_ml_source1_string_range_operand18395.slice(source_ml_source1_string_range_position18398, source_ml_source1_string_range_position18399)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_string_unit_operand18401 = 0f64;
                                                    source_ml_source1_string_unit_operand18400.code_point_at(source_ml_source1_string_unit_operand18401)
                                                }
                                            }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source1_string_unit_operand18393,
                                    source_ml_source1_string_unit_operand18394,
                                )
                            }
                        }
                    }
                    _ => -1f64,
                };
                {
                    let b = match right.clone() {
                        self::JavaScriptSemanticValue::Text(units_2) => ({
                            let ml_reference_array101 = units_2.clone();
                            let ml_reference_position101 = at;
                            self::ml_array::optional_at(
                                &*ml_reference_array101.borrow(),
                                self::ml_array::number_index(ml_reference_position101),
                            )
                        })
                        .clone()
                        .unwrap_or_else(|| -1f64),
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_f18422) => {
                            let text_2 = source_ml_source1_f18422.clone();
                            {
                                let source_ml_source1_string_unit_operand18476 = text_2.clone();
                                {
                                    let source_ml_source1_string_unit_operand18477 = at;
                                    (std::rc::Rc::new(
                                        move |text_3: self::ml_string::Text,
                                              position: f64|
                                              -> f64 {
                                            {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if (index < 0f64) || (index >= (text_3.length())) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                    let source_ml_source1_string_unit_operand18483 = {
                                                        let source_ml_source1_string_range_operand18478 = text_3.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand18479 = index;
                                                            {
                                                                let source_ml_source1_string_range_operand18480 = index + 1f64;
                                                                {
                                                                    let source_ml_source1_string_range_position18481 = source_ml_source1_string_range_operand18479;
                                                                    {
                                                                        let source_ml_source1_string_range_position18482 = source_ml_source1_string_range_operand18480;
                                                                        source_ml_source1_string_range_operand18478.slice(source_ml_source1_string_range_position18481, source_ml_source1_string_range_position18482)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_string_unit_operand18484 = 0f64;
                                                        source_ml_source1_string_unit_operand18483.code_point_at(source_ml_source1_string_unit_operand18484)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            }
                                        },
                                    ))(
                                        source_ml_source1_string_unit_operand18476,
                                        source_ml_source1_string_unit_operand18477,
                                    )
                                }
                            }
                        }
                        _ => -1f64,
                    };
                    {
                        let ml_library_inequality_1 = a != b;
                        if ml_library_inequality_1 {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Return(false)
                        } else {
                            {
                                let at_2 = at + 1f64;
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result {
    MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop7(
    mut value: self::JavaScriptSemanticValue,
    mut length: f64,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result {
    loop {
        if at < length {
            {
                let source_ml_temporary_construct_java_script_semantic_text_units_ignored8 = {
                    let ml_method_owner102 = units.clone();
                    let ml_method_argument102_0 =
                        self::read_java_script_semantic_text_unit(value.clone(), at);
                    let ml_inserted = vec![ml_method_argument102_0];
                    let mut ml_method_values102 = ml_method_owner102.borrow_mut();
                    let ml_length = ml_method_values102
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values102.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        }
        return {
            self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result {
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done,
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_property_descriptor_entry_loop9(
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut key: self::JavaScriptSemanticValue,
    mut at: f64,
) -> self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result {
    loop {
        return if at
            < ({
                let ml_reference_array103 = properties.clone();
                ml_reference_array103.borrow().len() as f64
            }) {
            {
                let property = {
                    let ml_reference_array104 = properties.clone();
                    let ml_reference_position104 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array104.borrow(),
                        self::ml_array::number_index(ml_reference_position104),
                    )
                };
                if !(match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && self::test_java_script_semantic_same_value(
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value18506,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value18506 =
                                *source_ml_source1_data_property_value18506;
                            source_ml_source1_data_property_value18506.clone()
                        }
                    },
                    key.clone(),
                ) {
                    self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(at)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop11(
    mut key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut value: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result {
    loop {
        return if at
            < ({
                let ml_reference_array105 = key.clone();
                ml_reference_array105.borrow().len() as f64
            }) {
            {
                let digit = match {
                    let ml_reference_array106 = key.clone();
                    let ml_reference_position106 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array106.borrow(),
                        self::ml_array::number_index(ml_reference_position106),
                    )
                } {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                } - 48f64;
                if (!(self::ml_math::is_integer(digit)) || (digit < 0f64)) || (digit > 9f64) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(-1f64)
                } else {
                    {
                        let value_2 = (value * 10f64) + digit;
                        {
                            let at_2 = at + 1f64;
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result {
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Done,
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(f64),
}
