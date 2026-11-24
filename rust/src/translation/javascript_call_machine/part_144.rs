// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallArgumentList;
use super::JavaScriptCallArgumentListsUpdate;
use super::JavaScriptCallIteratorRegion;
use super::JavaScriptIteratorRegionsUpdate;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Result;
use super::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Result;
use super::find_java_script_iterator_region;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_heap_directory_value_error;
use super::read_java_script_iterator_regions_error;
use super::read_java_script_synchronous_iterator_record_error;
use super::source_ml_source1_temporary_append_java_script_call_argument_loop1222;
use super::source_ml_source1_temporary_count_java_script_call_argument_list_values_loop1214;
use super::source_ml_source1_temporary_find_java_script_call_argument_list_loop1212;
use super::source_ml_source1_temporary_insert_java_script_iterator_region_loop1210;
use super::source_ml_source1_temporary_open_java_script_call_argument_list_loop1220;
use super::source_ml_source1_temporary_read_java_script_call_argument_lists_error_loop1217;
use super::test_java_script_iterator_region_slot;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn insert_java_script_iterator_region(
    heap: self::JavaScriptOrdinaryHeap,
    regions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>,
    slot: f64,
    record: self::JavaScriptSynchronousIteratorRecord,
) -> self::JavaScriptIteratorRegionsUpdate {
    match record.clone() {
        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let error =
                    self::read_java_script_iterator_regions_error(heap.clone(), regions.clone());
                {
                    let invalid = self::read_java_script_synchronous_iterator_record_error(
                        heap.clone(),
                        record.clone(),
                    );
                    if (((error != (self::ml_string::Text::from_units(vec![])))
                        || (invalid != (self::ml_string::Text::from_units(vec![]))))
                        || !(self::test_java_script_iterator_region_slot(slot)))
                    {
                        self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid({
                            let source_ml_source2_logical27113 = if !error.0.is_empty() {
                                error.clone()
                            } else {
                                invalid.clone()
                            };
                            if !source_ml_source2_logical27113.0.is_empty() {
                                source_ml_source2_logical27113.clone()
                            } else {
                                self::ml_string::Text::from_units(vec![
                                    105, 116, 101, 114, 97, 116, 111, 114, 32, 111, 112, 101, 110,
                                    105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                    116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117,
                                    114, 99, 101, 32, 115, 108, 111, 116,
                                ])
                            }
                        })
                    } else {
                        if ((self::find_java_script_iterator_region(regions.clone(), slot))
                            >= (0f64))
                        {
                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 97, 99, 116, 105, 118, 101, 32, 115, 111, 117,
                                    114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32,
                                    114, 101, 103, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116,
                                    32, 98, 101, 32, 111, 118, 101, 114, 119, 114, 105, 116, 116,
                                    101, 110, 32, 98, 121, 32, 97, 110, 111, 116, 104, 101, 114,
                                    32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110,
                                ]),
                            )
                        } else {
                            if (({
                                let ml_reference_array1503 = regions.clone();
                                let ml_reference_projection1503 =
                                    ml_reference_array1503.borrow().len() as f64;
                                ml_reference_projection1503
                            }) >= (128f64))
                            {
                                self::JavaScriptIteratorRegionsUpdate::IteratorRegionsResourceLimit(
                                    self::ml_string::Text::from_units(vec![
                                        105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 103,
                                        105, 111, 110, 32, 111, 112, 101, 110, 105, 110, 103, 32,
                                        101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32,
                                        97, 99, 116, 117, 97, 108, 32, 112, 101, 114, 45, 102, 114,
                                        97, 109, 101, 32, 110, 101, 115, 116, 105, 110, 103, 32,
                                        98, 117, 100, 103, 101, 116,
                                    ]),
                                )
                            } else {
                                {
                                    let at = 0f64;
                                    {
                                        let at_2 = self::source_ml_source1_temporary_insert_java_script_iterator_region_loop1210(regions.clone(), slot, at);
                                        self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value27121 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1504 = regions.borrow().clone(); ml_reference_value1504 }, 0f64, at_2)));
                                            {
                                                let source_ml_source2_concat_value27122 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallIteratorRegion::CallIteratorRegion(slot, Box::new({
                                                    let source_ml_source2_nominal_field27115 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27117 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27119 = shared_constructor_field3;
                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(Box::new(source_ml_source2_nominal_field27115.clone()), Box::new(source_ml_source2_nominal_field27117.clone()), source_ml_source2_nominal_field27119)
                                                        }
                                                    }
                                                }))]));
                                                {
                                                    let source_ml_source2_concat_value27123 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1505 = regions.borrow().clone(); ml_reference_value1505 }, at_2, f64::INFINITY)));
                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallIteratorRegion>::new(), { let ml_reference_value1506 = source_ml_source2_concat_value27121.borrow().clone(); ml_reference_value1506 }), { let ml_reference_value1507 = source_ml_source2_concat_value27122.borrow().clone(); ml_reference_value1507 }), { let ml_reference_value1508 = source_ml_source2_concat_value27123.borrow().clone(); ml_reference_value1508 })
                                                }
                                            }
                                        })))
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
pub fn update_java_script_iterator_region(
    heap: self::JavaScriptOrdinaryHeap,
    regions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>,
    slot: f64,
    record: self::JavaScriptSynchronousIteratorRecord,
) -> self::JavaScriptIteratorRegionsUpdate {
    match record.clone() {
        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let error =
                    self::read_java_script_iterator_regions_error(heap.clone(), regions.clone());
                {
                    let invalid = self::read_java_script_synchronous_iterator_record_error(
                        heap.clone(),
                        record.clone(),
                    );
                    if (((error != (self::ml_string::Text::from_units(vec![])))
                        || (invalid != (self::ml_string::Text::from_units(vec![]))))
                        || !(self::test_java_script_iterator_region_slot(slot)))
                    {
                        self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid({
                            let source_ml_source2_logical27125 = if !error.0.is_empty() {
                                error.clone()
                            } else {
                                invalid.clone()
                            };
                            if !source_ml_source2_logical27125.0.is_empty() {
                                source_ml_source2_logical27125.clone()
                            } else {
                                self::ml_string::Text::from_units(vec![
                                    105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 100, 118, 97,
                                    110, 99, 101, 109, 101, 110, 116, 32, 114, 101, 116, 97, 105,
                                    110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                    115, 111, 117, 114, 99, 101, 32, 115, 108, 111, 116,
                                ])
                            }
                        })
                    } else {
                        {
                            let index =
                                self::find_java_script_iterator_region(regions.clone(), slot);
                            {
                                let prior = {
                                    let ml_reference_array1509 = regions.clone();
                                    let ml_reference_position1509 = index;
                                    let ml_reference_projection1509 = self::ml_array::optional_at(
                                        &*ml_reference_array1509.borrow(),
                                        self::ml_array::number_index(ml_reference_position1509),
                                    );
                                    ml_reference_projection1509
                                };
                                if match (prior.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(
                                        self::ml_string::Text::from_units(vec![
                                            105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 100,
                                            118, 97, 110, 99, 101, 109, 101, 110, 116, 32, 114,
                                            101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115,
                                            32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 110,
                                            101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114,
                                            101, 103, 105, 111, 110,
                                        ]),
                                    )
                                } else {
                                    if ((!(self::test_java_script_semantic_same_value(match match prior.clone().expect("a proven present value") {
                                        self::JavaScriptCallIteratorRegion::CallIteratorRegion(_, source_ml_source2_data_property_value27128) => {
                                            let source_ml_source2_data_property_value27128 = *source_ml_source2_data_property_value27128;
                                            source_ml_source2_data_property_value27128.clone()
                                        }
                                    } {
                                        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value27129, _, _) => {
                                            let source_ml_source2_data_property_value27129 = *source_ml_source2_data_property_value27129;
                                            source_ml_source2_data_property_value27129.clone()
                                        }
                                    }, shared_constructor_field1.clone())) || !(self::test_java_script_semantic_same_value(match match prior.clone().expect("a proven present value") {
                                        self::JavaScriptCallIteratorRegion::CallIteratorRegion(_, source_ml_source2_data_property_value27131) => {
                                            let source_ml_source2_data_property_value27131 = *source_ml_source2_data_property_value27131;
                                            source_ml_source2_data_property_value27131.clone()
                                        }
                                    } {
                                        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value27132, _) => {
                                            let source_ml_source2_data_property_value27132 = *source_ml_source2_data_property_value27132;
                                            source_ml_source2_data_property_value27132.clone()
                                        }
                                    }, shared_constructor_field2.clone()))) || (match match prior.clone().expect("a proven present value") {
                                        self::JavaScriptCallIteratorRegion::CallIteratorRegion(_, source_ml_source2_data_property_value27134) => {
                                            let source_ml_source2_data_property_value27134 = *source_ml_source2_data_property_value27134;
                                            source_ml_source2_data_property_value27134.clone()
                                        }
                                    } {
                                        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value27135) => {
                                            source_ml_source2_data_property_value27135
                                        }
                                    } && !shared_constructor_field3)) {
                                        self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 100, 118, 97, 110, 99, 101, 109, 101, 110, 116, 32, 112, 114, 101, 115, 101, 114, 118, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 99, 104, 101, 100, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 97, 110, 100, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 118, 105, 118, 101, 32, 101, 120, 104, 97, 117, 115, 116, 105, 111, 110]))
                                    } else {
                                        self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values1510 = { let ml_reference_value1511 = regions.borrow().clone(); ml_reference_value1511 }; let ml_callback1510 = { let index = index; let record = record.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallIteratorRegion, at: f64| -> self::JavaScriptCallIteratorRegion { if (at == index) {
                                            {
                                                let source_ml_source2_nominal_spread27144 = row.clone();
                                                {
                                                    let source_ml_source2_nominal_field27145 = match source_ml_source2_nominal_spread27144.clone() {
                                                        self::JavaScriptCallIteratorRegion::CallIteratorRegion(source_ml_source2_data_property_value27146, _) => {
                                                            source_ml_source2_data_property_value27146
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field27147 = match source_ml_source2_nominal_spread27144.clone() {
                                                            self::JavaScriptCallIteratorRegion::CallIteratorRegion(_, source_ml_source2_data_property_value27148) => {
                                                                let source_ml_source2_data_property_value27148 = *source_ml_source2_data_property_value27148;
                                                                source_ml_source2_data_property_value27148.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override27149 = {
                                                                let source_ml_source2_nominal_spread27137 = record.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field27138 = match source_ml_source2_nominal_spread27137.clone() {
                                                                        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value27139, _, _) => {
                                                                            let source_ml_source2_data_property_value27139 = *source_ml_source2_data_property_value27139;
                                                                            source_ml_source2_data_property_value27139.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field27140 = match source_ml_source2_nominal_spread27137.clone() {
                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value27141, _) => {
                                                                                let source_ml_source2_data_property_value27141 = *source_ml_source2_data_property_value27141;
                                                                                source_ml_source2_data_property_value27141.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field27142 = match source_ml_source2_nominal_spread27137.clone() {
                                                                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value27143) => {
                                                                                    source_ml_source2_data_property_value27143
                                                                                }
                                                                            };
                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(Box::new(source_ml_source2_nominal_field27138.clone()), Box::new(source_ml_source2_nominal_field27140.clone()), source_ml_source2_nominal_field27142)
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            self::JavaScriptCallIteratorRegion::CallIteratorRegion(source_ml_source2_nominal_field27145, Box::new(source_ml_source2_nominal_override27149.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            row.clone()
                                        } }) }; ml_values1510.iter().cloned().enumerate().map(|(ml_index1510, ml_value1510)| ml_callback1510(ml_value1510, (ml_index1510 as f64))).collect::<Vec<_>>() })))
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
pub fn remove_java_script_iterator_region(
    heap: self::JavaScriptOrdinaryHeap,
    regions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>,
    slot: f64,
) -> self::JavaScriptIteratorRegionsUpdate {
    {
        let error = self::read_java_script_iterator_regions_error(heap.clone(), regions.clone());
        if ((error != (self::ml_string::Text::from_units(vec![])))
            || !(self::test_java_script_iterator_region_slot(slot)))
        {
            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(if !error.0.is_empty() {
                error.clone()
            } else {
                self::ml_string::Text::from_units(vec![
                    105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 103, 105, 111, 110, 32,
                    114, 101, 108, 101, 97, 115, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101,
                    32, 115, 108, 111, 116,
                ])
            })
        } else {
            {
                let index = self::find_java_script_iterator_region(regions.clone(), slot);
                if (index < (0f64)) {
                    self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(
                        self::ml_string::Text::from_units(vec![
                            105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 108, 101, 97, 115,
                            101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32,
                            97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 110, 101, 100, 32, 115,
                            111, 117, 114, 99, 101, 32, 114, 101, 103, 105, 111, 110,
                        ]),
                    )
                } else {
                    self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(std::rc::Rc::new(
                        std::cell::RefCell::new({
                            let source_ml_source2_concat_value27151 =
                                std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(
                                    &{
                                        let ml_reference_value1512 = regions.borrow().clone();
                                        ml_reference_value1512
                                    },
                                    0f64,
                                    index,
                                )));
                            {
                                let source_ml_source2_concat_value27152 = std::rc::Rc::new(
                                    std::cell::RefCell::new(self::ml_array::slice(
                                        &{
                                            let ml_reference_value1513 = regions.borrow().clone();
                                            ml_reference_value1513
                                        },
                                        (index + 1f64),
                                        f64::INFINITY,
                                    )),
                                );
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::JavaScriptCallIteratorRegion>::new(),
                                        {
                                            let ml_reference_value1514 =
                                                source_ml_source2_concat_value27151
                                                    .borrow()
                                                    .clone();
                                            ml_reference_value1514
                                        },
                                    ),
                                    {
                                        let ml_reference_value1515 =
                                            source_ml_source2_concat_value27152.borrow().clone();
                                        ml_reference_value1515
                                    },
                                )
                            }
                        }),
                    ))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_argument_list(
    lists: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>,
    slot: f64,
) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source2_s27153 =
                self::source_ml_source1_temporary_find_java_script_call_argument_list_loop1212(
                    lists.clone(),
                    slot,
                    at,
                );
            match source_ml_source2_s27153.clone() {
                self::MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Result::MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Result::MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Return(source_ml_source1_temporary_result1212) => {
                    source_ml_source1_temporary_result1212
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn count_java_script_call_argument_list_values(
    lists: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>,
) -> f64 {
    {
        let count = 0f64;
        {
            let source_ml_source1_temporary_index1213 = self::ml::Big::from_i128(0);
            {
                let count_2 = self::source_ml_source1_temporary_count_java_script_call_argument_list_values_loop1214(count, lists.clone(), source_ml_source1_temporary_index1213.clone());
                count_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_argument_lists_error(
    heap: self::JavaScriptOrdinaryHeap,
    lists: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>,
    depth: f64,
) -> self::ml_string::Text {
    if ((((!(self::ml_math::is_safe_integer(depth)) || (depth < (0f64))) || (depth > (1024f64)))
        || {
            let source_ml_source2_same_value_operand27155 = (-0f64);
            (((depth == source_ml_source2_same_value_operand27155)
                && ((depth != (0f64))
                    || ((1f64 / depth) == (1f64 / source_ml_source2_same_value_operand27155))))
                || ((depth != depth)
                    && (source_ml_source2_same_value_operand27155
                        != source_ml_source2_same_value_operand27155)))
        })
        || (({
            let ml_reference_array1516 = lists.clone();
            let ml_reference_projection1516 = ml_reference_array1516.borrow().len() as f64;
            ml_reference_projection1516
        }) > (128f64)))
    {
        self::ml_string::Text::from_units(vec![
            97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115, 32, 114, 101, 116,
            97, 105, 110, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32,
            104, 97, 110, 100, 108, 101, 114, 32, 100, 101, 112, 116, 104, 32, 97, 110, 100, 32,
            110, 101, 115, 116, 105, 110, 103,
        ])
    } else {
        {
            let previous = (-1f64);
            {
                let count = 0f64;
                {
                    let source_ml_source1_temporary_index1215 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s27156 = self::source_ml_source1_temporary_read_java_script_call_argument_lists_error_loop1217(heap.clone(), depth, previous, count, lists.clone(), source_ml_source1_temporary_index1215.clone());
                        match source_ml_source2_s27156.clone() {
                            self::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Result::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Result::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Return(source_ml_source1_temporary_result1217) => {
                                source_ml_source1_temporary_result1217.clone()
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
pub fn open_java_script_call_argument_list(
    heap: self::JavaScriptOrdinaryHeap,
    lists: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>,
    slot: f64,
    depth: f64,
) -> self::JavaScriptCallArgumentListsUpdate {
    {
        let error =
            self::read_java_script_call_argument_lists_error(heap.clone(), lists.clone(), depth);
        if ((error != (self::ml_string::Text::from_units(vec![])))
            || !(self::test_java_script_iterator_region_slot(slot)))
        {
            self::JavaScriptCallArgumentListsUpdate::ArgumentListsInvalid(if !error.0.is_empty() {
                error.clone()
            } else {
                self::ml_string::Text::from_units(vec![
                    97, 114, 103, 117, 109, 101, 110, 116, 32, 111, 112, 101, 110, 105, 110, 103,
                    32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                    117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 115, 108, 111, 116,
                ])
            })
        } else {
            if ((self::find_java_script_call_argument_list(lists.clone(), slot)) >= (0f64)) {
                self::JavaScriptCallArgumentListsUpdate::ArgumentListsInvalid(
                    self::ml_string::Text::from_units(vec![
                        97, 114, 103, 117, 109, 101, 110, 116, 32, 111, 112, 101, 110, 105, 110,
                        103, 32, 99, 97, 110, 110, 111, 116, 32, 111, 118, 101, 114, 119, 114, 105,
                        116, 101, 32, 97, 110, 32, 117, 110, 102, 105, 110, 105, 115, 104, 101,
                        100, 32, 115, 111, 117, 114, 99, 101, 32, 67, 97, 108, 108,
                    ]),
                )
            } else {
                if (({
                    let ml_reference_array1517 = lists.clone();
                    let ml_reference_projection1517 = ml_reference_array1517.borrow().len() as f64;
                    ml_reference_projection1517
                }) >= (128f64))
                {
                    self::JavaScriptCallArgumentListsUpdate::ArgumentListsResourceLimit(
                        self::ml_string::Text::from_units(vec![
                            97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115, 32,
                            101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99,
                            116, 117, 97, 108, 32, 102, 114, 97, 109, 101, 32, 110, 101, 115, 116,
                            105, 110, 103, 32, 98, 117, 100, 103, 101, 116,
                        ]),
                    )
                } else {
                    {
                        let at = 0f64;
                        {
                            let at_2 = self::source_ml_source1_temporary_open_java_script_call_argument_list_loop1220(lists.clone(), slot, at);
                            self::JavaScriptCallArgumentListsUpdate::ArgumentListsUpdated(
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_concat_value27158 = std::rc::Rc::new(
                                        std::cell::RefCell::new(self::ml_array::slice(
                                            &{
                                                let ml_reference_value1518 = lists.borrow().clone();
                                                ml_reference_value1518
                                            },
                                            0f64,
                                            at_2,
                                        )),
                                    );
                                    {
                                        let source_ml_source2_concat_value27159 =
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                self::JavaScriptCallArgumentList::CallArgumentList(
                                                    slot,
                                                    depth,
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        Vec::<self::JavaScriptSemanticValue>::new(),
                                                    )),
                                                ),
                                            ]));
                                        {
                                            let source_ml_source2_concat_value27160 =
                                                std::rc::Rc::new(std::cell::RefCell::new(
                                                    self::ml_array::slice(
                                                        &{
                                                            let ml_reference_value1519 =
                                                                lists.borrow().clone();
                                                            ml_reference_value1519
                                                        },
                                                        at_2,
                                                        f64::INFINITY,
                                                    ),
                                                ));
                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallArgumentList>::new(), { let ml_reference_value1520 = source_ml_source2_concat_value27158.borrow().clone(); ml_reference_value1520 }), { let ml_reference_value1521 = source_ml_source2_concat_value27159.borrow().clone(); ml_reference_value1521 }), { let ml_reference_value1522 = source_ml_source2_concat_value27160.borrow().clone(); ml_reference_value1522 })
                                        }
                                    }
                                })),
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_call_argument(
    heap: self::JavaScriptOrdinaryHeap,
    lists: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>,
    slot: f64,
    depth: f64,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallArgumentListsUpdate {
    {
        let error =
            self::read_java_script_call_argument_lists_error(heap.clone(), lists.clone(), depth);
        {
            let invalid =
                self::read_java_script_heap_directory_value_error(heap.clone(), value.clone());
            {
                let at = self::find_java_script_call_argument_list(lists.clone(), slot);
                {
                    let list = {
                        let ml_reference_array1523 = lists.clone();
                        let ml_reference_position1523 = at;
                        let ml_reference_projection1523 = self::ml_array::optional_at(
                            &*ml_reference_array1523.borrow(),
                            self::ml_array::number_index(ml_reference_position1523),
                        );
                        ml_reference_projection1523
                    };
                    if (((error != (self::ml_string::Text::from_units(vec![])))
                        || (invalid != (self::ml_string::Text::from_units(vec![]))))
                        || match (list.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        })
                    {
                        self::JavaScriptCallArgumentListsUpdate::ArgumentListsInvalid({
                            let source_ml_source2_logical27164 = if !error.0.is_empty() {
                                error.clone()
                            } else {
                                invalid.clone()
                            };
                            if !source_ml_source2_logical27164.0.is_empty() {
                                source_ml_source2_logical27164.clone()
                            } else {
                                self::ml_string::Text::from_units(vec![
                                    97, 114, 103, 117, 109, 101, 110, 116, 32, 97, 112, 112, 101,
                                    110, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105,
                                    116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101,
                                    110, 101, 100, 32, 67, 97, 108, 108, 32, 76, 105, 115, 116,
                                ])
                            }
                        })
                    } else {
                        {
                            let count = 1f64;
                            {
                                let source_ml_source1_temporary_values1221 = lists.clone();
                                {
                                    let source_ml_source1_temporary_index1221 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let count_2 = self::source_ml_source1_temporary_append_java_script_call_argument_loop1222(count, source_ml_source1_temporary_values1221.clone(), source_ml_source1_temporary_index1221.clone());
                                        if (count_2 > (1048576f64)) {
                                            self::JavaScriptCallArgumentListsUpdate::ArgumentListsResourceLimit(self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 97, 112, 112, 101, 110, 100, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            self::JavaScriptCallArgumentListsUpdate::ArgumentListsUpdated(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values1524 = { let ml_reference_value1525 = lists.borrow().clone(); ml_reference_value1525 }; let ml_callback1524 = { let at = at; let value = value.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallArgumentList, index: f64| -> self::JavaScriptCallArgumentList { if (index == at) {
                                                {
                                                    let source_ml_source2_nominal_spread27168 = row.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27169 = match source_ml_source2_nominal_spread27168.clone() {
                                                            self::JavaScriptCallArgumentList::CallArgumentList(source_ml_source2_data_property_value27170, _, _) => {
                                                                source_ml_source2_data_property_value27170
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field27171 = match source_ml_source2_nominal_spread27168.clone() {
                                                                self::JavaScriptCallArgumentList::CallArgumentList(_, source_ml_source2_data_property_value27172, _) => {
                                                                    source_ml_source2_data_property_value27172
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field27173 = match source_ml_source2_nominal_spread27168.clone() {
                                                                    self::JavaScriptCallArgumentList::CallArgumentList(_, _, source_ml_source2_data_property_value27174) => {
                                                                        source_ml_source2_data_property_value27174.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override27175 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source2_concat_value27166 = match row.clone() {
                                                                            self::JavaScriptCallArgumentList::CallArgumentList(_, _, source_ml_source2_data_property_value27165) => {
                                                                                source_ml_source2_data_property_value27165.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_concat_value27167 = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()]));
                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), { let ml_reference_value1526 = source_ml_source2_concat_value27166.borrow().clone(); ml_reference_value1526 }), { let ml_reference_value1527 = source_ml_source2_concat_value27167.borrow().clone(); ml_reference_value1527 })
                                                                        }
                                                                    }));
                                                                    self::JavaScriptCallArgumentList::CallArgumentList(source_ml_source2_nominal_field27169, source_ml_source2_nominal_field27171, source_ml_source2_nominal_override27175.clone())
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                row.clone()
                                            } }) }; ml_values1524.iter().cloned().enumerate().map(|(ml_index1524, ml_value1524)| ml_callback1524(ml_value1524, (ml_index1524 as f64))).collect::<Vec<_>>() })))
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
pub fn retain_java_script_call_argument_lists(
    lists: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>,
    depth: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let ml_values1528 = {
            let ml_reference_value1529 = lists.borrow().clone();
            ml_reference_value1529
        };
        let ml_callback1528 = {
            let depth = depth;
            std::rc::Rc::new(move |list: self::JavaScriptCallArgumentList| -> bool {
                ((match list.clone() {
                    self::JavaScriptCallArgumentList::CallArgumentList(
                        _,
                        source_ml_source2_data_property_value27177,
                        _,
                    ) => source_ml_source2_data_property_value27177,
                }) <= depth)
            })
        };
        ml_values1528
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(ml_index1528, ml_value1528)| {
                if ml_callback1528(ml_value1528.clone()) {
                    Some(ml_value1528)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }))
}
