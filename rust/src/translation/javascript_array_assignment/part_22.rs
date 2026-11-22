// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_index;
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
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let object = {
                        let ml_reference_array239 = objects.clone();
                        let ml_reference_position239 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array239.borrow(),
                            self::ml_array::number_index(ml_reference_position239),
                        )
                    };
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(-1f64)
                    } else if (match object.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::ClosureObject(
                            source_ml_source2_data_property_value17215,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value17215.clone(),
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            source_ml_source2_data_property_value17216,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value17216.clone(),
                        self::JavaScriptOrdinaryObject::ArrayObject(
                            source_ml_source2_data_property_value17217,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value17217.clone(),
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            source_ml_source2_data_property_value17218,
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
                        ) => source_ml_source2_data_property_value17218.clone(),
                        self::JavaScriptOrdinaryObject::MapObject(
                            source_ml_source2_data_property_value17219,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value17219.clone(),
                        self::JavaScriptOrdinaryObject::SetObject(
                            source_ml_source2_data_property_value17220,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value17220.clone(),
                        self::JavaScriptOrdinaryObject::WeakMapObject(
                            source_ml_source2_data_property_value17221,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value17221.clone(),
                        self::JavaScriptOrdinaryObject::WeakSetObject(
                            source_ml_source2_data_property_value17222,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value17222.clone(),
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source2_data_property_value17223,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value17223.clone(),
                        self::JavaScriptOrdinaryObject::HostErrorObject(
                            source_ml_source2_data_property_value17224,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value17224.clone(),
                        self::JavaScriptOrdinaryObject::HostURLObject(
                            source_ml_source2_data_property_value17225,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value17225.clone(),
                        self::JavaScriptOrdinaryObject::HostArrayObject(
                            source_ml_source2_data_property_value17226,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value17226.clone(),
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            source_ml_source2_data_property_value17227,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value17227.clone(),
                    }) == identity
                    {
                        self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(middle)
                    } else {
                        if (match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source2_data_property_value17241,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17241.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source2_data_property_value17242,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17242.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source2_data_property_value17243,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17243.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source2_data_property_value17244,
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
                            ) => source_ml_source2_data_property_value17244.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source2_data_property_value17245,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17245.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source2_data_property_value17246,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17246.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source2_data_property_value17247,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17247.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source2_data_property_value17248,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17248.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source2_data_property_value17249,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17249.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source2_data_property_value17250,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17250.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source2_data_property_value17251,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17251.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source2_data_property_value17252,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17252.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source2_data_property_value17253,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17253.clone(),
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
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let symbol = {
                        let ml_reference_array240 = symbols.clone();
                        let ml_reference_position240 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array240.borrow(),
                            self::ml_array::number_index(ml_reference_position240),
                        )
                    };
                    if {
                        let source_ml_source2_comparison_operand17257 = None::<()>;
                        match (symbol.clone(), source_ml_source2_comparison_operand17257) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }
                    } {
                        self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Return(false)
                    } else if (symbol.clone().expect("a proven present value")) == identity {
                        self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Return(true)
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
        return if source_ml_source1_temporary_index14
            < ({
                let ml_reference_array241 = source_ml_source1_temporary_values14.clone();
                self::ml::Big::from_u128(ml_reference_array241.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array242 = source_ml_source1_temporary_values14.clone();
                    let ml_reference_position242 = source_ml_source1_temporary_index14.clone();
                    self::ml_array::at(
                        &*ml_reference_array242.borrow(),
                        ml_reference_position242.to_index(),
                    )
                };
                {
                    let index = self::read_java_script_array_index(match entry {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source2_data_property_value17260,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value17260 =
                                *source_ml_source2_data_property_value17260;
                            source_ml_source2_data_property_value17260.clone()
                        }
                    });
                    if index >= length {
                        self::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Result::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Return(self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 98, 101, 108, 111, 119, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 110, 103, 116, 104]))
                    } else {
                        {
                            let source_ml_source1_temporary_index14_2 =
                                source_ml_source1_temporary_index14
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index14 =
                                    source_ml_source1_temporary_index14_2;
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
        if source_ml_source1_temporary_index19
            < ({
                let ml_reference_array243 = source_ml_source1_temporary_values19.clone();
                self::ml::Big::from_u128(ml_reference_array243.borrow().len() as u128)
            })
        {
            {
                let entry = {
                    let ml_reference_array244 = source_ml_source1_temporary_values19.clone();
                    let ml_reference_position244 = source_ml_source1_temporary_index19.clone();
                    self::ml_array::at(
                        &*ml_reference_array244.borrow(),
                        ml_reference_position244.to_index(),
                    )
                };
                {
                    let source_ml_source1_temporary_plan_java_script_array_ordinary_definition_ignored23 = {
                        let ml_method_owner245 = properties.clone();
                        let ml_method_argument245_0 = if index == at {
                            replacement.clone()
                        } else {
                            entry
                        };
                        let ml_inserted = vec![ml_method_argument245_0];
                        let mut ml_method_values245 = ml_method_owner245.borrow_mut();
                        let ml_length = ml_method_values245
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values245.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            let source_ml_source1_temporary_index19_2 =
                                source_ml_source1_temporary_index19
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (index, source_ml_source1_temporary_index19) =
                                    (index_2, source_ml_source1_temporary_index19_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
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
        if source_ml_source1_temporary_index24
            < ({
                let ml_reference_array246 = source_ml_source1_temporary_values24.clone();
                self::ml::Big::from_u128(ml_reference_array246.borrow().len() as u128)
            })
        {
            {
                let property = {
                    let ml_reference_array247 = source_ml_source1_temporary_values24.clone();
                    let ml_reference_position247 = source_ml_source1_temporary_index24.clone();
                    self::ml_array::at(
                        &*ml_reference_array247.borrow(),
                        ml_reference_position247.to_index(),
                    )
                };
                {
                    let index = self::read_java_script_array_index(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source2_data_property_value17265,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value17265 =
                                *source_ml_source2_data_property_value17265;
                            source_ml_source2_data_property_value17265.clone()
                        }
                    });
                    if ((index >= unsigned_length)
                        && !(({
                            (match property.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    _,
                                    source_ml_source2_data_property_value17266,
                                ) => source_ml_source2_data_property_value17266.clone(),
                            })
                            .borrow()
                            .clone()
                        })
                        .0))
                        && (index > blocked)
                    {
                        {
                            let source_ml_source1_temporary_index24_2 =
                                source_ml_source1_temporary_index24
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (blocked, source_ml_source1_temporary_index24) =
                                    (index, source_ml_source1_temporary_index24_2);
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let source_ml_source1_temporary_index24_3 =
                                source_ml_source1_temporary_index24
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index24 =
                                    source_ml_source1_temporary_index24_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { blocked };
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
        if source_ml_source1_temporary_index25
            < ({
                let ml_reference_array249 = source_ml_source1_temporary_values25.clone();
                self::ml::Big::from_u128(ml_reference_array249.borrow().len() as u128)
            })
        {
            {
                let property = {
                    let ml_reference_array250 = source_ml_source1_temporary_values25.clone();
                    let ml_reference_position250 = source_ml_source1_temporary_index25.clone();
                    self::ml_array::at(
                        &*ml_reference_array250.borrow(),
                        ml_reference_position250.to_index(),
                    )
                };
                {
                    let index = self::read_java_script_array_index(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source2_data_property_value17269,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value17269 =
                                *source_ml_source2_data_property_value17269;
                            source_ml_source2_data_property_value17269.clone()
                        }
                    });
                    if (index < 0f64) || (index < final_length) {
                        {
                            let source_ml_source1_temporary_plan_java_script_array_length_definition_ignored28 = {
                                let ml_method_owner251 = properties.clone();
                                let ml_method_argument251_0 =
                                    if self::test_java_script_array_length_key(
                                        match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                                source_ml_source2_data_property_value17270,
                                                _,
                                            ) => {
                                                let source_ml_source2_data_property_value17270 =
                                                    *source_ml_source2_data_property_value17270;
                                                source_ml_source2_data_property_value17270.clone()
                                            }
                                        },
                                    ) {
                                        {
                                            let source_ml_source2_nominal_field17288 = match property.clone() {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value17289, _) => {
                                            let source_ml_source2_data_property_value17289 = *source_ml_source2_data_property_value17289;
                                            source_ml_source2_data_property_value17289.clone()
                                        }
                                    };
                                            {
                                                let source_ml_source2_nominal_field17290 = match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value17291) => {
                                                source_ml_source2_data_property_value17291.clone()
                                            }
                                        };
                                                {
                                                    let source_ml_source2_nominal_override17292 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            {
                                                                let source_ml_source2_spread17272 = {
                                                                    (match property.clone() {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value17271) => {
                                                        source_ml_source2_data_property_value17271.clone()
                                                    }
                                                }).borrow().clone()
                                                                };
                                                                {
                                                                    let source_ml_source2_spread_field17273 = (source_ml_source2_spread17272.clone()).0;
                                                                    {
                                                                        let source_ml_source2_spread_field17274 = (source_ml_source2_spread17272.clone()).1;
                                                                        {
                                                                            let source_ml_source2_spread_field17275 = (source_ml_source2_spread17272.clone()).2;
                                                                            {
                                                                                let source_ml_source2_spread_field17276 = (source_ml_source2_spread17272.clone()).3;
                                                                                {
                                                                                    let source_ml_source2_spread_field17277 = (source_ml_source2_spread17272.clone()).4;
                                                                                    {
                                                                                        let source_ml_source2_spread_field17278 = (source_ml_source2_spread17272.clone()).5;
                                                                                        {
                                                                                            let source_ml_source2_spread_field17279 = (source_ml_source2_spread17272.clone()).6;
                                                                                            {
                                                                                                let source_ml_source2_spread_field17280 = (source_ml_source2_spread17272.clone()).7;
                                                                                                {
                                                                                                    let source_ml_source2_spread_field17281 = (source_ml_source2_spread17272.clone()).8;
                                                                                                    {
                                                                                                        let source_ml_source2_spread_field17282 = (source_ml_source2_spread17272.clone()).9;
                                                                                                        {
                                                                                                            let source_ml_source2_spread_field17283 = (source_ml_source2_spread17272.clone()).10;
                                                                                                            {
                                                                                                                let source_ml_source2_spread_field17284 = (source_ml_source2_spread17272.clone()).11;
                                                                                                                {
                                                                                                                    let source_ml_source2_field17285 = self::JavaScriptSemanticValue::Number(final_length);
                                                                                                                    (source_ml_source2_spread_field17273, source_ml_source2_spread_field17274, source_ml_source2_spread_field17275, source_ml_source2_spread_field17276, source_ml_source2_spread_field17277, source_ml_source2_spread_field17278, source_ml_source2_spread_field17279, source_ml_source2_spread_field17280, source_ml_source2_field17285, source_ml_source2_spread_field17282, new_writable, source_ml_source2_spread_field17284)
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
                                                            },
                                                        ));
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source2_nominal_field17288), source_ml_source2_nominal_override17292)
                                                }
                                            }
                                        }
                                    } else {
                                        property.clone()
                                    };
                                let ml_inserted = vec![ml_method_argument251_0];
                                let mut ml_method_values251 = ml_method_owner251.borrow_mut();
                                let ml_length = ml_method_values251
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                ml_method_values251.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let source_ml_source1_temporary_index25_2 =
                                    source_ml_source1_temporary_index25
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index25 =
                                        source_ml_source1_temporary_index25_2;
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_source1_temporary_index25_3 =
                                source_ml_source1_temporary_index25
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index25 =
                                    source_ml_source1_temporary_index25_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryPlanJavaScriptArrayLengthDefinitionLoop27Result::MlSource1TemporaryPlanJavaScriptArrayLengthDefinitionLoop27Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result {
    MlSource1TemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Done,
}
