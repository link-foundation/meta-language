// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
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
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop113(
    mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>,
    mut entry: self::MlOptional<self::JavaScriptSymbolRegistryEntry>,
    mut low: f64,
    mut high: f64,
) -> f64 {
    loop {
        return if (low < high) {
            {
                let middle = ((low + high) / 2f64).floor();
                if ((({
                    let source_ml_source2_optional_chain_receiver39422 = ({
                        let ml_reference_array2601 = facts.clone();
                        let ml_reference_position2601 = middle;
                        let ml_reference_projection2601 = self::ml_array::optional_at(
                            &*ml_reference_array2601.borrow(),
                            self::ml_array::number_index(ml_reference_position2601),
                        );
                        ml_reference_projection2601
                    },);
                    if match (source_ml_source2_optional_chain_receiver39422.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        Some({
                            let source_ml_source1_temporary_optional_chain87 =
                                ((source_ml_source2_optional_chain_receiver39422.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            match source_ml_source1_temporary_optional_chain87.clone() {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    source_ml_source2_data_property_value39423,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39423.clone(),
                            }
                        })
                    } else {
                        None::<self::ml::Big>
                    }
                })
                .clone()
                .unwrap_or_else(|| {
                    let source_ml_source2_nullish_field_receiver39424 = (entry.clone(),);
                    if match (source_ml_source2_nullish_field_receiver39424.clone()).0 {
                        self::MlOptional::Value(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        self::MlOptional::Null => false,
                        self::MlOptional::Undefined => false,
                    } {
                        {
                            let source_ml_source2_nullish_field_present39425 =
                                match (source_ml_source2_nullish_field_receiver39424.clone()).0 {
                                    self::MlOptional::Value(ml_present) => ml_present,
                                    _ => unreachable!("a proven present value"),
                                };
                            match source_ml_source2_nullish_field_present39425.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    source_ml_source2_data_property_value39426,
                                    _,
                                ) => source_ml_source2_data_property_value39426.clone(),
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units:
                                (if ((match (source_ml_source2_nullish_field_receiver39424.clone())
                                    .0
                                {
                                    self::MlOptional::Value(ml_present) => {
                                        let _ = ml_present;
                                        self::ml_string::Text::from_units(vec![
                                            111, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::MlOptional::Null => {
                                        self::ml_string::Text::from_units(vec![
                                            111, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::MlOptional::Undefined => {
                                        self::ml_string::Text::from_units(vec![
                                            117, 110, 100, 101, 102, 105, 110, 101, 100,
                                        ])
                                    }
                                }) == (self::ml_string::Text::from_units(vec![
                                    111, 98, 106, 101, 99, 116,
                                ]))) {
                                    self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105,
                                        110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121,
                                        39, 41,
                                    ])
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101,
                                        110, 116, 105, 116, 121, 39, 41,
                                    ])
                                })
                                .0,
                            message_present: true,
                        })
                        .raise()
                    }
                })) < ({
                    let source_ml_source2_nullish_field_receiver39427 = (entry.clone(),);
                    if match (source_ml_source2_nullish_field_receiver39427.clone()).0 {
                        self::MlOptional::Value(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        self::MlOptional::Null => false,
                        self::MlOptional::Undefined => false,
                    } {
                        {
                            let source_ml_source2_nullish_field_present39428 =
                                match (source_ml_source2_nullish_field_receiver39427.clone()).0 {
                                    self::MlOptional::Value(ml_present) => ml_present,
                                    _ => unreachable!("a proven present value"),
                                };
                            match source_ml_source2_nullish_field_present39428.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    source_ml_source2_data_property_value39429,
                                    _,
                                ) => source_ml_source2_data_property_value39429.clone(),
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units:
                                (if ((match (source_ml_source2_nullish_field_receiver39427.clone())
                                    .0
                                {
                                    self::MlOptional::Value(ml_present) => {
                                        let _ = ml_present;
                                        self::ml_string::Text::from_units(vec![
                                            111, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::MlOptional::Null => {
                                        self::ml_string::Text::from_units(vec![
                                            111, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::MlOptional::Undefined => {
                                        self::ml_string::Text::from_units(vec![
                                            117, 110, 100, 101, 102, 105, 110, 101, 100,
                                        ])
                                    }
                                }) == (self::ml_string::Text::from_units(vec![
                                    111, 98, 106, 101, 99, 116,
                                ]))) {
                                    self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105,
                                        110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121,
                                        39, 41,
                                    ])
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101,
                                        110, 116, 105, 116, 121, 39, 41,
                                    ])
                                })
                                .0,
                            message_present: true,
                        })
                        .raise()
                    }
                })) {
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
        } else {
            low
        };
    }
}

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
        return if (source_ml_source1_temporary_index83
            < ({
                let ml_reference_array2602 = source_ml_source1_temporary_values83.clone();
                let ml_reference_projection2602 =
                    self::ml::Big::from_u128(ml_reference_array2602.borrow().len() as u128);
                ml_reference_projection2602
            })) {
            {
                let entry = {
                    let ml_reference_array2603 = source_ml_source1_temporary_values83.clone();
                    let ml_reference_position2603 = source_ml_source1_temporary_index83.clone();
                    let ml_reference_projection2603 = self::ml_array::at(
                        &*ml_reference_array2603.borrow(),
                        ml_reference_position2603.to_index(),
                    );
                    ml_reference_projection2603
                };
                match entry.clone() {
                    self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                        shared_constructor_field1,
                        shared_constructor_field2,
                    ) => {
                        let shared_constructor_field2 = *shared_constructor_field2;
                        if ((((({
                            let source_ml_source2_optional_chain_receiver39433 = (entry.clone(),);
                            {
                                let source_ml_source1_temporary_optional_chain84 =
                                    (source_ml_source2_optional_chain_receiver39433.clone()).0;
                                match source_ml_source1_temporary_optional_chain84.clone() {
                                    self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116,
                                        114, 121, 69, 110, 116, 114, 121,
                                    ]),
                                }
                            }
                        }) != (self::ml_string::Text::from_units(vec![
                            115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114, 121, 69,
                            110, 116, 114, 121,
                        ]))) || !(self::test_java_script_ordinary_symbol(
                            match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    _,
                                    source_ml_source2_data_property_value39434,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value39434.clone(),
                            },
                            shared_constructor_field1.clone(),
                        ))) || ((({
                            let source_ml_source2_optional_chain_receiver39439 =
                                (shared_constructor_field2.clone(),);
                            {
                                let source_ml_source1_temporary_optional_chain85 =
                                    (source_ml_source2_optional_chain_receiver39439.clone()).0;
                                match source_ml_source1_temporary_optional_chain85.clone() {
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
                            116, 101, 120, 116,
                        ]))) && (({
                            let source_ml_source2_optional_chain_receiver39443 =
                                (shared_constructor_field2.clone(),);
                            {
                                let source_ml_source1_temporary_optional_chain86 =
                                    (source_ml_source2_optional_chain_receiver39443.clone()).0;
                                match source_ml_source1_temporary_optional_chain86.clone() {
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
                                shared_constructor_field2.clone(),
                            )) != (self::ml_string::Text::from_units(vec![]))))
                        {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 97, 110, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 85, 84, 70, 49, 54, 32, 107, 101, 121, 115]))
                        } else {
                            {
                                let units_2 = (units
                                    + self::read_java_script_semantic_text_length(
                                        shared_constructor_field2.clone(),
                                    ));
                                if (((self::read_java_script_semantic_text_length(
                                    shared_constructor_field2.clone(),
                                )) > (1048576f64))
                                    || (units_2 > (16777216f64)))
                                {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101]))
                                } else {
                                    if (match previous_key.clone() {
                                        self::JavaScriptSemanticValue::Undefined => false,
                                        _ => true,
                                    } && ((self::compare_java_script_collection_keys(
                                        previous_key.clone(),
                                        shared_constructor_field2.clone(),
                                    )) >= (0f64)))
                                    {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 115, 116, 114, 105, 99, 116, 32, 85, 84, 70, 49, 54, 32, 111, 114, 100, 101, 114]))
                                    } else {
                                        {
                                            let previous_key_2 = shared_constructor_field2.clone();
                                            {
                                                let low = 0f64;
                                                {
                                                    let high = {
                                                        let ml_reference_array2604 = facts.clone();
                                                        let ml_reference_projection2604 =
                                                            ml_reference_array2604.borrow().len()
                                                                as f64;
                                                        ml_reference_projection2604
                                                    };
                                                    {
                                                        let low_2 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop113(facts.clone(), self::MlOptional::Value(entry.clone()), low, high);
                                                        {
                                                            let fact = {
                                                                let ml_reference_array2605 =
                                                                    facts.clone();
                                                                let ml_reference_position2605 =
                                                                    low_2;
                                                                let ml_reference_projection2605 = self::ml_array::optional_at(&*ml_reference_array2605.borrow(), self::ml_array::number_index(ml_reference_position2605));
                                                                ml_reference_projection2605
                                                            };
                                                            if ((match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match fact.clone().expect("a proven present value") {
                                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source2_data_property_value39456, _, _) => {
                                                                    source_ml_source2_data_property_value39456.clone()
                                                                }
                                                            }) != shared_constructor_field1)) || !(match fact.clone().expect("a proven present value") {
                                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source2_data_property_value39458, _) => {
                                                                    source_ml_source2_data_property_value39458
                                                                }
                                                            })) {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115]))
                                                            } else {
                                                                {
                                                                    let key = match fact.clone().expect("a proven present value") {
                                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source2_data_property_value39459) => {
                                                                            ((source_ml_source2_data_property_value39459.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                        }
                                                                    };
                                                                    if (match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(key.clone().expect("a proven present value"), shared_constructor_field2.clone()))) {
                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115]))
                                                                    } else {
                                                                        {
                                                                            let source_ml_source1_temporary_read_java_script_ordinary_heap_error_ignored114 = { let ml_method_owner2606 = identities.clone(); let ml_method_argument2606_0 = self::JavaScriptSemanticValue::Symbol(shared_constructor_field1.clone()); let ml_inserted = vec![ml_method_argument2606_0]; let mut ml_method_values2606 = ml_method_owner2606.borrow_mut(); let ml_length = ml_method_values2606.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values2606.extend(ml_inserted); ml_length as f64 };
                                                                            {
                                                                                let source_ml_source1_temporary_index83_2 = source_ml_source1_temporary_index83.add(&self::ml::Big::from_i128(1));
                                                                                {
                                                                                    (previous_key, units, source_ml_source1_temporary_index83) = (previous_key_2.clone(), units_2, source_ml_source1_temporary_index83_2.clone());
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
        return if (source_ml_source1_temporary_index82
            < ({
                let ml_reference_array2607 = source_ml_source1_temporary_values82.clone();
                let ml_reference_projection2607 =
                    self::ml::Big::from_u128(ml_reference_array2607.borrow().len() as u128);
                ml_reference_projection2607
            })) {
            {
                let key = {
                    let ml_reference_array2608 = source_ml_source1_temporary_values82.clone();
                    let ml_reference_position2608 = source_ml_source1_temporary_index82.clone();
                    let ml_reference_projection2608 = self::ml_array::at(
                        &*ml_reference_array2608.borrow(),
                        ml_reference_position2608.to_index(),
                    );
                    ml_reference_projection2608
                };
                {
                    let witnessed_units_2 = (witnessed_units
                        + self::read_java_script_semantic_text_length(key.clone()));
                    if (witnessed_units_2 > (16777216f64)) {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101]))
                    } else {
                        if (!(match (previous_witness.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }) && self::test_java_script_semantic_same_value(
                            previous_witness.clone().expect("a proven present value"),
                            key.clone(),
                        )) {
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
                                        source_ml_source1_temporary_index82_2.clone(),
                                    );
                                    continue;
                                }
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
        return if (source_ml_source1_temporary_index81
            < ({
                let ml_reference_array2609 = source_ml_source1_temporary_values81.clone();
                let ml_reference_projection2609 =
                    self::ml::Big::from_u128(ml_reference_array2609.borrow().len() as u128);
                ml_reference_projection2609
            })) {
            {
                let fact = {
                    let ml_reference_array2610 = source_ml_source1_temporary_values81.clone();
                    let ml_reference_position2610 = source_ml_source1_temporary_index81.clone();
                    let ml_reference_projection2610 = self::ml_array::at(
                        &*ml_reference_array2610.borrow(),
                        ml_reference_position2610.to_index(),
                    );
                    ml_reference_projection2610
                };
                {
                    let key = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            _,
                            source_ml_source2_data_property_value39466,
                        ) => ((source_ml_source2_data_property_value39466
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
                                let ml_method_owner2611 = registered_keys.clone();
                                let ml_method_argument2611_0 =
                                    key.clone().expect("a proven present value");
                                let ml_inserted = vec![ml_method_argument2611_0];
                                let mut ml_method_values2611 = ml_method_owner2611.borrow_mut();
                                let ml_length = ml_method_values2611
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4294967295usize)
                                    .expect("Invalid array length");
                                ml_method_values2611.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let source_ml_source1_temporary_index81_2 =
                                    source_ml_source1_temporary_index81
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index81 =
                                        source_ml_source1_temporary_index81_2.clone();
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source1_temporary_index81_3 =
                                source_ml_source1_temporary_index81
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index81 =
                                    source_ml_source1_temporary_index81_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
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
        return if (source_ml_source1_temporary_index79
            < ({
                let ml_reference_array2612 = source_ml_source1_temporary_values79.clone();
                let ml_reference_projection2612 =
                    self::ml::Big::from_u128(ml_reference_array2612.borrow().len() as u128);
                ml_reference_projection2612
            })) {
            {
                let fact = {
                    let ml_reference_array2613 = source_ml_source1_temporary_values79.clone();
                    let ml_reference_position2613 = source_ml_source1_temporary_index79.clone();
                    let ml_reference_projection2613 = self::ml_array::at(
                        &*ml_reference_array2613.borrow(),
                        ml_reference_position2613.to_index(),
                    );
                    ml_reference_projection2613
                };
                match fact.clone() {
                    self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                        shared_constructor_field1,
                        shared_constructor_field2,
                        _,
                    ) => {
                        if (((((({
                            let source_ml_source2_optional_chain_receiver39470 = (fact.clone(),);
                            {
                                let source_ml_source1_temporary_optional_chain80 =
                                    (source_ml_source2_optional_chain_receiver39470.clone()).0;
                                match source_ml_source1_temporary_optional_chain80.clone() {
                                    self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, _) => {
                                        self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 70, 97, 99, 116])
                                    }
                                }
                            }
                        }) != (self::ml_string::Text::from_units(vec![
                            115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114, 97, 116,
                            105, 111, 110, 70, 97, 99, 116,
                        ]))) || (({
                            let source_ml_source2_type_query_operand39474 =
                                shared_constructor_field2;
                            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                        }) != (self::ml_string::Text::from_units(vec![
                            98, 111, 111, 108, 101, 97, 110,
                        ]))))
                            || (({
                                let source_ml_source2_type_query_operand39478 =
                                    shared_constructor_field1.clone();
                                self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                            }) != (self::ml_string::Text::from_units(vec![
                                98, 105, 103, 105, 110, 116,
                            ]))))
                            || (shared_constructor_field1 <= prior))
                            || !(self::test_java_script_ordinary_symbol(
                                match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        _,
                                        source_ml_source2_data_property_value39481,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39481.clone(),
                                },
                                shared_constructor_field1.clone(),
                            )))
                        {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 102, 97, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 105, 113, 117, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                        } else {
                            {
                                let prior_2 = shared_constructor_field1.clone();
                                {
                                    let key = match fact.clone() {
                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source2_data_property_value39484) => {
                                            ((source_ml_source2_data_property_value39484.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                        }
                                    };
                                    if (!(match (key.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) && (((!shared_constructor_field2
                                        || (match key.clone().expect("a proven present value") {
                                            self::JavaScriptSemanticValue::Text(_) => false,
                                            _ => true,
                                        } && match key.clone().expect("a proven present value")
                                        {
                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                false
                                            }
                                            _ => true,
                                        }))
                                        || ((self::read_java_script_ordinary_value_error(
                                            heap.clone(),
                                            key.clone().expect("a proven present value"),
                                        )) != (self::ml_string::Text::from_units(vec![]))))
                                        || ((self::read_java_script_semantic_text_length(
                                            key.clone().expect("a proven present value"),
                                        )) > (1048576f64))))
                                    {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 85, 84, 70, 49, 54, 32, 116, 101, 120, 116]))
                                    } else {
                                        {
                                            let source_ml_source1_temporary_index79_2 =
                                                source_ml_source1_temporary_index79
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (prior, source_ml_source1_temporary_index79) = (
                                                    prior_2.clone(),
                                                    source_ml_source1_temporary_index79_2.clone(),
                                                );
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
        return if (source_ml_source1_temporary_index78
            < ({
                let ml_reference_array2614 = source_ml_source1_temporary_values78.clone();
                let ml_reference_projection2614 =
                    self::ml::Big::from_u128(ml_reference_array2614.borrow().len() as u128);
                ml_reference_projection2614
            })) {
            {
                let symbol = {
                    let ml_reference_array2615 = source_ml_source1_temporary_values78.clone();
                    let ml_reference_position2615 = source_ml_source1_temporary_index78.clone();
                    let ml_reference_projection2615 = self::ml_array::at(
                        &*ml_reference_array2615.borrow(),
                        ml_reference_position2615.to_index(),
                    );
                    ml_reference_projection2615
                };
                if (((symbol <= previous)
                    || (symbol > (self::ml::Big::from_i128(18446744073709551615))))
                    || ((self::find_java_script_ordinary_object(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source2_data_property_value39492,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39492.clone(),
                        },
                        symbol.clone(),
                    )) >= (0f64)))
                {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index78_2 =
                            source_ml_source1_temporary_index78.add(&self::ml::Big::from_i128(1));
                        {
                            (previous, source_ml_source1_temporary_index78) = (
                                symbol.clone(),
                                source_ml_source1_temporary_index78_2.clone(),
                            );
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
