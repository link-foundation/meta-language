// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
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
                        let ml_reference_array211 = objects.clone();
                        let ml_reference_position211 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array211.borrow(),
                            self::ml_array::number_index(ml_reference_position211),
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
                            source_ml_source1_data_property_value16802,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16802.clone(),
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            source_ml_source1_data_property_value16803,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16803.clone(),
                        self::JavaScriptOrdinaryObject::ArrayObject(
                            source_ml_source1_data_property_value16804,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16804.clone(),
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            source_ml_source1_data_property_value16805,
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
                        ) => source_ml_source1_data_property_value16805.clone(),
                        self::JavaScriptOrdinaryObject::MapObject(
                            source_ml_source1_data_property_value16806,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16806.clone(),
                        self::JavaScriptOrdinaryObject::SetObject(
                            source_ml_source1_data_property_value16807,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16807.clone(),
                        self::JavaScriptOrdinaryObject::WeakMapObject(
                            source_ml_source1_data_property_value16808,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16808.clone(),
                        self::JavaScriptOrdinaryObject::WeakSetObject(
                            source_ml_source1_data_property_value16809,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16809.clone(),
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source1_data_property_value16810,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16810.clone(),
                        self::JavaScriptOrdinaryObject::HostErrorObject(
                            source_ml_source1_data_property_value16811,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16811.clone(),
                        self::JavaScriptOrdinaryObject::HostURLObject(
                            source_ml_source1_data_property_value16812,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16812.clone(),
                        self::JavaScriptOrdinaryObject::HostArrayObject(
                            source_ml_source1_data_property_value16813,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16813.clone(),
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            source_ml_source1_data_property_value16814,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16814.clone(),
                    }) == identity
                    {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(middle)
                    } else {
                        if (match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source1_data_property_value16828,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16828.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source1_data_property_value16829,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16829.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source1_data_property_value16830,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16830.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source1_data_property_value16831,
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
                            ) => source_ml_source1_data_property_value16831.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source1_data_property_value16832,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16832.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source1_data_property_value16833,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16833.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source1_data_property_value16834,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16834.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source1_data_property_value16835,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16835.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value16836,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16836.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source1_data_property_value16837,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16837.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source1_data_property_value16838,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16838.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source1_data_property_value16839,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16839.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source1_data_property_value16840,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16840.clone(),
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
                        let ml_reference_array212 = symbols.clone();
                        let ml_reference_position212 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array212.borrow(),
                            self::ml_array::number_index(ml_reference_position212),
                        )
                    };
                    if {
                        let source_ml_source1_comparison_operand16844 = None::<()>;
                        match (symbol.clone(), source_ml_source1_comparison_operand16844) {
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
                let ml_reference_array213 = source_ml_temporary_values14.clone();
                self::ml::Big::from_u128(ml_reference_array213.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array214 = source_ml_temporary_values14.clone();
                    let ml_reference_position214 = source_ml_temporary_index14.clone();
                    self::ml_array::at(
                        &*ml_reference_array214.borrow(),
                        ml_reference_position214.to_index(),
                    )
                };
                {
                    let index = self::read_java_script_array_index(match entry {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value16847,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value16847 =
                                *source_ml_source1_data_property_value16847;
                            source_ml_source1_data_property_value16847.clone()
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
                let ml_reference_array215 = source_ml_temporary_values19.clone();
                self::ml::Big::from_u128(ml_reference_array215.borrow().len() as u128)
            })
        {
            {
                let entry = {
                    let ml_reference_array216 = source_ml_temporary_values19.clone();
                    let ml_reference_position216 = source_ml_temporary_index19.clone();
                    self::ml_array::at(
                        &*ml_reference_array216.borrow(),
                        ml_reference_position216.to_index(),
                    )
                };
                {
                    let source_ml_temporary_plan_java_script_array_ordinary_definition_ignored23 = {
                        let ml_method_owner217 = properties.clone();
                        let ml_method_argument217_0 = if index == at {
                            replacement.clone()
                        } else {
                            entry
                        };
                        let ml_inserted = vec![ml_method_argument217_0];
                        let mut ml_method_values217 = ml_method_owner217.borrow_mut();
                        let ml_length = ml_method_values217
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values217.extend(ml_inserted);
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
                let ml_reference_array218 = source_ml_temporary_values24.clone();
                self::ml::Big::from_u128(ml_reference_array218.borrow().len() as u128)
            })
        {
            {
                let property = {
                    let ml_reference_array219 = source_ml_temporary_values24.clone();
                    let ml_reference_position219 = source_ml_temporary_index24.clone();
                    self::ml_array::at(
                        &*ml_reference_array219.borrow(),
                        ml_reference_position219.to_index(),
                    )
                };
                {
                    let index = self::read_java_script_array_index(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value16852,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value16852 =
                                *source_ml_source1_data_property_value16852;
                            source_ml_source1_data_property_value16852.clone()
                        }
                    });
                    if ((index >= unsigned_length)
                        && !(({
                            (match property.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    _,
                                    source_ml_source1_data_property_value16853,
                                ) => source_ml_source1_data_property_value16853.clone(),
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
                let ml_reference_array221 = source_ml_temporary_values25.clone();
                self::ml::Big::from_u128(ml_reference_array221.borrow().len() as u128)
            })
        {
            {
                let property = {
                    let ml_reference_array222 = source_ml_temporary_values25.clone();
                    let ml_reference_position222 = source_ml_temporary_index25.clone();
                    self::ml_array::at(
                        &*ml_reference_array222.borrow(),
                        ml_reference_position222.to_index(),
                    )
                };
                {
                    let index = self::read_java_script_array_index(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value16856,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value16856 =
                                *source_ml_source1_data_property_value16856;
                            source_ml_source1_data_property_value16856.clone()
                        }
                    });
                    if (index < 0f64) || (index < final_length) {
                        {
                            let source_ml_temporary_plan_java_script_array_length_definition_ignored28 = {
                                let ml_method_owner223 = properties.clone();
                                let ml_method_argument223_0 =
                                    if self::test_java_script_array_length_key(
                                        match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                                source_ml_source1_data_property_value16857,
                                                _,
                                            ) => {
                                                let source_ml_source1_data_property_value16857 =
                                                    *source_ml_source1_data_property_value16857;
                                                source_ml_source1_data_property_value16857.clone()
                                            }
                                        },
                                    ) {
                                        {
                                            let source_ml_source1_nominal_field16875 = match property.clone() {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value16876, _) => {
                                            let source_ml_source1_data_property_value16876 = *source_ml_source1_data_property_value16876;
                                            source_ml_source1_data_property_value16876.clone()
                                        }
                                    };
                                            {
                                                let source_ml_source1_nominal_field16877 = match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value16878) => {
                                                source_ml_source1_data_property_value16878.clone()
                                            }
                                        };
                                                {
                                                    let source_ml_source1_nominal_override16879 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            {
                                                                let source_ml_source1_spread16859 = {
                                                                    (match property.clone() {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value16858) => {
                                                        source_ml_source1_data_property_value16858.clone()
                                                    }
                                                }).borrow().clone()
                                                                };
                                                                {
                                                                    let source_ml_source1_spread_field16860 = (source_ml_source1_spread16859.clone()).0;
                                                                    {
                                                                        let source_ml_source1_spread_field16861 = (source_ml_source1_spread16859.clone()).1;
                                                                        {
                                                                            let source_ml_source1_spread_field16862 = (source_ml_source1_spread16859.clone()).2;
                                                                            {
                                                                                let source_ml_source1_spread_field16863 = (source_ml_source1_spread16859.clone()).3;
                                                                                {
                                                                                    let source_ml_source1_spread_field16864 = (source_ml_source1_spread16859.clone()).4;
                                                                                    {
                                                                                        let source_ml_source1_spread_field16865 = (source_ml_source1_spread16859.clone()).5;
                                                                                        {
                                                                                            let source_ml_source1_spread_field16866 = (source_ml_source1_spread16859.clone()).6;
                                                                                            {
                                                                                                let source_ml_source1_spread_field16867 = (source_ml_source1_spread16859.clone()).7;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field16868 = (source_ml_source1_spread16859.clone()).8;
                                                                                                    {
                                                                                                        let source_ml_source1_spread_field16869 = (source_ml_source1_spread16859.clone()).9;
                                                                                                        {
                                                                                                            let source_ml_source1_spread_field16870 = (source_ml_source1_spread16859.clone()).10;
                                                                                                            {
                                                                                                                let source_ml_source1_spread_field16871 = (source_ml_source1_spread16859.clone()).11;
                                                                                                                {
                                                                                                                    let source_ml_source1_field16872 = self::JavaScriptSemanticValue::Number(final_length);
                                                                                                                    (source_ml_source1_spread_field16860, source_ml_source1_spread_field16861, source_ml_source1_spread_field16862, source_ml_source1_spread_field16863, source_ml_source1_spread_field16864, source_ml_source1_spread_field16865, source_ml_source1_spread_field16866, source_ml_source1_spread_field16867, source_ml_source1_field16872, source_ml_source1_spread_field16869, new_writable, source_ml_source1_spread_field16871)
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
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source1_nominal_field16875), source_ml_source1_nominal_override16879)
                                                }
                                            }
                                        }
                                    } else {
                                        property.clone()
                                    };
                                let ml_inserted = vec![ml_method_argument223_0];
                                let mut ml_method_values223 = ml_method_owner223.borrow_mut();
                                let ml_length = ml_method_values223
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                ml_method_values223.extend(ml_inserted);
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
