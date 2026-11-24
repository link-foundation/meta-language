// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::MlOptional;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result;
use super::compare_java_script_collection_keys;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop113;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop109(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>,
    mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut previous_key: self::JavaScriptSemanticValue,
    mut units: f64,
    mut source_ml_source1_temporary_values83: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>,
    >,
    mut source_ml_source1_temporary_index83: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result {
    loop {
        return if source_ml_source1_temporary_index83
            < ({
                let ml_reference_array312 = source_ml_source1_temporary_values83.clone();
                self::ml::Big::from_u128(ml_reference_array312.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array313 = source_ml_source1_temporary_values83.clone();
                    let ml_reference_position313 = source_ml_source1_temporary_index83.clone();
                    self::ml_array::at(
                        &*ml_reference_array313.borrow(),
                        ml_reference_position313.to_index(),
                    )
                };
                if (((({
                    let source_ml_source2_optional_chain_receiver18922 = (entry.clone(),);
                    {
                        let source_ml_source1_temporary_optional_chain84 =
                            (source_ml_source2_optional_chain_receiver18922).0;
                        match source_ml_source1_temporary_optional_chain84 {
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
                            source_ml_source2_data_property_value18923,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18923.clone(),
                    },
                    match entry.clone() {
                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                            source_ml_source2_data_property_value18924,
                            _,
                        ) => source_ml_source2_data_property_value18924.clone(),
                    },
                ))) || ((({
                    let source_ml_source2_optional_chain_receiver18928 = (match entry.clone() {
                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                            _,
                            source_ml_source2_data_property_value18927,
                        ) => {
                            let source_ml_source2_data_property_value18927 =
                                *source_ml_source2_data_property_value18927;
                            source_ml_source2_data_property_value18927.clone()
                        }
                    },);
                    {
                        let source_ml_source1_temporary_optional_chain85 =
                            (source_ml_source2_optional_chain_receiver18928).0;
                        match source_ml_source1_temporary_optional_chain85 {
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
                        let source_ml_source2_optional_chain_receiver18932 =
                            (match entry.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    _,
                                    source_ml_source2_data_property_value18931,
                                ) => {
                                    let source_ml_source2_data_property_value18931 =
                                        *source_ml_source2_data_property_value18931;
                                    source_ml_source2_data_property_value18931.clone()
                                }
                            },);
                        {
                            let source_ml_source1_temporary_optional_chain86 =
                                (source_ml_source2_optional_chain_receiver18932).0;
                            match source_ml_source1_temporary_optional_chain86 {
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
                                source_ml_source2_data_property_value18934,
                            ) => {
                                let source_ml_source2_data_property_value18934 =
                                    *source_ml_source2_data_property_value18934;
                                source_ml_source2_data_property_value18934.clone()
                            }
                        },
                    )) != (self::ml_string::Text::from_units(vec![])))
                {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 97, 110, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 85, 84, 70, 49, 54, 32, 107, 101, 121, 115]))
                } else {
                    {
                        let units_2 = units
                            + self::read_java_script_semantic_text_length(match entry.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    _,
                                    source_ml_source2_data_property_value18935,
                                ) => {
                                    let source_ml_source2_data_property_value18935 =
                                        *source_ml_source2_data_property_value18935;
                                    source_ml_source2_data_property_value18935.clone()
                                }
                            });
                        if ((self::read_java_script_semantic_text_length(match entry.clone() {
                            self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                _,
                                source_ml_source2_data_property_value18937,
                            ) => {
                                let source_ml_source2_data_property_value18937 =
                                    *source_ml_source2_data_property_value18937;
                                source_ml_source2_data_property_value18937.clone()
                            }
                        })) > 1_048_576f64)
                            || (units_2 > 16_777_216f64)
                        {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101]))
                        } else if match previous_key.clone() {
                            self::JavaScriptSemanticValue::Undefined => false,
                            _ => true,
                        } && ((self::compare_java_script_collection_keys(
                            previous_key.clone(),
                            match entry.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    _,
                                    source_ml_source2_data_property_value18948,
                                ) => {
                                    let source_ml_source2_data_property_value18948 =
                                        *source_ml_source2_data_property_value18948;
                                    source_ml_source2_data_property_value18948.clone()
                                }
                            },
                        )) >= 0f64)
                        {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 115, 116, 114, 105, 99, 116, 32, 85, 84, 70, 49, 54, 32, 111, 114, 100, 101, 114]))
                        } else {
                            {
                                let previous_key_2 = match entry.clone() {
                                    self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                        _,
                                        source_ml_source2_data_property_value18949,
                                    ) => {
                                        let source_ml_source2_data_property_value18949 =
                                            *source_ml_source2_data_property_value18949;
                                        source_ml_source2_data_property_value18949.clone()
                                    }
                                };
                                {
                                    let low = 0f64;
                                    {
                                        let high = {
                                            let ml_reference_array314 = facts.clone();
                                            ml_reference_array314.borrow().len() as f64
                                        };
                                        {
                                            let low_2 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop113(facts.clone(), self::MlOptional::Value(entry.clone()), low, high);
                                            {
                                                let fact = {
                                                    let ml_reference_array315 = facts.clone();
                                                    let ml_reference_position315 = low_2;
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array315.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position315,
                                                        ),
                                                    )
                                                };
                                                if (match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match fact.clone().expect("a proven present value") {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source2_data_property_value18954, _, _) => {
                                                            source_ml_source2_data_property_value18954.clone()
                                                        }
                                                    }) != (match entry.clone() {
                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source2_data_property_value18955, _) => {
                                                            source_ml_source2_data_property_value18955.clone()
                                                        }
                                                    }))) || !(match fact.clone().expect("a proven present value") {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source2_data_property_value18956, _) => {
                                                            source_ml_source2_data_property_value18956
                                                        }
                                                    }) {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115]))
                                                    } else {
                                                        {
                                                            let key = match fact.clone().expect("a proven present value") {
                                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source2_data_property_value18957) => {
                                                                    ((source_ml_source2_data_property_value18957.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                }
                                                            };
                                                            if match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(key.clone().expect("a proven present value"), match entry.clone() {
                                                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(_, source_ml_source2_data_property_value18958) => {
                                                                    let source_ml_source2_data_property_value18958 = *source_ml_source2_data_property_value18958;
                                                                    source_ml_source2_data_property_value18958.clone()
                                                                }
                                                            })) {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_read_java_script_ordinary_heap_error_ignored114 = { let ml_method_owner316 = identities.clone(); let ml_method_argument316_0 = self::JavaScriptSemanticValue::Symbol(match entry.clone() {
                                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source2_data_property_value18959, _) => {
                                                                            source_ml_source2_data_property_value18959.clone()
                                                                        }
                                                                    }); let ml_inserted = vec![ml_method_argument316_0]; let mut ml_method_values316 = ml_method_owner316.borrow_mut(); let ml_length = ml_method_values316.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values316.extend(ml_inserted); ml_length as f64 };
                                                                    {
                                                                        let source_ml_source1_temporary_index83_2 = source_ml_source1_temporary_index83.add(&self::ml::Big::from_i128(1));
                                                                        {
                                                                            (previous_key, units, source_ml_source1_temporary_index83) = (previous_key_2, units_2, source_ml_source1_temporary_index83_2);
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
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop97(
    mut witnessed_units: f64,
    mut previous_witness: Option<self::JavaScriptSemanticValue>,
    mut source_ml_source1_temporary_values82: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index82: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result {
    loop {
        return if source_ml_source1_temporary_index82
            < ({
                let ml_reference_array317 = source_ml_source1_temporary_values82.clone();
                self::ml::Big::from_u128(ml_reference_array317.borrow().len() as u128)
            }) {
            {
                let key = {
                    let ml_reference_array318 = source_ml_source1_temporary_values82.clone();
                    let ml_reference_position318 = source_ml_source1_temporary_index82.clone();
                    self::ml_array::at(
                        &*ml_reference_array318.borrow(),
                        ml_reference_position318.to_index(),
                    )
                };
                {
                    let witnessed_units_2 =
                        witnessed_units + self::read_java_script_semantic_text_length(key.clone());
                    if witnessed_units_2 > 16_777_216f64 {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101]))
                    } else if !(match (previous_witness.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && self::test_java_script_semantic_same_value(
                        previous_witness.clone().expect("a proven present value"),
                        key.clone(),
                    ) {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 104, 97, 118, 101, 32, 117, 110, 105, 113, 117, 101, 32, 103, 108, 111, 98, 97, 108, 32, 107, 101, 121, 115]))
                    } else {
                        {
                            let source_ml_source1_temporary_index82_2 =
                                source_ml_source1_temporary_index82
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (
                                    witnessed_units,
                                    previous_witness,
                                    source_ml_source1_temporary_index82,
                                ) = (
                                    witnessed_units_2,
                                    Some(key.clone()),
                                    source_ml_source1_temporary_index82_2,
                                );
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop96Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop96(
    mut registered_keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_source1_temporary_values81: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
    >,
    mut source_ml_source1_temporary_index81: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result {
    loop {
        if source_ml_source1_temporary_index81
            < ({
                let ml_reference_array319 = source_ml_source1_temporary_values81.clone();
                self::ml::Big::from_u128(ml_reference_array319.borrow().len() as u128)
            })
        {
            {
                let fact = {
                    let ml_reference_array320 = source_ml_source1_temporary_values81.clone();
                    let ml_reference_position320 = source_ml_source1_temporary_index81.clone();
                    self::ml_array::at(
                        &*ml_reference_array320.borrow(),
                        ml_reference_position320.to_index(),
                    )
                };
                {
                    let key = match fact {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            _,
                            source_ml_source2_data_property_value18964,
                        ) => ((source_ml_source2_data_property_value18964
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
                            let source_ml_source1_temporary_read_java_script_ordinary_heap_error_ignored115 = {
                                let ml_method_owner321 = registered_keys.clone();
                                let ml_method_argument321_0 =
                                    key.clone().expect("a proven present value");
                                let ml_inserted = vec![ml_method_argument321_0];
                                let mut ml_method_values321 = ml_method_owner321.borrow_mut();
                                let ml_length = ml_method_values321
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                ml_method_values321.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let source_ml_source1_temporary_index81_2 =
                                    source_ml_source1_temporary_index81
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index81 =
                                        source_ml_source1_temporary_index81_2;
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_source1_temporary_index81_3 =
                                source_ml_source1_temporary_index81
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index81 =
                                    source_ml_source1_temporary_index81_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop96Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop116(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut prior: self::ml::Big,
    mut source_ml_source1_temporary_values79: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
    >,
    mut source_ml_source1_temporary_index79: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result {
    loop {
        return if source_ml_source1_temporary_index79
            < ({
                let ml_reference_array322 = source_ml_source1_temporary_values79.clone();
                self::ml::Big::from_u128(ml_reference_array322.borrow().len() as u128)
            }) {
            {
                let fact = {
                    let ml_reference_array323 = source_ml_source1_temporary_values79.clone();
                    let ml_reference_position323 = source_ml_source1_temporary_index79.clone();
                    self::ml_array::at(
                        &*ml_reference_array323.borrow(),
                        ml_reference_position323.to_index(),
                    )
                };
                if ((((({
                    let source_ml_source2_optional_chain_receiver18968 = (fact.clone(),);
                    {
                        let source_ml_source1_temporary_optional_chain80 =
                            (source_ml_source2_optional_chain_receiver18968).0;
                        match source_ml_source1_temporary_optional_chain80 {
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
                    let source_ml_source2_type_query_operand18972 = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            source_ml_source2_data_property_value18971,
                            _,
                        ) => source_ml_source2_data_property_value18971,
                    };
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                }) != (self::ml_string::Text::from_units(vec![
                    98, 111, 111, 108, 101, 97, 110,
                ]))))
                    || (({
                        let source_ml_source2_type_query_operand18976 = match fact.clone() {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                source_ml_source2_data_property_value18975,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18975.clone(),
                        };
                        self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                    }) != (self::ml_string::Text::from_units(vec![
                        98, 105, 103, 105, 110, 116,
                    ]))))
                    || ((match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            source_ml_source2_data_property_value18978,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18978.clone(),
                    }) <= prior))
                    || !(self::test_java_script_ordinary_symbol(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                _,
                                source_ml_source2_data_property_value18979,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18979.clone(),
                        },
                        match fact.clone() {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                source_ml_source2_data_property_value18980,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18980.clone(),
                        },
                    ))
                {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 102, 97, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 105, 113, 117, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let prior_2 = match fact.clone() {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                source_ml_source2_data_property_value18981,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value18981.clone(),
                        };
                        {
                            let key = match fact.clone() {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value18982,
                                ) => ((source_ml_source2_data_property_value18982
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
                                    source_ml_source2_data_property_value18983,
                                    _,
                                ) => source_ml_source2_data_property_value18983,
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
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 85, 84, 70, 49, 54, 32, 116, 101, 120, 116]))
                            } else {
                                {
                                    let source_ml_source1_temporary_index79_2 =
                                        source_ml_source1_temporary_index79
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (prior, source_ml_source1_temporary_index79) =
                                            (prior_2, source_ml_source1_temporary_index79_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop94(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut previous: self::ml::Big,
    mut source_ml_source1_temporary_values78: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut source_ml_source1_temporary_index78: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result {
    loop {
        return if source_ml_source1_temporary_index78
            < ({
                let ml_reference_array324 = source_ml_source1_temporary_values78.clone();
                self::ml::Big::from_u128(ml_reference_array324.borrow().len() as u128)
            }) {
            {
                let symbol = {
                    let ml_reference_array325 = source_ml_source1_temporary_values78.clone();
                    let ml_reference_position325 = source_ml_source1_temporary_index78.clone();
                    self::ml_array::at(
                        &*ml_reference_array325.borrow(),
                        ml_reference_position325.to_index(),
                    )
                };
                if ((symbol <= previous)
                    || (symbol > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                    || ((self::find_java_script_ordinary_object(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source2_data_property_value19005,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value19005.clone(),
                        },
                        symbol.clone(),
                    )) >= 0f64)
                {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index78_2 =
                            source_ml_source1_temporary_index78.add(&self::ml::Big::from_i128(1));
                        {
                            (previous, source_ml_source1_temporary_index78) =
                                (symbol.clone(), source_ml_source1_temporary_index78_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text),
}
