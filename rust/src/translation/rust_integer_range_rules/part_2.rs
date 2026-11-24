// Generated from complete js/src/translation/rust-integer-range-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustIntegerRangeRowsLoop9Result;
use super::MlTemporaryPlanRustIntegerRangeRowsLoop10Result;
use super::MlTemporaryPlanRustIntegerRangeRowsLoop11Result;
use super::MlTemporaryReadRustIntegerRangeCutsLoop5Result;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::refuse_required_boolean_fact;
use super::refuse_required_text_fact;
use super::source_ml_temporary_plan_rust_integer_range_rows_loop9;
use super::source_ml_temporary_plan_rust_integer_range_rows_loop10;
use super::source_ml_temporary_plan_rust_integer_range_rows_loop11;
use super::source_ml_temporary_read_rust_integer_range_cuts_loop5;
use super::source_ml_temporary_read_rust_integer_range_cuts_loop6;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(
    values: &[self::ml_string::Text],
    index: f64,
) -> self::ml_string::Text {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112,
                111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109,
                110, 32, 112, 111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_integer_range_canonical_decimal(value: &self::ml_string::Text) -> bool {
    {
        let digits = value.clone();
        {
            let digits_3 = if digits.starts_with(&self::ml_string::Text::from_units(vec![45])) {
                {
                    {
                        let source_ml_source1_string_range_operand1 = digits;
                        {
                            let source_ml_source1_string_range_operand2 = 1f64;
                            {
                                let source_ml_source1_string_range_position3 =
                                    source_ml_source1_string_range_operand2;
                                {
                                    let source_ml_source1_string_range_position4 = f64::INFINITY;
                                    source_ml_source1_string_range_operand1.slice(
                                        source_ml_source1_string_range_position3,
                                        source_ml_source1_string_range_position4,
                                    )
                                }
                            }
                        }
                    }
                }
            } else {
                digits
            };
            if ((digits_3 == (self::ml_string::Text::from_units(vec![])))
                || (((digits_3.length()) > 1f64)
                    && digits_3.starts_with(&self::ml_string::Text::from_units(vec![48]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![45, 48])))
            {
                false
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s5 = self::source_ml_temporary_test_rust_integer_range_canonical_decimal_loop1(&(digits_3), at);
                        match source_ml_source1_s5 {
                            self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Done => {
                                true
                            }
                            self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Return(source_ml_temporary_result1) => {
                                source_ml_temporary_result1
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
pub fn compare_rust_integer_range_decimals(
    left: &self::ml_string::Text,
    right: &self::ml_string::Text,
) -> f64 {
    if !(self::test_rust_integer_range_canonical_decimal(left))
        || !(self::test_rust_integer_range_canonical_decimal(right))
    {
        2f64
    } else {
        {
            let negative_left = left.starts_with(&self::ml_string::Text::from_units(vec![45]));
            {
                let negative_right =
                    right.starts_with(&self::ml_string::Text::from_units(vec![45]));
                {
                    let ml_library_inequality_1 = negative_left != negative_right;
                    if ml_library_inequality_1 {
                        if negative_left { -1f64 } else { 1f64 }
                    } else {
                        {
                            let a = if negative_left {
                                {
                                    let source_ml_source1_string_range_operand6 = left.clone();
                                    {
                                        let source_ml_source1_string_range_operand7 = 1f64;
                                        {
                                            let source_ml_source1_string_range_position8 =
                                                source_ml_source1_string_range_operand7;
                                            {
                                                let source_ml_source1_string_range_position9 =
                                                    f64::INFINITY;
                                                source_ml_source1_string_range_operand6.slice(
                                                    source_ml_source1_string_range_position8,
                                                    source_ml_source1_string_range_position9,
                                                )
                                            }
                                        }
                                    }
                                }
                            } else {
                                left.clone()
                            };
                            {
                                let b = if negative_right {
                                    {
                                        let source_ml_source1_string_range_operand10 =
                                            right.clone();
                                        {
                                            let source_ml_source1_string_range_operand11 = 1f64;
                                            {
                                                let source_ml_source1_string_range_position12 =
                                                    source_ml_source1_string_range_operand11;
                                                {
                                                    let source_ml_source1_string_range_position13 =
                                                        f64::INFINITY;
                                                    source_ml_source1_string_range_operand10.slice(
                                                        source_ml_source1_string_range_position12,
                                                        source_ml_source1_string_range_position13,
                                                    )
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    right.clone()
                                };
                                {
                                    let order = if (a.length()) < (b.length()) {
                                        -1f64
                                    } else if (a.length()) > (b.length()) {
                                        1f64
                                    } else {
                                        0f64
                                    };
                                    {
                                        let order_3 = if order == 0f64 {
                                            {
                                                let at = 0f64;
                                                {
                                                    self::source_ml_temporary_compare_rust_integer_range_decimals_loop2(&(a), &(b), order, at)
                                                }
                                            }
                                        } else {
                                            order
                                        };
                                        if order_3 == 0f64 {
                                            0f64
                                        } else if negative_left {
                                            -order_3
                                        } else {
                                            order_3
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
pub fn read_rust_integer_range_successor(value: &self::ml_string::Text) -> self::ml_string::Text {
    if (self::test_rust_integer_range_canonical_decimal(value)) {
        {
            let negative = value.starts_with(&self::ml_string::Text::from_units(vec![45]));
            {
                let digits = if negative {
                    {
                        let source_ml_source1_string_range_operand14 = value.clone();
                        {
                            let source_ml_source1_string_range_operand15 = 1f64;
                            {
                                let source_ml_source1_string_range_position16 =
                                    source_ml_source1_string_range_operand15;
                                {
                                    let source_ml_source1_string_range_position17 = f64::INFINITY;
                                    source_ml_source1_string_range_operand14.slice(
                                        source_ml_source1_string_range_position16,
                                        source_ml_source1_string_range_position17,
                                    )
                                }
                            }
                        }
                    }
                } else {
                    value.clone()
                };
                {
                    let result = self::ml_string::Text::from_units(vec![]);
                    {
                        let carry = true;
                        {
                            let at = digits.length() - 1f64;
                            {
                                let source_ml_source1_s18 = self::source_ml_temporary_read_rust_integer_range_successor_loop3(negative, &(digits), result, carry, at);
                                match source_ml_source1_s18 {
                                    self::MlTemporaryReadRustIntegerRangeSuccessorLoop3Result::MlTemporaryReadRustIntegerRangeSuccessorLoop3Done(result_2, carry_2) => {
                                        if negative {
                                            {
                                                let result_4 = if result_2.starts_with(&self::ml_string::Text::from_units(vec![48])) && ((result_2.length()) > 1f64) {
                                                    {
                                                        {
                                                            let source_ml_source1_string_range_operand19 = result_2.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand20 = 1f64;
                                                                {
                                                                    let source_ml_source1_string_range_position21 = source_ml_source1_string_range_operand20;
                                                                    {
                                                                        let source_ml_source1_string_range_position22 = f64::INFINITY;
                                                                        source_ml_source1_string_range_operand19.slice(source_ml_source1_string_range_position21, source_ml_source1_string_range_position22)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    result_2.clone()
                                                };
                                                if result_4 == (self::ml_string::Text::from_units(vec![48])) {
                                                    self::ml_string::Text::from_units(vec![48])
                                                } else {
                                                    (self::ml_string::Text::from_units(vec![45])).concat(&result_4)
                                                }
                                            }
                                        } else if carry_2 {
                                                (self::ml_string::Text::from_units(vec![49])).concat(&result_2)
                                            } else {
                                                result_2.clone()
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
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_integer_range_source_error(
    ordinary: bool,
    operation: &self::ml_string::Text,
    lower: bool,
    upper: bool,
) -> self::ml_string::Text {
    if ordinary {
        if ((*operation) != (self::ml_string::Text::from_units(vec![46, 46])))
            && ((*operation) != (self::ml_string::Text::from_units(vec![46, 46, 61])))
        {
            self::ml_string::Text::from_units(vec![
                97, 32, 114, 97, 110, 103, 101, 32, 112, 97, 116, 116, 101, 114, 110, 32, 114, 101,
                113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                114, 97, 110, 103, 101, 32, 111, 112, 101, 114, 97, 116, 111, 114,
            ])
        } else if ((lower) || (upper)) {
            if ((*operation) == (self::ml_string::Text::from_units(vec![46, 46, 61]))) && !upper {
                self::ml_string::Text::from_units(vec![
                    97, 110, 32, 105, 110, 99, 108, 117, 115, 105, 118, 101, 32, 114, 97, 110, 103,
                    101, 32, 112, 97, 116, 116, 101, 114, 110, 32, 110, 101, 101, 100, 115, 32,
                    105, 116, 115, 32, 117, 112, 112, 101, 114, 32, 101, 110, 100, 112, 111, 105,
                    110, 116,
                ])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        } else {
            self::ml_string::Text::from_units(vec![
                97, 32, 114, 97, 110, 103, 101, 32, 112, 97, 116, 116, 101, 114, 110, 32, 110, 101,
                101, 100, 115, 32, 97, 116, 32, 108, 101, 97, 115, 116, 32, 111, 110, 101, 32, 101,
                110, 100, 112, 111, 105, 110, 116,
            ])
        }
    } else {
        self::ml_string::Text::from_units(vec![
            114, 97, 110, 103, 101, 115, 32, 97, 114, 101, 32, 111, 117, 116, 115, 105, 100, 101,
            32, 116, 104, 101, 32, 112, 111, 114, 116, 97, 98, 108, 101, 32, 112, 97, 116, 116,
            101, 114, 110, 32, 108, 97, 110, 103, 117, 97, 103, 101,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_integer_range_bounds_error(
    lower: &self::ml_string::Text,
    upper: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let order = self::compare_rust_integer_range_decimals(lower, upper);
        if order == 2f64 {
            self::ml_string::Text::from_units(vec![
                114, 97, 110, 103, 101, 32, 101, 110, 100, 112, 111, 105, 110, 116, 115, 32, 114,
                101, 113, 117, 105, 114, 101, 32, 101, 120, 97, 99, 116, 32, 99, 97, 110, 111, 110,
                105, 99, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 115,
            ])
        } else if order >= 0f64 {
            self::ml_string::Text::from_units(vec![
                97, 32, 114, 97, 110, 103, 101, 32, 112, 97, 116, 116, 101, 114, 110, 32, 109, 117,
                115, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 97, 116, 32, 108, 101, 97, 115,
                116, 32, 111, 110, 101, 32, 105, 110, 116, 101, 103, 101, 114,
            ])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_integer_range_cuts(
    minimum: &self::ml_string::Text,
    maximum: &self::ml_string::Text,
    lowers: &[self::ml_string::Text],
    uppers: &[self::ml_string::Text],
) -> Vec<self::ml_string::Text> {
    if ((lowers.len() as f64) != (uppers.len() as f64))
        || ((self::read_rust_integer_range_bounds_error(minimum, maximum))
            != (self::ml_string::Text::from_units(vec![])))
    {
        Vec::<self::ml_string::Text>::new()
    } else {
        {
            let row = 0f64;
            {
                let source_ml_source1_s23 =
                    self::source_ml_temporary_read_rust_integer_range_cuts_loop5(
                        minimum, maximum, lowers, uppers, row,
                    );
                match source_ml_source1_s23 {
                    self::MlTemporaryReadRustIntegerRangeCutsLoop5Result::MlTemporaryReadRustIntegerRangeCutsLoop5Done => {
                        {
                            let result = vec![minimum.clone()];
                            {
                                let current = minimum.clone();
                                {
                                    self::source_ml_temporary_read_rust_integer_range_cuts_loop6(maximum, lowers, uppers, result, current)
                                }
                            }
                        }
                    }
                    self::MlTemporaryReadRustIntegerRangeCutsLoop5Result::MlTemporaryReadRustIntegerRangeCutsLoop5Return(source_ml_temporary_result7) => {
                        source_ml_temporary_result7
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_integer_range_rows(
    kinds: &[self::ml_string::Text],
    lowers: &[self::ml_string::Text],
    uppers: &[self::ml_string::Text],
    cuts: &[self::ml_string::Text],
) -> Vec<f64> {
    if (((kinds.len() as f64) != (lowers.len() as f64))
        || ((kinds.len() as f64) != (uppers.len() as f64)))
        || ((cuts.len() as f64) < 2f64)
    {
        vec![-1f64]
    } else {
        {
            let cell = 0f64;
            {
                let source_ml_source1_s24 =
                    self::source_ml_temporary_plan_rust_integer_range_rows_loop9(cuts, cell);
                match source_ml_source1_s24 {
                    self::MlTemporaryPlanRustIntegerRangeRowsLoop9Result::MlTemporaryPlanRustIntegerRangeRowsLoop9Done => {
                        {
                            let row = 0f64;
                            {
                                let source_ml_source1_s25 = self::source_ml_temporary_plan_rust_integer_range_rows_loop10(kinds, lowers, uppers, cuts, row);
                                match source_ml_source1_s25 {
                                    self::MlTemporaryPlanRustIntegerRangeRowsLoop10Result::MlTemporaryPlanRustIntegerRangeRowsLoop10Done => {
                                        {
                                            let result = vec![0f64, ((cuts.len() as f64) - 1f64)];
                                            {
                                                let cell_2 = 0f64;
                                                {
                                                    let source_ml_source1_s26 = self::source_ml_temporary_plan_rust_integer_range_rows_loop11(kinds, lowers, uppers, cuts, result, cell_2);
                                                    match source_ml_source1_s26 {
                                                        self::MlTemporaryPlanRustIntegerRangeRowsLoop11Result::MlTemporaryPlanRustIntegerRangeRowsLoop11Done(result_2) => {
                                                            result_2
                                                        }
                                                        self::MlTemporaryPlanRustIntegerRangeRowsLoop11Result::MlTemporaryPlanRustIntegerRangeRowsLoop11Return(source_ml_temporary_result11) => {
                                                            source_ml_temporary_result11
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustIntegerRangeRowsLoop10Result::MlTemporaryPlanRustIntegerRangeRowsLoop10Return(source_ml_temporary_result13) => {
                                        source_ml_temporary_result13
                                    }
                                }
                            }
                        }
                    }
                    self::MlTemporaryPlanRustIntegerRangeRowsLoop9Result::MlTemporaryPlanRustIntegerRangeRowsLoop9Return(source_ml_temporary_result13_2) => {
                        source_ml_temporary_result13_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_integer_range_rest_follower(next: &self::ml_string::Text) -> bool {
    ((*next) == (self::ml_string::Text::from_units(vec![44])))
        || ((*next) == (self::ml_string::Text::from_units(vec![41])))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result {
    MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Done,
    MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_rust_integer_range_canonical_decimal_loop1(
    digits: &self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result {
    loop {
        return if at < (digits.length()) {
            {
                let ml_product_condition_1 = {
                    let source_ml_source1_string_search_operand45 =
                        self::ml_string::Text::from_units(vec![
                            48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                        ]);
                    {
                        let source_ml_source1_string_search_operand46 = {
                            let source_ml_source1_string_unit_operand38 = digits.clone();
                            {
                                let source_ml_source1_string_unit_operand39 = at;
                                (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                let index = if position.is_nan() {
                                    0f64
                                } else {
                                    position.trunc()
                                };
                                if (index < 0f64) || (index >= (text.length())) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    {
                                        let source_ml_source1_string_range_operand40 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand41 = index;
                                            {
                                                let source_ml_source1_string_range_operand42 = index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position43 = source_ml_source1_string_range_operand41;
                                                    {
                                                        let source_ml_source1_string_range_position44 = source_ml_source1_string_range_operand42;
                                                        source_ml_source1_string_range_operand40.slice(source_ml_source1_string_range_position43, source_ml_source1_string_range_position44)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } }))(source_ml_source1_string_unit_operand38, source_ml_source1_string_unit_operand39)
                            }
                        };
                        {
                            let source_ml_source1_string_search_text47 =
                                source_ml_source1_string_search_operand46;
                            {
                                let source_ml_source1_string_search_position48 = 0f64;
                                source_ml_source1_string_search_operand45.index_of(
                                    &source_ml_source1_string_search_text47,
                                    source_ml_source1_string_search_position48,
                                )
                            }
                        }
                    }
                };
                if ml_product_condition_1 {
                    self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Return(false)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_rust_integer_range_decimals_loop2(
    a: &self::ml_string::Text,
    b: &self::ml_string::Text,
    mut order: f64,
    mut at: f64,
) -> f64 {
    loop {
        return if at < (a.length()) {
            {
                let x = {
                    let source_ml_source1_string_search_operand56 =
                        self::ml_string::Text::from_units(vec![
                            48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                        ]);
                    {
                        let source_ml_source1_string_search_operand57 = {
                            let source_ml_source1_string_unit_operand49 = a.clone();
                            {
                                let source_ml_source1_string_unit_operand50 = at;
                                (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                    let index = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index < 0f64) || (index >= (text.length())) {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        {
                                            let source_ml_source1_string_range_operand51 = text.clone();
                                            {
                                                let source_ml_source1_string_range_operand52 = index;
                                                {
                                                    let source_ml_source1_string_range_operand53 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position54 = source_ml_source1_string_range_operand52;
                                                        {
                                                            let source_ml_source1_string_range_position55 = source_ml_source1_string_range_operand53;
                                                            source_ml_source1_string_range_operand51.slice(source_ml_source1_string_range_position54, source_ml_source1_string_range_position55)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand49, source_ml_source1_string_unit_operand50)
                            }
                        };
                        {
                            let source_ml_source1_string_search_text58 =
                                source_ml_source1_string_search_operand57;
                            {
                                let source_ml_source1_string_search_position59 = 0f64;
                                source_ml_source1_string_search_operand56.index_of(
                                    &source_ml_source1_string_search_text58,
                                    source_ml_source1_string_search_position59,
                                )
                            }
                        }
                    }
                };
                {
                    let y = {
                        let source_ml_source1_string_search_operand67 =
                            self::ml_string::Text::from_units(vec![
                                48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                            ]);
                        {
                            let source_ml_source1_string_search_operand68 = {
                                let source_ml_source1_string_unit_operand60 = b.clone();
                                {
                                    let source_ml_source1_string_unit_operand61 = at;
                                    (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if (index < 0f64) || (index >= (text.length())) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            {
                                                let source_ml_source1_string_range_operand62 = text.clone();
                                                {
                                                    let source_ml_source1_string_range_operand63 = index;
                                                    {
                                                        let source_ml_source1_string_range_operand64 = index + 1f64;
                                                        {
                                                            let source_ml_source1_string_range_position65 = source_ml_source1_string_range_operand63;
                                                            {
                                                                let source_ml_source1_string_range_position66 = source_ml_source1_string_range_operand64;
                                                                source_ml_source1_string_range_operand62.slice(source_ml_source1_string_range_position65, source_ml_source1_string_range_position66)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } }))(source_ml_source1_string_unit_operand60, source_ml_source1_string_unit_operand61)
                                }
                            };
                            {
                                let source_ml_source1_string_search_text69 =
                                    source_ml_source1_string_search_operand68;
                                {
                                    let source_ml_source1_string_search_position70 = 0f64;
                                    source_ml_source1_string_search_operand67.index_of(
                                        &source_ml_source1_string_search_text69,
                                        source_ml_source1_string_search_position70,
                                    )
                                }
                            }
                        }
                    };
                    if x < y {
                        { -1f64 }
                    } else if x > y {
                        { 1f64 }
                    } else {
                        {
                            let at_2 = at + 1f64;
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            order
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustIntegerRangeSuccessorLoop3Result {
    MlTemporaryReadRustIntegerRangeSuccessorLoop3Done(self::ml_string::Text, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustIntegerRangeSuccessorJoin4 {
    MlTemporaryReadRustIntegerRangeSuccessorJoin4Next(bool, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_integer_range_successor_loop3(
    mut negative: bool,
    digits: &self::ml_string::Text,
    mut result: self::ml_string::Text,
    mut carry: bool,
    mut at: f64,
) -> self::MlTemporaryReadRustIntegerRangeSuccessorLoop3Result {
    loop {
        return if at >= 0f64 {
            {
                let digit = {
                    let source_ml_source1_string_search_operand78 =
                        self::ml_string::Text::from_units(vec![
                            48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                        ]);
                    {
                        let source_ml_source1_string_search_operand79 = {
                            let source_ml_source1_string_unit_operand71 = digits.clone();
                            {
                                let source_ml_source1_string_unit_operand72 = at;
                                (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                    let index = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index < 0f64) || (index >= (text.length())) {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        {
                                            let source_ml_source1_string_range_operand73 = text.clone();
                                            {
                                                let source_ml_source1_string_range_operand74 = index;
                                                {
                                                    let source_ml_source1_string_range_operand75 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position76 = source_ml_source1_string_range_operand74;
                                                        {
                                                            let source_ml_source1_string_range_position77 = source_ml_source1_string_range_operand75;
                                                            source_ml_source1_string_range_operand73.slice(source_ml_source1_string_range_position76, source_ml_source1_string_range_position77)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand71, source_ml_source1_string_unit_operand72)
                            }
                        };
                        {
                            let source_ml_source1_string_search_text80 =
                                source_ml_source1_string_search_operand79;
                            {
                                let source_ml_source1_string_search_position81 = 0f64;
                                source_ml_source1_string_search_operand78.index_of(
                                    &source_ml_source1_string_search_text80,
                                    source_ml_source1_string_search_position81,
                                )
                            }
                        }
                    }
                };
                {
                    let source_ml_source1_s82 = if carry {
                        if negative {
                            if digit == 0f64 {
                                {
                                    let digit_2 = 9f64;
                                    self::MlTemporaryReadRustIntegerRangeSuccessorJoin4::MlTemporaryReadRustIntegerRangeSuccessorJoin4Next(carry, digit_2)
                                }
                            } else {
                                {
                                    let digit_3 = digit - 1f64;
                                    {
                                        let carry_2 = false;
                                        self::MlTemporaryReadRustIntegerRangeSuccessorJoin4::MlTemporaryReadRustIntegerRangeSuccessorJoin4Next(carry_2, digit_3)
                                    }
                                }
                            }
                        } else if digit == 9f64 {
                            {
                                let digit_4 = 0f64;
                                self::MlTemporaryReadRustIntegerRangeSuccessorJoin4::MlTemporaryReadRustIntegerRangeSuccessorJoin4Next(carry, digit_4)
                            }
                        } else {
                            {
                                let digit_5 = digit + 1f64;
                                {
                                    let carry_3 = false;
                                    self::MlTemporaryReadRustIntegerRangeSuccessorJoin4::MlTemporaryReadRustIntegerRangeSuccessorJoin4Next(carry_3, digit_5)
                                }
                            }
                        }
                    } else {
                        self::MlTemporaryReadRustIntegerRangeSuccessorJoin4::MlTemporaryReadRustIntegerRangeSuccessorJoin4Next(carry, digit)
                    };
                    match source_ml_source1_s82 {
                        self::MlTemporaryReadRustIntegerRangeSuccessorJoin4::MlTemporaryReadRustIntegerRangeSuccessorJoin4Next(carry_4, digit_6) => {
                            {
                                let result_2 = ({
                                    let source_ml_source1_string_unit_operand83 = self::ml_string::Text::from_units(vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57]);
                                    {
                                        let source_ml_source1_string_unit_operand84 = digit_6;
                                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                            let index = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if (index < 0f64) || (index >= (text.length())) {
                                                self::ml_string::Text::from_units(vec![])
                                            } else {
                                                {
                                                    let source_ml_source1_string_range_operand85 = text.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand86 = index;
                                                        {
                                                            let source_ml_source1_string_range_operand87 = index + 1f64;
                                                            {
                                                                let source_ml_source1_string_range_position88 = source_ml_source1_string_range_operand86;
                                                                {
                                                                    let source_ml_source1_string_range_position89 = source_ml_source1_string_range_operand87;
                                                                    source_ml_source1_string_range_operand85.slice(source_ml_source1_string_range_position88, source_ml_source1_string_range_position89)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } }))(source_ml_source1_string_unit_operand83, source_ml_source1_string_unit_operand84)
                                    }
                                }).concat(&result);
                                {
                                    let at_2 = at - 1f64;
                                    {
                                        (result, carry, at) = (result_2, carry_4, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustIntegerRangeSuccessorLoop3Result::MlTemporaryReadRustIntegerRangeSuccessorLoop3Done(result.clone(), carry)
        };
    }
}
