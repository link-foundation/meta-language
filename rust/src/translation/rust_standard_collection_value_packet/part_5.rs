// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlSource1TemporaryFindRustStandardCollectionEntryLoop20Result;
use super::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Result;
use super::MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19;
use super::MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Result;
use super::RustStandardCollectionOrigin;
use super::RustStandardCollectionScalar;
use super::RustStandardCollectionState;
use super::RustStandardCollectionValue;
use super::accept_rust_owned_local_text_units;
use super::ml;
use super::ml_array;
use super::ml_big_integer_bitwise;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_rust_standard_collection_domain_error;
use super::read_rust_standard_collection_integer_width;
use super::read_rust_standard_collection_kind;
use super::read_rust_standard_collection_source_origin_error;
use super::resolve_rust_standard_collection_owner;
use super::source_ml_source1_temporary_accept_rust_owned_local_text_input_loop17;
use super::source_ml_source1_temporary_find_rust_standard_collection_entry_loop20;
use super::source_ml_source1_temporary_read_rust_standard_collection_state_error_loop22;
use super::source_ml_source1_temporary_test_rust_standard_collection_scalar_equality_loop18;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_owned_local_text_input() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        102, 117, 110, 99, 116, 105, 111, 110, 32, 109, 108, 95, 111, 119, 110, 101, 100, 84, 101,
        120, 116, 73, 110, 112, 117, 116, 40, 116, 101, 120, 116, 41, 123, 10, 32, 105, 102, 40,
        116, 121, 112, 101, 111, 102, 32, 116, 101, 120, 116, 32, 33, 61, 61, 32, 34, 115, 116,
        114, 105, 110, 103, 34, 41, 114, 101, 116, 117, 114, 110, 32, 102, 97, 108, 115, 101, 59,
        10, 32, 102, 111, 114, 40, 108, 101, 116, 32, 105, 110, 100, 101, 120, 61, 48, 59, 105,
        110, 100, 101, 120, 60, 116, 101, 120, 116, 46, 108, 101, 110, 103, 116, 104, 59, 105, 110,
        100, 101, 120, 43, 43, 41, 123, 10, 32, 32, 99, 111, 110, 115, 116, 32, 117, 110, 105, 116,
        61, 116, 101, 120, 116, 46, 99, 104, 97, 114, 67, 111, 100, 101, 65, 116, 40, 105, 110,
        100, 101, 120, 41, 59, 10, 32, 32, 105, 102, 40, 117, 110, 105, 116, 62, 61, 53, 53, 50,
        57, 54, 38, 38, 117, 110, 105, 116, 60, 61, 53, 54, 51, 49, 57, 41, 123, 10, 32, 32, 32,
        105, 110, 100, 101, 120, 43, 43, 59, 10, 32, 32, 32, 99, 111, 110, 115, 116, 32, 110, 101,
        120, 116, 61, 116, 101, 120, 116, 46, 99, 104, 97, 114, 67, 111, 100, 101, 65, 116, 40,
        105, 110, 100, 101, 120, 41, 59, 10, 32, 32, 32, 105, 102, 40, 33, 40, 110, 101, 120, 116,
        62, 61, 53, 54, 51, 50, 48, 38, 38, 110, 101, 120, 116, 60, 61, 53, 55, 51, 52, 51, 41, 41,
        114, 101, 116, 117, 114, 110, 32, 102, 97, 108, 115, 101, 59, 10, 32, 32, 125, 101, 108,
        115, 101, 32, 105, 102, 40, 117, 110, 105, 116, 62, 61, 53, 54, 51, 50, 48, 38, 38, 117,
        110, 105, 116, 60, 61, 53, 55, 51, 52, 51, 41, 114, 101, 116, 117, 114, 110, 32, 102, 97,
        108, 115, 101, 59, 10, 32, 125, 10, 32, 114, 101, 116, 117, 114, 110, 32, 116, 114, 117,
        101, 59, 10, 125,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_local_text_input(text: &self::ml_string::Text) -> bool {
    if (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
        == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
    {
        {
            let units = Vec::<f64>::new();
            {
                let index = 0f64;
                {
                    let units_2 =
                        self::source_ml_source1_temporary_accept_rust_owned_local_text_input_loop17(
                            text, units, index,
                        );
                    self::accept_rust_owned_local_text_units(&(units_2))
                }
            }
        }
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_maximum_rust_standard_collection_text_units() -> f64 {
    1_048_576f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn copy_rust_standard_collection_scalar(
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
pub fn create_rust_standard_collection_entry(
    key: self::RustStandardCollectionScalar,
    value: Option<self::RustStandardCollectionScalar>,
) -> (
    self::RustStandardCollectionScalar,
    Option<(Option<self::RustStandardCollectionScalar>,)>,
) {
    {
        let copied_key = self::copy_rust_standard_collection_scalar(key.clone());
        if match (value.clone(), None::<()>) {
            (Some(_ml_left), Some(_ml_right)) => false,
            (None, None) => true,
            _ => false,
        } {
            {
                let source_ml_source2_field100 = copied_key.clone();
                (
                    source_ml_source2_field100,
                    None::<(Option<self::RustStandardCollectionScalar>,)>,
                )
            }
        } else {
            {
                let source_ml_source2_field101 = copied_key.clone();
                {
                    let source_ml_source2_field102 =
                        Some(Some(self::copy_rust_standard_collection_scalar(
                            value.clone().expect("a proven present value"),
                        )));
                    (source_ml_source2_field101, source_ml_source2_field102)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn retain_rust_standard_collection_entry(
    entry: (
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    ),
) -> (
    self::RustStandardCollectionScalar,
    Option<(Option<self::RustStandardCollectionScalar>,)>,
) {
    {
        let source_ml_source2_spread103 = entry.clone();
        {
            let source_ml_source2_spread_field104 = (source_ml_source2_spread103.clone()).0;
            {
                let source_ml_source2_spread_field105 = (source_ml_source2_spread103.clone()).1;
                (
                    source_ml_source2_spread_field104,
                    source_ml_source2_spread_field105,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn replace_rust_standard_collection_entry_value(
    entry: (
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    ),
    value: self::RustStandardCollectionScalar,
) -> (
    self::RustStandardCollectionScalar,
    Option<(Option<self::RustStandardCollectionScalar>,)>,
) {
    {
        let source_ml_source2_spread106 = entry.clone();
        {
            let source_ml_source2_spread_field107 = (source_ml_source2_spread106.clone()).0;
            {
                let source_ml_source2_spread_field108 = (source_ml_source2_spread106.clone()).1;
                {
                    let source_ml_source2_field109 = Some(Some(
                        self::copy_rust_standard_collection_scalar(value.clone()),
                    ));
                    (
                        source_ml_source2_spread_field107,
                        source_ml_source2_field109,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_entry_value(
    entry: Option<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
) -> Option<self::RustStandardCollectionScalar> {
    if match (entry.clone(), None::<()>) {
        (Some(_ml_left), Some(_ml_right)) => false,
        (None, None) => true,
        _ => false,
    } {
        None::<self::RustStandardCollectionScalar>
    } else {
        ((((entry.clone().expect("a proven present value")).1)
            .clone()
            .unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,)))
        .0)
            .clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_stored_scalar(
    entry: Option<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    kind: &self::ml_string::Text,
) -> Option<self::RustStandardCollectionScalar> {
    if match (entry.clone(), None::<()>) {
        (Some(_ml_left), Some(_ml_right)) => false,
        (None, None) => true,
        _ => false,
    } {
        None::<self::RustStandardCollectionScalar>
    } else if (*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
        ((((entry.clone().expect("a proven present value")).1)
            .clone()
            .unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,)))
        .0)
            .clone()
    } else {
        Some((entry.clone().expect("a proven present value")).0)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_rust_standard_collection_boolean(value: bool) -> self::RustStandardCollectionValue {
    self::RustStandardCollectionValue::RustCollectionBoolean(value)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_standard_collection_scalar_equality(
    left: self::RustStandardCollectionScalar,
    right: self::RustStandardCollectionScalar,
) -> bool {
    match left.clone() {
        self::RustStandardCollectionScalar::RustCollectionBoolean(left_value) => {
            match right.clone() {
                self::RustStandardCollectionScalar::RustCollectionBoolean(value) => {
                    left_value == value
                }
                self::RustStandardCollectionScalar::RustCollectionInteger(
                    source_ml_source2_f111,
                ) => false,
                self::RustStandardCollectionScalar::RustCollectionString(
                    source_ml_source2_f112,
                ) => false,
            }
        }
        self::RustStandardCollectionScalar::RustCollectionInteger(source_ml_source2_f110) => {
            let source_ml_source2_s117 = {
                let left_value_2 = source_ml_source2_f110.clone();
                match right.clone() {
                        self::RustStandardCollectionScalar::RustCollectionBoolean(source_ml_source2_f120) => {
                            self::MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19::MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19Next
                        }
                        self::RustStandardCollectionScalar::RustCollectionInteger(value_2) => {
                            self::MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19::MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19Return(left_value_2 == value_2)
                        }
                        self::RustStandardCollectionScalar::RustCollectionString(source_ml_source2_f121) => {
                            self::MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19::MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19Next
                        }
                    }
            };
            match source_ml_source2_s117 {
                    self::MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19::MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19Next => {
                        false
                    }
                    self::MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19::MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19Return(source_ml_source1_temporary_result19) => {
                        source_ml_source1_temporary_result19
                    }
                }
        }
        self::RustStandardCollectionScalar::RustCollectionString(left_units) => {
            match right.clone() {
                self::RustStandardCollectionScalar::RustCollectionBoolean(
                    source_ml_source2_f122,
                ) => false,
                self::RustStandardCollectionScalar::RustCollectionInteger(
                    source_ml_source2_f123,
                ) => false,
                self::RustStandardCollectionScalar::RustCollectionString(units) => {
                    let ml_library_inequality_1 = (left_units.len() as f64) != (units.len() as f64);
                    if ml_library_inequality_1 {
                        false
                    } else {
                        {
                            let index = 0f64;
                            {
                                let source_ml_source2_s124 = self::source_ml_source1_temporary_test_rust_standard_collection_scalar_equality_loop18(left.clone(), right.clone(), index);
                                match source_ml_source2_s124 {
                                    self::MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Result::MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Done => {
                                        true
                                    }
                                    self::MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Result::MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Return(source_ml_source1_temporary_result18) => {
                                        source_ml_source1_temporary_result18
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
pub fn read_rust_standard_collection_scalar_error(
    value: Option<self::RustStandardCollectionScalar>,
    domain: &self::ml_string::Text,
    pointer_width: f64,
) -> self::ml_string::Text {
    {
        let error = self::read_rust_standard_collection_domain_error(domain, pointer_width);
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                error.clone()
            } else if match (value.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } || match (value.clone().expect("a proven present value"), None::<()>) {
                (_ml_left, Some(_ml_right)) => false,
                _ => false,
            } {
                if (*domain)
                    == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))
                {
                    self::ml_string::Text::from_units(vec![
                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 83, 116, 114, 105, 110,
                        103, 32, 118, 97, 108, 117, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32,
                        98, 111, 117, 110, 100, 101, 100, 32, 118, 97, 108, 105, 100, 32, 85, 110,
                        105, 99, 111, 100, 101, 32, 115, 99, 97, 108, 97, 114, 32, 116, 101, 120,
                        116, 32, 119, 105, 116, 104, 111, 117, 116, 32, 108, 111, 110, 101, 32,
                        115, 117, 114, 114, 111, 103, 97, 116, 101, 115,
                    ])
                } else if (*domain) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))
                {
                    self::ml_string::Text::from_units(vec![
                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 66, 111, 111, 108, 101,
                        97, 110, 32, 100, 111, 109, 97, 105, 110, 32, 97, 99, 99, 101, 112, 116,
                        115, 32, 111, 110, 108, 121, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111,
                        111, 108, 101, 97, 110, 32, 115, 99, 97, 108, 97, 114, 115,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![
                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 116, 101,
                        103, 101, 114, 32, 100, 111, 109, 97, 105, 110, 115, 32, 97, 99, 99, 101,
                        112, 116, 32, 111, 110, 108, 121, 32, 101, 120, 97, 99, 116, 32, 105, 110,
                        116, 101, 103, 101, 114, 32, 115, 99, 97, 108, 97, 114, 115,
                    ])
                }
            } else {
                self::read_rust_standard_collection_present_scalar_error(
                    value.clone().expect("a proven present value"),
                    domain,
                    pointer_width,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_present_scalar_error(
    value: self::RustStandardCollectionScalar,
    domain: &self::ml_string::Text,
    pointer_width: f64,
) -> self::ml_string::Text {
    if (*domain) == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])) {
        {
            let invalid = self::ml_string::Text::from_units(vec![
                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 83, 116, 114, 105, 110, 103,
                32, 118, 97, 108, 117, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 98, 111, 117,
                110, 100, 101, 100, 32, 118, 97, 108, 105, 100, 32, 85, 110, 105, 99, 111, 100,
                101, 32, 115, 99, 97, 108, 97, 114, 32, 116, 101, 120, 116, 32, 119, 105, 116, 104,
                111, 117, 116, 32, 108, 111, 110, 101, 32, 115, 117, 114, 114, 111, 103, 97, 116,
                101, 115,
            ]);
            match value.clone() {
                self::RustStandardCollectionScalar::RustCollectionBoolean(
                    source_ml_source2_f125,
                ) => invalid.clone(),
                self::RustStandardCollectionScalar::RustCollectionInteger(
                    source_ml_source2_f126,
                ) => invalid.clone(),
                self::RustStandardCollectionScalar::RustCollectionString(units) => {
                    if (((units.len() as f64)
                        <= (self::read_maximum_rust_standard_collection_text_units()))
                        && {
                            let ml_values1 = units.clone();
                            let ml_callback1 = std::rc::Rc::new(move |unit: f64| -> bool {
                                (self::ml_math::is_integer(unit) && (unit >= 0f64))
                                    && (unit <= 65_535f64)
                            });
                            ml_values1
                                .iter()
                                .cloned()
                                .enumerate()
                                .all(|(ml_index1, ml_value1)| ml_callback1(ml_value1))
                        })
                        && self::accept_rust_owned_local_text_units(&(units))
                    {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        invalid.clone()
                    }
                }
            }
        }
    } else if (*domain) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
        {
            let invalid_2 = self::ml_string::Text::from_units(vec![
                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 66, 111, 111, 108, 101, 97,
                110, 32, 100, 111, 109, 97, 105, 110, 32, 97, 99, 99, 101, 112, 116, 115, 32, 111,
                110, 108, 121, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111, 111, 108, 101, 97, 110,
                32, 115, 99, 97, 108, 97, 114, 115,
            ]);
            match value.clone() {
                self::RustStandardCollectionScalar::RustCollectionBoolean(value_value) => {
                    if (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))
                        == (self::ml_string::Text::from_units(vec![
                            98, 111, 111, 108, 101, 97, 110,
                        ]))
                    {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        invalid_2.clone()
                    }
                }
                self::RustStandardCollectionScalar::RustCollectionInteger(
                    source_ml_source2_f127,
                ) => invalid_2.clone(),
                self::RustStandardCollectionScalar::RustCollectionString(
                    source_ml_source2_f128,
                ) => invalid_2.clone(),
            }
        }
    } else {
        {
            let invalid_3 = self::ml_string::Text::from_units(vec![
                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 116, 101, 103, 101,
                114, 32, 100, 111, 109, 97, 105, 110, 115, 32, 97, 99, 99, 101, 112, 116, 32, 111,
                110, 108, 121, 32, 101, 120, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114,
                32, 115, 99, 97, 108, 97, 114, 115,
            ]);
            match value.clone() {
                self::RustStandardCollectionScalar::RustCollectionBoolean(
                    source_ml_source2_f129,
                ) => invalid_3.clone(),
                self::RustStandardCollectionScalar::RustCollectionInteger(value_value_2) => {
                    let ml_library_inequality_1 =
                        (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]))
                            != (self::ml_string::Text::from_units(vec![
                                98, 105, 103, 105, 110, 116,
                            ]));
                    if ml_library_inequality_1 {
                        invalid_3.clone()
                    } else {
                        {
                            let width = self::read_rust_standard_collection_integer_width(
                                domain,
                                pointer_width,
                            );
                            {
                                let signed = {
                                    let source_ml_source2_comparison_operand138 = {
                                        let source_ml_source2_string_index_operands131 =
                                            (domain.clone(), 0f64);
                                        {
                                            let source_ml_source2_string_index_number132 =
                                                (source_ml_source2_string_index_operands131
                                                    .clone())
                                                .1;
                                            if self::ml_math::is_integer(source_ml_source2_string_index_number132) && ((source_ml_source2_string_index_number132 >= 0f64) && (source_ml_source2_string_index_number132 < (((source_ml_source2_string_index_operands131.clone()).0).length()))) {
                                                    Some((std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                                        let index = if position.is_nan() {
                                                            0f64
                                                        } else {
                                                            position.trunc()
                                                        };
                                                        if (index < 0f64) || (index >= (text.length())) {
                                                            self::ml_string::Text::from_units(vec![])
                                                        } else {
                                                            {
                                                                let source_ml_source2_string_range_operand133 = text.clone();
                                                                {
                                                                    let source_ml_source2_string_range_operand134 = index;
                                                                    {
                                                                        let source_ml_source2_string_range_operand135 = index + 1f64;
                                                                        {
                                                                            let source_ml_source2_string_range_position136 = source_ml_source2_string_range_operand134;
                                                                            {
                                                                                let source_ml_source2_string_range_position137 = source_ml_source2_string_range_operand135;
                                                                                source_ml_source2_string_range_operand133.slice(source_ml_source2_string_range_position136, source_ml_source2_string_range_position137)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } }))((source_ml_source2_string_index_operands131.clone()).0, source_ml_source2_string_index_number132))
                                                } else {
                                                    None::<self::ml_string::Text>
                                                }
                                        }
                                    };
                                    {
                                        let source_ml_source2_comparison_operand139 =
                                            self::ml_string::Text::from_units(vec![105]);
                                        match (
                                            source_ml_source2_comparison_operand138,
                                            source_ml_source2_comparison_operand139,
                                        ) {
                                            (Some(ml_left), ml_right) => ml_left == ml_right,
                                            _ => false,
                                        }
                                    }
                                };
                                {
                                    let magnitude = self::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(3f64, self::ml::Big::from_i128(1), {
                                            let source_ml_source2_integer_operand140 = if signed {
                                                width - 1f64
                                            } else {
                                                width
                                            };
                                            {
                                                let source_ml_source2_integer_result141 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(source_ml_source2_integer_operand140);
                                                if (source_ml_source2_integer_result141.len() as f64) == 1f64 {
                                                    (self::ml_array::optional_at(&source_ml_source2_integer_result141, self::ml_array::number_index(0f64))).unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_integer_operand140)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                }
                                            }
                                        });
                                    {
                                        let minimum = if signed {
                                            magnitude.neg()
                                        } else {
                                            self::ml::Big::from_i128(0)
                                        };
                                        {
                                            let maximum =
                                                magnitude.sub(&self::ml::Big::from_i128(1));
                                            if (value_value_2 >= minimum)
                                                && (value_value_2 <= maximum)
                                            {
                                                self::ml_string::Text::from_units(vec![])
                                            } else {
                                                self::ml_string::Text::from_units(vec![
                                                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110,
                                                    32, 105, 110, 116, 101, 103, 101, 114, 32, 115,
                                                    99, 97, 108, 97, 114, 32, 105, 115, 32, 111,
                                                    117, 116, 115, 105, 100, 101, 32, 105, 116,
                                                    115, 32, 97, 99, 116, 117, 97, 108, 32, 100,
                                                    101, 99, 108, 97, 114, 101, 100, 32, 115, 111,
                                                    117, 114, 99, 101, 32, 100, 111, 109, 97, 105,
                                                    110,
                                                ])
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::RustStandardCollectionScalar::RustCollectionString(
                    source_ml_source2_f130,
                ) => invalid_3.clone(),
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_rust_standard_collection_entry(
    entries: Vec<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    key: self::RustStandardCollectionScalar,
) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s142 =
                self::source_ml_source1_temporary_find_rust_standard_collection_entry_loop20(
                    entries,
                    key.clone(),
                    index,
                );
            match source_ml_source2_s142 {
                self::MlSource1TemporaryFindRustStandardCollectionEntryLoop20Result::MlSource1TemporaryFindRustStandardCollectionEntryLoop20Done => {
                    -1f64
                }
                self::MlSource1TemporaryFindRustStandardCollectionEntryLoop20Result::MlSource1TemporaryFindRustStandardCollectionEntryLoop20Return(source_ml_source1_temporary_result20) => {
                    source_ml_source1_temporary_result20
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_scalar_text_length(
    scalar: self::RustStandardCollectionScalar,
) -> f64 {
    match scalar.clone() {
        self::RustStandardCollectionScalar::RustCollectionBoolean(source_ml_source2_f143) => 0f64,
        self::RustStandardCollectionScalar::RustCollectionInteger(source_ml_source2_f144) => 0f64,
        self::RustStandardCollectionScalar::RustCollectionString(units) => units.len() as f64,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_state_error(
    state: self::RustStandardCollectionState,
) -> self::ml_string::Text {
    if (((((({
        let source_ml_source2_optional_chain_receiver146 = (state.clone(),);
        {
            let source_ml_source1_temporary_optional_chain21 =
                (source_ml_source2_optional_chain_receiver146).0;
            match source_ml_source1_temporary_optional_chain21 {
                self::RustStandardCollectionState::RustStandardCollectionState(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => self::ml_string::Text::from_units(vec![
                    114, 117, 115, 116, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108,
                    101, 99, 116, 105, 111, 110, 83, 116, 97, 116, 101,
                ]),
            }
        }
    }) != (self::ml_string::Text::from_units(vec![
        114, 117, 115, 116, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116,
        105, 111, 110, 83, 116, 97, 116, 101,
    ]))) || (({
        let source_ml_source2_type_query_operand150 = match state.clone() {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                source_ml_source2_data_property_value149,
                _,
                _,
                _,
                _,
                _,
            ) => source_ml_source2_data_property_value149.clone(),
        };
        self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
    }) != (self::ml_string::Text::from_units(vec![
        115, 116, 114, 105, 110, 103,
    ]))))
        || (((match state.clone() {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                source_ml_source2_data_property_value152,
                _,
                _,
                _,
                _,
                _,
            ) => source_ml_source2_data_property_value152.clone(),
        })
        .length())
            == 0f64))
        || !(self::ml_math::is_safe_integer(match state.clone() {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value153,
                _,
            ) => source_ml_source2_data_property_value153,
        })))
        || ((match state.clone() {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value155,
                _,
            ) => source_ml_source2_data_property_value155,
        }) < 0f64))
        || {
            let source_ml_source2_same_value_operand157 = match state.clone() {
                self::RustStandardCollectionState::RustStandardCollectionState(
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value156,
                    _,
                ) => source_ml_source2_data_property_value156,
            };
            {
                let source_ml_source2_same_value_operand158 = -0f64;
                (((source_ml_source2_same_value_operand157
                    == source_ml_source2_same_value_operand158)
                    && ((source_ml_source2_same_value_operand157 != 0f64)
                        || ((1f64 / source_ml_source2_same_value_operand157)
                            == (1f64 / source_ml_source2_same_value_operand158))))
                    || ((source_ml_source2_same_value_operand157
                        != source_ml_source2_same_value_operand157)
                        && (source_ml_source2_same_value_operand158
                            != source_ml_source2_same_value_operand158)))
            }
        }
    {
        self::ml_string::Text::from_units(vec![
            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 97, 116, 101, 32, 114,
            101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
            111, 117, 114, 99, 101, 32, 114, 111, 111, 116, 32, 97, 110, 100, 32, 101, 120, 97, 99,
            116, 32, 110, 111, 110, 110, 101, 103, 97, 116, 105, 118, 101, 32, 114, 101, 118, 105,
            115, 105, 111, 110,
        ])
    } else {
        {
            let source = self::read_rust_standard_collection_source_origin_error(
                match state.clone() {
                    self::RustStandardCollectionState::RustStandardCollectionState(
                        source_ml_source2_data_property_value159,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value159 =
                            *source_ml_source2_data_property_value159;
                        source_ml_source2_data_property_value159.clone()
                    }
                },
                &(match state.clone() {
                    self::RustStandardCollectionState::RustStandardCollectionState(
                        _,
                        _,
                        source_ml_source2_data_property_value160,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value160.clone(),
                }),
                &(match state.clone() {
                    self::RustStandardCollectionState::RustStandardCollectionState(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value161,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value161.clone(),
                }),
                match state.clone() {
                    self::RustStandardCollectionState::RustStandardCollectionState(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value162,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value162,
                },
            );
            {
                let ml_library_inequality_1 = source != (self::ml_string::Text::from_units(vec![]));
                if ml_library_inequality_1 {
                    source.clone()
                } else if ((match &state {
                    self::RustStandardCollectionState::RustStandardCollectionState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value164,
                    ) => source_ml_source2_data_property_value164.as_slice(),
                })
                .len() as f64)
                    > 4096f64
                {
                    self::ml_string::Text::from_units(vec![
                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 97, 116, 101,
                        32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112,
                        108, 105, 99, 105, 116, 32, 101, 110, 116, 114, 121, 32, 115, 116, 111,
                        114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101,
                    ])
                } else {
                    {
                        let kind = self::read_rust_standard_collection_kind(&(self::resolve_rust_standard_collection_owner(&(match match state.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value165, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value165 = *source_ml_source2_data_property_value165;
                                source_ml_source2_data_property_value165.clone()
                            }
                        } {
                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value166, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value166.clone()
                            }
                        }), true, false)));
                        {
                            let units = 0f64;
                            {
                                let index = 0f64;
                                {
                                    let source_ml_source2_s167 = self::source_ml_source1_temporary_read_rust_standard_collection_state_error_loop22(state.clone(), &(kind), units, index);
                                    match source_ml_source2_s167 {
                                        self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Result::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Done(units_2) => {
                                            if units_2 <= (self::read_maximum_rust_standard_collection_text_units()) {
                                                self::ml_string::Text::from_units(vec![])
                                            } else {
                                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 116, 111, 116, 97, 108, 32, 83, 116, 114, 105, 110, 103, 32, 115, 116, 111, 114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101])
                                            }
                                        }
                                        self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Result::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Return(source_ml_source1_temporary_result22) => {
                                            source_ml_source1_temporary_result22.clone()
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
pub fn initialize_rust_standard_collection_state(
    origin: self::RustStandardCollectionOrigin,
    key_domain: &self::ml_string::Text,
    value_domain: &self::ml_string::Text,
    pointer_width: f64,
    root: &self::ml_string::Text,
) -> self::RustStandardCollectionState {
    self::RustStandardCollectionState::RustStandardCollectionState(
        Box::new(origin.clone()),
        root.clone(),
        key_domain.clone(),
        value_domain.clone(),
        pointer_width,
        0f64,
        Vec::<(
            self::RustStandardCollectionScalar,
            Option<(Option<self::RustStandardCollectionScalar>,)>,
        )>::new(),
    )
}
