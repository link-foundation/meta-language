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
                            source_ml_source1_data_property_value18112,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18112.clone(),
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            source_ml_source1_data_property_value18113,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18113.clone(),
                        self::JavaScriptOrdinaryObject::ArrayObject(
                            source_ml_source1_data_property_value18114,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18114.clone(),
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            source_ml_source1_data_property_value18115,
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
                        ) => source_ml_source1_data_property_value18115.clone(),
                        self::JavaScriptOrdinaryObject::MapObject(
                            source_ml_source1_data_property_value18116,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18116.clone(),
                        self::JavaScriptOrdinaryObject::SetObject(
                            source_ml_source1_data_property_value18117,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18117.clone(),
                        self::JavaScriptOrdinaryObject::WeakMapObject(
                            source_ml_source1_data_property_value18118,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18118.clone(),
                        self::JavaScriptOrdinaryObject::WeakSetObject(
                            source_ml_source1_data_property_value18119,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18119.clone(),
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source1_data_property_value18120,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18120.clone(),
                        self::JavaScriptOrdinaryObject::HostErrorObject(
                            source_ml_source1_data_property_value18121,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18121.clone(),
                        self::JavaScriptOrdinaryObject::HostURLObject(
                            source_ml_source1_data_property_value18122,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18122.clone(),
                        self::JavaScriptOrdinaryObject::HostArrayObject(
                            source_ml_source1_data_property_value18123,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18123.clone(),
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            source_ml_source1_data_property_value18124,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value18124.clone(),
                    }) == identity
                    {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(middle)
                    } else {
                        if (match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source1_data_property_value18138,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18138.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source1_data_property_value18139,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18139.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source1_data_property_value18140,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18140.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source1_data_property_value18141,
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
                            ) => source_ml_source1_data_property_value18141.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source1_data_property_value18142,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18142.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source1_data_property_value18143,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18143.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source1_data_property_value18144,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18144.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source1_data_property_value18145,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18145.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value18146,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18146.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source1_data_property_value18147,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18147.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source1_data_property_value18148,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18148.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source1_data_property_value18149,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18149.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source1_data_property_value18150,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18150.clone(),
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
                        let source_ml_source1_comparison_operand18154 = None::<()>;
                        match (symbol.clone(), source_ml_source1_comparison_operand18154) {
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
                            source_ml_source1_data_property_value18157,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value18157 =
                                *source_ml_source1_data_property_value18157;
                            source_ml_source1_data_property_value18157.clone()
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
                            source_ml_source1_data_property_value18162,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value18162 =
                                *source_ml_source1_data_property_value18162;
                            source_ml_source1_data_property_value18162.clone()
                        }
                    });
                    if ((index >= unsigned_length)
                        && !(({
                            (match property.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    _,
                                    source_ml_source1_data_property_value18163,
                                ) => source_ml_source1_data_property_value18163.clone(),
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
                            source_ml_source1_data_property_value18166,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value18166 =
                                *source_ml_source1_data_property_value18166;
                            source_ml_source1_data_property_value18166.clone()
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
                                                source_ml_source1_data_property_value18167,
                                                _,
                                            ) => {
                                                let source_ml_source1_data_property_value18167 =
                                                    *source_ml_source1_data_property_value18167;
                                                source_ml_source1_data_property_value18167.clone()
                                            }
                                        },
                                    ) {
                                        {
                                            let source_ml_source1_nominal_field18185 = match property.clone() {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value18186, _) => {
                                            let source_ml_source1_data_property_value18186 = *source_ml_source1_data_property_value18186;
                                            source_ml_source1_data_property_value18186.clone()
                                        }
                                    };
                                            {
                                                let source_ml_source1_nominal_field18187 = match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value18188) => {
                                                source_ml_source1_data_property_value18188.clone()
                                            }
                                        };
                                                {
                                                    let source_ml_source1_nominal_override18189 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            {
                                                                let source_ml_source1_spread18169 = {
                                                                    (match property.clone() {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value18168) => {
                                                        source_ml_source1_data_property_value18168.clone()
                                                    }
                                                }).borrow().clone()
                                                                };
                                                                {
                                                                    let source_ml_source1_spread_field18170 = (source_ml_source1_spread18169.clone()).0;
                                                                    {
                                                                        let source_ml_source1_spread_field18171 = (source_ml_source1_spread18169.clone()).1;
                                                                        {
                                                                            let source_ml_source1_spread_field18172 = (source_ml_source1_spread18169.clone()).2;
                                                                            {
                                                                                let source_ml_source1_spread_field18173 = (source_ml_source1_spread18169.clone()).3;
                                                                                {
                                                                                    let source_ml_source1_spread_field18174 = (source_ml_source1_spread18169.clone()).4;
                                                                                    {
                                                                                        let source_ml_source1_spread_field18175 = (source_ml_source1_spread18169.clone()).5;
                                                                                        {
                                                                                            let source_ml_source1_spread_field18176 = (source_ml_source1_spread18169.clone()).6;
                                                                                            {
                                                                                                let source_ml_source1_spread_field18177 = (source_ml_source1_spread18169.clone()).7;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field18178 = (source_ml_source1_spread18169.clone()).8;
                                                                                                    {
                                                                                                        let source_ml_source1_spread_field18179 = (source_ml_source1_spread18169.clone()).9;
                                                                                                        {
                                                                                                            let source_ml_source1_spread_field18180 = (source_ml_source1_spread18169.clone()).10;
                                                                                                            {
                                                                                                                let source_ml_source1_spread_field18181 = (source_ml_source1_spread18169.clone()).11;
                                                                                                                {
                                                                                                                    let source_ml_source1_field18182 = self::JavaScriptSemanticValue::Number(final_length);
                                                                                                                    (source_ml_source1_spread_field18170, source_ml_source1_spread_field18171, source_ml_source1_spread_field18172, source_ml_source1_spread_field18173, source_ml_source1_spread_field18174, source_ml_source1_spread_field18175, source_ml_source1_spread_field18176, source_ml_source1_spread_field18177, source_ml_source1_field18182, source_ml_source1_spread_field18179, new_writable, source_ml_source1_spread_field18181)
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
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source1_nominal_field18185), source_ml_source1_nominal_override18189)
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
