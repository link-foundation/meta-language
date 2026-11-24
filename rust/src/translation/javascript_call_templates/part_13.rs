// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::compare_java_script_collection_keys;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_index;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_unit;

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Result {
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Done,
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_heap_directory_object_loop23(
    mut objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let object = {
                        let ml_reference_array109 = objects.clone();
                        let ml_reference_position109 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array109.borrow(),
                            self::ml_array::number_index(ml_reference_position109),
                        )
                    };
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Return(-1f64)
                    } else if (match object.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::ClosureObject(
                            source_ml_source1_data_property_value17816,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17816.clone(),
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            source_ml_source1_data_property_value17817,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17817.clone(),
                        self::JavaScriptOrdinaryObject::ArrayObject(
                            source_ml_source1_data_property_value17818,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17818.clone(),
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            source_ml_source1_data_property_value17819,
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
                        ) => source_ml_source1_data_property_value17819.clone(),
                        self::JavaScriptOrdinaryObject::MapObject(
                            source_ml_source1_data_property_value17820,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17820.clone(),
                        self::JavaScriptOrdinaryObject::SetObject(
                            source_ml_source1_data_property_value17821,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17821.clone(),
                        self::JavaScriptOrdinaryObject::WeakMapObject(
                            source_ml_source1_data_property_value17822,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17822.clone(),
                        self::JavaScriptOrdinaryObject::WeakSetObject(
                            source_ml_source1_data_property_value17823,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17823.clone(),
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source1_data_property_value17824,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17824.clone(),
                        self::JavaScriptOrdinaryObject::HostErrorObject(
                            source_ml_source1_data_property_value17825,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17825.clone(),
                        self::JavaScriptOrdinaryObject::HostURLObject(
                            source_ml_source1_data_property_value17826,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17826.clone(),
                        self::JavaScriptOrdinaryObject::HostArrayObject(
                            source_ml_source1_data_property_value17827,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17827.clone(),
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            source_ml_source1_data_property_value17828,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17828.clone(),
                    }) == identity
                    {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Return(middle)
                    } else {
                        if (match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source1_data_property_value17842,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17842.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source1_data_property_value17843,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17843.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source1_data_property_value17844,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17844.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source1_data_property_value17845,
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
                            ) => source_ml_source1_data_property_value17845.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source1_data_property_value17846,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17846.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source1_data_property_value17847,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17847.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source1_data_property_value17848,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17848.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source1_data_property_value17849,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17849.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value17850,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17850.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source1_data_property_value17851,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17851.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source1_data_property_value17852,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17852.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source1_data_property_value17853,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17853.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source1_data_property_value17854,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17854.clone(),
                        }) < identity
                        {
                            {
                                let low_2 = middle + 1f64;
                                {
                                    low = low_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                high = middle;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Result {
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Done,
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_heap_directory_symbol_loop24(
    mut symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let symbol = {
                        let ml_reference_array110 = symbols.clone();
                        let ml_reference_position110 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array110.borrow(),
                            self::ml_array::number_index(ml_reference_position110),
                        )
                    };
                    if {
                        let source_ml_source1_comparison_operand17858 = None::<()>;
                        match (symbol.clone(), source_ml_source1_comparison_operand17858) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }
                    } {
                        self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Return(false)
                    } else if (symbol.clone().expect("a proven present value")) == identity {
                        self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Return(true)
                    } else {
                        if (symbol.clone().expect("a proven present value")) < identity {
                            {
                                let low_2 = middle + 1f64;
                                {
                                    low = low_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                high = middle;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptArrayObjectErrorLoop26Result {
    MlTemporaryReadJavaScriptArrayObjectErrorLoop26Done,
    MlTemporaryReadJavaScriptArrayObjectErrorLoop26Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_array_object_error_loop26(
    mut length: f64,
    mut source_ml_temporary_values25: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_temporary_index25: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptArrayObjectErrorLoop26Result {
    loop {
        return if source_ml_temporary_index25
            < ({
                let ml_reference_array111 = source_ml_temporary_values25.clone();
                self::ml::Big::from_u128(ml_reference_array111.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array112 = source_ml_temporary_values25.clone();
                    let ml_reference_position112 = source_ml_temporary_index25.clone();
                    self::ml_array::at(
                        &*ml_reference_array112.borrow(),
                        ml_reference_position112.to_index(),
                    )
                };
                {
                    let index = self::read_java_script_array_index(match entry {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value17861,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value17861 =
                                *source_ml_source1_data_property_value17861;
                            source_ml_source1_data_property_value17861.clone()
                        }
                    });
                    if index >= length {
                        self::MlTemporaryReadJavaScriptArrayObjectErrorLoop26Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop26Return(self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 98, 101, 108, 111, 119, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 110, 103, 116, 104]))
                    } else {
                        {
                            let source_ml_temporary_index25_2 =
                                source_ml_temporary_index25.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index25 = source_ml_temporary_index25_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptArrayObjectErrorLoop26Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop26Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptCollectionKeyJoin43 {
    MlTemporaryTestJavaScriptCollectionKeyJoin43Next,
    MlTemporaryTestJavaScriptCollectionKeyJoin43Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareJavaScriptCollectionKeysLoop46Result {
    MlTemporaryCompareJavaScriptCollectionKeysLoop46Done,
    MlTemporaryCompareJavaScriptCollectionKeysLoop46Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_java_script_collection_keys_loop46(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut index: f64,
) -> self::MlTemporaryCompareJavaScriptCollectionKeysLoop46Result {
    loop {
        return if index < length {
            {
                let x = self::read_java_script_semantic_text_unit(left.clone(), index);
                {
                    let y = self::read_java_script_semantic_text_unit(right.clone(), index);
                    if x == y {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop46Result::MlTemporaryCompareJavaScriptCollectionKeysLoop46Return(if x < y {
                            -1f64
                        } else {
                            1f64
                        })
                    }
                }
            }
        } else {
            self::MlTemporaryCompareJavaScriptCollectionKeysLoop46Result::MlTemporaryCompareJavaScriptCollectionKeysLoop46Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareJavaScriptCollectionKeysJoin45 {
    MlTemporaryCompareJavaScriptCollectionKeysJoin45Next,
    MlTemporaryCompareJavaScriptCollectionKeysJoin45Return(f64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryOrderJavaScriptCollectionKeysLoop48Result {
    MlTemporaryOrderJavaScriptCollectionKeysLoop48Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryOrderJavaScriptCollectionKeysLoop49Result {
    MlTemporaryOrderJavaScriptCollectionKeysLoop49Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryOrderJavaScriptCollectionKeysJoin50 {
    MlTemporaryOrderJavaScriptCollectionKeysJoin50Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryOrderJavaScriptCollectionKeysJoin52 {
    MlTemporaryOrderJavaScriptCollectionKeysJoin52Next,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_collection_keys_loop49(
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut middle: f64,
    mut end: f64,
    mut left: f64,
    mut right: f64,
) -> self::MlTemporaryOrderJavaScriptCollectionKeysLoop49Result {
    loop {
        return if (left < middle) || (right < end) {
            {
                let a = {
                    let ml_reference_array113 = source.clone();
                    let ml_reference_position113 = left;
                    self::ml_array::optional_at(
                        &*ml_reference_array113.borrow(),
                        self::ml_array::number_index(ml_reference_position113),
                    )
                };
                {
                    let b = {
                        let ml_reference_array114 = source.clone();
                        let ml_reference_position114 = right;
                        self::ml_array::optional_at(
                            &*ml_reference_array114.borrow(),
                            self::ml_array::number_index(ml_reference_position114),
                        )
                    };
                    {
                        let take_left = right >= end;
                        {
                            let take_left_3 = if !take_left && (left < middle) {
                                if (match (a.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    take_left
                                } else if (match (b.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    take_left
                                } else {
                                    {
                                        (self::compare_java_script_collection_keys(
                                            a.clone().expect("a proven present value"),
                                            b.clone().expect("a proven present value"),
                                        )) <= 0f64
                                    }
                                }
                            } else {
                                take_left
                            };
                            if take_left_3 {
                                {
                                    let source_ml_source1_s17922 = if (match (a.clone(), None::<()>)
                                    {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        self::MlTemporaryOrderJavaScriptCollectionKeysJoin50::MlTemporaryOrderJavaScriptCollectionKeysJoin50Next
                                    } else {
                                        {
                                            let source_ml_temporary_order_java_script_collection_keys_ignored51 = {
                                                let ml_method_owner115 = target.clone();
                                                let ml_method_argument115_0 =
                                                    a.clone().expect("a proven present value");
                                                let ml_inserted = vec![ml_method_argument115_0];
                                                let mut ml_method_values115 =
                                                    ml_method_owner115.borrow_mut();
                                                let ml_length = ml_method_values115
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4_294_967_295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values115.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            self::MlTemporaryOrderJavaScriptCollectionKeysJoin50::MlTemporaryOrderJavaScriptCollectionKeysJoin50Next
                                        }
                                    };
                                    match source_ml_source1_s17922 {
                                        self::MlTemporaryOrderJavaScriptCollectionKeysJoin50::MlTemporaryOrderJavaScriptCollectionKeysJoin50Next => {
                                            {
                                                let left_2 = left + 1f64;
                                                {
                                                    left = left_2;
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let source_ml_source1_s17923 = if (match (b.clone(), None::<()>)
                                    {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        self::MlTemporaryOrderJavaScriptCollectionKeysJoin52::MlTemporaryOrderJavaScriptCollectionKeysJoin52Next
                                    } else {
                                        {
                                            let source_ml_temporary_order_java_script_collection_keys_ignored53 = {
                                                let ml_method_owner116 = target.clone();
                                                let ml_method_argument116_0 =
                                                    b.clone().expect("a proven present value");
                                                let ml_inserted = vec![ml_method_argument116_0];
                                                let mut ml_method_values116 =
                                                    ml_method_owner116.borrow_mut();
                                                let ml_length = ml_method_values116
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4_294_967_295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values116.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            self::MlTemporaryOrderJavaScriptCollectionKeysJoin52::MlTemporaryOrderJavaScriptCollectionKeysJoin52Next
                                        }
                                    };
                                    match source_ml_source1_s17923 {
                                        self::MlTemporaryOrderJavaScriptCollectionKeysJoin52::MlTemporaryOrderJavaScriptCollectionKeysJoin52Next => {
                                            {
                                                let right_2 = right + 1f64;
                                                {
                                                    right = right_2;
                                                    continue;
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
        } else {
            self::MlTemporaryOrderJavaScriptCollectionKeysLoop49Result::MlTemporaryOrderJavaScriptCollectionKeysLoop49Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_collection_keys_loop48(
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut width: f64,
    mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut start: f64,
) -> self::MlTemporaryOrderJavaScriptCollectionKeysLoop48Result {
    loop {
        return if start
            < ({
                let ml_reference_array117 = source.clone();
                ml_reference_array117.borrow().len() as f64
            }) {
            {
                let middle = self::ml_math::min((start + width), {
                    let ml_reference_array118 = source.clone();
                    ml_reference_array118.borrow().len() as f64
                });
                {
                    let end = self::ml_math::min((start + (width * 2f64)), {
                        let ml_reference_array119 = source.clone();
                        ml_reference_array119.borrow().len() as f64
                    });
                    {
                        let left = start;
                        {
                            let source_ml_source1_s17924 =
                                self::source_ml_temporary_order_java_script_collection_keys_loop49(
                                    source.clone(),
                                    target.clone(),
                                    middle,
                                    end,
                                    left,
                                    middle,
                                );
                            match source_ml_source1_s17924 {
                                self::MlTemporaryOrderJavaScriptCollectionKeysLoop49Result::MlTemporaryOrderJavaScriptCollectionKeysLoop49Done => {
                                    {
                                        let start_2 = start + (width * 2f64);
                                        {
                                            start = start_2;
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryOrderJavaScriptCollectionKeysLoop48Result::MlTemporaryOrderJavaScriptCollectionKeysLoop48Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_collection_keys_loop47(
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut width: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    loop {
        return if width
            < ({
                let ml_reference_array120 = source.clone();
                ml_reference_array120.borrow().len() as f64
            }) {
            {
                let target = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                    self::JavaScriptSemanticValue,
                >::new()));
                {
                    let start = 0f64;
                    {
                        let source_ml_source1_s17925 =
                            self::source_ml_temporary_order_java_script_collection_keys_loop48(
                                source.clone(),
                                width,
                                target.clone(),
                                start,
                            );
                        match source_ml_source1_s17925 {
                            self::MlTemporaryOrderJavaScriptCollectionKeysLoop48Result::MlTemporaryOrderJavaScriptCollectionKeysLoop48Done => {
                                {
                                    let width_2 = width * 2f64;
                                    {
                                        (source, width) = (target.clone(), width_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            source.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55 {
    MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55Next,
    MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Result {
    MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Done,
    MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_opaque_object_error_loop56(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values54: std::rc::Rc<
        std::cell::RefCell<Vec<Option<self::JavaScriptSemanticValue>>>,
    >,
    mut source_ml_temporary_index54: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Result {
    loop {
        return if source_ml_temporary_index54
            < ({
                let ml_reference_array121 = source_ml_temporary_values54.clone();
                self::ml::Big::from_u128(ml_reference_array121.borrow().len() as u128)
            }) {
            {
                let hook = {
                    let ml_reference_array122 = source_ml_temporary_values54.clone();
                    let ml_reference_position122 = source_ml_temporary_index54.clone();
                    self::ml_array::at(
                        &*ml_reference_array122.borrow(),
                        ml_reference_position122.to_index(),
                    )
                };
                if match (hook.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 97, 108, 108, 32, 104, 111, 111, 107, 115]))
                } else {
                    {
                        let error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            hook.clone().expect("a proven present value"),
                        );
                        {
                            let ml_library_inequality_1 =
                                error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_1 {
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Return(error.clone())
                            } else if match hook.clone().expect("a proven present value") {
                                self::JavaScriptSemanticValue::Object(_, _) => false,
                                _ => true,
                            } || !(match match hook
                                .clone()
                                .expect("a proven present value")
                            {
                                self::JavaScriptSemanticValue::Undefined => None::<bool>,
                                self::JavaScriptSemanticValue::Null => None::<bool>,
                                self::JavaScriptSemanticValue::Boolean(_) => None::<bool>,
                                self::JavaScriptSemanticValue::Number(_) => None::<bool>,
                                self::JavaScriptSemanticValue::Integer(_) => None::<bool>,
                                self::JavaScriptSemanticValue::Text(_) => None::<bool>,
                                self::JavaScriptSemanticValue::SourceText(_, _) => None::<bool>,
                                self::JavaScriptSemanticValue::Symbol(_) => None::<bool>,
                                self::JavaScriptSemanticValue::Object(
                                    _,
                                    source_ml_source1_data_property_value17937,
                                ) => Some(source_ml_source1_data_property_value17937),
                            } {
                                Some(ml_present) => ml_present,
                                None => false,
                            }) {
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 97, 108, 108, 32, 104, 111, 111, 107, 115]))
                            } else {
                                {
                                    let source_ml_temporary_index54_2 = source_ml_temporary_index54
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index54 = source_ml_temporary_index54_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptClosureObjectErrorJoin58 {
    MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next,
    MlTemporaryReadJavaScriptClosureObjectErrorJoin58Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptClosureObjectErrorLoop60Result {
    MlTemporaryReadJavaScriptClosureObjectErrorLoop60Done,
    MlTemporaryReadJavaScriptClosureObjectErrorLoop60Return(self::ml_string::Text),
}
