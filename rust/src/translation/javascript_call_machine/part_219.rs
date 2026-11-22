// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_index;
use super::source_ml_source3_record_order;
use super::test_java_script_array_length_key;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_heap_directory_object_loop12(
    mut objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result {
    loop {
        return if (low < high) {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let object = {
                        let ml_reference_array2529 = objects.clone();
                        let ml_reference_position2529 = middle;
                        let ml_reference_projection2529 = self::ml_array::optional_at(
                            &*ml_reference_array2529.borrow(),
                            self::ml_array::number_index(ml_reference_position2529),
                        );
                        ml_reference_projection2529
                    };
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source2_data_property_value39025,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39025.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source2_data_property_value39026,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39026.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source2_data_property_value39027,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39027.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source2_data_property_value39028,
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
                            ) => source_ml_source2_data_property_value39028.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source2_data_property_value39029,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39029.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source2_data_property_value39030,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39030.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source2_data_property_value39031,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39031.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source2_data_property_value39032,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39032.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source2_data_property_value39033,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39033.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source2_data_property_value39034,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39034.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source2_data_property_value39035,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39035.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source2_data_property_value39036,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39036.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source2_data_property_value39037,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39037.clone(),
                        }) == identity)
                        {
                            self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    source_ml_source2_data_property_value39051,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39051.clone(),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    source_ml_source2_data_property_value39052,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39052.clone(),
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    source_ml_source2_data_property_value39053,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39053.clone(),
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    source_ml_source2_data_property_value39054,
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
                                ) => source_ml_source2_data_property_value39054.clone(),
                                self::JavaScriptOrdinaryObject::MapObject(
                                    source_ml_source2_data_property_value39055,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39055.clone(),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    source_ml_source2_data_property_value39056,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39056.clone(),
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    source_ml_source2_data_property_value39057,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39057.clone(),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    source_ml_source2_data_property_value39058,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39058.clone(),
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    source_ml_source2_data_property_value39059,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39059.clone(),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    source_ml_source2_data_property_value39060,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39060.clone(),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    source_ml_source2_data_property_value39061,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39061.clone(),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    source_ml_source2_data_property_value39062,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39062.clone(),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    source_ml_source2_data_property_value39063,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39063.clone(),
                            }) < identity)
                            {
                                {
                                    let low_2 = (middle + 1f64);
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
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Result {
    MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Done,
    MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop13(
    mut symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Result {
    loop {
        return if (low < high) {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let symbol = {
                        let ml_reference_array2530 = symbols.clone();
                        let ml_reference_position2530 = middle;
                        let ml_reference_projection2530 = self::ml_array::optional_at(
                            &*ml_reference_array2530.borrow(),
                            self::ml_array::number_index(ml_reference_position2530),
                        );
                        ml_reference_projection2530
                    };
                    if {
                        let source_ml_source2_comparison_operand39067 = None::<()>;
                        match (
                            symbol.clone(),
                            source_ml_source2_comparison_operand39067.clone(),
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }
                    } {
                        self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Return(true)
                        } else {
                            if ((symbol.clone().expect("a proven present value")) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
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
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Result {
    MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Done,
    MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_object_error_loop15(
    mut length: f64,
    mut source_ml_source1_temporary_values14: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_source1_temporary_index14: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Result {
    loop {
        return if (source_ml_source1_temporary_index14
            < ({
                let ml_reference_array2531 = source_ml_source1_temporary_values14.clone();
                let ml_reference_projection2531 =
                    self::ml::Big::from_u128(ml_reference_array2531.borrow().len() as u128);
                ml_reference_projection2531
            })) {
            {
                let entry = {
                    let ml_reference_array2532 = source_ml_source1_temporary_values14.clone();
                    let ml_reference_position2532 = source_ml_source1_temporary_index14.clone();
                    let ml_reference_projection2532 = self::ml_array::at(
                        &*ml_reference_array2532.borrow(),
                        ml_reference_position2532.to_index(),
                    );
                    ml_reference_projection2532
                };
                {
                    let index = self::read_java_script_array_index(match entry.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source2_data_property_value39070,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value39070 =
                                *source_ml_source2_data_property_value39070;
                            source_ml_source2_data_property_value39070.clone()
                        }
                    });
                    if (index >= length) {
                        self::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Result::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Return(self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 98, 101, 108, 111, 119, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 110, 103, 116, 104]))
                    } else {
                        {
                            let source_ml_source1_temporary_index14_2 =
                                source_ml_source1_temporary_index14
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index14 =
                                    source_ml_source1_temporary_index14_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Result::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21 {
    MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Result {
    MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_ordinary_definition_loop20(
    mut at: f64,
    mut replacement: self::JavaScriptOrdinaryProperty,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut index: f64,
    mut source_ml_source1_temporary_values19: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_source1_temporary_index19: self::ml::Big,
) -> self::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Result {
    loop {
        return if (source_ml_source1_temporary_index19
            < ({
                let ml_reference_array2533 = source_ml_source1_temporary_values19.clone();
                let ml_reference_projection2533 =
                    self::ml::Big::from_u128(ml_reference_array2533.borrow().len() as u128);
                ml_reference_projection2533
            })) {
            {
                let entry = {
                    let ml_reference_array2534 = source_ml_source1_temporary_values19.clone();
                    let ml_reference_position2534 = source_ml_source1_temporary_index19.clone();
                    let ml_reference_projection2534 = self::ml_array::at(
                        &*ml_reference_array2534.borrow(),
                        ml_reference_position2534.to_index(),
                    );
                    ml_reference_projection2534
                };
                {
                    let source_ml_source1_temporary_plan_java_script_array_ordinary_definition_ignored23 = {
                        let ml_method_owner2535 = properties.clone();
                        let ml_method_argument2535_0 = if (index == at) {
                            replacement.clone()
                        } else {
                            entry.clone()
                        };
                        let ml_inserted = vec![ml_method_argument2535_0];
                        let mut ml_method_values2535 = ml_method_owner2535.borrow_mut();
                        let ml_length = ml_method_values2535
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4294967295usize)
                            .expect("Invalid array length");
                        ml_method_values2535.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let index_2 = (index + 1f64);
                        {
                            let source_ml_source1_temporary_index19_2 =
                                source_ml_source1_temporary_index19
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (index, source_ml_source1_temporary_index19) =
                                    (index_2, source_ml_source1_temporary_index19_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Result::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_length_definition_loop26(
    mut unsigned_length: f64,
    mut blocked: f64,
    mut source_ml_source1_temporary_values24: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_source1_temporary_index24: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index24
            < ({
                let ml_reference_array2536 = source_ml_source1_temporary_values24.clone();
                let ml_reference_projection2536 =
                    self::ml::Big::from_u128(ml_reference_array2536.borrow().len() as u128);
                ml_reference_projection2536
            })) {
            {
                let property = {
                    let ml_reference_array2537 = source_ml_source1_temporary_values24.clone();
                    let ml_reference_position2537 = source_ml_source1_temporary_index24.clone();
                    let ml_reference_projection2537 = self::ml_array::at(
                        &*ml_reference_array2537.borrow(),
                        ml_reference_position2537.to_index(),
                    );
                    ml_reference_projection2537
                };
                match property.clone() {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        shared_constructor_field1,
                        shared_constructor_field2,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        {
                            let index = self::read_java_script_array_index(
                                shared_constructor_field1.clone(),
                            );
                            if (((index >= unsigned_length)
                                && !((({
                                    let ml_reference_value2538 =
                                        shared_constructor_field2.borrow().clone();
                                    ml_reference_value2538
                                })
                                .0)
                                    .0))
                                && (index > blocked))
                            {
                                {
                                    let source_ml_source1_temporary_index24_2 =
                                        source_ml_source1_temporary_index24
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (blocked, source_ml_source1_temporary_index24) =
                                            (index, source_ml_source1_temporary_index24_2.clone());
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let source_ml_source1_temporary_index24_3 =
                                        source_ml_source1_temporary_index24
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index24 =
                                            source_ml_source1_temporary_index24_3.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            blocked
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayLengthDefinitionLoop27Result {
    MlSource1TemporaryPlanJavaScriptArrayLengthDefinitionLoop27Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_length_definition_loop27(
    mut new_writable: bool,
    mut final_length: f64,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut source_ml_source1_temporary_values25: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_source1_temporary_index25: self::ml::Big,
) -> self::MlSource1TemporaryPlanJavaScriptArrayLengthDefinitionLoop27Result {
    loop {
        return if (source_ml_source1_temporary_index25
            < ({
                let ml_reference_array2539 = source_ml_source1_temporary_values25.clone();
                let ml_reference_projection2539 =
                    self::ml::Big::from_u128(ml_reference_array2539.borrow().len() as u128);
                ml_reference_projection2539
            })) {
            {
                let property = {
                    let ml_reference_array2540 = source_ml_source1_temporary_values25.clone();
                    let ml_reference_position2540 = source_ml_source1_temporary_index25.clone();
                    let ml_reference_projection2540 = self::ml_array::at(
                        &*ml_reference_array2540.borrow(),
                        ml_reference_position2540.to_index(),
                    );
                    ml_reference_projection2540
                };
                match property.clone() {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        shared_constructor_field1,
                        shared_constructor_field2,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        {
                            let index = self::read_java_script_array_index(
                                shared_constructor_field1.clone(),
                            );
                            if ((index < (0f64)) || (index < final_length)) {
                                {
                                    let source_ml_source1_temporary_plan_java_script_array_length_definition_ignored28 = {
                                        let ml_method_owner2541 = properties.clone();
                                        let ml_method_argument2541_0 =
                                            if self::test_java_script_array_length_key(
                                                shared_constructor_field1.clone(),
                                            ) {
                                                {
                                                    let source_ml_source2_nominal_field39098 =
                                                        shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field39100 =
                                                            shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_override39102 =
                                                                std::rc::Rc::new(
                                                                    std::cell::RefCell::new({
                                                                        let source_ml_source2_spread39082 = {
                                                                            let ml_reference_value2542 = shared_constructor_field2.borrow().clone();
                                                                            ml_reference_value2542
                                                                        };
                                                                        {
                                                                            let source_ml_source2_spread_field39083 = ((source_ml_source2_spread39082.clone()).0).0;
                                                                            {
                                                                                let source_ml_source2_spread_field39084 = ((source_ml_source2_spread39082.clone()).0).1;
                                                                                {
                                                                                    let source_ml_source2_spread_field39085 = ((source_ml_source2_spread39082.clone()).0).2;
                                                                                    {
                                                                                        let source_ml_source2_spread_field39086 = ((source_ml_source2_spread39082.clone()).0).3;
                                                                                        {
                                                                                            let source_ml_source2_spread_field39087 = ((source_ml_source2_spread39082.clone()).0).4;
                                                                                            {
                                                                                                let source_ml_source2_spread_field39088 = ((source_ml_source2_spread39082.clone()).0).5;
                                                                                                {
                                                                                                    let source_ml_source2_spread_field39089 = ((source_ml_source2_spread39082.clone()).0).6;
                                                                                                    {
                                                                                                        let source_ml_source2_spread_field39090 = ((source_ml_source2_spread39082.clone()).0).7;
                                                                                                        {
                                                                                                            let source_ml_source2_spread_field39091 = ((source_ml_source2_spread39082.clone()).0).8;
                                                                                                            {
                                                                                                                let source_ml_source2_spread_field39092 = ((source_ml_source2_spread39082.clone()).0).9;
                                                                                                                {
                                                                                                                    let source_ml_source2_spread_field39093 = ((source_ml_source2_spread39082.clone()).0).10;
                                                                                                                    {
                                                                                                                        let source_ml_source2_spread_field39094 = ((source_ml_source2_spread39082.clone()).0).11;
                                                                                                                        {
                                                                                                                            let source_ml_source2_field39095 = self::JavaScriptSemanticValue::Number(final_length);
                                                                                                                            {
                                                                                                                                let source_ml_source3_record_field469 = source_ml_source2_spread_field39083;
                                                                                                                                {
                                                                                                                                    let source_ml_source3_record_field470 = source_ml_source2_spread_field39084;
                                                                                                                                    {
                                                                                                                                        let source_ml_source3_record_field471 = source_ml_source2_spread_field39085;
                                                                                                                                        {
                                                                                                                                            let source_ml_source3_record_field472 = source_ml_source2_spread_field39086;
                                                                                                                                            {
                                                                                                                                                let source_ml_source3_record_field473 = source_ml_source2_spread_field39087.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source3_record_field474 = source_ml_source2_spread_field39088;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source3_record_field475 = source_ml_source2_spread_field39089.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source3_record_field476 = source_ml_source2_spread_field39090;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source3_record_field477 = source_ml_source2_field39095.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source3_record_field478 = source_ml_source2_spread_field39092;
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source3_record_field479 = new_writable;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source3_record_field480 = source_ml_source2_spread_field39094;
                                                                                                                                                                            ((source_ml_source3_record_field469, source_ml_source3_record_field470, source_ml_source3_record_field471, source_ml_source3_record_field472, source_ml_source3_record_field473.clone(), source_ml_source3_record_field474, source_ml_source3_record_field475.clone(), source_ml_source3_record_field476, source_ml_source3_record_field477.clone(), source_ml_source3_record_field478, source_ml_source3_record_field479, source_ml_source3_record_field480), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], self::source_ml_source3_record_order::merge_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![self::source_ml_source3_record_order::rebase_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], (source_ml_source2_spread39082.clone()).1.clone()), vec![8f64], vec![10f64]])))
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
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }),
                                                                );
                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source2_nominal_field39098.clone()), source_ml_source2_nominal_override39102.clone())
                                                        }
                                                    }
                                                }
                                            } else {
                                                property.clone()
                                            };
                                        let ml_inserted = vec![ml_method_argument2541_0];
                                        let mut ml_method_values2541 =
                                            ml_method_owner2541.borrow_mut();
                                        let ml_length = ml_method_values2541
                                            .len()
                                            .checked_add(ml_inserted.len())
                                            .filter(|length| *length <= 4294967295usize)
                                            .expect("Invalid array length");
                                        ml_method_values2541.extend(ml_inserted);
                                        ml_length as f64
                                    };
                                    {
                                        let source_ml_source1_temporary_index25_2 =
                                            source_ml_source1_temporary_index25
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index25 =
                                                source_ml_source1_temporary_index25_2.clone();
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let source_ml_source1_temporary_index25_3 =
                                        source_ml_source1_temporary_index25
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index25 =
                                            source_ml_source1_temporary_index25_3.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptArrayLengthDefinitionLoop27Result::MlSource1TemporaryPlanJavaScriptArrayLengthDefinitionLoop27Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result {
    MlSource1TemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Done,
}
