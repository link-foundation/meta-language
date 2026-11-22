// Generated from complete js/src/translation/javascript-collection-iteration.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result;
use super::compare_java_script_collection_keys;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_index;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_unit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_heap_directory_object_loop12(
    mut objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let object = {
                        let ml_reference_array107 = objects.clone();
                        let ml_reference_position107 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array107.borrow(),
                            self::ml_array::number_index(ml_reference_position107),
                        )
                    };
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(-1f64)
                    } else if (match object.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::ClosureObject(
                            source_ml_source1_data_property_value17214,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17214.clone(),
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            source_ml_source1_data_property_value17215,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17215.clone(),
                        self::JavaScriptOrdinaryObject::ArrayObject(
                            source_ml_source1_data_property_value17216,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17216.clone(),
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            source_ml_source1_data_property_value17217,
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
                        ) => source_ml_source1_data_property_value17217.clone(),
                        self::JavaScriptOrdinaryObject::MapObject(
                            source_ml_source1_data_property_value17218,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17218.clone(),
                        self::JavaScriptOrdinaryObject::SetObject(
                            source_ml_source1_data_property_value17219,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17219.clone(),
                        self::JavaScriptOrdinaryObject::WeakMapObject(
                            source_ml_source1_data_property_value17220,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17220.clone(),
                        self::JavaScriptOrdinaryObject::WeakSetObject(
                            source_ml_source1_data_property_value17221,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17221.clone(),
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source1_data_property_value17222,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17222.clone(),
                        self::JavaScriptOrdinaryObject::HostErrorObject(
                            source_ml_source1_data_property_value17223,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17223.clone(),
                        self::JavaScriptOrdinaryObject::HostURLObject(
                            source_ml_source1_data_property_value17224,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17224.clone(),
                        self::JavaScriptOrdinaryObject::HostArrayObject(
                            source_ml_source1_data_property_value17225,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17225.clone(),
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            source_ml_source1_data_property_value17226,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17226.clone(),
                    }) == identity
                    {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(middle)
                    } else {
                        if (match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source1_data_property_value17240,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17240.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source1_data_property_value17241,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17241.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source1_data_property_value17242,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17242.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source1_data_property_value17243,
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
                            ) => source_ml_source1_data_property_value17243.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source1_data_property_value17244,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17244.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source1_data_property_value17245,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17245.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source1_data_property_value17246,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17246.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source1_data_property_value17247,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17247.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value17248,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17248.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source1_data_property_value17249,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17249.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source1_data_property_value17250,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17250.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source1_data_property_value17251,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17251.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source1_data_property_value17252,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17252.clone(),
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
            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result {
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Done,
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_heap_directory_symbol_loop13(
    mut symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let symbol = {
                        let ml_reference_array108 = symbols.clone();
                        let ml_reference_position108 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array108.borrow(),
                            self::ml_array::number_index(ml_reference_position108),
                        )
                    };
                    if {
                        let source_ml_source1_comparison_operand17256 = None::<()>;
                        match (symbol.clone(), source_ml_source1_comparison_operand17256) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }
                    } {
                        self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Return(false)
                    } else if (symbol.clone().expect("a proven present value")) == identity {
                        self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Return(true)
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
            self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptArrayObjectErrorLoop15Result {
    MlTemporaryReadJavaScriptArrayObjectErrorLoop15Done,
    MlTemporaryReadJavaScriptArrayObjectErrorLoop15Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_array_object_error_loop15(
    mut length: f64,
    mut source_ml_temporary_values14: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_temporary_index14: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptArrayObjectErrorLoop15Result {
    loop {
        return if source_ml_temporary_index14
            < ({
                let ml_reference_array109 = source_ml_temporary_values14.clone();
                self::ml::Big::from_u128(ml_reference_array109.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array110 = source_ml_temporary_values14.clone();
                    let ml_reference_position110 = source_ml_temporary_index14.clone();
                    self::ml_array::at(
                        &*ml_reference_array110.borrow(),
                        ml_reference_position110.to_index(),
                    )
                };
                {
                    let index = self::read_java_script_array_index(match entry {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value17259,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value17259 =
                                *source_ml_source1_data_property_value17259;
                            source_ml_source1_data_property_value17259.clone()
                        }
                    });
                    if index >= length {
                        self::MlTemporaryReadJavaScriptArrayObjectErrorLoop15Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop15Return(self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 98, 101, 108, 111, 119, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 110, 103, 116, 104]))
                    } else {
                        {
                            let source_ml_temporary_index14_2 =
                                source_ml_temporary_index14.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index14 = source_ml_temporary_index14_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptArrayObjectErrorLoop15Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop15Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptCollectionKeyJoin32 {
    MlTemporaryTestJavaScriptCollectionKeyJoin32Next,
    MlTemporaryTestJavaScriptCollectionKeyJoin32Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareJavaScriptCollectionKeysLoop35Result {
    MlTemporaryCompareJavaScriptCollectionKeysLoop35Done,
    MlTemporaryCompareJavaScriptCollectionKeysLoop35Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_java_script_collection_keys_loop35(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut index: f64,
) -> self::MlTemporaryCompareJavaScriptCollectionKeysLoop35Result {
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
                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop35Result::MlTemporaryCompareJavaScriptCollectionKeysLoop35Return(if x < y {
                            -1f64
                        } else {
                            1f64
                        })
                    }
                }
            }
        } else {
            self::MlTemporaryCompareJavaScriptCollectionKeysLoop35Result::MlTemporaryCompareJavaScriptCollectionKeysLoop35Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareJavaScriptCollectionKeysJoin34 {
    MlTemporaryCompareJavaScriptCollectionKeysJoin34Next,
    MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(f64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryOrderJavaScriptCollectionKeysLoop37Result {
    MlTemporaryOrderJavaScriptCollectionKeysLoop37Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryOrderJavaScriptCollectionKeysLoop38Result {
    MlTemporaryOrderJavaScriptCollectionKeysLoop38Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryOrderJavaScriptCollectionKeysJoin39 {
    MlTemporaryOrderJavaScriptCollectionKeysJoin39Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryOrderJavaScriptCollectionKeysJoin41 {
    MlTemporaryOrderJavaScriptCollectionKeysJoin41Next,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_collection_keys_loop38(
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut middle: f64,
    mut end: f64,
    mut left: f64,
    mut right: f64,
) -> self::MlTemporaryOrderJavaScriptCollectionKeysLoop38Result {
    loop {
        return if (left < middle) || (right < end) {
            {
                let a = {
                    let ml_reference_array111 = source.clone();
                    let ml_reference_position111 = left;
                    self::ml_array::optional_at(
                        &*ml_reference_array111.borrow(),
                        self::ml_array::number_index(ml_reference_position111),
                    )
                };
                {
                    let b = {
                        let ml_reference_array112 = source.clone();
                        let ml_reference_position112 = right;
                        self::ml_array::optional_at(
                            &*ml_reference_array112.borrow(),
                            self::ml_array::number_index(ml_reference_position112),
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
                                    let source_ml_source1_s17320 = if (match (a.clone(), None::<()>)
                                    {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        self::MlTemporaryOrderJavaScriptCollectionKeysJoin39::MlTemporaryOrderJavaScriptCollectionKeysJoin39Next
                                    } else {
                                        {
                                            let source_ml_temporary_order_java_script_collection_keys_ignored40 = {
                                                let ml_method_owner113 = target.clone();
                                                let ml_method_argument113_0 =
                                                    a.clone().expect("a proven present value");
                                                let ml_inserted = vec![ml_method_argument113_0];
                                                let mut ml_method_values113 =
                                                    ml_method_owner113.borrow_mut();
                                                let ml_length = ml_method_values113
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4_294_967_295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values113.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            self::MlTemporaryOrderJavaScriptCollectionKeysJoin39::MlTemporaryOrderJavaScriptCollectionKeysJoin39Next
                                        }
                                    };
                                    match source_ml_source1_s17320 {
                                        self::MlTemporaryOrderJavaScriptCollectionKeysJoin39::MlTemporaryOrderJavaScriptCollectionKeysJoin39Next => {
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
                                    let source_ml_source1_s17321 = if (match (b.clone(), None::<()>)
                                    {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        self::MlTemporaryOrderJavaScriptCollectionKeysJoin41::MlTemporaryOrderJavaScriptCollectionKeysJoin41Next
                                    } else {
                                        {
                                            let source_ml_temporary_order_java_script_collection_keys_ignored42 = {
                                                let ml_method_owner114 = target.clone();
                                                let ml_method_argument114_0 =
                                                    b.clone().expect("a proven present value");
                                                let ml_inserted = vec![ml_method_argument114_0];
                                                let mut ml_method_values114 =
                                                    ml_method_owner114.borrow_mut();
                                                let ml_length = ml_method_values114
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4_294_967_295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values114.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            self::MlTemporaryOrderJavaScriptCollectionKeysJoin41::MlTemporaryOrderJavaScriptCollectionKeysJoin41Next
                                        }
                                    };
                                    match source_ml_source1_s17321 {
                                        self::MlTemporaryOrderJavaScriptCollectionKeysJoin41::MlTemporaryOrderJavaScriptCollectionKeysJoin41Next => {
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
            self::MlTemporaryOrderJavaScriptCollectionKeysLoop38Result::MlTemporaryOrderJavaScriptCollectionKeysLoop38Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_collection_keys_loop37(
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut width: f64,
    mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut start: f64,
) -> self::MlTemporaryOrderJavaScriptCollectionKeysLoop37Result {
    loop {
        return if start
            < ({
                let ml_reference_array115 = source.clone();
                ml_reference_array115.borrow().len() as f64
            }) {
            {
                let middle = self::ml_math::min((start + width), {
                    let ml_reference_array116 = source.clone();
                    ml_reference_array116.borrow().len() as f64
                });
                {
                    let end = self::ml_math::min((start + (width * 2f64)), {
                        let ml_reference_array117 = source.clone();
                        ml_reference_array117.borrow().len() as f64
                    });
                    {
                        let left = start;
                        {
                            let source_ml_source1_s17322 =
                                self::source_ml_temporary_order_java_script_collection_keys_loop38(
                                    source.clone(),
                                    target.clone(),
                                    middle,
                                    end,
                                    left,
                                    middle,
                                );
                            match source_ml_source1_s17322 {
                                self::MlTemporaryOrderJavaScriptCollectionKeysLoop38Result::MlTemporaryOrderJavaScriptCollectionKeysLoop38Done => {
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
            self::MlTemporaryOrderJavaScriptCollectionKeysLoop37Result::MlTemporaryOrderJavaScriptCollectionKeysLoop37Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_collection_keys_loop36(
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut width: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    loop {
        return if width
            < ({
                let ml_reference_array118 = source.clone();
                ml_reference_array118.borrow().len() as f64
            }) {
            {
                let target = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                    self::JavaScriptSemanticValue,
                >::new()));
                {
                    let start = 0f64;
                    {
                        let source_ml_source1_s17323 =
                            self::source_ml_temporary_order_java_script_collection_keys_loop37(
                                source.clone(),
                                width,
                                target.clone(),
                                start,
                            );
                        match source_ml_source1_s17323 {
                            self::MlTemporaryOrderJavaScriptCollectionKeysLoop37Result::MlTemporaryOrderJavaScriptCollectionKeysLoop37Done => {
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
pub enum MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44 {
    MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44Next,
    MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result {
    MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Done,
    MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_opaque_object_error_loop45(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values43: std::rc::Rc<
        std::cell::RefCell<Vec<Option<self::JavaScriptSemanticValue>>>,
    >,
    mut source_ml_temporary_index43: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result {
    loop {
        return if source_ml_temporary_index43
            < ({
                let ml_reference_array119 = source_ml_temporary_values43.clone();
                self::ml::Big::from_u128(ml_reference_array119.borrow().len() as u128)
            }) {
            {
                let hook = {
                    let ml_reference_array120 = source_ml_temporary_values43.clone();
                    let ml_reference_position120 = source_ml_temporary_index43.clone();
                    self::ml_array::at(
                        &*ml_reference_array120.borrow(),
                        ml_reference_position120.to_index(),
                    )
                };
                if match (hook.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 97, 108, 108, 32, 104, 111, 111, 107, 115]))
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
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(error.clone())
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
                                    source_ml_source1_data_property_value17335,
                                ) => Some(source_ml_source1_data_property_value17335),
                            } {
                                Some(ml_present) => ml_present,
                                None => false,
                            }) {
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 97, 108, 108, 32, 104, 111, 111, 107, 115]))
                            } else {
                                {
                                    let source_ml_temporary_index43_2 = source_ml_temporary_index43
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index43 = source_ml_temporary_index43_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptClosureObjectErrorJoin47 {
    MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next,
    MlTemporaryReadJavaScriptClosureObjectErrorJoin47Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptClosureObjectErrorLoop49Result {
    MlTemporaryReadJavaScriptClosureObjectErrorLoop49Done,
    MlTemporaryReadJavaScriptClosureObjectErrorLoop49Return(self::ml_string::Text),
}
