// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Result;
use super::MlSource1TemporaryOrderingWorkJoin35;
use super::RustStandardCollectionCallOrigin;
use super::RustStandardCollectionOptionValue;
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
                    self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value239, _, _, _, _, _, _) => {
                        let source_ml_source2_data_property_value239 = *source_ml_source2_data_property_value239;
                        source_ml_source2_data_property_value239.clone()
                    }
                } {
                    self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value240, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value240.clone()
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
                                    source_ml_source2_data_property_value241,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value241.clone(),
                            }
                        } else {
                            match state.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value242,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value242.clone(),
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
                                        self::RustStandardCollectionState::RustStandardCollectionState(_, source_ml_source2_data_property_value247, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value247.clone()
                                        }
                                    })) || (revision != (match state.clone() {
                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, source_ml_source2_data_property_value249, _) => {
                                            source_ml_source2_data_property_value249
                                        }
                                    }))) || (reference_domain != domain)) || !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![99, 111, 112, 105, 101, 100]), self::ml_string::Text::from_units(vec![99, 108, 111, 110, 101, 100])], operation, 0f64, false, |a, b| a == b))) || (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 112, 105, 101, 100]))) && (domain == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])))) {
                                        self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                    } else {
                                        {
                                            let key_error = self::read_rust_standard_collection_scalar_error(Some(key.clone()), &(match state.clone() {
                                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value250, _, _, _, _) => {
                                                    source_ml_source2_data_property_value250.clone()
                                                }
                                            }), match state.clone() {
                                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, source_ml_source2_data_property_value251, _, _) => {
                                                    source_ml_source2_data_property_value251
                                                }
                                            });
                                            {
let ml_library_inequality_1 = key_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                self::RustStandardCollectionTransition::RustCollectionInvalid(key_error.clone())
                                            } else {
                                                {
                                                    let position = self::find_rust_standard_collection_entry(match state.clone() {
                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value252) => {
                                                            source_ml_source2_data_property_value252
                                                        }
                                                    }, key.clone());
                                                    if position < 0f64 {
                                                        self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 105, 118, 101, 32, 101, 110, 116, 114, 121]))
                                                    } else {
                                                        {
                                                            let value = self::read_rust_standard_collection_stored_scalar(self::ml_array::optional_at((match &state { self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value253) => source_ml_source2_data_property_value253.as_slice() }), self::ml_array::number_index(position)), &(kind));
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
                                self::RustStandardCollectionOptionValue::RustCollectionBoolean(source_ml_source2_f243) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionOptionValue::RustCollectionInteger(source_ml_source2_f244) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionOptionValue::RustCollectionString(source_ml_source2_f245) => {
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
                    self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value254, _, _, _, _, _, _) => {
                        let source_ml_source2_data_property_value254 = *source_ml_source2_data_property_value254;
                        source_ml_source2_data_property_value254.clone()
                    }
                } {
                    self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value255, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value255.clone()
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
                                    source_ml_source2_data_property_value256,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value256.clone(),
                            }
                        } else {
                            match state.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value257,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value257.clone(),
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
                                self::RustStandardCollectionValue::RustCollectionOrderedValues(source_ml_source2_f258) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionValue::RustCollectionOrderedEntries(source_ml_source2_f259) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionValue::RustCollectionUnit => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionValue::RustCollectionBoolean(source_ml_source2_f260) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionValue::RustCollectionInteger(source_ml_source2_f261) => {
                                    self::RustStandardCollectionTransition::RustCollectionInvalid(invalid.clone())
                                }
                                self::RustStandardCollectionValue::RustCollectionString(source_ml_source2_f262) => {
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
                                                            self::RustStandardCollectionOptionValue::RustCollectionReference(source_ml_source2_f270, source_ml_source2_f271, source_ml_source2_f272, source_ml_source2_f273) => {
                                                                let source_ml_source2_f272 = *source_ml_source2_f272;
                                                                {
                                                                    let read = self::read_rust_standard_collection_reference(state.clone(), payload.clone().expect("a proven present value"), operation);
                                                                    match read.clone() {
                                                                        self::RustStandardCollectionTransition::RustCollectionCreated(source_ml_source2_f277) => {
                                                                            let source_ml_source2_f277 = *source_ml_source2_f277;
                                                                            read.clone()
                                                                        }
                                                                        self::RustStandardCollectionTransition::RustCollectionNormal(state_2, value, writeback) => {
                                                                            let state_2 = *state_2;
                                                                            let value = *value;
                                                                            match value.clone() {
                                                                                self::RustStandardCollectionValue::RustCollectionOrderedValues(source_ml_source2_f281) => {
                                                                                    read.clone()
                                                                                }
                                                                                self::RustStandardCollectionValue::RustCollectionOrderedEntries(source_ml_source2_f282) => {
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
                                                                                self::RustStandardCollectionValue::RustCollectionOption(source_ml_source2_f283, source_ml_source2_f284) => {
                                                                                    read.clone()
                                                                                }
                                                                                self::RustStandardCollectionValue::RustStandardCollectionState(source_ml_source2_f285, source_ml_source2_f286, source_ml_source2_f287, source_ml_source2_f288, source_ml_source2_f289, source_ml_source2_f290, source_ml_source2_f291) => {
                                                                                    let source_ml_source2_f285 = *source_ml_source2_f285;
                                                                                    read.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                        self::RustStandardCollectionTransition::RustCollectionInvalid(source_ml_source2_f278) => {
                                                                            read.clone()
                                                                        }
                                                                        self::RustStandardCollectionTransition::RustCollectionResourceLimit(source_ml_source2_f279, source_ml_source2_f280) => {
                                                                            read.clone()
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::RustStandardCollectionOptionValue::RustCollectionBoolean(source_ml_source2_f274) => {
                                                                self::RustStandardCollectionTransition::RustCollectionInvalid(invalid_payload.clone())
                                                            }
                                                            self::RustStandardCollectionOptionValue::RustCollectionInteger(source_ml_source2_f275) => {
                                                                self::RustStandardCollectionTransition::RustCollectionInvalid(invalid_payload.clone())
                                                            }
                                                            self::RustStandardCollectionOptionValue::RustCollectionString(source_ml_source2_f276) => {
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
                                self::RustStandardCollectionValue::RustStandardCollectionState(source_ml_source2_f263, source_ml_source2_f264, source_ml_source2_f265, source_ml_source2_f266, source_ml_source2_f267, source_ml_source2_f268, source_ml_source2_f269) => {
                                    let source_ml_source2_f263 = *source_ml_source2_f263;
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
                                source_ml_source2_data_property_value292,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value292 =
                                    *source_ml_source2_data_property_value292;
                                source_ml_source2_data_property_value292.clone()
                            }
                        },
                        &(match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(
                                _,
                                source_ml_source2_data_property_value293,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value293.clone(),
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
                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value294, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value294 = *source_ml_source2_data_property_value294;
                                source_ml_source2_data_property_value294.clone()
                            }
                        } {
                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value295, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value295.clone()
                            }
                        }));
                                if !(self::test_rust_standard_collection_ordered(&(match match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value296, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value296 = *source_ml_source2_data_property_value296;
                                source_ml_source2_data_property_value296.clone()
                            }
                        } {
                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value297, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value297.clone()
                            }
                        }))) || !(((kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) && ((*method) == (self::ml_string::Text::from_units(vec![105, 116, 101, 114])))) || ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![105, 116, 101, 114]), self::ml_string::Text::from_units(vec![107, 101, 121, 115]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])], method, 0f64, false, |a, b| a == b))) {
                            self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![97, 32, 115, 104, 97, 114, 101, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 66, 84, 114, 101, 101, 32, 111, 119, 110, 101, 114, 32, 97, 110, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110]))
                        } else {
                            self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state.clone()), Box::new(self::RustStandardCollectionValue::RustCollectionOrderedEntries(match state.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value298) => {
                                    source_ml_source2_data_property_value298
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
pub fn execute_rust_standard_collection_ordered_move(
    state: self::RustStandardCollectionState,
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
                                source_ml_source2_data_property_value299,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value299 =
                                    *source_ml_source2_data_property_value299;
                                source_ml_source2_data_property_value299.clone()
                            }
                        },
                        &(match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(
                                _,
                                source_ml_source2_data_property_value300,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value300.clone(),
                        }),
                        &(self::ml_string::Text::from_units(vec![
                            105, 110, 116, 111, 95, 105, 116, 101, 114,
                        ])),
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
                            self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state.clone()), Box::new(self::RustStandardCollectionValue::RustCollectionOrderedEntries(match state.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value301) => {
                            source_ml_source2_data_property_value301
                        }
                    })), false)
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
                                source_ml_source2_data_property_value302,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value302 =
                                    *source_ml_source2_data_property_value302;
                                source_ml_source2_data_property_value302.clone()
                            }
                        },
                        &(match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(
                                _,
                                source_ml_source2_data_property_value303,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value303.clone(),
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
                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value304, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value304 = *source_ml_source2_data_property_value304;
                                source_ml_source2_data_property_value304.clone()
                            }
                        } {
                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value305, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value305.clone()
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
                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, source_ml_source2_data_property_value306, _, _, _) => {
                                                source_ml_source2_data_property_value306.clone()
                                            }
                                        }
                                            } else {
                                                match state.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value307, _, _, _, _) => {
                                                source_ml_source2_data_property_value307.clone()
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
                                                    self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value308) => {
                                                        source_ml_source2_data_property_value308
                                                    }
                                                };
                                                        {
                                                            let source_ml_source1_temporary_index33 =
                                                                self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s309 = self::source_ml_source1_temporary_execute_rust_standard_collection_ordered_snapshot_loop34(method, values, source_ml_source1_temporary_values33, source_ml_source1_temporary_index33);
                                                                match source_ml_source2_s309 {
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
        let source_ml_source2_s314 = match left.clone() {
            self::RustStandardCollectionScalar::RustCollectionBoolean(source_ml_source2_f315) => {
                self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Next
            }
            self::RustStandardCollectionScalar::RustCollectionInteger(source_ml_source2_f316) => {
                self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Next
            }
            self::RustStandardCollectionScalar::RustCollectionString(left_units) => {
                match right.clone() {
                    self::RustStandardCollectionScalar::RustCollectionBoolean(source_ml_source2_f317) => {
                        self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Next
                    }
                    self::RustStandardCollectionScalar::RustCollectionInteger(source_ml_source2_f318) => {
                        self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Next
                    }
                    self::RustStandardCollectionScalar::RustCollectionString(units) => {
                        self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Return(self::ml_math::max(1f64, ((left_units.len() as f64) + (units.len() as f64))))
                    }
                }
            }
        };
        match source_ml_source2_s314 {
            self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Next => {
                1f64
            }
            self::MlSource1TemporaryOrderingWorkJoin35::MlSource1TemporaryOrderingWorkJoin35Return(source_ml_source1_temporary_result35) => {
                source_ml_source1_temporary_result35
            }
        }
    }
}
