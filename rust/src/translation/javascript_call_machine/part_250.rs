// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptErrorAllocationDirectory;
use super::JavaScriptErrorAllocationIdentity;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Result;
use super::accept_java_script_whitespace_unit;
use super::find_java_script_heap_directory_object;
use super::match_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_heap_directory_value_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop1257(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut before: self::JavaScriptErrorAllocationDirectory,
    mut previous: self::ml::Big,
    mut source_ml_source1_temporary_values1255: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut source_ml_source1_temporary_index1255: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Result {
    loop {
        return if (source_ml_source1_temporary_index1255
            < ({
                let ml_reference_array3055 = source_ml_source1_temporary_values1255.clone();
                let ml_reference_projection3055 =
                    self::ml::Big::from_u128(ml_reference_array3055.borrow().len() as u128);
                ml_reference_projection3055
            })) {
            {
                let symbol = {
                    let ml_reference_array3056 = source_ml_source1_temporary_values1255.clone();
                    let ml_reference_position3056 = source_ml_source1_temporary_index1255.clone();
                    let ml_reference_projection3056 = self::ml_array::at(
                        &*ml_reference_array3056.borrow(),
                        ml_reference_position3056.to_index(),
                    );
                    ml_reference_projection3056
                };
                if (((symbol <= previous)
                    || (symbol > (self::ml::Big::from_i128(18446744073709551615))))
                    || {
                        let ml_values3057 = {
                            let ml_reference_value3058 = (match before.clone() {
                    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(source_ml_source2_data_property_value41883, _) => {
                        source_ml_source2_data_property_value41883.clone()
                    }
                }).borrow().clone();
                            ml_reference_value3058
                        };
                        let ml_callback3057 = {
                            let symbol = symbol.clone();
                            std::rc::Rc::new(
                                move |row: self::JavaScriptErrorAllocationIdentity| -> bool {
                                    ((match row.clone() {
                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value41885, _, _) => {
                        source_ml_source2_data_property_value41885.clone()
                    }
                }) == symbol)
                                },
                            )
                        };
                        ml_values3057
                            .iter()
                            .cloned()
                            .enumerate()
                            .any(|(ml_index3057, ml_value3057)| ml_callback3057(ml_value3057))
                    })
                {
                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Return(self::ml_string::Text::from_units(vec![112, 114, 105, 111, 114, 32, 83, 121, 109, 98, 111, 108, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    if !(self::ml_array::includes(
                        &{
                            let ml_reference_value3059 = (match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    _,
                                    source_ml_source2_data_property_value41887,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value41887.clone(),
                            })
                            .borrow()
                            .clone();
                            ml_reference_value3059
                        },
                        &symbol.clone(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )) {
                        self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Return(self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                    } else {
                        {
                            let source_ml_source1_temporary_index1255_2 =
                                source_ml_source1_temporary_index1255
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_source1_temporary_index1255) = (
                                    symbol.clone(),
                                    source_ml_source1_temporary_index1255_2.clone(),
                                );
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Result {
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop1256(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut previous: self::ml::Big,
    mut source_ml_source1_temporary_values1254: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptErrorAllocationIdentity>>,
    >,
    mut source_ml_source1_temporary_index1254: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                shared_constructor_field4,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (source_ml_source1_temporary_index1254
                    < ({
                        let ml_reference_array3060 = source_ml_source1_temporary_values1254.clone();
                        let ml_reference_projection3060 =
                            self::ml::Big::from_u128(ml_reference_array3060.borrow().len() as u128);
                        ml_reference_projection3060
                    }))
                {
                    {
                        let row = {
                            let ml_reference_array3061 =
                                source_ml_source1_temporary_values1254.clone();
                            let ml_reference_position3061 =
                                source_ml_source1_temporary_index1254.clone();
                            let ml_reference_projection3061 = self::ml_array::at(
                                &*ml_reference_array3061.borrow(),
                                ml_reference_position3061.to_index(),
                            );
                            ml_reference_projection3061
                        };
                        match row.clone() {
                            self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(
                                shared_constructor_field1,
                                shared_constructor_field2,
                                shared_constructor_field3,
                            ) => {
                                if ((shared_constructor_field1 <= previous)
                                    || (shared_constructor_field1
                                        > (self::ml::Big::from_i128(18446744073709551615))))
                                {
                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Return(self::ml_string::Text::from_units(vec![112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 115, 116, 114, 105, 99, 116, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 111, 114, 100, 101, 114]))
                                } else {
                                    {
                                        let previous_2 = shared_constructor_field1.clone();
                                        {
                                            let object = {
                                                let source_ml_source2_array_read_value41898 =
                                                    shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_array_read_index41899 = self::find_java_script_heap_directory_object(shared_constructor_field4.clone(), shared_constructor_field1.clone());
                                                    {
                                                        let ml_reference_array3062 =
                                                            source_ml_source2_array_read_value41898
                                                                .clone();
                                                        let ml_reference_position3062 =
                                                            source_ml_source2_array_read_index41899;
                                                        let ml_reference_projection3062 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array3062.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position3062,
                                                                ),
                                                            );
                                                        ml_reference_projection3062
                                                    }
                                                }
                                            };
                                            if ((match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                                }
                                            }) != shared_constructor_field2)) || ((match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value41916, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value41916
                                                }
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value41917, _, _, _, _) => {
                                                    source_ml_source2_data_property_value41917
                                                }
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value41918, _) => {
                                                    source_ml_source2_data_property_value41918
                                                }
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value41919, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value41919
                                                }
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value41920, _, _) => {
                                                    source_ml_source2_data_property_value41920
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value41921, _, _) => {
                                                    source_ml_source2_data_property_value41921
                                                }
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value41922, _, _) => {
                                                    source_ml_source2_data_property_value41922
                                                }
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value41923, _, _) => {
                                                    source_ml_source2_data_property_value41923
                                                }
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value41924, _) => {
                                                    source_ml_source2_data_property_value41924
                                                }
                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value41925, _, _, _) => {
                                                    source_ml_source2_data_property_value41925
                                                }
                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value41926, _, _, _) => {
                                                    source_ml_source2_data_property_value41926
                                                }
                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value41927, _, _, _) => {
                                                    source_ml_source2_data_property_value41927
                                                }
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value41928, _, _, _) => {
                                                    source_ml_source2_data_property_value41928
                                                }
                                            }) != shared_constructor_field3)) {
                                                self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Return(self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_index1254_2 = source_ml_source1_temporary_index1254.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (previous, source_ml_source1_temporary_index1254) = (previous_2.clone(), source_ml_source1_temporary_index1254_2.clone());
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
                } else {
                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Done(previous.clone())
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Result {
    MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Done,
    MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop1259(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1258: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1258: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Result {
    loop {
        return if (source_ml_source1_temporary_index1258
            < ({
                let ml_reference_array3063 = source_ml_source1_temporary_values1258.clone();
                let ml_reference_projection3063 =
                    self::ml::Big::from_u128(ml_reference_array3063.borrow().len() as u128);
                ml_reference_projection3063
            })) {
            {
                let value = {
                    let ml_reference_array3064 = source_ml_source1_temporary_values1258.clone();
                    let ml_reference_position3064 = source_ml_source1_temporary_index1258.clone();
                    let ml_reference_projection3064 = self::ml_array::at(
                        &*ml_reference_array3064.borrow(),
                        ml_reference_position3064.to_index(),
                    );
                    ml_reference_projection3064
                };
                {
                    let error = self::read_java_script_heap_directory_value_error(
                        heap.clone(),
                        value.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1258_2 =
                                source_ml_source1_temporary_index1258
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1258 =
                                    source_ml_source1_temporary_index1258_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporarySliceUtf16TextLoop1260Result {
    MlSource1TemporarySliceUtf16TextLoop1260Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_slice_utf16_text_loop1260(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut last: f64,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporarySliceUtf16TextLoop1260Result {
    loop {
        return if (at < last) {
            {
                let source_ml_source1_temporary_slice_utf16_text_ignored1261 = {
                    let ml_method_owner3065 = result.clone();
                    let ml_method_argument3065_0 = ({
                        let ml_reference_array3066 = text.clone();
                        let ml_reference_position3066 = at;
                        let ml_reference_projection3066 = self::ml_array::optional_at(
                            &*ml_reference_array3066.borrow(),
                            self::ml_array::number_index(ml_reference_position3066),
                        );
                        ml_reference_projection3066
                    })
                    .clone()
                    .unwrap_or_else(|| 0f64);
                    let ml_inserted = vec![ml_method_argument3065_0];
                    let mut ml_method_values3065 = ml_method_owner3065.borrow_mut();
                    let ml_length = ml_method_values3065
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3065.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporarySliceUtf16TextLoop1260Result::MlSource1TemporarySliceUtf16TextLoop1260Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryMatchUtf16TextLoop1270Result {
    MlSource1TemporaryMatchUtf16TextLoop1270Done,
    MlSource1TemporaryMatchUtf16TextLoop1270Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_match_utf16_text_loop1270(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut at: f64,
) -> self::MlSource1TemporaryMatchUtf16TextLoop1270Result {
    loop {
        return if (at
            < ({
                let ml_reference_array3067 = search.clone();
                let ml_reference_projection3067 = ml_reference_array3067.borrow().len() as f64;
                ml_reference_projection3067
            })) {
            if {
                let source_ml_source2_comparison_operand41944 = {
                    let source_ml_source2_array_read_index41941 = (start + at);
                    {
                        let ml_reference_array3068 = text.clone();
                        let ml_reference_position3068 = source_ml_source2_array_read_index41941;
                        let ml_reference_projection3068 = self::ml_array::optional_at(
                            &*ml_reference_array3068.borrow(),
                            self::ml_array::number_index(ml_reference_position3068),
                        );
                        ml_reference_projection3068
                    }
                };
                {
                    let source_ml_source2_comparison_operand41945 = {
                        let ml_reference_array3069 = search.clone();
                        let ml_reference_position3069 = at;
                        let ml_reference_projection3069 = self::ml_array::optional_at(
                            &*ml_reference_array3069.borrow(),
                            self::ml_array::number_index(ml_reference_position3069),
                        );
                        ml_reference_projection3069
                    };
                    !(match (
                        source_ml_source2_comparison_operand41944.clone(),
                        source_ml_source2_comparison_operand41945.clone(),
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } {
                self::MlSource1TemporaryMatchUtf16TextLoop1270Result::MlSource1TemporaryMatchUtf16TextLoop1270Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryMatchUtf16TextLoop1270Result::MlSource1TemporaryMatchUtf16TextLoop1270Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryIndexUtf16TextLoop1271Result {
    MlSource1TemporaryIndexUtf16TextLoop1271Done,
    MlSource1TemporaryIndexUtf16TextLoop1271Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_index_utf16_text_loop1271(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryIndexUtf16TextLoop1271Result {
    loop {
        return if (at
            <= ({
                let ml_reference_array3070 = text.clone();
                let ml_reference_projection3070 = ml_reference_array3070.borrow().len() as f64;
                ml_reference_projection3070
            } - {
                let ml_reference_array3071 = search.clone();
                let ml_reference_projection3071 = ml_reference_array3071.borrow().len() as f64;
                ml_reference_projection3071
            })) {
            if self::match_utf16_text(text.clone(), search.clone(), at) {
                self::MlSource1TemporaryIndexUtf16TextLoop1271Result::MlSource1TemporaryIndexUtf16TextLoop1271Return(at)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryIndexUtf16TextLoop1271Result::MlSource1TemporaryIndexUtf16TextLoop1271Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLastIndexUtf16TextLoop1272Result {
    MlSource1TemporaryLastIndexUtf16TextLoop1272Done,
    MlSource1TemporaryLastIndexUtf16TextLoop1272Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_last_index_utf16_text_loop1272(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryLastIndexUtf16TextLoop1272Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text.clone(), search.clone(), at) {
                self::MlSource1TemporaryLastIndexUtf16TextLoop1272Result::MlSource1TemporaryLastIndexUtf16TextLoop1272Return(at)
            } else {
                {
                    let at_2 = (at - 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLastIndexUtf16TextLoop1272Result::MlSource1TemporaryLastIndexUtf16TextLoop1272Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompareUtf16TextLoop1273Result {
    MlSource1TemporaryCompareUtf16TextLoop1273Done,
    MlSource1TemporaryCompareUtf16TextLoop1273Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compare_utf16_text_loop1273(
    mut left: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut right: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut count: f64,
    mut at: f64,
) -> self::MlSource1TemporaryCompareUtf16TextLoop1273Result {
    loop {
        return if (at < count) {
            {
                let a = ({
                    let ml_reference_array3072 = left.clone();
                    let ml_reference_position3072 = at;
                    let ml_reference_projection3072 = self::ml_array::optional_at(
                        &*ml_reference_array3072.borrow(),
                        self::ml_array::number_index(ml_reference_position3072),
                    );
                    ml_reference_projection3072
                })
                .clone()
                .unwrap_or_else(|| 0f64);
                {
                    let b = ({
                        let ml_reference_array3073 = right.clone();
                        let ml_reference_position3073 = at;
                        let ml_reference_projection3073 = self::ml_array::optional_at(
                            &*ml_reference_array3073.borrow(),
                            self::ml_array::number_index(ml_reference_position3073),
                        );
                        ml_reference_projection3073
                    })
                    .clone()
                    .unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlSource1TemporaryCompareUtf16TextLoop1273Result::MlSource1TemporaryCompareUtf16TextLoop1273Return(if (a < b) {
                            (-1f64)
                        } else {
                            1f64
                        })
                    } else {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCompareUtf16TextLoop1273Result::MlSource1TemporaryCompareUtf16TextLoop1273Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_trim_utf16_text_loop1280(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut first: f64,
    mut last: f64,
) -> f64 {
    loop {
        return if ((first < last)
            && self::accept_java_script_whitespace_unit(
                ({
                    let ml_reference_array3074 = text.clone();
                    let ml_reference_position3074 = first;
                    let ml_reference_projection3074 = self::ml_array::optional_at(
                        &*ml_reference_array3074.borrow(),
                        self::ml_array::number_index(ml_reference_position3074),
                    );
                    ml_reference_projection3074
                })
                .clone()
                .unwrap_or_else(|| (-1f64)),
            )) {
            {
                let first_2 = (first + 1f64);
                {
                    first = first_2;
                    continue;
                }
            }
        } else {
            first
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_trim_utf16_text_loop1281(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut first: f64,
    mut last: f64,
) -> f64 {
    loop {
        return if ((last > first)
            && self::accept_java_script_whitespace_unit(
                ({
                    let source_ml_source2_array_read_index41963 = (last - 1f64);
                    {
                        let ml_reference_array3075 = text.clone();
                        let ml_reference_position3075 = source_ml_source2_array_read_index41963;
                        let ml_reference_projection3075 = self::ml_array::optional_at(
                            &*ml_reference_array3075.borrow(),
                            self::ml_array::number_index(ml_reference_position3075),
                        );
                        ml_reference_projection3075
                    }
                })
                .clone()
                .unwrap_or_else(|| (-1f64)),
            )) {
            {
                let last_2 = (last - 1f64);
                {
                    last = last_2;
                    continue;
                }
            }
        } else {
            last
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCharacterUnitsUtf16TextLoop1282Result {
    MlSource1TemporaryCharacterUnitsUtf16TextLoop1282Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_character_units_utf16_text_loop1282(
    mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryCharacterUnitsUtf16TextLoop1282Result {
    loop {
        return if (at
            < ({
                let ml_reference_array3076 = values.clone();
                let ml_reference_projection3076 = ml_reference_array3076.borrow().len() as f64;
                ml_reference_projection3076
            })) {
            {
                let value = ({
                    let ml_reference_array3077 = values.clone();
                    let ml_reference_position3077 = at;
                    let ml_reference_projection3077 = self::ml_array::optional_at(
                        &*ml_reference_array3077.borrow(),
                        self::ml_array::number_index(ml_reference_position3077),
                    );
                    ml_reference_projection3077
                })
                .clone()
                .unwrap_or_else(|| 0f64);
                {
                    let integer = if value.is_finite() {
                        value.trunc()
                    } else {
                        0f64
                    };
                    {
                        let residue = (integer % 65536f64);
                        {
                            let source_ml_source1_temporary_character_units_utf16_text_ignored1283 = {
                                let ml_method_owner3078 = result.clone();
                                let ml_method_argument3078_0 = if (residue < (0f64)) {
                                    (residue + 65536f64)
                                } else {
                                    if (residue == (0f64)) { 0f64 } else { residue }
                                };
                                let ml_inserted = vec![ml_method_argument3078_0];
                                let mut ml_method_values3078 = ml_method_owner3078.borrow_mut();
                                let ml_length = ml_method_values3078
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4294967295usize)
                                    .expect("Invalid array length");
                                ml_method_values3078.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let at_2 = (at + 1f64);
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
            self::MlSource1TemporaryCharacterUnitsUtf16TextLoop1282Result::MlSource1TemporaryCharacterUnitsUtf16TextLoop1282Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryPlanUtf16TextSubstitutionJoin1297 {
    MlSource1TemporaryPlanUtf16TextSubstitutionJoin1297Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryPlanUtf16TextSubstitutionJoin1296 {
    MlSource1TemporaryPlanUtf16TextSubstitutionJoin1296Next,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_utf16_text_substitution_loop1295(
    mut replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut at: f64,
) -> f64 {
    loop {
        return if (at
            < ({
                let ml_reference_array3079 = replacement.clone();
                let ml_reference_projection3079 = ml_reference_array3079.borrow().len() as f64;
                ml_reference_projection3079
            })) {
            {
                let next = ({
                    let source_ml_source2_array_read_index41978 = (at + 1f64);
                    {
                        let ml_reference_array3080 = replacement.clone();
                        let ml_reference_position3080 = source_ml_source2_array_read_index41978;
                        let ml_reference_projection3080 = self::ml_array::optional_at(
                            &*ml_reference_array3080.borrow(),
                            self::ml_array::number_index(ml_reference_position3080),
                        );
                        ml_reference_projection3080
                    }
                })
                .clone()
                .unwrap_or_else(|| (-1f64));
                if (((({
                    let ml_reference_array3081 = replacement.clone();
                    let ml_reference_position3081 = at;
                    let ml_reference_projection3081 = self::ml_array::optional_at(
                        &*ml_reference_array3081.borrow(),
                        self::ml_array::number_index(ml_reference_position3081),
                    );
                    ml_reference_projection3081
                })
                .clone()
                .unwrap_or_else(|| 0f64))
                    == (36f64))
                    && ((((next == (36f64)) || (next == (38f64))) || (next == (96f64)))
                        || (next == (39f64))))
                {
                    {
                        let source_ml_source2_s41983 = if (start < at) {
                            {
                                let source_ml_source1_temporary_plan_utf16_text_substitution_ignored1300 = {
                                    let ml_method_owner3082 = result.clone();
                                    let ml_method_argument3082_0 = 0f64;
                                    let ml_method_argument3082_1 = start;
                                    let ml_method_argument3082_2 = at;
                                    let ml_inserted = vec![
                                        ml_method_argument3082_0,
                                        ml_method_argument3082_1,
                                        ml_method_argument3082_2,
                                    ];
                                    let mut ml_method_values3082 = ml_method_owner3082.borrow_mut();
                                    let ml_length = ml_method_values3082
                                        .len()
                                        .checked_add(ml_inserted.len())
                                        .filter(|length| *length <= 4294967295usize)
                                        .expect("Invalid array length");
                                    ml_method_values3082.extend(ml_inserted);
                                    ml_length as f64
                                };
                                self::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1296::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1296Next
                            }
                        } else {
                            self::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1296::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1296Next
                        };
                        match source_ml_source2_s41983.clone() {
                            self::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1296::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1296Next => {
                                {
                                    let source_ml_source2_s41984 = if (next == (36f64)) {
                                        {
                                            let source_ml_source1_temporary_plan_utf16_text_substitution_ignored1298 = { let ml_method_owner3083 = result.clone(); let ml_method_argument3083_0 = 0f64; let ml_method_argument3083_1 = at; let ml_method_argument3083_2 = (at + 1f64); let ml_inserted = vec![ml_method_argument3083_0, ml_method_argument3083_1, ml_method_argument3083_2]; let mut ml_method_values3083 = ml_method_owner3083.borrow_mut(); let ml_length = ml_method_values3083.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values3083.extend(ml_inserted); ml_length as f64 };
                                            self::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1297::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1297Next
                                        }
                                    } else {
                                        {
                                            let source_ml_source1_temporary_plan_utf16_text_substitution_ignored1299 = { let ml_method_owner3084 = result.clone(); let ml_method_argument3084_0 = if (next == (38f64)) {
                                                1f64
                                            } else {
                                                if (next == (96f64)) {
                                                    2f64
                                                } else {
                                                    3f64
                                                }
                                            }; let ml_method_argument3084_1 = 0f64; let ml_method_argument3084_2 = 0f64; let ml_inserted = vec![ml_method_argument3084_0, ml_method_argument3084_1, ml_method_argument3084_2]; let mut ml_method_values3084 = ml_method_owner3084.borrow_mut(); let ml_length = ml_method_values3084.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values3084.extend(ml_inserted); ml_length as f64 };
                                            self::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1297::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1297Next
                                        }
                                    };
                                    match source_ml_source2_s41984.clone() {
                                        self::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1297::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1297Next => {
                                            {
                                                let at_2 = (at + 2f64);
                                                {
                                                    (start, at) = (at_2, at_2);
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
                    {
                        let at_3 = (at + 1f64);
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            start
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryPlanUtf16TextSubstitutionJoin1301 {
    MlSource1TemporaryPlanUtf16TextSubstitutionJoin1301Next,
}
