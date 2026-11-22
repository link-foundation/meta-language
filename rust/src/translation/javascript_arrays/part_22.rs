// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::MlOptional;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result;
use super::compare_java_script_collection_keys;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop113;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop109(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>,
    mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut previous_key: self::JavaScriptSemanticValue,
    mut units: f64,
    mut source_ml_temporary_values83: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>,
    >,
    mut source_ml_temporary_index83: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result {
    loop {
        return if source_ml_temporary_index83
            < ({
                let ml_reference_array284 = source_ml_temporary_values83.clone();
                self::ml::Big::from_u128(ml_reference_array284.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array285 = source_ml_temporary_values83.clone();
                    let ml_reference_position285 = source_ml_temporary_index83.clone();
                    self::ml_array::at(
                        &*ml_reference_array285.borrow(),
                        ml_reference_position285.to_index(),
                    )
                };
                if (((({
                    let source_ml_source1_optional_chain_receiver19819 = (entry.clone(),);
                    {
                        let source_ml_temporary_optional_chain84 =
                            (source_ml_source1_optional_chain_receiver19819).0;
                        match source_ml_temporary_optional_chain84 {
                            self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(_, _) => {
                                self::ml_string::Text::from_units(vec![
                                    115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114,
                                    121, 69, 110, 116, 114, 121,
                                ])
                            }
                        }
                    }
                }) != (self::ml_string::Text::from_units(vec![
                    115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114, 121, 69, 110,
                    116, 114, 121,
                ]))) || !(self::test_java_script_ordinary_symbol(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            _,
                            source_ml_source1_data_property_value19820,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value19820.clone(),
                    },
                    match entry.clone() {
                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                            source_ml_source1_data_property_value19821,
                            _,
                        ) => source_ml_source1_data_property_value19821.clone(),
                    },
                ))) || ((({
                    let source_ml_source1_optional_chain_receiver19825 = (match entry.clone() {
                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                            _,
                            source_ml_source1_data_property_value19824,
                        ) => {
                            let source_ml_source1_data_property_value19824 =
                                *source_ml_source1_data_property_value19824;
                            source_ml_source1_data_property_value19824.clone()
                        }
                    },);
                    {
                        let source_ml_temporary_optional_chain85 =
                            (source_ml_source1_optional_chain_receiver19825).0;
                        match source_ml_temporary_optional_chain85 {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::ml_string::Text::from_units(vec![
                                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                                ])
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                self::ml_string::Text::from_units(vec![
                                    98, 111, 111, 108, 101, 97, 110,
                                ])
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                self::ml_string::Text::from_units(vec![
                                    105, 110, 116, 101, 103, 101, 114,
                                ])
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                            }
                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                self::ml_string::Text::from_units(vec![
                                    115, 111, 117, 114, 99, 101, 84, 101, 120, 116,
                                ])
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                            }
                        }
                    }
                }) != (self::ml_string::Text::from_units(vec![116, 101, 120, 116])))
                    && (({
                        let source_ml_source1_optional_chain_receiver19829 =
                            (match entry.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    _,
                                    source_ml_source1_data_property_value19828,
                                ) => {
                                    let source_ml_source1_data_property_value19828 =
                                        *source_ml_source1_data_property_value19828;
                                    source_ml_source1_data_property_value19828.clone()
                                }
                            },);
                        {
                            let source_ml_temporary_optional_chain86 =
                                (source_ml_source1_optional_chain_receiver19829).0;
                            match source_ml_temporary_optional_chain86 {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::ml_string::Text::from_units(vec![
                                        117, 110, 100, 101, 102, 105, 110, 101, 100,
                                    ])
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::ml_string::Text::from_units(vec![
                                        98, 111, 111, 108, 101, 97, 110,
                                    ])
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    self::ml_string::Text::from_units(vec![
                                        110, 117, 109, 98, 101, 114,
                                    ])
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::ml_string::Text::from_units(vec![
                                        105, 110, 116, 101, 103, 101, 114,
                                    ])
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    self::ml_string::Text::from_units(vec![
                                        115, 111, 117, 114, 99, 101, 84, 101, 120, 116,
                                    ])
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::ml_string::Text::from_units(vec![
                                        115, 121, 109, 98, 111, 108,
                                    ])
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    self::ml_string::Text::from_units(vec![
                                        111, 98, 106, 101, 99, 116,
                                    ])
                                }
                            }
                        }
                    }) != (self::ml_string::Text::from_units(vec![
                        115, 111, 117, 114, 99, 101, 84, 101, 120, 116,
                    ])))))
                    || ((self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        match entry.clone() {
                            self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                _,
                                source_ml_source1_data_property_value19831,
                            ) => {
                                let source_ml_source1_data_property_value19831 =
                                    *source_ml_source1_data_property_value19831;
                                source_ml_source1_data_property_value19831.clone()
                            }
                        },
                    )) != (self::ml_string::Text::from_units(vec![])))
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 97, 110, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 85, 84, 70, 49, 54, 32, 107, 101, 121, 115]))
                } else {
                    {
                        let units_2 = units
                            + self::read_java_script_semantic_text_length(match entry.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    _,
                                    source_ml_source1_data_property_value19832,
                                ) => {
                                    let source_ml_source1_data_property_value19832 =
                                        *source_ml_source1_data_property_value19832;
                                    source_ml_source1_data_property_value19832.clone()
                                }
                            });
                        if ((self::read_java_script_semantic_text_length(match entry.clone() {
                            self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                _,
                                source_ml_source1_data_property_value19834,
                            ) => {
                                let source_ml_source1_data_property_value19834 =
                                    *source_ml_source1_data_property_value19834;
                                source_ml_source1_data_property_value19834.clone()
                            }
                        })) > 1_048_576f64)
                            || (units_2 > 16_777_216f64)
                        {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101]))
                        } else if match previous_key.clone() {
                            self::JavaScriptSemanticValue::Undefined => false,
                            _ => true,
                        } && ((self::compare_java_script_collection_keys(
                            previous_key.clone(),
                            match entry.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    _,
                                    source_ml_source1_data_property_value19845,
                                ) => {
                                    let source_ml_source1_data_property_value19845 =
                                        *source_ml_source1_data_property_value19845;
                                    source_ml_source1_data_property_value19845.clone()
                                }
                            },
                        )) >= 0f64)
                        {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 115, 116, 114, 105, 99, 116, 32, 85, 84, 70, 49, 54, 32, 111, 114, 100, 101, 114]))
                        } else {
                            {
                                let previous_key_2 = match entry.clone() {
                                    self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                        _,
                                        source_ml_source1_data_property_value19846,
                                    ) => {
                                        let source_ml_source1_data_property_value19846 =
                                            *source_ml_source1_data_property_value19846;
                                        source_ml_source1_data_property_value19846.clone()
                                    }
                                };
                                {
                                    let low = 0f64;
                                    {
                                        let high = {
                                            let ml_reference_array286 = facts.clone();
                                            ml_reference_array286.borrow().len() as f64
                                        };
                                        {
                                            let low_2 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop113(facts.clone(), self::MlOptional::Value(entry.clone()), low, high);
                                            {
                                                let fact = {
                                                    let ml_reference_array287 = facts.clone();
                                                    let ml_reference_position287 = low_2;
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array287.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position287,
                                                        ),
                                                    )
                                                };
                                                if (match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match fact.clone().expect("a proven present value") {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value19851, _, _) => {
                                                            source_ml_source1_data_property_value19851.clone()
                                                        }
                                                    }) != (match entry.clone() {
                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source1_data_property_value19852, _) => {
                                                            source_ml_source1_data_property_value19852.clone()
                                                        }
                                                    }))) || !(match fact.clone().expect("a proven present value") {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source1_data_property_value19853, _) => {
                                                            source_ml_source1_data_property_value19853
                                                        }
                                                    }) {
                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115]))
                                                    } else {
                                                        {
                                                            let key = match fact.clone().expect("a proven present value") {
                                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source1_data_property_value19854) => {
                                                                    ((source_ml_source1_data_property_value19854.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                }
                                                            };
                                                            if match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(key.clone().expect("a proven present value"), match entry.clone() {
                                                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(_, source_ml_source1_data_property_value19855) => {
                                                                    let source_ml_source1_data_property_value19855 = *source_ml_source1_data_property_value19855;
                                                                    source_ml_source1_data_property_value19855.clone()
                                                                }
                                                            })) {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_temporary_read_java_script_ordinary_heap_error_ignored114 = { let ml_method_owner288 = identities.clone(); let ml_method_argument288_0 = self::JavaScriptSemanticValue::Symbol(match entry.clone() {
                                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source1_data_property_value19856, _) => {
                                                                            source_ml_source1_data_property_value19856.clone()
                                                                        }
                                                                    }); let ml_inserted = vec![ml_method_argument288_0]; let mut ml_method_values288 = ml_method_owner288.borrow_mut(); let ml_length = ml_method_values288.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values288.extend(ml_inserted); ml_length as f64 };
                                                                    {
                                                                        let source_ml_temporary_index83_2 = source_ml_temporary_index83.add(&self::ml::Big::from_i128(1));
                                                                        {
                                                                            (previous_key, units, source_ml_temporary_index83) = (previous_key_2, units_2, source_ml_temporary_index83_2);
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
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop97(
    mut witnessed_units: f64,
    mut previous_witness: Option<self::JavaScriptSemanticValue>,
    mut source_ml_temporary_values82: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index82: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result {
    loop {
        return if source_ml_temporary_index82
            < ({
                let ml_reference_array289 = source_ml_temporary_values82.clone();
                self::ml::Big::from_u128(ml_reference_array289.borrow().len() as u128)
            }) {
            {
                let key = {
                    let ml_reference_array290 = source_ml_temporary_values82.clone();
                    let ml_reference_position290 = source_ml_temporary_index82.clone();
                    self::ml_array::at(
                        &*ml_reference_array290.borrow(),
                        ml_reference_position290.to_index(),
                    )
                };
                {
                    let witnessed_units_2 =
                        witnessed_units + self::read_java_script_semantic_text_length(key.clone());
                    if witnessed_units_2 > 16_777_216f64 {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101]))
                    } else if !(match (previous_witness.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && self::test_java_script_semantic_same_value(
                        previous_witness.clone().expect("a proven present value"),
                        key.clone(),
                    ) {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 104, 97, 118, 101, 32, 117, 110, 105, 113, 117, 101, 32, 103, 108, 111, 98, 97, 108, 32, 107, 101, 121, 115]))
                    } else {
                        {
                            let source_ml_temporary_index82_2 =
                                source_ml_temporary_index82.add(&self::ml::Big::from_i128(1));
                            {
                                (
                                    witnessed_units,
                                    previous_witness,
                                    source_ml_temporary_index82,
                                ) = (
                                    witnessed_units_2,
                                    Some(key.clone()),
                                    source_ml_temporary_index82_2,
                                );
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop96(
    mut registered_keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_temporary_values81: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
    >,
    mut source_ml_temporary_index81: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result {
    loop {
        if source_ml_temporary_index81
            < ({
                let ml_reference_array291 = source_ml_temporary_values81.clone();
                self::ml::Big::from_u128(ml_reference_array291.borrow().len() as u128)
            })
        {
            {
                let fact = {
                    let ml_reference_array292 = source_ml_temporary_values81.clone();
                    let ml_reference_position292 = source_ml_temporary_index81.clone();
                    self::ml_array::at(
                        &*ml_reference_array292.borrow(),
                        ml_reference_position292.to_index(),
                    )
                };
                {
                    let key = match fact {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            _,
                            source_ml_source1_data_property_value19861,
                        ) => ((source_ml_source1_data_property_value19861
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                        .0)
                            .clone(),
                    };
                    if !(match (key.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        {
                            let source_ml_temporary_read_java_script_ordinary_heap_error_ignored115 = {
                                let ml_method_owner293 = registered_keys.clone();
                                let ml_method_argument293_0 =
                                    key.clone().expect("a proven present value");
                                let ml_inserted = vec![ml_method_argument293_0];
                                let mut ml_method_values293 = ml_method_owner293.borrow_mut();
                                let ml_length = ml_method_values293
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                ml_method_values293.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let source_ml_temporary_index81_2 =
                                    source_ml_temporary_index81.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_temporary_index81 = source_ml_temporary_index81_2;
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_temporary_index81_3 =
                                source_ml_temporary_index81.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index81 = source_ml_temporary_index81_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop116(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut prior: self::ml::Big,
    mut source_ml_temporary_values79: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
    >,
    mut source_ml_temporary_index79: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result {
    loop {
        return if source_ml_temporary_index79
            < ({
                let ml_reference_array294 = source_ml_temporary_values79.clone();
                self::ml::Big::from_u128(ml_reference_array294.borrow().len() as u128)
            }) {
            {
                let fact = {
                    let ml_reference_array295 = source_ml_temporary_values79.clone();
                    let ml_reference_position295 = source_ml_temporary_index79.clone();
                    self::ml_array::at(
                        &*ml_reference_array295.borrow(),
                        ml_reference_position295.to_index(),
                    )
                };
                if ((((({
                    let source_ml_source1_optional_chain_receiver19865 = (fact.clone(),);
                    {
                        let source_ml_temporary_optional_chain80 =
                            (source_ml_source1_optional_chain_receiver19865).0;
                        match source_ml_temporary_optional_chain80 {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                _,
                                _,
                                _,
                            ) => self::ml_string::Text::from_units(vec![
                                115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114, 97,
                                116, 105, 111, 110, 70, 97, 99, 116,
                            ]),
                        }
                    }
                }) != (self::ml_string::Text::from_units(vec![
                    115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114, 97, 116, 105,
                    111, 110, 70, 97, 99, 116,
                ]))) || (({
                    let source_ml_source1_type_query_operand19869 = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            source_ml_source1_data_property_value19868,
                            _,
                        ) => source_ml_source1_data_property_value19868,
                    };
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                }) != (self::ml_string::Text::from_units(vec![
                    98, 111, 111, 108, 101, 97, 110,
                ]))))
                    || (({
                        let source_ml_source1_type_query_operand19873 = match fact.clone() {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                source_ml_source1_data_property_value19872,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19872.clone(),
                        };
                        self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                    }) != (self::ml_string::Text::from_units(vec![
                        98, 105, 103, 105, 110, 116,
                    ]))))
                    || ((match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            source_ml_source1_data_property_value19875,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value19875.clone(),
                    }) <= prior))
                    || !(self::test_java_script_ordinary_symbol(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                _,
                                source_ml_source1_data_property_value19876,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19876.clone(),
                        },
                        match fact.clone() {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                source_ml_source1_data_property_value19877,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19877.clone(),
                        },
                    ))
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 102, 97, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 105, 113, 117, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let prior_2 = match fact.clone() {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                source_ml_source1_data_property_value19878,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19878.clone(),
                        };
                        {
                            let key = match fact.clone() {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    _,
                                    _,
                                    source_ml_source1_data_property_value19879,
                                ) => ((source_ml_source1_data_property_value19879
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                            };
                            if !(match (key.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }) && (((!(match fact.clone() {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    _,
                                    source_ml_source1_data_property_value19880,
                                    _,
                                ) => source_ml_source1_data_property_value19880,
                            }) || (match key.clone().expect("a proven present value") {
                                self::JavaScriptSemanticValue::Text(_) => false,
                                _ => true,
                            } && match key
                                .clone()
                                .expect("a proven present value")
                            {
                                self::JavaScriptSemanticValue::SourceText(_, _) => false,
                                _ => true,
                            })) || ((self::read_java_script_ordinary_value_error(
                                heap.clone(),
                                key.clone().expect("a proven present value"),
                            )) != (self::ml_string::Text::from_units(vec![]))))
                                || ((self::read_java_script_semantic_text_length(
                                    key.clone().expect("a proven present value"),
                                )) > 1_048_576f64))
                            {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 85, 84, 70, 49, 54, 32, 116, 101, 120, 116]))
                            } else {
                                {
                                    let source_ml_temporary_index79_2 = source_ml_temporary_index79
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (prior, source_ml_temporary_index79) =
                                            (prior_2, source_ml_temporary_index79_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop94(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut previous: self::ml::Big,
    mut source_ml_temporary_values78: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut source_ml_temporary_index78: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result {
    loop {
        return if source_ml_temporary_index78
            < ({
                let ml_reference_array296 = source_ml_temporary_values78.clone();
                self::ml::Big::from_u128(ml_reference_array296.borrow().len() as u128)
            }) {
            {
                let symbol = {
                    let ml_reference_array297 = source_ml_temporary_values78.clone();
                    let ml_reference_position297 = source_ml_temporary_index78.clone();
                    self::ml_array::at(
                        &*ml_reference_array297.borrow(),
                        ml_reference_position297.to_index(),
                    )
                };
                if ((symbol <= previous)
                    || (symbol > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                    || ((self::find_java_script_ordinary_object(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value19902,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19902.clone(),
                        },
                        symbol.clone(),
                    )) >= 0f64)
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let source_ml_temporary_index78_2 =
                            source_ml_temporary_index78.add(&self::ml::Big::from_i128(1));
                        {
                            (previous, source_ml_temporary_index78) =
                                (symbol.clone(), source_ml_temporary_index78_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Done(self::ml::Big),
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text),
}
