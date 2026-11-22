// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Result;
use super::MlSource1TemporaryOrderingWorkJoin35;
use super::RustStandardCollectionCallOrigin;
use super::RustStandardCollectionOptionValue;
use super::RustStandardCollectionOrderingOrigin;
use super::RustStandardCollectionOrigin;
use super::RustStandardCollectionScalar;
use super::RustStandardCollectionState;
use super::RustStandardCollectionTransition;
use super::RustStandardCollectionValue;
use super::copy_rust_standard_collection_result_scalar;
use super::create_rust_standard_collection_option;
use super::find_rust_standard_collection_entry;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_rust_standard_collection_call_origin_error;
use super::read_rust_standard_collection_kind;
use super::read_rust_standard_collection_scalar_error;
use super::read_rust_standard_collection_source_version_error;
use super::read_rust_standard_collection_state_error;
use super::read_rust_standard_collection_stored_scalar;
use super::resolve_rust_standard_collection_owner;
use super::source_ml_source1_temporary_execute_rust_standard_collection_ordered_snapshot_loop34;
use super::test_rust_standard_collection_ordered;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_reference(
    state: self::RustStandardCollectionState,
    reference: self::RustStandardCollectionOptionValue,
    operation: &self::ml_string::Text,
) -> self::RustStandardCollectionTransition {
    {
        let error = self::read_rust_standard_collection_state_error(state.clone());
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::RustStandardCollectionTransition::RustCollectionInvalid(error.clone())
            } else {
                {
                    let kind = self::read_rust_standard_collection_kind(&(self::resolve_rust_standard_collection_owner(&(match match state.clone() {
                    self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value231, _, _, _, _, _, _) => {
                        let source_ml_source2_data_property_value231 = *source_ml_source2_data_property_value231;
                        source_ml_source2_data_property_value231.clone()
                    }
                } {
                    self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value232, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value232.clone()
                    }
                }), true, false)));
                    {
                        let domain = if kind
                            == (self::ml_string::Text::from_units(vec![109, 97, 112]))
                        {
                            match state.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value233,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value233.clone(),
                            }
                        } else {
                            match state.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value234,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value234.clone(),
                            }
                        };
                        {
                            let invalid = self::ml_string::Text::from_units(vec![
                                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 102,
                                101, 114, 101, 110, 99, 101, 32, 112, 114, 111, 106, 101, 99, 116,
                                105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 114, 111, 111, 116, 32, 114, 101, 118, 105,
                                115, 105, 111, 110, 32, 100, 111, 109, 97, 105, 110, 32, 97, 110,
                                100, 32, 67, 111, 112, 121, 32, 111, 114, 32, 67, 108, 111, 110,
                                101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110,
                            ]);
                            if match (reference.clone(), None::<()>) {
                                (_ml_left, Some(_ml_right)) => false,
                                _ => false,
                            } || match (reference.clone(), None::<()>) {
                                (_ml_left, Some(_ml_right)) => false,
                                _ => false,
                            } {
                                self::RustStandardCollectionTransition::RustCollectionInvalid(
                                    invalid.clone(),
                                )
                            } else {
                                match reference.clone() {
                                self::RustStandardCollectionOptionValue::RustCollectionReference(root, revision, key, reference_domain) => {
                                    let key = *key;
                                    if ((((root != (match state.clone() {
                                        self::RustStandardCollectionState::RustStandardCollectionState(_, source_ml_source2_data_property_value239, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value239.clone()
                                        }
                                    })) || (revision != (match state.clone() {
                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, source_ml_source2_data_property_value241, _) => {
                                            source_ml_source2_data_property_value241
                                        }
                                    }))) || (reference_domain != domain)) || !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![99, 111, 112, 105, 101, 100]), self::ml_string::Text::from_units(vec![99, 108, 111, 110, 101, 100])], operation, 0f64, false, |a, b| a == b))) || (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 112, 105, 101, 100]))) && (domain == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])))) {
                                        self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                    } else {
                                        {
                                            let key_error = self::read_rust_standard_collection_scalar_error(Some(key.clone()), &(match state.clone() {
                                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value242, _, _, _, _) => {
                                                    source_ml_source2_data_property_value242.clone()
                                                }
                                            }), match state.clone() {
                                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, source_ml_source2_data_property_value243, _, _) => {
                                                    source_ml_source2_data_property_value243
                                                }
                                            });
                                            {
let ml_library_inequality_1 = key_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                self::RustStandardCollectionTransition::RustCollectionInvalid(key_error.clone())
                                            } else {
                                                {
                                                    let position = self::find_rust_standard_collection_entry(match state.clone() {
                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value244) => {
                                                            source_ml_source2_data_property_value244
                                                        }
                                                    }, key.clone());
                                                    if position < 0f64 {
                                                        self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 105, 118, 101, 32, 101, 110, 116, 114, 121]))
                                                    } else {
                                                        {
                                                            let value = self::read_rust_standard_collection_stored_scalar(self::ml_array::optional_at((match &state { self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value245) => source_ml_source2_data_property_value245.as_slice() }), self::ml_array::number_index(position)), &(kind));
                                                            if match (value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 111, 114, 101, 100, 32, 118, 97, 108, 117, 101]))
                                                            } else {
                                                                self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state.clone()), Box::new(self::copy_rust_standard_collection_result_scalar(value.clone().expect("a proven present value"))), false)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
}
                                        }
                                    }
                                }
                                self::RustStandardCollectionOptionValue::RustCollectionBoolean(source_ml_source2_f235) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionOptionValue::RustCollectionInteger(source_ml_source2_f236) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionOptionValue::RustCollectionString(source_ml_source2_f237) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn project_rust_standard_collection_option(
    state: self::RustStandardCollectionState,
    option: self::RustStandardCollectionValue,
    operation: &self::ml_string::Text,
) -> self::RustStandardCollectionTransition {
    {
        let error = self::read_rust_standard_collection_state_error(state.clone());
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::RustStandardCollectionTransition::RustCollectionInvalid(error.clone())
            } else {
                {
                    let kind = self::read_rust_standard_collection_kind(&(self::resolve_rust_standard_collection_owner(&(match match state.clone() {
                    self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value246, _, _, _, _, _, _) => {
                        let source_ml_source2_data_property_value246 = *source_ml_source2_data_property_value246;
                        source_ml_source2_data_property_value246.clone()
                    }
                } {
                    self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value247, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value247.clone()
                    }
                }), true, false)));
                    {
                        let domain = if kind
                            == (self::ml_string::Text::from_units(vec![109, 97, 112]))
                        {
                            match state.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value248,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value248.clone(),
                            }
                        } else {
                            match state.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value249,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value249.clone(),
                            }
                        };
                        {
                            let invalid = self::ml_string::Text::from_units(vec![
                                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 79, 112, 116,
                                105, 111, 110, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110,
                                32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32,
                                97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114,
                                100, 32, 98, 111, 114, 114, 111, 119, 101, 100, 32, 79, 112, 116,
                                105, 111, 110, 32, 67, 111, 112, 121, 32, 111, 114, 32, 67, 108,
                                111, 110, 101, 32, 99, 111, 110, 116, 114, 97, 99, 116,
                            ]);
                            if match (option.clone(), None::<()>) {
                                (_ml_left, Some(_ml_right)) => false,
                                _ => false,
                            } || match (option.clone(), None::<()>) {
                                (_ml_left, Some(_ml_right)) => false,
                                _ => false,
                            } {
                                self::RustStandardCollectionTransition::RustCollectionInvalid(
                                    invalid.clone(),
                                )
                            } else {
                                match option.clone() {
                                self::RustStandardCollectionValue::RustCollectionOrderedValues(source_ml_source2_f250) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionValue::RustCollectionOrderedEntries(source_ml_source2_f251) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionValue::RustCollectionUnit => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionValue::RustCollectionBoolean(source_ml_source2_f252) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionValue::RustCollectionInteger(source_ml_source2_f253) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionValue::RustCollectionString(source_ml_source2_f254) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionValue::RustCollectionOption(present, option_value) => {
                                    if (((self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) || !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![99, 111, 112, 105, 101, 100]), self::ml_string::Text::from_units(vec![99, 108, 111, 110, 101, 100])], operation, 0f64, false, |a, b| a == b))) || (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 112, 105, 101, 100]))) && (domain == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])))) {
                                        self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                    } else if present {
                                            {
                                                let payload = ((option_value.clone().unwrap_or_else(|| (None::<self::RustStandardCollectionOptionValue>,))).0).clone();
                                                {
                                                    let invalid_payload = self::ml_string::Text::from_units(vec![99, 111, 112, 105, 101, 100, 32, 97, 110, 100, 32, 99, 108, 111, 110, 101, 100, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 114, 114, 111, 119, 101, 100, 32, 79, 112, 116, 105, 111, 110, 32, 112, 97, 121, 108, 111, 97, 100, 115]);
                                                    if match (payload.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match (payload.clone().expect("a proven present value"), None::<()>) { (_ml_left, Some(_ml_right)) => false, _ => false } {
                                                        self::RustStandardCollectionTransition::RustCollectionInvalid(invalid_payload.clone())
                                                    } else {
                                                        match payload.clone().expect("a proven present value") {
                                                            self::RustStandardCollectionOptionValue::RustCollectionReference(source_ml_source2_f262, source_ml_source2_f263, source_ml_source2_f264, source_ml_source2_f265) => {
                                                                let source_ml_source2_f264 = *source_ml_source2_f264;
                                                                {
                                                                    let read = self::read_rust_standard_collection_reference(state.clone(), payload.clone().expect("a proven present value"), operation);
                                                                    match read.clone() {
                                                                        self::RustStandardCollectionTransition::RustCollectionCreated(source_ml_source2_f269) => {
                                                                            let source_ml_source2_f269 = *source_ml_source2_f269;
                                                                            read.clone()
                                                                        }
                                                                        self::RustStandardCollectionTransition::RustCollectionNormal(state_2, value, writeback) => {
                                                                            let state_2 = *state_2;
                                                                            let value = *value;
                                                                            match value.clone() {
                                                                                self::RustStandardCollectionValue::RustCollectionOrderedValues(source_ml_source2_f273) => {
                                                                                    read.clone()
                                                                                }
                                                                                self::RustStandardCollectionValue::RustCollectionOrderedEntries(source_ml_source2_f274) => {
                                                                                    read.clone()
                                                                                }
                                                                                self::RustStandardCollectionValue::RustCollectionUnit => {
                                                                                    read.clone()
                                                                                }
                                                                                self::RustStandardCollectionValue::RustCollectionBoolean(value_value) => {
                                                                                    self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state_2.clone()), Box::new(self::create_rust_standard_collection_option(Some(self::RustStandardCollectionOptionValue::RustCollectionBoolean(value_value)))), writeback)
                                                                                }
                                                                                self::RustStandardCollectionValue::RustCollectionInteger(value_value_2) => {
                                                                                    self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state_2.clone()), Box::new(self::create_rust_standard_collection_option(Some(self::RustStandardCollectionOptionValue::RustCollectionInteger(value_value_2.clone())))), writeback)
                                                                                }
                                                                                self::RustStandardCollectionValue::RustCollectionString(units) => {
                                                                                    self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state_2.clone()), Box::new(self::create_rust_standard_collection_option(Some(self::RustStandardCollectionOptionValue::RustCollectionString(units)))), writeback)
                                                                                }
                                                                                self::RustStandardCollectionValue::RustCollectionOption(source_ml_source2_f275, source_ml_source2_f276) => {
                                                                                    read.clone()
                                                                                }
                                                                                self::RustStandardCollectionValue::RustStandardCollectionState(source_ml_source2_f277, source_ml_source2_f278, source_ml_source2_f279, source_ml_source2_f280, source_ml_source2_f281, source_ml_source2_f282, source_ml_source2_f283) => {
                                                                                    let source_ml_source2_f277 = *source_ml_source2_f277;
                                                                                    read.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                        self::RustStandardCollectionTransition::RustCollectionInvalid(source_ml_source2_f270) => {
                                                                            read.clone()
                                                                        }
                                                                        self::RustStandardCollectionTransition::RustCollectionResourceLimit(source_ml_source2_f271, source_ml_source2_f272) => {
                                                                            read.clone()
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::RustStandardCollectionOptionValue::RustCollectionBoolean(source_ml_source2_f266) => {
                                                                self::RustStandardCollectionTransition::RustCollectionInvalid(invalid_payload.clone())
                                                            }
                                                            self::RustStandardCollectionOptionValue::RustCollectionInteger(source_ml_source2_f267) => {
                                                                self::RustStandardCollectionTransition::RustCollectionInvalid(invalid_payload.clone())
                                                            }
                                                            self::RustStandardCollectionOptionValue::RustCollectionString(source_ml_source2_f268) => {
                                                                self::RustStandardCollectionTransition::RustCollectionInvalid(invalid_payload.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else if match (((option_value.clone().unwrap_or_else(|| (None::<self::RustStandardCollectionOptionValue>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state.clone()), Box::new(self::create_rust_standard_collection_option(None::<self::RustStandardCollectionOptionValue>)), false)
                                            } else {
                                                self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![78, 111, 110, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 110, 111, 32, 105, 110, 118, 101, 110, 116, 101, 100, 32, 112, 97, 121, 108, 111, 97, 100]))
                                            }
                                }
                                self::RustStandardCollectionValue::RustStandardCollectionState(source_ml_source2_f255, source_ml_source2_f256, source_ml_source2_f257, source_ml_source2_f258, source_ml_source2_f259, source_ml_source2_f260, source_ml_source2_f261) => {
                                    let source_ml_source2_f255 = *source_ml_source2_f255;
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_rust_standard_collection_ordered_borrow(
    state: self::RustStandardCollectionState,
    method: &self::ml_string::Text,
    call: self::RustStandardCollectionCallOrigin,
) -> self::RustStandardCollectionTransition {
    {
        let source = self::read_rust_standard_collection_state_error(state.clone());
        {
            let ml_library_inequality_2 = source != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::RustStandardCollectionTransition::RustCollectionInvalid(source.clone())
            } else {
                {
                    let actual = self::read_rust_standard_collection_call_origin_error(
                        match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(
                                source_ml_source2_data_property_value284,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value284 =
                                    *source_ml_source2_data_property_value284;
                                source_ml_source2_data_property_value284.clone()
                            }
                        },
                        &(match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(
                                _,
                                source_ml_source2_data_property_value285,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value285.clone(),
                        }),
                        method,
                        0f64,
                        call.clone(),
                    );
                    {
                        let ml_library_inequality_1 =
                            actual != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            self::RustStandardCollectionTransition::RustCollectionInvalid(
                                actual.clone(),
                            )
                        } else {
                            {
                                let kind = self::read_rust_standard_collection_kind(&(match match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value286, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value286 = *source_ml_source2_data_property_value286;
                                source_ml_source2_data_property_value286.clone()
                            }
                        } {
                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value287, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value287.clone()
                            }
                        }));
                                if !(self::test_rust_standard_collection_ordered(&(match match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value288, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value288 = *source_ml_source2_data_property_value288;
                                source_ml_source2_data_property_value288.clone()
                            }
                        } {
                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value289, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value289.clone()
                            }
                        }))) || !(((kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) && ((*method) == (self::ml_string::Text::from_units(vec![105, 116, 101, 114])))) || ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![105, 116, 101, 114]), self::ml_string::Text::from_units(vec![107, 101, 121, 115]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])], method, 0f64, false, |a, b| a == b))) {
                            self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![97, 32, 115, 104, 97, 114, 101, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 66, 84, 114, 101, 101, 32, 111, 119, 110, 101, 114, 32, 97, 110, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110]))
                        } else {
                            self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state.clone()), Box::new(self::RustStandardCollectionValue::RustCollectionOrderedEntries(match state.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value290) => {
                                    source_ml_source2_data_property_value290
                                }
                            })), false)
                        }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn copy_rust_standard_collection_snapshot_scalar(
    value: self::RustStandardCollectionScalar,
) -> self::RustStandardCollectionScalar {
    match value.clone() {
        self::RustStandardCollectionScalar::RustCollectionBoolean(value_value) => {
            self::RustStandardCollectionScalar::RustCollectionBoolean(value_value)
        }
        self::RustStandardCollectionScalar::RustCollectionInteger(value_value_2) => {
            self::RustStandardCollectionScalar::RustCollectionInteger(value_value_2.clone())
        }
        self::RustStandardCollectionScalar::RustCollectionString(units) => {
            self::RustStandardCollectionScalar::RustCollectionString(units)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_snapshot_scalar(
    entry: (
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    ),
    method: &self::ml_string::Text,
) -> Option<self::RustStandardCollectionScalar> {
    if (*method) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])) {
        ((((entry.clone()).1)
            .clone()
            .unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,)))
        .0)
            .clone()
    } else {
        Some((entry.clone()).0)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_rust_standard_collection_ordered_snapshot(
    state: self::RustStandardCollectionState,
    method: &self::ml_string::Text,
    call: self::RustStandardCollectionCallOrigin,
    projection: &self::ml_string::Text,
) -> self::RustStandardCollectionTransition {
    {
        let source = self::read_rust_standard_collection_state_error(state.clone());
        {
            let ml_library_inequality_2 = source != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::RustStandardCollectionTransition::RustCollectionInvalid(source.clone())
            } else {
                {
                    let actual = self::read_rust_standard_collection_call_origin_error(
                        match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(
                                source_ml_source2_data_property_value291,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value291 =
                                    *source_ml_source2_data_property_value291;
                                source_ml_source2_data_property_value291.clone()
                            }
                        },
                        &(match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(
                                _,
                                source_ml_source2_data_property_value292,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value292.clone(),
                        }),
                        method,
                        0f64,
                        call.clone(),
                    );
                    {
                        let ml_library_inequality_1 =
                            actual != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            self::RustStandardCollectionTransition::RustCollectionInvalid(
                                actual.clone(),
                            )
                        } else {
                            {
                                let owner = match match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value293, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value293 = *source_ml_source2_data_property_value293;
                                source_ml_source2_data_property_value293.clone()
                            }
                        } {
                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value294, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value294.clone()
                            }
                        };
                                {
                                    let kind = self::read_rust_standard_collection_kind(&(owner));
                                    if !(self::test_rust_standard_collection_ordered(&(owner)))
                                        || !(((kind
                                            == (self::ml_string::Text::from_units(vec![
                                                115, 101, 116,
                                            ])))
                                            && ((*method)
                                                == (self::ml_string::Text::from_units(vec![
                                                    105, 116, 101, 114,
                                                ]))))
                                            || ((kind
                                                == (self::ml_string::Text::from_units(vec![
                                                    109, 97, 112,
                                                ])))
                                                && self::ml_array::includes(
                                                    &vec![
                                                        self::ml_string::Text::from_units(vec![
                                                            107, 101, 121, 115,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            118, 97, 108, 117, 101, 115,
                                                        ]),
                                                    ],
                                                    method,
                                                    0f64,
                                                    false,
                                                    |a, b| a == b,
                                                )))
                                    {
                                        self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 101, 100, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 66, 84, 114, 101, 101, 83, 101, 116, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 111, 114, 32, 66, 84, 114, 101, 101, 77, 97, 112, 32, 107, 101, 121, 47, 118, 97, 108, 117, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 111, 119, 110, 101, 114]))
                                    } else {
                                        {
                                            let domain = if (*method)
                                                == (self::ml_string::Text::from_units(vec![
                                                    118, 97, 108, 117, 101, 115,
                                                ])) {
                                                match state.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, source_ml_source2_data_property_value295, _, _, _) => {
                                                source_ml_source2_data_property_value295.clone()
                                            }
                                        }
                                            } else {
                                                match state.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value296, _, _, _, _) => {
                                                source_ml_source2_data_property_value296.clone()
                                            }
                                        }
                                            };
                                            if !(self::ml_array::includes(
                                                &vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        99, 111, 112, 105, 101, 100,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        99, 108, 111, 110, 101, 100,
                                                    ]),
                                                ],
                                                projection,
                                                0f64,
                                                false,
                                                |a, b| a == b,
                                            )) || (((*projection)
                                                == (self::ml_string::Text::from_units(vec![
                                                    99, 111, 112, 105, 101, 100,
                                                ])))
                                                && (domain
                                                    == (self::ml_string::Text::from_units(vec![
                                                        83, 116, 114, 105, 110, 103,
                                                    ]))))
                                            {
                                                self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 101, 100, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 67, 111, 112, 121, 32, 111, 114, 32, 111, 119, 110, 101, 100, 32, 67, 108, 111, 110, 101, 32, 98, 101, 102, 111, 114, 101, 32, 105, 109, 109, 101, 100, 105, 97, 116, 101, 32, 86, 101, 99, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110]))
                                            } else {
                                                {
                                                    let values = Vec::<
                                                        self::RustStandardCollectionScalar,
                                                    >::new(
                                                    );
                                                    {
                                                        let source_ml_source1_temporary_values33 = match state.clone() {
                                                    self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value297) => {
                                                        source_ml_source2_data_property_value297
                                                    }
                                                };
                                                        {
                                                            let source_ml_source1_temporary_index33 =
                                                                self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s298 = self::source_ml_source1_temporary_execute_rust_standard_collection_ordered_snapshot_loop34(method, values, source_ml_source1_temporary_values33, source_ml_source1_temporary_index33);
                                                                match source_ml_source2_s298 {
                                                            self::MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Result::MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Done(values_2) => {
                                                                self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state.clone()), Box::new(self::RustStandardCollectionValue::RustCollectionOrderedValues(values_2)), false)
                                                            }
                                                            self::MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Result::MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Return(source_ml_source1_temporary_result34) => {
                                                                let source_ml_source1_temporary_result34 = *source_ml_source1_temporary_result34;
                                                                source_ml_source1_temporary_result34.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ordering_work(
    left: self::RustStandardCollectionScalar,
    right: self::RustStandardCollectionScalar,
) -> f64 {
    {
        let source_ml_source2_s303 = match left.clone() {
            self::RustStandardCollectionScalar::RustCollectionBoolean(source_ml_source2_f304) => {
                self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Next
            }
            self::RustStandardCollectionScalar::RustCollectionInteger(source_ml_source2_f305) => {
                self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Next
            }
            self::RustStandardCollectionScalar::RustCollectionString(left_units) => {
                match right.clone() {
                    self::RustStandardCollectionScalar::RustCollectionBoolean(source_ml_source2_f306) => {
                        self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Next
                    }
                    self::RustStandardCollectionScalar::RustCollectionInteger(source_ml_source2_f307) => {
                        self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Next
                    }
                    self::RustStandardCollectionScalar::RustCollectionString(units) => {
                        self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Return(self::ml_math::max(1f64, ((left_units.len() as f64) + (units.len() as f64))))
                    }
                }
            }
        };
        match source_ml_source2_s303 {
            self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Next => {
                1f64
            }
            self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Return(source_ml_source1_temporary_result35) => {
                source_ml_source1_temporary_result35
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_ordering_origin_error(
    left: self::RustStandardCollectionState,
    right: self::RustStandardCollectionState,
    origin: self::RustStandardCollectionOrderingOrigin,
) -> self::ml_string::Text {
    {
        let first = self::read_rust_standard_collection_state_error(left.clone());
        {
            let ml_library_inequality_2 = first != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                first.clone()
            } else {
                {
                    let second = self::read_rust_standard_collection_state_error(right.clone());
                    {
                        let ml_library_inequality_1 =
                            second != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                    second.clone()
                } else if ((((!(self::test_rust_standard_collection_ordered(&(match match left.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value308, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value308 = *source_ml_source2_data_property_value308;
                            source_ml_source2_data_property_value308.clone()
                        }
                    } {
                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value309, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value309.clone()
                        }
                    }))) || ((self::resolve_rust_standard_collection_owner(&(match match left.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value314, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value314 = *source_ml_source2_data_property_value314;
                            source_ml_source2_data_property_value314.clone()
                        }
                    } {
                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value315, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value315.clone()
                        }
                    }), true, false)) != (self::resolve_rust_standard_collection_owner(&(match match right.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value316, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value316 = *source_ml_source2_data_property_value316;
                            source_ml_source2_data_property_value316.clone()
                        }
                    } {
                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value317, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value317.clone()
                        }
                    }), true, false)))) || ((match match left.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value322, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value322 = *source_ml_source2_data_property_value322;
                            source_ml_source2_data_property_value322.clone()
                        }
                    } {
                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, source_ml_source2_data_property_value323, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value323.clone()
                        }
                    }) != (match match right.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value324, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value324 = *source_ml_source2_data_property_value324;
                            source_ml_source2_data_property_value324.clone()
                        }
                    } {
                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, source_ml_source2_data_property_value325, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value325.clone()
                        }
                    }))) || ((match left.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value328, _, _, _, _) => {
                            source_ml_source2_data_property_value328.clone()
                        }
                    }) != (match right.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value329, _, _, _, _) => {
                            source_ml_source2_data_property_value329.clone()
                        }
                    }))) || ((match left.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, source_ml_source2_data_property_value332, _, _, _) => {
                            source_ml_source2_data_property_value332.clone()
                        }
                    }) != (match right.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, source_ml_source2_data_property_value333, _, _, _) => {
                            source_ml_source2_data_property_value333.clone()
                        }
                    }))) || ((match left.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, source_ml_source2_data_property_value336, _, _) => {
                            source_ml_source2_data_property_value336
                        }
                    }) != (match right.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, source_ml_source2_data_property_value337, _, _) => {
                            source_ml_source2_data_property_value337
                        }
                    })) {
                        self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 66, 84, 114, 101, 101, 32, 111, 119, 110, 101, 114, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 99, 108, 111, 115, 101, 100, 32, 107, 101, 121, 47, 118, 97, 108, 117, 101, 32, 100, 111, 109, 97, 105, 110, 115])
                    } else if ((((((({
                            let source_ml_source2_optional_chain_receiver339 = (origin.clone(),);
                            {
                                let source_ml_source1_temporary_optional_chain36 = (source_ml_source2_optional_chain_receiver339).0;
                                match source_ml_source1_temporary_optional_chain36 {
                                    self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, _, _, _, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![114, 117, 115, 116, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 79, 114, 100, 101, 114, 105, 110, 103, 79, 114, 105, 103, 105, 110])
                                    }
                                }
                            }
                        }) != (self::ml_string::Text::from_units(vec![114, 117, 115, 116, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 79, 114, 100, 101, 114, 105, 110, 103, 79, 114, 105, 103, 105, 110]))) || !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![99, 111, 114, 101, 58, 58, 99, 109, 112, 58, 58, 79, 114, 100]), self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 99, 109, 112, 58, 58, 79, 114, 100]), self::ml_string::Text::from_units(vec![99, 111, 114, 101, 58, 58, 99, 109, 112, 58, 58, 80, 97, 114, 116, 105, 97, 108, 79, 114, 100]), self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 99, 109, 112, 58, 58, 80, 97, 114, 116, 105, 97, 108, 79, 114, 100])], &match origin.clone() {
                            self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(source_ml_source2_data_property_value340, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value340.clone()
                            }
                        }, 0f64, false, |a, b| a == b))) || ((match origin.clone() {
                            self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, source_ml_source2_data_property_value344, _, _, _, _, _) => {
                                source_ml_source2_data_property_value344.clone()
                            }
                        }) != (match match left.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value345, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value345 = *source_ml_source2_data_property_value345;
                                source_ml_source2_data_property_value345.clone()
                            }
                        } {
                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, source_ml_source2_data_property_value346, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value346.clone()
                            }
                        }))) || !(match origin.clone() {
                            self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, _, _, source_ml_source2_data_property_value347, _, _, _) => {
                                source_ml_source2_data_property_value347
                            }
                        })) || !(match origin.clone() {
                            self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, _, _, _, source_ml_source2_data_property_value348, _, _) => {
                                source_ml_source2_data_property_value348
                            }
                        })) || !(match origin.clone() {
                            self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, _, _, _, _, source_ml_source2_data_property_value349, _) => {
                                source_ml_source2_data_property_value349
                            }
                        })) || ((match origin.clone() {
                            self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, _, _, _, _, _, source_ml_source2_data_property_value351) => {
                                source_ml_source2_data_property_value351
                            }
                        }) != (false)) {
                            self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 99, 108, 111, 115, 101, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 79, 114, 100, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 119, 105, 116, 104, 111, 117, 116, 32, 99, 117, 115, 116, 111, 109, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110])
                        } else {
                            self::read_rust_standard_collection_source_version_error(match origin.clone() {
                                self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, _, source_ml_source2_data_property_value352, _, _, _, _) => {
                                    source_ml_source2_data_property_value352
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}
