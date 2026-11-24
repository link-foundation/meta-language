// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptCollectionIteration;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::move_java_script_collection_iterator_cursor;
use super::read_java_script_ordinary_value_error;

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Result {
    MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Done,
    MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Return(
        Box<self::JavaScriptCollectionIteration>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_advance_java_script_collection_iterator_loop136(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut iterator: Option<self::JavaScriptOrdinaryObject>,
    mut collection_identity: self::ml::Big,
    kind: &self::ml_string::Text,
    mut entries: Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>,
    mut cursor: f64,
) -> self::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Result {
    loop {
        return if cursor
            < ({
                let source_ml_source1_nullish_field_receiver21584 = (entries.clone(),);
                if match (source_ml_source1_nullish_field_receiver21584.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source1_nullish_field_present21585 =
                            ((source_ml_source1_nullish_field_receiver21584.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        {
                            let ml_reference_array352 =
                                source_ml_source1_nullish_field_present21585;
                            ml_reference_array352.borrow().len() as f64
                        }
                    }
                } else {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (self::ml_string::Text::from_units(vec![
                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111,
                            112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                            101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103,
                            32, 39, 108, 101, 110, 103, 116, 104, 39, 41,
                        ]))
                        .0,
                        message_present: true,
                    })
                    .raise()
                }
            }) {
            {
                let entry = {
                    let source_ml_source1_nullish_field_receiver21586 = (entries.clone(), cursor);
                    {
                        let source_ml_source1_computed_property_key21587 =
                            (source_ml_source1_nullish_field_receiver21586.clone()).1;
                        if match (source_ml_source1_nullish_field_receiver21586.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present21588 =
                                    ((source_ml_source1_nullish_field_receiver21586.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let ml_reference_array353 =
                                        source_ml_source1_nullish_field_present21588;
                                    let ml_reference_position353 =
                                        source_ml_source1_computed_property_key21587;
                                    self::ml_array::optional_at(
                                        &*ml_reference_array353.borrow(),
                                        self::ml_array::number_index(ml_reference_position353),
                                    )
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (((self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39,
                                ]))
                                .concat(
                                    &self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        (source_ml_source1_nullish_field_receiver21586.clone()).1,
                                    )),
                                ))
                                .concat(&self::ml_string::Text::from_units(vec![39, 41])))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                };
                if match (entry.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } || !(match entry.clone().expect("a proven present value") {
                    self::JavaScriptCollectionEntry::CollectionEntry(
                        _,
                        _,
                        source_ml_source1_data_property_value21591,
                    ) => source_ml_source1_data_property_value21591,
                }) {
                    {
                        let cursor_2 = cursor + 1f64;
                        {
                            cursor = cursor_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let updated = self::move_java_script_collection_iterator_cursor(
                            heap.clone(),
                            iterator.clone(),
                            collection_identity.clone(),
                            (cursor + 1f64),
                        );
                        self::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Result::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Return(Box::new(self::JavaScriptCollectionIteration::CollectionIteration(Box::new(updated), false, Box::new(match entry.clone().expect("a proven present value") {
                            self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value21592, _, _) => {
                                let source_ml_source1_data_property_value21592 = *source_ml_source1_data_property_value21592;
                                source_ml_source1_data_property_value21592.clone()
                            }
                        }), Box::new(if (*kind) == (self::ml_string::Text::from_units(vec![107, 101, 121, 115])) {
                            match entry.clone().expect("a proven present value") {
                                self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value21593, _, _) => {
                                    let source_ml_source1_data_property_value21593 = *source_ml_source1_data_property_value21593;
                                    source_ml_source1_data_property_value21593.clone()
                                }
                            }
                        } else {
                            match entry.clone().expect("a proven present value") {
                                self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source1_data_property_value21594, _) => {
                                    let source_ml_source1_data_property_value21594 = *source_ml_source1_data_property_value21594;
                                    source_ml_source1_data_property_value21594.clone()
                                }
                            }
                        }), ((*kind) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))))))
                    }
                }
            }
        } else {
            self::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Result::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_allocate_java_script_array_loop139(
    mut count: f64,
    mut source_ml_temporary_values137: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index137: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index137
            < ({
                let ml_reference_array354 = source_ml_temporary_values137.clone();
                self::ml::Big::from_u128(ml_reference_array354.borrow().len() as u128)
            })
        {
            {
                let object = {
                    let ml_reference_array355 = source_ml_temporary_values137.clone();
                    let ml_reference_position355 = source_ml_temporary_index137.clone();
                    self::ml_array::at(
                        &*ml_reference_array355.borrow(),
                        ml_reference_position355.to_index(),
                    )
                };
                {
                    let count_2 = count + {
                        let ml_reference_array356 = match object {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21597,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21597.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21598,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21598.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21599,
                            ) => source_ml_source1_data_property_value21599.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21600,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21600.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21601,
                                _,
                            ) => source_ml_source1_data_property_value21601.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21602,
                                _,
                            ) => source_ml_source1_data_property_value21602.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21603,
                                _,
                            ) => source_ml_source1_data_property_value21603.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21604,
                                _,
                            ) => source_ml_source1_data_property_value21604.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21605,
                            ) => source_ml_source1_data_property_value21605.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21606,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21606.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21607,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21607.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21608,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21608.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21609,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21609.clone(),
                        };
                        ml_reference_array356.borrow().len() as f64
                    };
                    {
                        let source_ml_temporary_index137_2 =
                            source_ml_temporary_index137.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_temporary_index137) =
                                (count_2, source_ml_temporary_index137_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { count };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_allocate_java_script_array_loop140(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut at: f64,
) -> f64 {
    loop {
        if (at
            < ({
                let ml_reference_array357 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value21611,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21611.clone(),
                };
                ml_reference_array357.borrow().len() as f64
            }))
            && ((({
                let source_ml_source1_optional_chain_receiver21632 = ({
                    let source_ml_source1_array_read_value21630 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value21629,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value21629.clone(),
                    };
                    {
                        let ml_reference_array358 = source_ml_source1_array_read_value21630;
                        let ml_reference_position358 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array358.borrow(),
                            self::ml_array::number_index(ml_reference_position358),
                        )
                    }
                },);
                if match (source_ml_source1_optional_chain_receiver21632.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    Some({
                        let source_ml_temporary_optional_chain138 =
                            ((source_ml_source1_optional_chain_receiver21632.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        match source_ml_temporary_optional_chain138 {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source1_data_property_value21633,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21633.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source1_data_property_value21634,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21634.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source1_data_property_value21635,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21635.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source1_data_property_value21636,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21636.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source1_data_property_value21637,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21637.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source1_data_property_value21638,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21638.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source1_data_property_value21639,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21639.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source1_data_property_value21640,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21640.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value21641,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21641.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source1_data_property_value21642,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21642.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source1_data_property_value21643,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21643.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source1_data_property_value21644,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21644.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source1_data_property_value21645,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21645.clone(),
                        }
                    })
                } else {
                    None::<self::ml::Big>
                }
            })
            .clone()
            .unwrap_or_else(|| identity.clone()))
                < identity)
        {
            {
                let at_2 = at + 1f64;
                {
                    at = at_2;
                    continue;
                }
            }
        }
        return { at };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Result {
    MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Done,
    MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_collection_iterator_result_prototypes_error_loop144(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values143: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index143: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Result {
    loop {
        return if source_ml_temporary_index143
            < ({
                let ml_reference_array359 = source_ml_temporary_values143.clone();
                self::ml::Big::from_u128(ml_reference_array359.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array360 = source_ml_temporary_values143.clone();
                    let ml_reference_position360 = source_ml_temporary_index143.clone();
                    self::ml_array::at(
                        &*ml_reference_array360.borrow(),
                        ml_reference_position360.to_index(),
                    )
                };
                {
                    let invalid = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if invalid == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_temporary_index143_2 =
                                source_ml_temporary_index143.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index143 = source_ml_temporary_index143_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Result::MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Return(invalid.clone())
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Result::MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Result {
    MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Done,
    MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_collection_iterator_result_state_error_loop146(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values145: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index145: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Result {
    loop {
        return if source_ml_temporary_index145
            < ({
                let ml_reference_array361 = source_ml_temporary_values145.clone();
                self::ml::Big::from_u128(ml_reference_array361.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array362 = source_ml_temporary_values145.clone();
                    let ml_reference_position362 = source_ml_temporary_index145.clone();
                    self::ml_array::at(
                        &*ml_reference_array362.borrow(),
                        ml_reference_position362.to_index(),
                    )
                };
                {
                    let invalid = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if invalid == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_temporary_index145_2 =
                                source_ml_temporary_index145.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index145 = source_ml_temporary_index145_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Result::MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Return(invalid.clone())
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Result::MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_step_java_script_collection_iterator_result_loop148(
    mut properties: f64,
    mut source_ml_temporary_values147: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index147: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index147
            < ({
                let ml_reference_array363 = source_ml_temporary_values147.clone();
                self::ml::Big::from_u128(ml_reference_array363.borrow().len() as u128)
            })
        {
            {
                let object = {
                    let ml_reference_array364 = source_ml_temporary_values147.clone();
                    let ml_reference_position364 = source_ml_temporary_index147.clone();
                    self::ml_array::at(
                        &*ml_reference_array364.borrow(),
                        ml_reference_position364.to_index(),
                    )
                };
                {
                    let properties_2 = properties + {
                        let ml_reference_array365 = match object {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21654,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21654.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21655,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21655.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21656,
                            ) => source_ml_source1_data_property_value21656.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21657,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21657.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21658,
                                _,
                            ) => source_ml_source1_data_property_value21658.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21659,
                                _,
                            ) => source_ml_source1_data_property_value21659.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21660,
                                _,
                            ) => source_ml_source1_data_property_value21660.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21661,
                                _,
                            ) => source_ml_source1_data_property_value21661.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21662,
                            ) => source_ml_source1_data_property_value21662.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21663,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21663.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21664,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21664.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21665,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21665.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21666,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21666.clone(),
                        };
                        ml_reference_array365.borrow().len() as f64
                    };
                    {
                        let source_ml_temporary_index147_2 =
                            source_ml_temporary_index147.add(&self::ml::Big::from_i128(1));
                        {
                            (properties, source_ml_temporary_index147) =
                                (properties_2, source_ml_temporary_index147_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { properties };
    }
}
