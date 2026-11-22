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
                            source_ml_source2_data_property_value18525,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18525.clone(),
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            source_ml_source2_data_property_value18526,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18526.clone(),
                        self::JavaScriptOrdinaryObject::ArrayObject(
                            source_ml_source2_data_property_value18527,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18527.clone(),
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            source_ml_source2_data_property_value18528,
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
                        ) => source_ml_source2_data_property_value18528.clone(),
                        self::JavaScriptOrdinaryObject::MapObject(
                            source_ml_source2_data_property_value18529,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18529.clone(),
                        self::JavaScriptOrdinaryObject::SetObject(
                            source_ml_source2_data_property_value18530,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18530.clone(),
                        self::JavaScriptOrdinaryObject::WeakMapObject(
                            source_ml_source2_data_property_value18531,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18531.clone(),
                        self::JavaScriptOrdinaryObject::WeakSetObject(
                            source_ml_source2_data_property_value18532,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18532.clone(),
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source2_data_property_value18533,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18533.clone(),
                        self::JavaScriptOrdinaryObject::HostErrorObject(
                            source_ml_source2_data_property_value18534,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18534.clone(),
                        self::JavaScriptOrdinaryObject::HostURLObject(
                            source_ml_source2_data_property_value18535,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18535.clone(),
                        self::JavaScriptOrdinaryObject::HostArrayObject(
                            source_ml_source2_data_property_value18536,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18536.clone(),
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            source_ml_source2_data_property_value18537,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18537.clone(),
                    }) == identity
                    {
                        self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(middle)
                    } else {
                        if (match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source2_data_property_value18551,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18551.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source2_data_property_value18552,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18552.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source2_data_property_value18553,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18553.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source2_data_property_value18554,
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
                            ) => source_ml_source2_data_property_value18554.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source2_data_property_value18555,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18555.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source2_data_property_value18556,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18556.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source2_data_property_value18557,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18557.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source2_data_property_value18558,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18558.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source2_data_property_value18559,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18559.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source2_data_property_value18560,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18560.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source2_data_property_value18561,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18561.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source2_data_property_value18562,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18562.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source2_data_property_value18563,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18563.clone(),
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
                        let source_ml_source2_comparison_operand18567 = None::<()>;
                        match (symbol.clone(), source_ml_source2_comparison_operand18567) {
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
                            source_ml_source2_data_property_value18570,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value18570 =
                                *source_ml_source2_data_property_value18570;
                            source_ml_source2_data_property_value18570.clone()
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
                            source_ml_source2_data_property_value18575,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value18575 =
                                *source_ml_source2_data_property_value18575;
                            source_ml_source2_data_property_value18575.clone()
                        }
                    });
                    if ((index >= unsigned_length)
                        && !(({
                            (match property.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    _,
                                    source_ml_source2_data_property_value18576,
                                ) => source_ml_source2_data_property_value18576.clone(),
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
                            source_ml_source2_data_property_value18579,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value18579 =
                                *source_ml_source2_data_property_value18579;
                            source_ml_source2_data_property_value18579.clone()
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
                                                source_ml_source2_data_property_value18580,
                                                _,
                                            ) => {
                                                let source_ml_source2_data_property_value18580 =
                                                    *source_ml_source2_data_property_value18580;
                                                source_ml_source2_data_property_value18580.clone()
                                            }
                                        },
                                    ) {
                                        {
                                            let source_ml_source2_nominal_field18598 = match property.clone() {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value18599, _) => {
                                            let source_ml_source2_data_property_value18599 = *source_ml_source2_data_property_value18599;
                                            source_ml_source2_data_property_value18599.clone()
                                        }
                                    };
                                            {
                                                let source_ml_source2_nominal_field18600 = match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value18601) => {
                                                source_ml_source2_data_property_value18601.clone()
                                            }
                                        };
                                                {
                                                    let source_ml_source2_nominal_override18602 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            {
                                                                let source_ml_source2_spread18582 = {
                                                                    (match property.clone() {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value18581) => {
                                                        source_ml_source2_data_property_value18581.clone()
                                                    }
                                                }).borrow().clone()
                                                                };
                                                                {
                                                                    let source_ml_source2_spread_field18583 = (source_ml_source2_spread18582.clone()).0;
                                                                    {
                                                                        let source_ml_source2_spread_field18584 = (source_ml_source2_spread18582.clone()).1;
                                                                        {
                                                                            let source_ml_source2_spread_field18585 = (source_ml_source2_spread18582.clone()).2;
                                                                            {
                                                                                let source_ml_source2_spread_field18586 = (source_ml_source2_spread18582.clone()).3;
                                                                                {
                                                                                    let source_ml_source2_spread_field18587 = (source_ml_source2_spread18582.clone()).4;
                                                                                    {
                                                                                        let source_ml_source2_spread_field18588 = (source_ml_source2_spread18582.clone()).5;
                                                                                        {
                                                                                            let source_ml_source2_spread_field18589 = (source_ml_source2_spread18582.clone()).6;
                                                                                            {
                                                                                                let source_ml_source2_spread_field18590 = (source_ml_source2_spread18582.clone()).7;
                                                                                                {
                                                                                                    let source_ml_source2_spread_field18591 = (source_ml_source2_spread18582.clone()).8;
                                                                                                    {
                                                                                                        let source_ml_source2_spread_field18592 = (source_ml_source2_spread18582.clone()).9;
                                                                                                        {
                                                                                                            let source_ml_source2_spread_field18593 = (source_ml_source2_spread18582.clone()).10;
                                                                                                            {
                                                                                                                let source_ml_source2_spread_field18594 = (source_ml_source2_spread18582.clone()).11;
                                                                                                                {
                                                                                                                    let source_ml_source2_field18595 = self::JavaScriptSemanticValue::Number(final_length);
                                                                                                                    (source_ml_source2_spread_field18583, source_ml_source2_spread_field18584, source_ml_source2_spread_field18585, source_ml_source2_spread_field18586, source_ml_source2_spread_field18587, source_ml_source2_spread_field18588, source_ml_source2_spread_field18589, source_ml_source2_spread_field18590, source_ml_source2_field18595, source_ml_source2_spread_field18592, new_writable, source_ml_source2_spread_field18594)
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
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source2_nominal_field18598), source_ml_source2_nominal_override18602)
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
