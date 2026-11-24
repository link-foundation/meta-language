// Generated from complete js/src/translation/rust-source-constant-evaluation-rules.js. Do not edit by hand.
use super::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop5Result;
use super::constant_element;
use super::constant_expression;
use super::constant_remaining;
use super::constant_text;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_rust_integer_range_canonical_decimal_loop5(
    digits: &self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop5Result {
    loop {
        return if at < (digits.length()) {
            {
                let ml_product_condition_1 = {
                    let source_ml_source1_string_search_operand112 =
                        self::ml_string::Text::from_units(vec![
                            48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                        ]);
                    {
                        let source_ml_source1_string_search_operand113 = {
                            let source_ml_source1_string_unit_operand105 = digits.clone();
                            {
                                let source_ml_source1_string_unit_operand106 = at;
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
                                        let source_ml_source1_string_range_operand107 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand108 = index;
                                            {
                                                let source_ml_source1_string_range_operand109 = index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position110 = source_ml_source1_string_range_operand108;
                                                    {
                                                        let source_ml_source1_string_range_position111 = source_ml_source1_string_range_operand109;
                                                        source_ml_source1_string_range_operand107.slice(source_ml_source1_string_range_position110, source_ml_source1_string_range_position111)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } }))(source_ml_source1_string_unit_operand105, source_ml_source1_string_unit_operand106)
                            }
                        };
                        {
                            let source_ml_source1_string_search_text114 =
                                source_ml_source1_string_search_operand113;
                            {
                                let source_ml_source1_string_search_position115 = 0f64;
                                source_ml_source1_string_search_operand112.index_of(
                                    &source_ml_source1_string_search_text114,
                                    source_ml_source1_string_search_position115,
                                )
                            }
                        }
                    }
                };
                if ml_product_condition_1 {
                    self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop5Result::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop5Return(false)
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
            self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop5Result::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop5Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_constant_integer_decimal_loop19(
    mut digit: f64,
    mut value: self::ml::Big,
    mut count: f64,
) -> self::ml::Big {
    loop {
        if count < digit {
            {
                let value_2 = value.add(&self::ml::Big::from_i128(1));
                {
                    let count_2 = count + 1f64;
                    {
                        (value, count) = (value_2, count_2);
                        continue;
                    }
                }
            }
        }
        return { value.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_constant_integer_decimal_loop18(
    digits: &self::ml_string::Text,
    mut result: self::ml::Big,
    mut at: f64,
) -> self::ml::Big {
    loop {
        if at < (digits.length()) {
            {
                let digit = {
                    let source_ml_source1_string_search_operand165 =
                        self::ml_string::Text::from_units(vec![
                            48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                        ]);
                    {
                        let source_ml_source1_string_search_operand166 = {
                            let source_ml_source1_string_unit_operand158 = digits.clone();
                            {
                                let source_ml_source1_string_unit_operand159 = at;
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
                                            let source_ml_source1_string_range_operand160 = text.clone();
                                            {
                                                let source_ml_source1_string_range_operand161 = index;
                                                {
                                                    let source_ml_source1_string_range_operand162 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position163 = source_ml_source1_string_range_operand161;
                                                        {
                                                            let source_ml_source1_string_range_position164 = source_ml_source1_string_range_operand162;
                                                            source_ml_source1_string_range_operand160.slice(source_ml_source1_string_range_position163, source_ml_source1_string_range_position164)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand158, source_ml_source1_string_unit_operand159)
                            }
                        };
                        {
                            let source_ml_source1_string_search_text167 =
                                source_ml_source1_string_search_operand166;
                            {
                                let source_ml_source1_string_search_position168 = 0f64;
                                source_ml_source1_string_search_operand165.index_of(
                                    &source_ml_source1_string_search_text167,
                                    source_ml_source1_string_search_position168,
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
                                self::source_ml_temporary_read_rust_constant_integer_decimal_loop19(
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
pub fn source_ml_temporary_evaluate_rust_constant_integer_operation_loop20(
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
pub enum MlTemporaryEvaluateRustConstantIntegerOperationJoin21 {
    MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(self::ml::Big),
    MlTemporaryEvaluateRustConstantIntegerOperationJoin21Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_evaluate_rust_constant_integer_operation_loop22(
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
pub enum MlTemporaryConstantFieldLoop23Result {
    MlTemporaryConstantFieldLoop23Done,
    MlTemporaryConstantFieldLoop23Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_constant_field_loop23(
    labels: &[self::ml_string::Text],
    targets: &[f64],
    name: &self::ml_string::Text,
    mut start: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlTemporaryConstantFieldLoop23Result {
    loop {
        return if at < (start + count) {
            if (self::read_required_text_fact(labels, at)) == (*name) {
                self::MlTemporaryConstantFieldLoop23Result::MlTemporaryConstantFieldLoop23Return(
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
            self::MlTemporaryConstantFieldLoop23Result::MlTemporaryConstantFieldLoop23Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_constant_domain_loop24(
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
pub enum MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Result {
    MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Done(self::ml::Big),
    MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_source_constant_integer_literal_loop28(
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
pub fn source_ml_temporary_read_rust_source_constant_integer_literal_loop27(
    digits: &self::ml_string::Text,
    mut base: self::ml::Big,
    mut limit: f64,
    mut value: self::ml::Big,
    mut at: f64,
) -> self::MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Result {
    loop {
        return if at < (digits.length()) {
            {
                let digit = {
                    let source_ml_source1_string_search_operand176 =
                        self::ml_string::Text::from_units(vec![
                            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 65,
                            66, 67, 68, 69, 70,
                        ]);
                    {
                        let source_ml_source1_string_search_operand177 = {
                            let source_ml_source1_string_unit_operand169 = digits.clone();
                            {
                                let source_ml_source1_string_unit_operand170 = at;
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
                                            let source_ml_source1_string_range_operand171 = text.clone();
                                            {
                                                let source_ml_source1_string_range_operand172 = index;
                                                {
                                                    let source_ml_source1_string_range_operand173 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position174 = source_ml_source1_string_range_operand172;
                                                        {
                                                            let source_ml_source1_string_range_position175 = source_ml_source1_string_range_operand173;
                                                            source_ml_source1_string_range_operand171.slice(source_ml_source1_string_range_position174, source_ml_source1_string_range_position175)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand169, source_ml_source1_string_unit_operand170)
                            }
                        };
                        {
                            let source_ml_source1_string_search_text178 =
                                source_ml_source1_string_search_operand177;
                            {
                                let source_ml_source1_string_search_position179 = 0f64;
                                source_ml_source1_string_search_operand176.index_of(
                                    &source_ml_source1_string_search_text178,
                                    source_ml_source1_string_search_position179,
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
                        self::MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Result::MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 108, 105, 116, 101, 114, 97, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32, 115, 112, 101, 108, 108, 105, 110, 103])])
                    } else {
                        {
                            let add = self::ml::Big::from_i128(0);
                            {
                                let index = 0f64;
                                {
                                    let add_2 = self::source_ml_temporary_read_rust_source_constant_integer_literal_loop28(digit_3, add.clone(), index);
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
            self::MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Result::MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Done(value.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustSourceConstantIntegerLiteralJoin26 {
    MlTemporaryReadRustSourceConstantIntegerLiteralJoin26Next(
        self::ml_string::Text,
        self::ml::Big,
        f64,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustSourceConstantIntegerLiteralJoin25 {
    MlTemporaryReadRustSourceConstantIntegerLiteralJoin25Next(self::ml_string::Text, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Result {
    MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Done,
    MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_source_constant_initializer_boundary_loop29(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut depth: f64,
    mut at: f64,
) -> self::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Result {
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
                        self::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Result::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Return(at)
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
                                    self::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Result::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Return(-1f64)
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
            self::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Result::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryConstantRowLoop30Result {
    MlTemporaryConstantRowLoop30Done,
    MlTemporaryConstantRowLoop30Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_constant_row_loop30(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut rows: f64,
    owner: &self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryConstantRowLoop30Result {
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
                    self::MlTemporaryConstantRowLoop30Result::MlTemporaryConstantRowLoop30Return(
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
            self::MlTemporaryConstantRowLoop30Result::MlTemporaryConstantRowLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryConstantFunctionLoop31Result {
    MlTemporaryConstantFunctionLoop31Done(
        f64,
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
    ),
    MlTemporaryConstantFunctionLoop31Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_constant_function_loop31(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut rows: f64,
    names: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    visiting: &[self::ml_string::Text],
    mut depth: f64,
    mut parameters: f64,
    mut args: f64,
    mut arity: f64,
    mut available: f64,
    mut bound_names: Vec<self::ml_string::Text>,
    mut bound_values: Vec<self::ml_string::Text>,
    mut at: f64,
) -> self::MlTemporaryConstantFunctionLoop31Result {
    loop {
        return if at < arity {
            {
                let parameter = self::constant_element(starts, counts, targets, parameters, at);
                {
                    let domain = self::constant_text(
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        parameter,
                        &(self::ml_string::Text::from_units(vec![100, 111, 109, 97, 105, 110])),
                    );
                    {
                        let value = self::constant_expression(
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            rows,
                            self::constant_element(starts, counts, targets, args, at),
                            &(domain),
                            names,
                            values,
                            visiting,
                            (depth + 1f64),
                            available,
                        );
                        if (self::read_required_text_fact(&(value), 0f64))
                            == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]))
                        {
                            self::MlTemporaryConstantFunctionLoop31Result::MlTemporaryConstantFunctionLoop31Return(value)
                        } else if (self::read_required_text_fact(&(value), 1f64)) == domain {
                            {
                                let bound_names_2 = {
                                    let source_ml_source1_concat_value180 = bound_names;
                                    {
                                        let source_ml_source1_concat_value181 =
                                            vec![self::constant_text(
                                                texts,
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                parameter,
                                                &(self::ml_string::Text::from_units(vec![
                                                    110, 97, 109, 101,
                                                ])),
                                            )];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                source_ml_source1_concat_value180,
                                            ),
                                            source_ml_source1_concat_value181,
                                        )
                                    }
                                };
                                {
                                    let bound_values_2 = {
                                        let source_ml_source1_concat_value182 = bound_values;
                                        {
                                            let source_ml_source1_concat_value183 =
                                                self::ml_array::slice(&value, 0f64, 3f64);
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source1_concat_value182,
                                                ),
                                                source_ml_source1_concat_value183,
                                            )
                                        }
                                    };
                                    {
                                        let available_2 = self::constant_remaining(&(value));
                                        {
                                            let at_2 = at + 1f64;
                                            {
                                                (available, bound_names, bound_values, at) = (
                                                    available_2,
                                                    bound_names_2,
                                                    bound_values_2,
                                                    at_2,
                                                );
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlTemporaryConstantFunctionLoop31Result::MlTemporaryConstantFunctionLoop31Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114])])
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryConstantFunctionLoop31Result::MlTemporaryConstantFunctionLoop31Done(
                available,
                bound_names,
                bound_values,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryConstantRemainingLoop32Result {
    MlTemporaryConstantRemainingLoop32Done(f64),
    MlTemporaryConstantRemainingLoop32Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_constant_remaining_loop32(
    text: &self::ml_string::Text,
    mut result: f64,
    mut at: f64,
) -> self::MlTemporaryConstantRemainingLoop32Result {
    loop {
        return if at < (text.length()) {
            {
                let digit = {
                    let source_ml_source1_string_search_operand191 =
                        self::ml_string::Text::from_units(vec![
                            48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                        ]);
                    {
                        let source_ml_source1_string_search_operand192 = {
                            let source_ml_source1_string_unit_operand184 = text.clone();
                            {
                                let source_ml_source1_string_unit_operand185 = at;
                                (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                    let index = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index < 0f64) || (index >= (text_2.length())) {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        {
                                            let source_ml_source1_string_range_operand186 = text_2.clone();
                                            {
                                                let source_ml_source1_string_range_operand187 = index;
                                                {
                                                    let source_ml_source1_string_range_operand188 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position189 = source_ml_source1_string_range_operand187;
                                                        {
                                                            let source_ml_source1_string_range_position190 = source_ml_source1_string_range_operand188;
                                                            source_ml_source1_string_range_operand186.slice(source_ml_source1_string_range_position189, source_ml_source1_string_range_position190)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand184, source_ml_source1_string_unit_operand185)
                            }
                        };
                        {
                            let source_ml_source1_string_search_text193 =
                                source_ml_source1_string_search_operand192;
                            {
                                let source_ml_source1_string_search_position194 = 0f64;
                                source_ml_source1_string_search_operand191.index_of(
                                    &source_ml_source1_string_search_text193,
                                    source_ml_source1_string_search_position194,
                                )
                            }
                        }
                    }
                };
                if digit < 0f64 {
                    self::MlTemporaryConstantRemainingLoop32Result::MlTemporaryConstantRemainingLoop32Return(-1f64)
                } else {
                    {
                        let result_2 = (result * 10f64) + digit;
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
        } else {
            self::MlTemporaryConstantRemainingLoop32Result::MlTemporaryConstantRemainingLoop32Done(
                result,
            )
        };
    }
}
