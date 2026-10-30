// Generated from complete js/src/translation/rust-source-constant-evaluation-rules.js. Do not edit by hand.
use super::constant_element;
use super::constant_text;
use super::ml;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_temporary_read_rust_constant_integer_decimal_loop15;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_constant_integer_decimal_loop14(
    digits: &self::ml_string::Text,
    mut result: self::ml::Big,
    mut at: f64,
) -> self::ml::Big {
    loop {
        if at < (digits.length()) {
            {
                let digit = {
                    let source_ml_source1_string_search_operand157 =
                        self::ml_string::Text::from_units(vec![
                            48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                        ]);
                    {
                        let source_ml_source1_string_search_operand158 = {
                            let source_ml_source1_string_unit_operand150 = digits.clone();
                            {
                                let source_ml_source1_string_unit_operand151 = at;
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
                                            let source_ml_source1_string_range_operand152 = text.clone();
                                            {
                                                let source_ml_source1_string_range_operand153 = index;
                                                {
                                                    let source_ml_source1_string_range_operand154 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position155 = source_ml_source1_string_range_operand153;
                                                        {
                                                            let source_ml_source1_string_range_position156 = source_ml_source1_string_range_operand154;
                                                            source_ml_source1_string_range_operand152.slice(source_ml_source1_string_range_position155, source_ml_source1_string_range_position156)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand150, source_ml_source1_string_unit_operand151)
                            }
                        };
                        {
                            let source_ml_source1_string_search_text159 =
                                source_ml_source1_string_search_operand158;
                            {
                                let source_ml_source1_string_search_position160 = 0f64;
                                source_ml_source1_string_search_operand157.index_of(
                                    &source_ml_source1_string_search_text159,
                                    source_ml_source1_string_search_position160,
                                )
                            }
                        }
                    }
                };
                {
                    let value = self::ml::Big::from_i128(0);
                    {
                        let count = 0f64;
                        {
                            let value_2 =
                                self::source_ml_temporary_read_rust_constant_integer_decimal_loop15(
                                    digit, value, count,
                                );
                            {
                                let result_2 =
                                    (result.mul(&self::ml::Big::from_i128(10))).add(&value_2);
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (result, at) = (result_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_evaluate_rust_constant_integer_operation_loop16(
    mut bits: f64,
    mut modulus: self::ml::Big,
    mut at: f64,
) -> self::ml::Big {
    loop {
        if at < bits {
            {
                let modulus_2 = modulus.mul(&self::ml::Big::from_i128(2));
                {
                    let at_2 = at + 1f64;
                    {
                        (modulus, at) = (modulus_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { modulus.clone() };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryEvaluateRustConstantIntegerOperationJoin17 {
    MlTemporaryEvaluateRustConstantIntegerOperationJoin17Next(self::ml::Big),
    MlTemporaryEvaluateRustConstantIntegerOperationJoin17Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_evaluate_rust_constant_integer_operation_loop18(
    mut bits: f64,
    mut width: self::ml::Big,
    mut at: f64,
) -> self::ml::Big {
    loop {
        if at < bits {
            {
                let width_2 = width.add(&self::ml::Big::from_i128(1));
                {
                    let at_2 = at + 1f64;
                    {
                        (width, at) = (width_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { width.clone() };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryConstantFieldLoop19Result {
    MlTemporaryConstantFieldLoop19Done,
    MlTemporaryConstantFieldLoop19Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_constant_field_loop19(
    labels: &[self::ml_string::Text],
    targets: &[f64],
    name: &self::ml_string::Text,
    mut start: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlTemporaryConstantFieldLoop19Result {
    loop {
        return if at < (start + count) {
            if (self::read_required_text_fact(labels, at)) == (*name) {
                self::MlTemporaryConstantFieldLoop19Result::MlTemporaryConstantFieldLoop19Return(
                    self::read_required_number_fact(targets, at),
                )
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryConstantFieldLoop19Result::MlTemporaryConstantFieldLoop19Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_constant_domain_loop20(
    mut bits: f64,
    mut modulus: self::ml::Big,
    mut at: f64,
) -> self::ml::Big {
    loop {
        if at < bits {
            {
                let modulus_2 = modulus.mul(&self::ml::Big::from_i128(2));
                {
                    let at_2 = at + 1f64;
                    {
                        (modulus, at) = (modulus_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { modulus.clone() };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustSourceConstantIntegerLiteralLoop23Result {
    MlTemporaryReadRustSourceConstantIntegerLiteralLoop23Done(self::ml::Big),
    MlTemporaryReadRustSourceConstantIntegerLiteralLoop23Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_source_constant_integer_literal_loop24(
    mut digit: f64,
    mut add: self::ml::Big,
    mut index: f64,
) -> self::ml::Big {
    loop {
        if index < digit {
            {
                let add_2 = add.add(&self::ml::Big::from_i128(1));
                {
                    let index_2 = index + 1f64;
                    {
                        (add, index) = (add_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { add.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_source_constant_integer_literal_loop23(
    digits: &self::ml_string::Text,
    mut base: self::ml::Big,
    mut limit: f64,
    mut value: self::ml::Big,
    mut at: f64,
) -> self::MlTemporaryReadRustSourceConstantIntegerLiteralLoop23Result {
    loop {
        return if at < (digits.length()) {
            {
                let digit = {
                    let source_ml_source1_string_search_operand168 =
                        self::ml_string::Text::from_units(vec![
                            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 65,
                            66, 67, 68, 69, 70,
                        ]);
                    {
                        let source_ml_source1_string_search_operand169 = {
                            let source_ml_source1_string_unit_operand161 = digits.clone();
                            {
                                let source_ml_source1_string_unit_operand162 = at;
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
                                            let source_ml_source1_string_range_operand163 = text.clone();
                                            {
                                                let source_ml_source1_string_range_operand164 = index;
                                                {
                                                    let source_ml_source1_string_range_operand165 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position166 = source_ml_source1_string_range_operand164;
                                                        {
                                                            let source_ml_source1_string_range_position167 = source_ml_source1_string_range_operand165;
                                                            source_ml_source1_string_range_operand163.slice(source_ml_source1_string_range_position166, source_ml_source1_string_range_position167)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand161, source_ml_source1_string_unit_operand162)
                            }
                        };
                        {
                            let source_ml_source1_string_search_text170 =
                                source_ml_source1_string_search_operand169;
                            {
                                let source_ml_source1_string_search_position171 = 0f64;
                                source_ml_source1_string_search_operand168.index_of(
                                    &source_ml_source1_string_search_text170,
                                    source_ml_source1_string_search_position171,
                                )
                            }
                        }
                    }
                };
                {
                    let digit_3 = if digit >= 16f64 {
                        { digit - 6f64 }
                    } else {
                        digit
                    };
                    if (digit_3 < 0f64) || (digit_3 >= limit) {
                        self::MlTemporaryReadRustSourceConstantIntegerLiteralLoop23Result::MlTemporaryReadRustSourceConstantIntegerLiteralLoop23Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 108, 105, 116, 101, 114, 97, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32, 115, 112, 101, 108, 108, 105, 110, 103])])
                    } else {
                        {
                            let add = self::ml::Big::from_i128(0);
                            {
                                let index = 0f64;
                                {
                                    let add_2 = self::source_ml_temporary_read_rust_source_constant_integer_literal_loop24(digit_3, add.clone(), index);
                                    {
                                        let value_2 = (value.mul(&base)).add(&add_2);
                                        {
                                            let at_2 = at + 1f64;
                                            {
                                                (value, at) = (value_2, at_2);
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
            self::MlTemporaryReadRustSourceConstantIntegerLiteralLoop23Result::MlTemporaryReadRustSourceConstantIntegerLiteralLoop23Done(value.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustSourceConstantIntegerLiteralJoin22 {
    MlTemporaryReadRustSourceConstantIntegerLiteralJoin22Next(
        self::ml_string::Text,
        self::ml::Big,
        f64,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustSourceConstantIntegerLiteralJoin21 {
    MlTemporaryReadRustSourceConstantIntegerLiteralJoin21Next(self::ml_string::Text, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustSourceConstantInitializerBoundaryLoop25Result {
    MlTemporaryReadRustSourceConstantInitializerBoundaryLoop25Done,
    MlTemporaryReadRustSourceConstantInitializerBoundaryLoop25Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_source_constant_initializer_boundary_loop25(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut depth: f64,
    mut at: f64,
) -> self::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop25Result {
    loop {
        return if at < (values.len() as f64) {
            if (self::read_required_text_fact(kinds, at))
                != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))
            {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                {
                    let token = self::read_required_text_fact(values, at);
                    if (token == (self::ml_string::Text::from_units(vec![59]))) && (depth == 0f64) {
                        self::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop25Result::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop25Return(at)
                    } else {
                        if ((token == (self::ml_string::Text::from_units(vec![40])))
                            || (token == (self::ml_string::Text::from_units(vec![91]))))
                            || (token == (self::ml_string::Text::from_units(vec![123])))
                        {
                            {
                                let depth_2 = depth + 1f64;
                                {
                                    let at_3 = at + 1f64;
                                    {
                                        (depth, at) = (depth_2, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if ((token == (self::ml_string::Text::from_units(vec![41])))
                                || (token == (self::ml_string::Text::from_units(vec![93]))))
                                || (token == (self::ml_string::Text::from_units(vec![125])))
                            {
                                if depth == 0f64 {
                                    self::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop25Result::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop25Return(-1f64)
                                } else {
                                    {
                                        let depth_3 = depth - 1f64;
                                        {
                                            let at_4 = at + 1f64;
                                            {
                                                (depth, at) = (depth_3, at_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at_5 = at + 1f64;
                                    {
                                        at = at_5;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop25Result::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop25Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryConstantRowLoop26Result {
    MlTemporaryConstantRowLoop26Done,
    MlTemporaryConstantRowLoop26Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_constant_row_loop26(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut rows: f64,
    owner: &self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryConstantRowLoop26Result {
    loop {
        return if at < (self::read_required_number_fact(counts, rows)) {
            {
                let row = self::constant_element(starts, counts, targets, rows, at);
                if (self::constant_text(
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    row,
                    &(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 114])),
                )) == (*owner)
                {
                    self::MlTemporaryConstantRowLoop26Result::MlTemporaryConstantRowLoop26Return(
                        row,
                    )
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
            self::MlTemporaryConstantRowLoop26Result::MlTemporaryConstantRowLoop26Done
        };
    }
}
