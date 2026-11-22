// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Result;
use super::RustStandardCollectionCallOrigin;
use super::RustStandardCollectionOptionValue;
use super::RustStandardCollectionOrigin;
use super::RustStandardCollectionScalar;
use super::RustStandardCollectionState;
use super::RustStandardCollectionTransition;
use super::RustStandardCollectionValue;
use super::copy_rust_standard_collection_scalar;
use super::create_rust_standard_collection_boolean;
use super::create_rust_standard_collection_entry;
use super::find_rust_standard_collection_entry;
use super::initialize_rust_standard_collection_state;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_rust_standard_collection_call_origin_error;
use super::read_rust_standard_collection_entry_value;
use super::read_rust_standard_collection_kind;
use super::read_rust_standard_collection_source_origin_error;
use super::read_rust_standard_collection_state_error;
use super::replace_rust_standard_collection_entry_value;
use super::resolve_rust_standard_collection_owner;
use super::retain_rust_standard_collection_entry;
use super::source_ml_source1_temporary_execute_rust_standard_collection_operation_loop31;
use super::source_ml_source1_temporary_execute_rust_standard_collection_operation_loop32;
use super::source_ml_source1_temporary_retain_rust_standard_collection_insertion_loop28;
use super::source_ml_source1_temporary_retain_rust_standard_collection_insertion_loop29;
use super::test_rust_standard_collection_mutation;
use super::test_rust_standard_collection_ordered;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_rust_standard_collection_state(
    origin: self::RustStandardCollectionOrigin,
    key_domain: &self::ml_string::Text,
    value_domain: &self::ml_string::Text,
    pointer_width: f64,
    root: &self::ml_string::Text,
    call: self::RustStandardCollectionCallOrigin,
) -> self::RustStandardCollectionTransition {
    {
        let source = self::read_rust_standard_collection_source_origin_error(
            origin.clone(),
            key_domain,
            value_domain,
            pointer_width,
        );
        {
            let ml_library_inequality_2 = source != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::RustStandardCollectionTransition::RustCollectionInvalid(source.clone())
            } else if (self::ml_array::includes(
                &vec![
                    self::ml_string::Text::from_units(vec![110, 101, 119]),
                    self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]),
                ],
                &{
                    let source_ml_source2_optional_chain_receiver168 = (call.clone(),);
                    {
                        let source_ml_source1_temporary_optional_chain25 =
                            (source_ml_source2_optional_chain_receiver168).0;
                        match source_ml_source1_temporary_optional_chain25 {
                        self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, source_ml_source2_data_property_value169, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value169.clone()
                        }
                    }
                    }
                },
                0f64,
                false,
                |a, b| a == b,
            )) {
                {
                    let actual = self::read_rust_standard_collection_call_origin_error(origin.clone(), root, &(match call.clone() {
                        self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, source_ml_source2_data_property_value170, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value170.clone()
                        }
                    }), 0f64, call.clone());
                    {
                        let ml_library_inequality_1 =
                            actual != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            self::RustStandardCollectionTransition::RustCollectionInvalid(
                                actual.clone(),
                            )
                        } else {
                            {
                                let state = self::initialize_rust_standard_collection_state(
                                    origin.clone(),
                                    key_domain,
                                    value_domain,
                                    pointer_width,
                                    root,
                                );
                                {
                                    let error = self::read_rust_standard_collection_state_error(
                                        state.clone(),
                                    );
                                    if error == (self::ml_string::Text::from_units(vec![])) {
                                        self::RustStandardCollectionTransition::RustCollectionCreated(Box::new(state.clone()))
                                    } else {
                                        self::RustStandardCollectionTransition::RustCollectionInvalid(error.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                self::RustStandardCollectionTransition::RustCollectionInvalid(
                    self::ml_string::Text::from_units(vec![
                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 114, 101, 97, 116,
                        105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                        97, 99, 116, 117, 97, 108, 32, 110, 101, 119, 32, 111, 114, 32, 68, 101,
                        102, 97, 117, 108, 116, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111,
                        114, 32, 100, 105, 115, 112, 97, 116, 99, 104,
                    ]),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_rust_standard_collection_option(
    value: Option<self::RustStandardCollectionOptionValue>,
) -> self::RustStandardCollectionValue {
    if match (value.clone(), None::<()>) {
        (Some(_ml_left), Some(_ml_right)) => false,
        (None, None) => true,
        _ => false,
    } {
        {
            let source_ml_source2_constructor_field171 = false;
            self::RustStandardCollectionValue::RustCollectionOption(
                source_ml_source2_constructor_field171,
                None::<(Option<self::RustStandardCollectionOptionValue>,)>,
            )
        }
    } else {
        {
            let source_ml_source2_constructor_field172 = true;
            {
                let source_ml_source2_constructor_field173 =
                    Some(value.clone().expect("a proven present value"));
                self::RustStandardCollectionValue::RustCollectionOption(
                    source_ml_source2_constructor_field172,
                    Some(source_ml_source2_constructor_field173),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn copy_rust_standard_collection_option_scalar(
    value: self::RustStandardCollectionScalar,
) -> self::RustStandardCollectionOptionValue {
    match value.clone() {
        self::RustStandardCollectionScalar::RustCollectionBoolean(value_value) => {
            self::RustStandardCollectionOptionValue::RustCollectionBoolean(value_value)
        }
        self::RustStandardCollectionScalar::RustCollectionInteger(value_value_2) => {
            self::RustStandardCollectionOptionValue::RustCollectionInteger(value_value_2.clone())
        }
        self::RustStandardCollectionScalar::RustCollectionString(units) => {
            self::RustStandardCollectionOptionValue::RustCollectionString(units)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn copy_rust_standard_collection_result_scalar(
    value: self::RustStandardCollectionScalar,
) -> self::RustStandardCollectionValue {
    match value.clone() {
        self::RustStandardCollectionScalar::RustCollectionBoolean(value_value) => {
            self::RustStandardCollectionValue::RustCollectionBoolean(value_value)
        }
        self::RustStandardCollectionScalar::RustCollectionInteger(value_value_2) => {
            self::RustStandardCollectionValue::RustCollectionInteger(value_value_2.clone())
        }
        self::RustStandardCollectionScalar::RustCollectionString(units) => {
            self::RustStandardCollectionValue::RustCollectionString(units)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn retain_rust_standard_collection_mutation(
    state: self::RustStandardCollectionState,
    entries: Vec<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    value: self::RustStandardCollectionValue,
) -> self::RustStandardCollectionTransition {
    if (match state.clone() {
        self::RustStandardCollectionState::RustStandardCollectionState(
            _,
            _,
            _,
            _,
            _,
            source_ml_source2_data_property_value175,
            _,
        ) => source_ml_source2_data_property_value175,
    }) >= 9_007_199_254_740_991f64
    {
        {
            let source_ml_source2_constructor_field176 = Some(state.clone());
            {
                let source_ml_source2_constructor_field177 =
                    self::ml_string::Text::from_units(vec![
                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 109, 117, 116, 97, 116,
                        105, 111, 110, 32, 114, 101, 118, 105, 115, 105, 111, 110, 32, 101, 120,
                        99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99,
                        105, 116, 32, 101, 120, 97, 99, 116, 32, 99, 111, 111, 114, 100, 105, 110,
                        97, 116, 101, 32, 112, 114, 111, 102, 105, 108, 101,
                    ]);
                self::RustStandardCollectionTransition::RustCollectionResourceLimit(
                    Some(source_ml_source2_constructor_field176),
                    source_ml_source2_constructor_field177,
                )
            }
        }
    } else {
        {
            let next = {
                let source_ml_source2_nominal_spread179 = state.clone();
                {
                    let source_ml_source2_nominal_field180 =
                        match source_ml_source2_nominal_spread179.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(
                                source_ml_source2_data_property_value181,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value181 =
                                    *source_ml_source2_data_property_value181;
                                source_ml_source2_data_property_value181.clone()
                            }
                        };
                    {
                        let source_ml_source2_nominal_field182 =
                            match source_ml_source2_nominal_spread179.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(
                                    _,
                                    source_ml_source2_data_property_value183,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value183.clone(),
                            };
                        {
                            let source_ml_source2_nominal_field184 = match source_ml_source2_nominal_spread179.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value185, _, _, _, _) => {
                                    source_ml_source2_data_property_value185.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field186 = match source_ml_source2_nominal_spread179.clone() {
                                    self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, source_ml_source2_data_property_value187, _, _, _) => {
                                        source_ml_source2_data_property_value187.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field188 = match source_ml_source2_nominal_spread179.clone() {
                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, source_ml_source2_data_property_value189, _, _) => {
                                            source_ml_source2_data_property_value189
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field190 = match source_ml_source2_nominal_spread179.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, source_ml_source2_data_property_value191, _) => {
                                                source_ml_source2_data_property_value191
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field192 = match source_ml_source2_nominal_spread179.clone() {
                                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value193) => {
                                                    source_ml_source2_data_property_value193
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override194 = match state.clone() {
                                                    self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, source_ml_source2_data_property_value178, _) => {
                                                        source_ml_source2_data_property_value178
                                                    }
                                                } + 1f64;
                                                {
                                                    let source_ml_source2_nominal_override195 =
                                                        entries;
                                                    self::RustStandardCollectionState::RustStandardCollectionState(Box::new(source_ml_source2_nominal_field180), source_ml_source2_nominal_field182, source_ml_source2_nominal_field184, source_ml_source2_nominal_field186, source_ml_source2_nominal_field188, source_ml_source2_nominal_override194, source_ml_source2_nominal_override195)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            {
                let error = self::read_rust_standard_collection_state_error(next.clone());
                {
                    let ml_library_inequality_1 =
                        error != (self::ml_string::Text::from_units(vec![]));
                    if ml_library_inequality_1 {
                        {
                            let source_ml_source2_constructor_field196 = Some(state.clone());
                            {
                                let source_ml_source2_constructor_field197 = error.clone();
                                self::RustStandardCollectionTransition::RustCollectionResourceLimit(
                                    Some(source_ml_source2_constructor_field196),
                                    source_ml_source2_constructor_field197,
                                )
                            }
                        }
                    } else {
                        self::RustStandardCollectionTransition::RustCollectionNormal(
                            Box::new(next.clone()),
                            Box::new(value.clone()),
                            true,
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn retain_rust_standard_collection_insertion(
    state: self::RustStandardCollectionState,
    key: self::RustStandardCollectionScalar,
    position: f64,
    ordered: bool,
    replacement: (
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    ),
    value: self::RustStandardCollectionValue,
) -> self::RustStandardCollectionTransition {
    {
        let present = position >= 0f64;
        {
            let insertion = position;
            {
                let insertion_5 = if present {
                    insertion
                } else {
                    {
                        let insertion_2 = (match &state {
                            self::RustStandardCollectionState::RustStandardCollectionState(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value198,
                            ) => source_ml_source2_data_property_value198.as_slice(),
                        })
                        .len() as f64;
                        if ordered {
                            {
                                let insertion_3 = 0f64;
                                {
                                    let source_ml_source1_temporary_values26 = match state.clone() {
                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value199) => {
                                            source_ml_source2_data_property_value199
                                        }
                                    };
                                    {
                                        let source_ml_source1_temporary_index26 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            self::source_ml_source1_temporary_retain_rust_standard_collection_insertion_loop28(key.clone(), insertion_3, source_ml_source1_temporary_values26, source_ml_source1_temporary_index26)
                                        }
                                    }
                                }
                            }
                        } else {
                            insertion_2
                        }
                    }
                };
                {
                    let entries = Vec::<(
                        self::RustStandardCollectionScalar,
                        Option<(Option<self::RustStandardCollectionScalar>,)>,
                    )>::new();
                    {
                        let index = 0f64;
                        {
                            let source_ml_source1_temporary_values27 = match state.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value200,
                                ) => source_ml_source2_data_property_value200,
                            };
                            {
                                let source_ml_source1_temporary_index27 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let entries_2 = self::source_ml_source1_temporary_retain_rust_standard_collection_insertion_loop29(replacement.clone(), present, insertion_5, entries, index, source_ml_source1_temporary_values27, source_ml_source1_temporary_index27);
                                    {
                                        let entries_4 = if insertion_5 == ((match &state { self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value202) => source_ml_source2_data_property_value202.as_slice() }).len() as f64) {
                                            {
                                                { let (ml_append_right, ml_append_left) = (vec![replacement.clone()], entries_2); self::ml_array::append(ml_append_left, ml_append_right) }
                                            }
                                        } else {
                                            entries_2
                                        };
                                        self::retain_rust_standard_collection_mutation(
                                            state.clone(),
                                            entries_4,
                                            value.clone(),
                                        )
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
pub fn execute_rust_standard_collection_operation(
    state: self::RustStandardCollectionState,
    method: &self::ml_string::Text,
    arguments_: Vec<self::RustStandardCollectionScalar>,
    call: self::RustStandardCollectionCallOrigin,
) -> self::RustStandardCollectionTransition {
    {
        let error = self::read_rust_standard_collection_state_error(state.clone());
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::RustStandardCollectionTransition::RustCollectionInvalid(error.clone())
            } else {
                {
                    let actual = self::read_rust_standard_collection_call_origin_error(
                        match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(
                                source_ml_source2_data_property_value203,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value203 =
                                    *source_ml_source2_data_property_value203;
                                source_ml_source2_data_property_value203.clone()
                            }
                        },
                        &(match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(
                                _,
                                source_ml_source2_data_property_value204,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value204.clone(),
                        }),
                        method,
                        (arguments_.len() as f64),
                        call.clone(),
                    );
                    {
                        let ml_library_inequality_1 =
                            actual != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            self::RustStandardCollectionTransition::RustCollectionInvalid(
                                actual.clone(),
                            )
                        } else if self::ml_array::includes(
                            &vec![
                                self::ml_string::Text::from_units(vec![105, 116, 101, 114]),
                                self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                                self::ml_string::Text::from_units(vec![
                                    118, 97, 108, 117, 101, 115,
                                ]),
                            ],
                            method,
                            0f64,
                            false,
                            |a, b| a == b,
                        ) {
                            self::RustStandardCollectionTransition::RustCollectionInvalid(
                                self::ml_string::Text::from_units(vec![
                                    111, 114, 100, 101, 114, 101, 100, 32, 105, 116, 101, 114, 97,
                                    116, 111, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115,
                                    32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117,
                                    97, 108, 32, 99, 111, 112, 105, 101, 100, 47, 99, 108, 111,
                                    110, 101, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111,
                                    110, 32, 116, 104, 114, 111, 117, 103, 104, 32, 116, 104, 101,
                                    32, 105, 109, 109, 101, 100, 105, 97, 116, 101, 32, 118, 97,
                                    108, 117, 101, 45, 112, 97, 99, 107, 101, 116, 32, 97, 100, 97,
                                    112, 116, 101, 114,
                                ]),
                            )
                        } else if ((*method)
                            == (self::ml_string::Text::from_units(vec![110, 101, 119])))
                            || ((*method)
                                == (self::ml_string::Text::from_units(vec![
                                    100, 101, 102, 97, 117, 108, 116,
                                ])))
                        {
                            self::RustStandardCollectionTransition::RustCollectionInvalid(
                                self::ml_string::Text::from_units(vec![
                                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111,
                                    110, 115, 116, 114, 117, 99, 116, 111, 114, 115, 32, 117, 115,
                                    101, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105,
                                    116, 32, 115, 111, 117, 114, 99, 101, 32, 99, 114, 101, 97,
                                    116, 105, 111, 110, 32, 101, 110, 116, 114, 121, 32, 112, 111,
                                    105, 110, 116,
                                ]),
                            )
                        } else {
                            {
                                let owner = self::resolve_rust_standard_collection_owner(&(match match state.clone() {
                                    self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value205, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value205 = *source_ml_source2_data_property_value205;
                                        source_ml_source2_data_property_value205.clone()
                                    }
                                } {
                                    self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value206, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value206.clone()
                                    }
                                }), true, false);
                                {
                                    let kind = self::read_rust_standard_collection_kind(&(owner));
                                    {
                                        let borrowed = (*method)
                                            != (self::ml_string::Text::from_units(vec![
                                                105, 110, 115, 101, 114, 116,
                                            ]));
                                        {
                                            let index = 0f64;
                                            {
                                                let source_ml_source2_s207 = self::source_ml_source1_temporary_execute_rust_standard_collection_operation_loop31(state.clone(), arguments_.clone(), call.clone(), borrowed, index);
                                                match source_ml_source2_s207 {
                                                    self::MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Result::MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Done => {
                                                        if (*method) == (self::ml_string::Text::from_units(vec![108, 101, 110])) {
                                                            self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state.clone()), Box::new(self::RustStandardCollectionValue::RustCollectionInteger({
                                                                let source_ml_source2_integer_operand209 = (match &state { self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value208) => source_ml_source2_data_property_value208.as_slice() }).len() as f64;
                                                                {
                                                                    let source_ml_source2_integer_result210 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(source_ml_source2_integer_operand209);
                                                                    if (source_ml_source2_integer_result210.len() as f64) == 1f64 {
                                                                        (self::ml_array::optional_at(&source_ml_source2_integer_result210, self::ml_array::number_index(0f64))).unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                    } else {
                                                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_integer_operand209)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                    }
                                                                }
                                                            })), false)
                                                        } else if (*method) == (self::ml_string::Text::from_units(vec![105, 115, 95, 101, 109, 112, 116, 121])) {
                                                                self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state.clone()), Box::new(self::RustStandardCollectionValue::RustCollectionBoolean(((match &state { self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value212) => source_ml_source2_data_property_value212.as_slice() }).len() as f64) == 0f64)), false)
                                                            } else if (*method) == (self::ml_string::Text::from_units(vec![99, 108, 111, 110, 101])) {
                                                                    {
                                                                        let entries = { let ml_values2 = match state.clone() {
                                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value213) => {
                                                                                source_ml_source2_data_property_value213
                                                                            }
                                                                        }; let ml_callback2 = std::rc::Rc::new(move |entry: (self::RustStandardCollectionScalar, Option<(Option<self::RustStandardCollectionScalar>,)>)| -> (self::RustStandardCollectionScalar, Option<(Option<self::RustStandardCollectionScalar>,)>) { self::create_rust_standard_collection_entry((entry.clone()).0, ((((entry.clone()).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()) }); ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2)).collect::<Vec<_>>() };
                                                                        self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state.clone()), Box::new(self::RustStandardCollectionValue::RustStandardCollectionState(Box::new(match state.clone() {
                                                                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value214, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value214 = *source_ml_source2_data_property_value214;
                                                                                source_ml_source2_data_property_value214.clone()
                                                                            }
                                                                        }), (match call.clone() {
                                                                            self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value215) => {
                                                                                ((source_ml_source2_data_property_value215.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), match state.clone() {
                                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value216, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value216.clone()
                                                                            }
                                                                        }, match state.clone() {
                                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, source_ml_source2_data_property_value217, _, _, _) => {
                                                                                source_ml_source2_data_property_value217.clone()
                                                                            }
                                                                        }, match state.clone() {
                                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, source_ml_source2_data_property_value218, _, _) => {
                                                                                source_ml_source2_data_property_value218
                                                                            }
                                                                        }, 0f64, entries)), false)
                                                                    }
                                                                } else if (*method) == (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114])) {
                                                                        self::retain_rust_standard_collection_mutation(state.clone(), Vec::<(self::RustStandardCollectionScalar, Option<(Option<self::RustStandardCollectionScalar>,)>)>::new(), self::RustStandardCollectionValue::RustCollectionUnit)
                                                                    } else {
                                                                        {
                                                                            let key = self::ml_array::optional_at(&arguments_, self::ml_array::number_index(0f64));
                                                                            if match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 108, 111, 111, 107, 117, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 107, 101, 121, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                                            } else {
                                                                                {
                                                                                    let position = self::find_rust_standard_collection_entry(match state.clone() {
                                                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value219) => {
                                                                                            source_ml_source2_data_property_value219
                                                                                        }
                                                                                    }, key.clone().expect("a proven present value"));
                                                                                    {
                                                                                        let present = position >= 0f64;
                                                                                        if ((*method) == (self::ml_string::Text::from_units(vec![99, 111, 110, 116, 97, 105, 110, 115]))) || ((*method) == (self::ml_string::Text::from_units(vec![99, 111, 110, 116, 97, 105, 110, 115, 95, 107, 101, 121]))) {
                                                                                            self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state.clone()), Box::new(self::RustStandardCollectionValue::RustCollectionBoolean(present)), false)
                                                                                        } else if (*method) == (self::ml_string::Text::from_units(vec![103, 101, 116])) {
                                                                                                self::RustStandardCollectionTransition::RustCollectionNormal(Box::new(state.clone()), Box::new(self::create_rust_standard_collection_option(if present {
                                                                                                    Some(self::RustStandardCollectionOptionValue::RustCollectionReference(match state.clone() {
                                                                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, source_ml_source2_data_property_value220, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value220.clone()
                                                                                                        }
                                                                                                    }, match state.clone() {
                                                                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, source_ml_source2_data_property_value221, _) => {
                                                                                                            source_ml_source2_data_property_value221
                                                                                                        }
                                                                                                    }, Box::new(self::copy_rust_standard_collection_scalar({
                                                                                                        let source_ml_source2_nullish_field_receiver223 = (self::ml_array::optional_at((match &state { self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value222) => source_ml_source2_data_property_value222.as_slice() }), self::ml_array::number_index(position)),);
                                                                                                        if match (source_ml_source2_nullish_field_receiver223.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                            {
                                                                                                                let source_ml_source2_nullish_field_present224 = ((source_ml_source2_nullish_field_receiver223.clone()).0).clone().expect("a proven present value");
                                                                                                                (source_ml_source2_nullish_field_present224).0
                                                                                                            }
                                                                                                        } else {
                                                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                                                                        }
                                                                                                    })), if kind == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                                                                                                        match state.clone() {
                                                                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, source_ml_source2_data_property_value225, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value225.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        match state.clone() {
                                                                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value226, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value226.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    }))
                                                                                                } else {
                                                                                                    None::<self::RustStandardCollectionOptionValue>
                                                                                                })), false)
                                                                                            } else if (self::test_rust_standard_collection_mutation(&(owner), method)) {if (*method) == (self::ml_string::Text::from_units(vec![105, 110, 115, 101, 114, 116])) {
                                                                                                        {
                                                                                                            let ordered = self::test_rust_standard_collection_ordered(&(owner));
                                                                                                            if kind == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                                                                                                                {
                                                                                                                    let input = self::ml_array::optional_at(&arguments_, self::ml_array::number_index(1f64));
                                                                                                                    if match (input.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                                                        self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![115, 116, 97, 110, 100, 97, 114, 100, 32, 77, 97, 112, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 118, 97, 108, 117, 101]))
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let existing = self::ml_array::optional_at((match &state { self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value227) => source_ml_source2_data_property_value227.as_slice() }), self::ml_array::number_index(position));
                                                                                                                            {
                                                                                                                                let previous = self::read_rust_standard_collection_entry_value(existing.clone());
                                                                                                                                if present && match (previous.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                                                                    self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![115, 116, 97, 110, 100, 97, 114, 100, 32, 77, 97, 112, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 108, 100, 32, 111, 119, 110, 101, 100, 32, 118, 97, 108, 117, 101]))
                                                                                                                                } else {
                                                                                                                                    {
                                                                                                                                        let value = self::create_rust_standard_collection_option(if match (previous.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                                                                            None::<self::RustStandardCollectionOptionValue>
                                                                                                                                        } else {
                                                                                                                                            Some(self::copy_rust_standard_collection_option_scalar(previous.clone().expect("a proven present value")))
                                                                                                                                        });
                                                                                                                                        {
                                                                                                                                            let replacement = if match (existing.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                                                                                self::create_rust_standard_collection_entry(key.clone().expect("a proven present value"), Some(input.clone().expect("a proven present value")))
                                                                                                                                            } else {
                                                                                                                                                self::replace_rust_standard_collection_entry_value(existing.clone().expect("a proven present value"), input.clone().expect("a proven present value"))
                                                                                                                                            };
                                                                                                                                            self::retain_rust_standard_collection_insertion(state.clone(), key.clone().expect("a proven present value"), position, ordered, replacement, value)
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let value_2 = self::create_rust_standard_collection_boolean(!present);
                                                                                                                    if present {
                                                                                                                        self::retain_rust_standard_collection_mutation(state.clone(), { let ml_values3 = match state.clone() {
                                                                                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value228) => {
                                                                                                                                source_ml_source2_data_property_value228
                                                                                                                            }
                                                                                                                        }; let ml_callback3 = std::rc::Rc::new(move |entry: (self::RustStandardCollectionScalar, Option<(Option<self::RustStandardCollectionScalar>,)>)| -> (self::RustStandardCollectionScalar, Option<(Option<self::RustStandardCollectionScalar>,)>) { self::retain_rust_standard_collection_entry(entry.clone()) }); ml_values3.iter().cloned().enumerate().map(|(ml_index3, ml_value3)| ml_callback3(ml_value3)).collect::<Vec<_>>() }, value_2.clone())
                                                                                                                    } else {
                                                                                                                        self::retain_rust_standard_collection_insertion(state.clone(), key.clone().expect("a proven present value"), position, ordered, self::create_rust_standard_collection_entry(key.clone().expect("a proven present value"), None::<self::RustStandardCollectionScalar>), value_2.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        {
                                                                                                            let previous_2 = self::read_rust_standard_collection_entry_value(self::ml_array::optional_at((match &state { self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value229) => source_ml_source2_data_property_value229.as_slice() }), self::ml_array::number_index(position)));
                                                                                                            if ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && present) && match (previous_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                                                self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![115, 116, 97, 110, 100, 97, 114, 100, 32, 77, 97, 112, 32, 114, 101, 109, 111, 118, 97, 108, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 108, 100, 32, 111, 119, 110, 101, 100, 32, 118, 97, 108, 117, 101]))
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let value_3 = if kind == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                                                                                                                        self::create_rust_standard_collection_option(if match (previous_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                                                            None::<self::RustStandardCollectionOptionValue>
                                                                                                                        } else {
                                                                                                                            Some(self::copy_rust_standard_collection_option_scalar(previous_2.clone().expect("a proven present value")))
                                                                                                                        })
                                                                                                                    } else {
                                                                                                                        self::create_rust_standard_collection_boolean(present)
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let entries_2 = Vec::<(self::RustStandardCollectionScalar, Option<(Option<self::RustStandardCollectionScalar>,)>)>::new();
                                                                                                                        {
                                                                                                                            let index_2 = 0f64;
                                                                                                                            {
                                                                                                                                let source_ml_source1_temporary_values30 = match state.clone() {
                                                                                                                                    self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value230) => {
                                                                                                                                        source_ml_source2_data_property_value230
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source1_temporary_index30 = self::ml::Big::from_i128(0);
                                                                                                                                    {
                                                                                                                                        let entries_3 = self::source_ml_source1_temporary_execute_rust_standard_collection_operation_loop32(position, entries_2, index_2, source_ml_source1_temporary_values30, source_ml_source1_temporary_index30);
                                                                                                                                        self::retain_rust_standard_collection_mutation(state.clone(), entries_3, value_3)
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }} else {
                                                                                                    self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 108, 111, 115, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115]))
                                                                                                }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                    }
                                                    self::MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Result::MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Return(source_ml_source1_temporary_result32) => {
                                                        let source_ml_source1_temporary_result32 = *source_ml_source1_temporary_result32;
                                                        source_ml_source1_temporary_result32.clone()
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
