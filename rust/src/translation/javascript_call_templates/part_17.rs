// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::MlOptional;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Result;
use super::compare_java_script_collection_keys;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop124;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop120(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>,
    mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut previous_key: self::JavaScriptSemanticValue,
    mut units: f64,
    mut source_ml_temporary_values94: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>,
    >,
    mut source_ml_temporary_index94: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Result {
    loop {
        return if source_ml_temporary_index94
            < ({
                let ml_reference_array168 = source_ml_temporary_values94.clone();
                self::ml::Big::from_u128(ml_reference_array168.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array169 = source_ml_temporary_values94.clone();
                    let ml_reference_position169 = source_ml_temporary_index94.clone();
                    self::ml_array::at(
                        &*ml_reference_array169.borrow(),
                        ml_reference_position169.to_index(),
                    )
                };
                if (((({
                    let source_ml_source1_optional_chain_receiver19523 = (entry.clone(),);
                    {
                        let source_ml_temporary_optional_chain95 =
                            (source_ml_source1_optional_chain_receiver19523).0;
                        match source_ml_temporary_optional_chain95 {
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
                            source_ml_source1_data_property_value19524,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value19524.clone(),
                    },
                    match entry.clone() {
                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                            source_ml_source1_data_property_value19525,
                            _,
                        ) => source_ml_source1_data_property_value19525.clone(),
                    },
                ))) || ((({
                    let source_ml_source1_optional_chain_receiver19529 = (match entry.clone() {
                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                            _,
                            source_ml_source1_data_property_value19528,
                        ) => {
                            let source_ml_source1_data_property_value19528 =
                                *source_ml_source1_data_property_value19528;
                            source_ml_source1_data_property_value19528.clone()
                        }
                    },);
                    {
                        let source_ml_temporary_optional_chain96 =
                            (source_ml_source1_optional_chain_receiver19529).0;
                        match source_ml_temporary_optional_chain96 {
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
                        let source_ml_source1_optional_chain_receiver19533 =
                            (match entry.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    _,
                                    source_ml_source1_data_property_value19532,
                                ) => {
                                    let source_ml_source1_data_property_value19532 =
                                        *source_ml_source1_data_property_value19532;
                                    source_ml_source1_data_property_value19532.clone()
                                }
                            },);
                        {
                            let source_ml_temporary_optional_chain97 =
                                (source_ml_source1_optional_chain_receiver19533).0;
                            match source_ml_temporary_optional_chain97 {
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
                                source_ml_source1_data_property_value19535,
                            ) => {
                                let source_ml_source1_data_property_value19535 =
                                    *source_ml_source1_data_property_value19535;
                                source_ml_source1_data_property_value19535.clone()
                            }
                        },
                    )) != (self::ml_string::Text::from_units(vec![])))
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 97, 110, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 85, 84, 70, 49, 54, 32, 107, 101, 121, 115]))
                } else {
                    {
                        let units_2 = units
                            + self::read_java_script_semantic_text_length(match entry.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    _,
                                    source_ml_source1_data_property_value19536,
                                ) => {
                                    let source_ml_source1_data_property_value19536 =
                                        *source_ml_source1_data_property_value19536;
                                    source_ml_source1_data_property_value19536.clone()
                                }
                            });
                        if ((self::read_java_script_semantic_text_length(match entry.clone() {
                            self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                _,
                                source_ml_source1_data_property_value19538,
                            ) => {
                                let source_ml_source1_data_property_value19538 =
                                    *source_ml_source1_data_property_value19538;
                                source_ml_source1_data_property_value19538.clone()
                            }
                        })) > 1_048_576f64)
                            || (units_2 > 16_777_216f64)
                        {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101]))
                        } else if match previous_key.clone() {
                            self::JavaScriptSemanticValue::Undefined => false,
                            _ => true,
                        } && ((self::compare_java_script_collection_keys(
                            previous_key.clone(),
                            match entry.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    _,
                                    source_ml_source1_data_property_value19549,
                                ) => {
                                    let source_ml_source1_data_property_value19549 =
                                        *source_ml_source1_data_property_value19549;
                                    source_ml_source1_data_property_value19549.clone()
                                }
                            },
                        )) >= 0f64)
                        {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 115, 116, 114, 105, 99, 116, 32, 85, 84, 70, 49, 54, 32, 111, 114, 100, 101, 114]))
                        } else {
                            {
                                let previous_key_2 = match entry.clone() {
                                    self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                        _,
                                        source_ml_source1_data_property_value19550,
                                    ) => {
                                        let source_ml_source1_data_property_value19550 =
                                            *source_ml_source1_data_property_value19550;
                                        source_ml_source1_data_property_value19550.clone()
                                    }
                                };
                                {
                                    let low = 0f64;
                                    {
                                        let high = {
                                            let ml_reference_array170 = facts.clone();
                                            ml_reference_array170.borrow().len() as f64
                                        };
                                        {
                                            let low_2 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop124(facts.clone(), self::MlOptional::Value(entry.clone()), low, high);
                                            {
                                                let fact = {
                                                    let ml_reference_array171 = facts.clone();
                                                    let ml_reference_position171 = low_2;
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array171.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position171,
                                                        ),
                                                    )
                                                };
                                                if (match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match fact.clone().expect("a proven present value") {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value19555, _, _) => {
                                                            source_ml_source1_data_property_value19555.clone()
                                                        }
                                                    }) != (match entry.clone() {
                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source1_data_property_value19556, _) => {
                                                            source_ml_source1_data_property_value19556.clone()
                                                        }
                                                    }))) || !(match fact.clone().expect("a proven present value") {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source1_data_property_value19557, _) => {
                                                            source_ml_source1_data_property_value19557
                                                        }
                                                    }) {
                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115]))
                                                    } else {
                                                        {
                                                            let key = match fact.clone().expect("a proven present value") {
                                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source1_data_property_value19558) => {
                                                                    ((source_ml_source1_data_property_value19558.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                }
                                                            };
                                                            if match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(key.clone().expect("a proven present value"), match entry.clone() {
                                                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(_, source_ml_source1_data_property_value19559) => {
                                                                    let source_ml_source1_data_property_value19559 = *source_ml_source1_data_property_value19559;
                                                                    source_ml_source1_data_property_value19559.clone()
                                                                }
                                                            })) {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_temporary_read_java_script_ordinary_heap_error_ignored125 = { let ml_method_owner172 = identities.clone(); let ml_method_argument172_0 = self::JavaScriptSemanticValue::Symbol(match entry.clone() {
                                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source1_data_property_value19560, _) => {
                                                                            source_ml_source1_data_property_value19560.clone()
                                                                        }
                                                                    }); let ml_inserted = vec![ml_method_argument172_0]; let mut ml_method_values172 = ml_method_owner172.borrow_mut(); let ml_length = ml_method_values172.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values172.extend(ml_inserted); ml_length as f64 };
                                                                    {
                                                                        let source_ml_temporary_index94_2 = source_ml_temporary_index94.add(&self::ml::Big::from_i128(1));
                                                                        {
                                                                            (previous_key, units, source_ml_temporary_index94) = (previous_key_2, units_2, source_ml_temporary_index94_2);
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
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop108(
    mut witnessed_units: f64,
    mut previous_witness: Option<self::JavaScriptSemanticValue>,
    mut source_ml_temporary_values93: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index93: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Result {
    loop {
        return if source_ml_temporary_index93
            < ({
                let ml_reference_array173 = source_ml_temporary_values93.clone();
                self::ml::Big::from_u128(ml_reference_array173.borrow().len() as u128)
            }) {
            {
                let key = {
                    let ml_reference_array174 = source_ml_temporary_values93.clone();
                    let ml_reference_position174 = source_ml_temporary_index93.clone();
                    self::ml_array::at(
                        &*ml_reference_array174.borrow(),
                        ml_reference_position174.to_index(),
                    )
                };
                {
                    let witnessed_units_2 =
                        witnessed_units + self::read_java_script_semantic_text_length(key.clone());
                    if witnessed_units_2 > 16_777_216f64 {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101]))
                    } else if !(match (previous_witness.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && self::test_java_script_semantic_same_value(
                        previous_witness.clone().expect("a proven present value"),
                        key.clone(),
                    ) {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 104, 97, 118, 101, 32, 117, 110, 105, 113, 117, 101, 32, 103, 108, 111, 98, 97, 108, 32, 107, 101, 121, 115]))
                    } else {
                        {
                            let source_ml_temporary_index93_2 =
                                source_ml_temporary_index93.add(&self::ml::Big::from_i128(1));
                            {
                                (
                                    witnessed_units,
                                    previous_witness,
                                    source_ml_temporary_index93,
                                ) = (
                                    witnessed_units_2,
                                    Some(key.clone()),
                                    source_ml_temporary_index93_2,
                                );
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop107Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop107Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop107(
    mut registered_keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_temporary_values92: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
    >,
    mut source_ml_temporary_index92: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop107Result {
    loop {
        if source_ml_temporary_index92
            < ({
                let ml_reference_array175 = source_ml_temporary_values92.clone();
                self::ml::Big::from_u128(ml_reference_array175.borrow().len() as u128)
            })
        {
            {
                let fact = {
                    let ml_reference_array176 = source_ml_temporary_values92.clone();
                    let ml_reference_position176 = source_ml_temporary_index92.clone();
                    self::ml_array::at(
                        &*ml_reference_array176.borrow(),
                        ml_reference_position176.to_index(),
                    )
                };
                {
                    let key = match fact {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            _,
                            source_ml_source1_data_property_value19565,
                        ) => ((source_ml_source1_data_property_value19565
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
                            let source_ml_temporary_read_java_script_ordinary_heap_error_ignored126 = {
                                let ml_method_owner177 = registered_keys.clone();
                                let ml_method_argument177_0 =
                                    key.clone().expect("a proven present value");
                                let ml_inserted = vec![ml_method_argument177_0];
                                let mut ml_method_values177 = ml_method_owner177.borrow_mut();
                                let ml_length = ml_method_values177
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                ml_method_values177.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let source_ml_temporary_index92_2 =
                                    source_ml_temporary_index92.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_temporary_index92 = source_ml_temporary_index92_2;
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_temporary_index92_3 =
                                source_ml_temporary_index92.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index92 = source_ml_temporary_index92_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop107Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop107Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop127(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut prior: self::ml::Big,
    mut source_ml_temporary_values90: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
    >,
    mut source_ml_temporary_index90: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Result {
    loop {
        return if source_ml_temporary_index90
            < ({
                let ml_reference_array178 = source_ml_temporary_values90.clone();
                self::ml::Big::from_u128(ml_reference_array178.borrow().len() as u128)
            }) {
            {
                let fact = {
                    let ml_reference_array179 = source_ml_temporary_values90.clone();
                    let ml_reference_position179 = source_ml_temporary_index90.clone();
                    self::ml_array::at(
                        &*ml_reference_array179.borrow(),
                        ml_reference_position179.to_index(),
                    )
                };
                if ((((({
                    let source_ml_source1_optional_chain_receiver19569 = (fact.clone(),);
                    {
                        let source_ml_temporary_optional_chain91 =
                            (source_ml_source1_optional_chain_receiver19569).0;
                        match source_ml_temporary_optional_chain91 {
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
                    let source_ml_source1_type_query_operand19573 = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            source_ml_source1_data_property_value19572,
                            _,
                        ) => source_ml_source1_data_property_value19572,
                    };
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                }) != (self::ml_string::Text::from_units(vec![
                    98, 111, 111, 108, 101, 97, 110,
                ]))))
                    || (({
                        let source_ml_source1_type_query_operand19577 = match fact.clone() {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                source_ml_source1_data_property_value19576,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19576.clone(),
                        };
                        self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                    }) != (self::ml_string::Text::from_units(vec![
                        98, 105, 103, 105, 110, 116,
                    ]))))
                    || ((match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            source_ml_source1_data_property_value19579,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value19579.clone(),
                    }) <= prior))
                    || !(self::test_java_script_ordinary_symbol(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                _,
                                source_ml_source1_data_property_value19580,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19580.clone(),
                        },
                        match fact.clone() {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                source_ml_source1_data_property_value19581,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19581.clone(),
                        },
                    ))
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 102, 97, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 105, 113, 117, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let prior_2 = match fact.clone() {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                source_ml_source1_data_property_value19582,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19582.clone(),
                        };
                        {
                            let key = match fact.clone() {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    _,
                                    _,
                                    source_ml_source1_data_property_value19583,
                                ) => ((source_ml_source1_data_property_value19583
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
                                    source_ml_source1_data_property_value19584,
                                    _,
                                ) => source_ml_source1_data_property_value19584,
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
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 85, 84, 70, 49, 54, 32, 116, 101, 120, 116]))
                            } else {
                                {
                                    let source_ml_temporary_index90_2 = source_ml_temporary_index90
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (prior, source_ml_temporary_index90) =
                                            (prior_2, source_ml_temporary_index90_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop105(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut previous: self::ml::Big,
    mut source_ml_temporary_values89: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut source_ml_temporary_index89: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result {
    loop {
        return if source_ml_temporary_index89
            < ({
                let ml_reference_array180 = source_ml_temporary_values89.clone();
                self::ml::Big::from_u128(ml_reference_array180.borrow().len() as u128)
            }) {
            {
                let symbol = {
                    let ml_reference_array181 = source_ml_temporary_values89.clone();
                    let ml_reference_position181 = source_ml_temporary_index89.clone();
                    self::ml_array::at(
                        &*ml_reference_array181.borrow(),
                        ml_reference_position181.to_index(),
                    )
                };
                if ((symbol <= previous)
                    || (symbol > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                    || ((self::find_java_script_ordinary_object(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value19606,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19606.clone(),
                        },
                        symbol.clone(),
                    )) >= 0f64)
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let source_ml_temporary_index89_2 =
                            source_ml_temporary_index89.add(&self::ml::Big::from_i128(1));
                        {
                            (previous, source_ml_temporary_index89) =
                                (symbol.clone(), source_ml_temporary_index89_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Done(self::ml::Big),
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Return(self::ml_string::Text),
}
