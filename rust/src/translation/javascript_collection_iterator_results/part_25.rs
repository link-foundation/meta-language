// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
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
                let ml_reference_array326 = source_ml_temporary_values83.clone();
                self::ml::Big::from_u128(ml_reference_array326.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array327 = source_ml_temporary_values83.clone();
                    let ml_reference_position327 = source_ml_temporary_index83.clone();
                    self::ml_array::at(
                        &*ml_reference_array327.borrow(),
                        ml_reference_position327.to_index(),
                    )
                };
                if (((({
                    let source_ml_source1_optional_chain_receiver20095 = (entry.clone(),);
                    {
                        let source_ml_temporary_optional_chain84 =
                            (source_ml_source1_optional_chain_receiver20095).0;
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
                            source_ml_source1_data_property_value20096,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value20096.clone(),
                    },
                    match entry.clone() {
                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                            source_ml_source1_data_property_value20097,
                            _,
                        ) => source_ml_source1_data_property_value20097.clone(),
                    },
                ))) || ((({
                    let source_ml_source1_optional_chain_receiver20101 = (match entry.clone() {
                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                            _,
                            source_ml_source1_data_property_value20100,
                        ) => {
                            let source_ml_source1_data_property_value20100 =
                                *source_ml_source1_data_property_value20100;
                            source_ml_source1_data_property_value20100.clone()
                        }
                    },);
                    {
                        let source_ml_temporary_optional_chain85 =
                            (source_ml_source1_optional_chain_receiver20101).0;
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
                        let source_ml_source1_optional_chain_receiver20105 =
                            (match entry.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    _,
                                    source_ml_source1_data_property_value20104,
                                ) => {
                                    let source_ml_source1_data_property_value20104 =
                                        *source_ml_source1_data_property_value20104;
                                    source_ml_source1_data_property_value20104.clone()
                                }
                            },);
                        {
                            let source_ml_temporary_optional_chain86 =
                                (source_ml_source1_optional_chain_receiver20105).0;
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
                                source_ml_source1_data_property_value20107,
                            ) => {
                                let source_ml_source1_data_property_value20107 =
                                    *source_ml_source1_data_property_value20107;
                                source_ml_source1_data_property_value20107.clone()
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
                                    source_ml_source1_data_property_value20108,
                                ) => {
                                    let source_ml_source1_data_property_value20108 =
                                        *source_ml_source1_data_property_value20108;
                                    source_ml_source1_data_property_value20108.clone()
                                }
                            });
                        if ((self::read_java_script_semantic_text_length(match entry.clone() {
                            self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                _,
                                source_ml_source1_data_property_value20110,
                            ) => {
                                let source_ml_source1_data_property_value20110 =
                                    *source_ml_source1_data_property_value20110;
                                source_ml_source1_data_property_value20110.clone()
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
                                    source_ml_source1_data_property_value20121,
                                ) => {
                                    let source_ml_source1_data_property_value20121 =
                                        *source_ml_source1_data_property_value20121;
                                    source_ml_source1_data_property_value20121.clone()
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
                                        source_ml_source1_data_property_value20122,
                                    ) => {
                                        let source_ml_source1_data_property_value20122 =
                                            *source_ml_source1_data_property_value20122;
                                        source_ml_source1_data_property_value20122.clone()
                                    }
                                };
                                {
                                    let low = 0f64;
                                    {
                                        let high = {
                                            let ml_reference_array328 = facts.clone();
                                            ml_reference_array328.borrow().len() as f64
                                        };
                                        {
                                            let low_2 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop113(facts.clone(), self::MlOptional::Value(entry.clone()), low, high);
                                            {
                                                let fact = {
                                                    let ml_reference_array329 = facts.clone();
                                                    let ml_reference_position329 = low_2;
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array329.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position329,
                                                        ),
                                                    )
                                                };
                                                if (match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match fact.clone().expect("a proven present value") {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value20127, _, _) => {
                                                            source_ml_source1_data_property_value20127.clone()
                                                        }
                                                    }) != (match entry.clone() {
                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source1_data_property_value20128, _) => {
                                                            source_ml_source1_data_property_value20128.clone()
                                                        }
                                                    }))) || !(match fact.clone().expect("a proven present value") {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source1_data_property_value20129, _) => {
                                                            source_ml_source1_data_property_value20129
                                                        }
                                                    }) {
                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115]))
                                                    } else {
                                                        {
                                                            let key = match fact.clone().expect("a proven present value") {
                                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source1_data_property_value20130) => {
                                                                    ((source_ml_source1_data_property_value20130.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                }
                                                            };
                                                            if match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(key.clone().expect("a proven present value"), match entry.clone() {
                                                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(_, source_ml_source1_data_property_value20131) => {
                                                                    let source_ml_source1_data_property_value20131 = *source_ml_source1_data_property_value20131;
                                                                    source_ml_source1_data_property_value20131.clone()
                                                                }
                                                            })) {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_temporary_read_java_script_ordinary_heap_error_ignored114 = { let ml_method_owner330 = identities.clone(); let ml_method_argument330_0 = self::JavaScriptSemanticValue::Symbol(match entry.clone() {
                                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source1_data_property_value20132, _) => {
                                                                            source_ml_source1_data_property_value20132.clone()
                                                                        }
                                                                    }); let ml_inserted = vec![ml_method_argument330_0]; let mut ml_method_values330 = ml_method_owner330.borrow_mut(); let ml_length = ml_method_values330.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values330.extend(ml_inserted); ml_length as f64 };
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
                let ml_reference_array331 = source_ml_temporary_values82.clone();
                self::ml::Big::from_u128(ml_reference_array331.borrow().len() as u128)
            }) {
            {
                let key = {
                    let ml_reference_array332 = source_ml_temporary_values82.clone();
                    let ml_reference_position332 = source_ml_temporary_index82.clone();
                    self::ml_array::at(
                        &*ml_reference_array332.borrow(),
                        ml_reference_position332.to_index(),
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
                let ml_reference_array333 = source_ml_temporary_values81.clone();
                self::ml::Big::from_u128(ml_reference_array333.borrow().len() as u128)
            })
        {
            {
                let fact = {
                    let ml_reference_array334 = source_ml_temporary_values81.clone();
                    let ml_reference_position334 = source_ml_temporary_index81.clone();
                    self::ml_array::at(
                        &*ml_reference_array334.borrow(),
                        ml_reference_position334.to_index(),
                    )
                };
                {
                    let key = match fact {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            _,
                            source_ml_source1_data_property_value20137,
                        ) => ((source_ml_source1_data_property_value20137
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
                                let ml_method_owner335 = registered_keys.clone();
                                let ml_method_argument335_0 =
                                    key.clone().expect("a proven present value");
                                let ml_inserted = vec![ml_method_argument335_0];
                                let mut ml_method_values335 = ml_method_owner335.borrow_mut();
                                let ml_length = ml_method_values335
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                ml_method_values335.extend(ml_inserted);
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
                let ml_reference_array336 = source_ml_temporary_values79.clone();
                self::ml::Big::from_u128(ml_reference_array336.borrow().len() as u128)
            }) {
            {
                let fact = {
                    let ml_reference_array337 = source_ml_temporary_values79.clone();
                    let ml_reference_position337 = source_ml_temporary_index79.clone();
                    self::ml_array::at(
                        &*ml_reference_array337.borrow(),
                        ml_reference_position337.to_index(),
                    )
                };
                if ((((({
                    let source_ml_source1_optional_chain_receiver20141 = (fact.clone(),);
                    {
                        let source_ml_temporary_optional_chain80 =
                            (source_ml_source1_optional_chain_receiver20141).0;
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
                    let source_ml_source1_type_query_operand20145 = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            source_ml_source1_data_property_value20144,
                            _,
                        ) => source_ml_source1_data_property_value20144,
                    };
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                }) != (self::ml_string::Text::from_units(vec![
                    98, 111, 111, 108, 101, 97, 110,
                ]))))
                    || (({
                        let source_ml_source1_type_query_operand20149 = match fact.clone() {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                source_ml_source1_data_property_value20148,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20148.clone(),
                        };
                        self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                    }) != (self::ml_string::Text::from_units(vec![
                        98, 105, 103, 105, 110, 116,
                    ]))))
                    || ((match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            source_ml_source1_data_property_value20151,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value20151.clone(),
                    }) <= prior))
                    || !(self::test_java_script_ordinary_symbol(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                _,
                                source_ml_source1_data_property_value20152,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20152.clone(),
                        },
                        match fact.clone() {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                source_ml_source1_data_property_value20153,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20153.clone(),
                        },
                    ))
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 102, 97, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 105, 113, 117, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let prior_2 = match fact.clone() {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                source_ml_source1_data_property_value20154,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20154.clone(),
                        };
                        {
                            let key = match fact.clone() {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    _,
                                    _,
                                    source_ml_source1_data_property_value20155,
                                ) => ((source_ml_source1_data_property_value20155
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
                                    source_ml_source1_data_property_value20156,
                                    _,
                                ) => source_ml_source1_data_property_value20156,
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
                let ml_reference_array338 = source_ml_temporary_values78.clone();
                self::ml::Big::from_u128(ml_reference_array338.borrow().len() as u128)
            }) {
            {
                let symbol = {
                    let ml_reference_array339 = source_ml_temporary_values78.clone();
                    let ml_reference_position339 = source_ml_temporary_index78.clone();
                    self::ml_array::at(
                        &*ml_reference_array339.borrow(),
                        ml_reference_position339.to_index(),
                    )
                };
                if ((symbol <= previous)
                    || (symbol > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                    || ((self::find_java_script_ordinary_object(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value20178,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20178.clone(),
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
