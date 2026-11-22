// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_index;
use super::test_java_script_array_length_key;

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
                        let ml_reference_array253 = objects.clone();
                        let ml_reference_position253 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array253.borrow(),
                            self::ml_array::number_index(ml_reference_position253),
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
                            source_ml_source1_data_property_value18388,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18388.clone(),
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            source_ml_source1_data_property_value18389,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18389.clone(),
                        self::JavaScriptOrdinaryObject::ArrayObject(
                            source_ml_source1_data_property_value18390,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18390.clone(),
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            source_ml_source1_data_property_value18391,
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
                        ) => source_ml_source1_data_property_value18391.clone(),
                        self::JavaScriptOrdinaryObject::MapObject(
                            source_ml_source1_data_property_value18392,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18392.clone(),
                        self::JavaScriptOrdinaryObject::SetObject(
                            source_ml_source1_data_property_value18393,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18393.clone(),
                        self::JavaScriptOrdinaryObject::WeakMapObject(
                            source_ml_source1_data_property_value18394,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18394.clone(),
                        self::JavaScriptOrdinaryObject::WeakSetObject(
                            source_ml_source1_data_property_value18395,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18395.clone(),
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source1_data_property_value18396,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18396.clone(),
                        self::JavaScriptOrdinaryObject::HostErrorObject(
                            source_ml_source1_data_property_value18397,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18397.clone(),
                        self::JavaScriptOrdinaryObject::HostURLObject(
                            source_ml_source1_data_property_value18398,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18398.clone(),
                        self::JavaScriptOrdinaryObject::HostArrayObject(
                            source_ml_source1_data_property_value18399,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18399.clone(),
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            source_ml_source1_data_property_value18400,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18400.clone(),
                    }) == identity
                    {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(middle)
                    } else {
                        if (match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source1_data_property_value18414,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18414.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source1_data_property_value18415,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18415.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source1_data_property_value18416,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18416.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source1_data_property_value18417,
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
                            ) => source_ml_source1_data_property_value18417.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source1_data_property_value18418,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18418.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source1_data_property_value18419,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18419.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source1_data_property_value18420,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18420.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source1_data_property_value18421,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18421.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value18422,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18422.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source1_data_property_value18423,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18423.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source1_data_property_value18424,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18424.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source1_data_property_value18425,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18425.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source1_data_property_value18426,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18426.clone(),
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
                        let ml_reference_array254 = symbols.clone();
                        let ml_reference_position254 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array254.borrow(),
                            self::ml_array::number_index(ml_reference_position254),
                        )
                    };
                    if {
                        let source_ml_source1_comparison_operand18430 = None::<()>;
                        match (symbol.clone(), source_ml_source1_comparison_operand18430) {
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
                let ml_reference_array255 = source_ml_temporary_values14.clone();
                self::ml::Big::from_u128(ml_reference_array255.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array256 = source_ml_temporary_values14.clone();
                    let ml_reference_position256 = source_ml_temporary_index14.clone();
                    self::ml_array::at(
                        &*ml_reference_array256.borrow(),
                        ml_reference_position256.to_index(),
                    )
                };
                {
                    let index = self::read_java_script_array_index(match entry {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value18433,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value18433 =
                                *source_ml_source1_data_property_value18433;
                            source_ml_source1_data_property_value18433.clone()
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
pub enum MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21 {
    MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Result {
    MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_java_script_array_ordinary_definition_loop20(
    mut at: f64,
    mut replacement: self::JavaScriptOrdinaryProperty,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut index: f64,
    mut source_ml_temporary_values19: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_temporary_index19: self::ml::Big,
) -> self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Result {
    loop {
        if source_ml_temporary_index19
            < ({
                let ml_reference_array257 = source_ml_temporary_values19.clone();
                self::ml::Big::from_u128(ml_reference_array257.borrow().len() as u128)
            })
        {
            {
                let entry = {
                    let ml_reference_array258 = source_ml_temporary_values19.clone();
                    let ml_reference_position258 = source_ml_temporary_index19.clone();
                    self::ml_array::at(
                        &*ml_reference_array258.borrow(),
                        ml_reference_position258.to_index(),
                    )
                };
                {
                    let source_ml_temporary_plan_java_script_array_ordinary_definition_ignored23 = {
                        let ml_method_owner259 = properties.clone();
                        let ml_method_argument259_0 = if index == at {
                            replacement.clone()
                        } else {
                            entry
                        };
                        let ml_inserted = vec![ml_method_argument259_0];
                        let mut ml_method_values259 = ml_method_owner259.borrow_mut();
                        let ml_length = ml_method_values259
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values259.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            let source_ml_temporary_index19_2 =
                                source_ml_temporary_index19.add(&self::ml::Big::from_i128(1));
                            {
                                (index, source_ml_temporary_index19) =
                                    (index_2, source_ml_temporary_index19_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Result::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_java_script_array_length_definition_loop26(
    mut unsigned_length: f64,
    mut blocked: f64,
    mut source_ml_temporary_values24: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_temporary_index24: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index24
            < ({
                let ml_reference_array260 = source_ml_temporary_values24.clone();
                self::ml::Big::from_u128(ml_reference_array260.borrow().len() as u128)
            })
        {
            {
                let property = {
                    let ml_reference_array261 = source_ml_temporary_values24.clone();
                    let ml_reference_position261 = source_ml_temporary_index24.clone();
                    self::ml_array::at(
                        &*ml_reference_array261.borrow(),
                        ml_reference_position261.to_index(),
                    )
                };
                {
                    let index = self::read_java_script_array_index(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value18438,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value18438 =
                                *source_ml_source1_data_property_value18438;
                            source_ml_source1_data_property_value18438.clone()
                        }
                    });
                    if ((index >= unsigned_length)
                        && !(({
                            (match property.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    _,
                                    source_ml_source1_data_property_value18439,
                                ) => source_ml_source1_data_property_value18439.clone(),
                            })
                            .borrow()
                            .clone()
                        })
                        .0))
                        && (index > blocked)
                    {
                        {
                            let source_ml_temporary_index24_2 =
                                source_ml_temporary_index24.add(&self::ml::Big::from_i128(1));
                            {
                                (blocked, source_ml_temporary_index24) =
                                    (index, source_ml_temporary_index24_2);
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let source_ml_temporary_index24_3 =
                                source_ml_temporary_index24.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index24 = source_ml_temporary_index24_3;
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
pub enum MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop27Result {
    MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop27Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_java_script_array_length_definition_loop27(
    mut new_writable: bool,
    mut final_length: f64,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut source_ml_temporary_values25: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_temporary_index25: self::ml::Big,
) -> self::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop27Result {
    loop {
        if source_ml_temporary_index25
            < ({
                let ml_reference_array263 = source_ml_temporary_values25.clone();
                self::ml::Big::from_u128(ml_reference_array263.borrow().len() as u128)
            })
        {
            {
                let property = {
                    let ml_reference_array264 = source_ml_temporary_values25.clone();
                    let ml_reference_position264 = source_ml_temporary_index25.clone();
                    self::ml_array::at(
                        &*ml_reference_array264.borrow(),
                        ml_reference_position264.to_index(),
                    )
                };
                {
                    let index = self::read_java_script_array_index(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value18442,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value18442 =
                                *source_ml_source1_data_property_value18442;
                            source_ml_source1_data_property_value18442.clone()
                        }
                    });
                    if (index < 0f64) || (index < final_length) {
                        {
                            let source_ml_temporary_plan_java_script_array_length_definition_ignored28 = {
                                let ml_method_owner265 = properties.clone();
                                let ml_method_argument265_0 =
                                    if self::test_java_script_array_length_key(
                                        match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                                source_ml_source1_data_property_value18443,
                                                _,
                                            ) => {
                                                let source_ml_source1_data_property_value18443 =
                                                    *source_ml_source1_data_property_value18443;
                                                source_ml_source1_data_property_value18443.clone()
                                            }
                                        },
                                    ) {
                                        {
                                            let source_ml_source1_nominal_field18461 = match property.clone() {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value18462, _) => {
                                            let source_ml_source1_data_property_value18462 = *source_ml_source1_data_property_value18462;
                                            source_ml_source1_data_property_value18462.clone()
                                        }
                                    };
                                            {
                                                let source_ml_source1_nominal_field18463 = match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value18464) => {
                                                source_ml_source1_data_property_value18464.clone()
                                            }
                                        };
                                                {
                                                    let source_ml_source1_nominal_override18465 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            {
                                                                let source_ml_source1_spread18445 = {
                                                                    (match property.clone() {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value18444) => {
                                                        source_ml_source1_data_property_value18444.clone()
                                                    }
                                                }).borrow().clone()
                                                                };
                                                                {
                                                                    let source_ml_source1_spread_field18446 = (source_ml_source1_spread18445.clone()).0;
                                                                    {
                                                                        let source_ml_source1_spread_field18447 = (source_ml_source1_spread18445.clone()).1;
                                                                        {
                                                                            let source_ml_source1_spread_field18448 = (source_ml_source1_spread18445.clone()).2;
                                                                            {
                                                                                let source_ml_source1_spread_field18449 = (source_ml_source1_spread18445.clone()).3;
                                                                                {
                                                                                    let source_ml_source1_spread_field18450 = (source_ml_source1_spread18445.clone()).4;
                                                                                    {
                                                                                        let source_ml_source1_spread_field18451 = (source_ml_source1_spread18445.clone()).5;
                                                                                        {
                                                                                            let source_ml_source1_spread_field18452 = (source_ml_source1_spread18445.clone()).6;
                                                                                            {
                                                                                                let source_ml_source1_spread_field18453 = (source_ml_source1_spread18445.clone()).7;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field18454 = (source_ml_source1_spread18445.clone()).8;
                                                                                                    {
                                                                                                        let source_ml_source1_spread_field18455 = (source_ml_source1_spread18445.clone()).9;
                                                                                                        {
                                                                                                            let source_ml_source1_spread_field18456 = (source_ml_source1_spread18445.clone()).10;
                                                                                                            {
                                                                                                                let source_ml_source1_spread_field18457 = (source_ml_source1_spread18445.clone()).11;
                                                                                                                {
                                                                                                                    let source_ml_source1_field18458 = self::JavaScriptSemanticValue::Number(final_length);
                                                                                                                    (source_ml_source1_spread_field18446, source_ml_source1_spread_field18447, source_ml_source1_spread_field18448, source_ml_source1_spread_field18449, source_ml_source1_spread_field18450, source_ml_source1_spread_field18451, source_ml_source1_spread_field18452, source_ml_source1_spread_field18453, source_ml_source1_field18458, source_ml_source1_spread_field18455, new_writable, source_ml_source1_spread_field18457)
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
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source1_nominal_field18461), source_ml_source1_nominal_override18465)
                                                }
                                            }
                                        }
                                    } else {
                                        property.clone()
                                    };
                                let ml_inserted = vec![ml_method_argument265_0];
                                let mut ml_method_values265 = ml_method_owner265.borrow_mut();
                                let ml_length = ml_method_values265
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                ml_method_values265.extend(ml_inserted);
                                ml_length as f64
                            };
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
                    {
                        {
                            let source_ml_temporary_index25_3 =
                                source_ml_temporary_index25.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index25 = source_ml_temporary_index25_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop27Result::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop27Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result {
    MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Done,
}
