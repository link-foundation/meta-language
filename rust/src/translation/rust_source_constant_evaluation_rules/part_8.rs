// Generated from complete js/src/translation/rust-source-constant-evaluation-rules.js. Do not edit by hand.
use super::compare_rust_integer_range_decimals;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_required_text_fact;
use super::read_rust_integer_range_bounds_error;
use super::read_rust_integer_range_successor;

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
                    let source_ml_source1_string_search_operand119 =
                        self::ml_string::Text::from_units(vec![
                            48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                        ]);
                    {
                        let source_ml_source1_string_search_operand120 = {
                            let source_ml_source1_string_unit_operand112 = a.clone();
                            {
                                let source_ml_source1_string_unit_operand113 = at;
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
                                            let source_ml_source1_string_range_operand114 = text.clone();
                                            {
                                                let source_ml_source1_string_range_operand115 = index;
                                                {
                                                    let source_ml_source1_string_range_operand116 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position117 = source_ml_source1_string_range_operand115;
                                                        {
                                                            let source_ml_source1_string_range_position118 = source_ml_source1_string_range_operand116;
                                                            source_ml_source1_string_range_operand114.slice(source_ml_source1_string_range_position117, source_ml_source1_string_range_position118)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand112, source_ml_source1_string_unit_operand113)
                            }
                        };
                        {
                            let source_ml_source1_string_search_text121 =
                                source_ml_source1_string_search_operand120;
                            {
                                let source_ml_source1_string_search_position122 = 0f64;
                                source_ml_source1_string_search_operand119.index_of(
                                    &source_ml_source1_string_search_text121,
                                    source_ml_source1_string_search_position122,
                                )
                            }
                        }
                    }
                };
                {
                    let y = {
                        let source_ml_source1_string_search_operand130 =
                            self::ml_string::Text::from_units(vec![
                                48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                            ]);
                        {
                            let source_ml_source1_string_search_operand131 = {
                                let source_ml_source1_string_unit_operand123 = b.clone();
                                {
                                    let source_ml_source1_string_unit_operand124 = at;
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
                                                let source_ml_source1_string_range_operand125 = text.clone();
                                                {
                                                    let source_ml_source1_string_range_operand126 = index;
                                                    {
                                                        let source_ml_source1_string_range_operand127 = index + 1f64;
                                                        {
                                                            let source_ml_source1_string_range_position128 = source_ml_source1_string_range_operand126;
                                                            {
                                                                let source_ml_source1_string_range_position129 = source_ml_source1_string_range_operand127;
                                                                source_ml_source1_string_range_operand125.slice(source_ml_source1_string_range_position128, source_ml_source1_string_range_position129)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } }))(source_ml_source1_string_unit_operand123, source_ml_source1_string_unit_operand124)
                                }
                            };
                            {
                                let source_ml_source1_string_search_text132 =
                                    source_ml_source1_string_search_operand131;
                                {
                                    let source_ml_source1_string_search_position133 = 0f64;
                                    source_ml_source1_string_search_operand130.index_of(
                                        &source_ml_source1_string_search_text132,
                                        source_ml_source1_string_search_position133,
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
                    let source_ml_source1_string_search_operand141 =
                        self::ml_string::Text::from_units(vec![
                            48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                        ]);
                    {
                        let source_ml_source1_string_search_operand142 = {
                            let source_ml_source1_string_unit_operand134 = digits.clone();
                            {
                                let source_ml_source1_string_unit_operand135 = at;
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
                                            let source_ml_source1_string_range_operand136 = text.clone();
                                            {
                                                let source_ml_source1_string_range_operand137 = index;
                                                {
                                                    let source_ml_source1_string_range_operand138 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position139 = source_ml_source1_string_range_operand137;
                                                        {
                                                            let source_ml_source1_string_range_position140 = source_ml_source1_string_range_operand138;
                                                            source_ml_source1_string_range_operand136.slice(source_ml_source1_string_range_position139, source_ml_source1_string_range_position140)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand134, source_ml_source1_string_unit_operand135)
                            }
                        };
                        {
                            let source_ml_source1_string_search_text143 =
                                source_ml_source1_string_search_operand142;
                            {
                                let source_ml_source1_string_search_position144 = 0f64;
                                source_ml_source1_string_search_operand141.index_of(
                                    &source_ml_source1_string_search_text143,
                                    source_ml_source1_string_search_position144,
                                )
                            }
                        }
                    }
                };
                {
                    let source_ml_source1_s145 = if carry {
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
                    match source_ml_source1_s145 {
                        self::MlTemporaryReadRustIntegerRangeSuccessorJoin4::MlTemporaryReadRustIntegerRangeSuccessorJoin4Next(carry_4, digit_6) => {
                            {
                                let result_2 = ({
                                    let source_ml_source1_string_unit_operand146 = self::ml_string::Text::from_units(vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57]);
                                    {
                                        let source_ml_source1_string_unit_operand147 = digit_6;
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
                                                    let source_ml_source1_string_range_operand148 = text.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand149 = index;
                                                        {
                                                            let source_ml_source1_string_range_operand150 = index + 1f64;
                                                            {
                                                                let source_ml_source1_string_range_position151 = source_ml_source1_string_range_operand149;
                                                                {
                                                                    let source_ml_source1_string_range_position152 = source_ml_source1_string_range_operand150;
                                                                    source_ml_source1_string_range_operand148.slice(source_ml_source1_string_range_position151, source_ml_source1_string_range_position152)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } }))(source_ml_source1_string_unit_operand146, source_ml_source1_string_unit_operand147)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_integer_range_cuts_loop7(
    lowers: &[self::ml_string::Text],
    uppers: &[self::ml_string::Text],
    current: &self::ml_string::Text,
    mut next: self::ml_string::Text,
    mut row: f64,
) -> self::ml_string::Text {
    loop {
        if row < (lowers.len() as f64) {
            {
                let lower = self::read_required_text_fact(lowers, row);
                {
                    let upper = self::read_required_text_fact(uppers, row);
                    {
                        let next_3 = if ((lower != (self::ml_string::Text::from_units(vec![])))
                            && ((self::compare_rust_integer_range_decimals(&(lower), current))
                                > 0f64))
                            && ((self::compare_rust_integer_range_decimals(&(lower), &(next)))
                                < 0f64)
                        {
                            { lower.clone() }
                        } else {
                            next.clone()
                        };
                        if ((upper != (self::ml_string::Text::from_units(vec![])))
                            && ((self::compare_rust_integer_range_decimals(&(upper), current))
                                > 0f64))
                            && ((self::compare_rust_integer_range_decimals(&(upper), &(next_3)))
                                < 0f64)
                        {
                            {
                                let next_4 = upper.clone();
                                {
                                    let row_2 = row + 1f64;
                                    {
                                        (next, row) = (next_4, row_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let row_3 = row + 1f64;
                                {
                                    (next, row) = (next_3.clone(), row_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { next.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_integer_range_cuts_loop6(
    maximum: &self::ml_string::Text,
    lowers: &[self::ml_string::Text],
    uppers: &[self::ml_string::Text],
    mut result: Vec<self::ml_string::Text>,
    mut current: self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    loop {
        if current != (*maximum) {
            {
                let next = maximum.clone();
                {
                    let row = 0f64;
                    {
                        let next_2 = self::source_ml_temporary_read_rust_integer_range_cuts_loop7(
                            lowers,
                            uppers,
                            &(current),
                            next,
                            row,
                        );
                        {
                            let result_2 = {
                                let (ml_append_right, ml_append_left) =
                                    (vec![next_2.clone()], result);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let current_2 = next_2;
                                {
                                    (result, current) = (result_2, current_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustIntegerRangeCutsLoop5Result {
    MlTemporaryReadRustIntegerRangeCutsLoop5Done,
    MlTemporaryReadRustIntegerRangeCutsLoop5Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_integer_range_cuts_loop5(
    minimum: &self::ml_string::Text,
    maximum: &self::ml_string::Text,
    lowers: &[self::ml_string::Text],
    uppers: &[self::ml_string::Text],
    mut row: f64,
) -> self::MlTemporaryReadRustIntegerRangeCutsLoop5Result {
    loop {
        return if row < (lowers.len() as f64) {
            {
                let lower = self::read_required_text_fact(lowers, row);
                {
                    let upper = self::read_required_text_fact(uppers, row);
                    if (lower == (self::ml_string::Text::from_units(vec![])))
                        && (upper == (self::ml_string::Text::from_units(vec![])))
                    {
                        {
                            let row_2 = row + 1f64;
                            {
                                row = row_2;
                                continue;
                            }
                        }
                    }
                    {
                        if (((self::read_rust_integer_range_bounds_error(&(lower), &(upper)))
                            != (self::ml_string::Text::from_units(vec![])))
                            || ((self::compare_rust_integer_range_decimals(&(lower), minimum))
                                < 0f64))
                            || ((self::compare_rust_integer_range_decimals(&(upper), maximum))
                                > 0f64)
                        {
                            self::MlTemporaryReadRustIntegerRangeCutsLoop5Result::MlTemporaryReadRustIntegerRangeCutsLoop5Return(Vec::<self::ml_string::Text>::new())
                        } else {
                            {
                                let row_3 = row + 1f64;
                                {
                                    row = row_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustIntegerRangeCutsLoop5Result::MlTemporaryReadRustIntegerRangeCutsLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustIntegerRangeRowsLoop11Result {
    MlTemporaryPlanRustIntegerRangeRowsLoop11Done(Vec<f64>),
    MlTemporaryPlanRustIntegerRangeRowsLoop11Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_integer_range_rows_loop13(
    mut result: Vec<f64>,
    source_ml_temporary_values8: &[f64],
    mut source_ml_temporary_index8: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_temporary_index8
            < (self::ml::Big::from_u128(source_ml_temporary_values8.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values8,
                    source_ml_temporary_index8.to_index(),
                );
                {
                    let result_2 = {
                        let (ml_append_right, ml_append_left) = (vec![row], result);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let source_ml_temporary_index8_2 =
                            source_ml_temporary_index8.add(&self::ml::Big::from_i128(1));
                        {
                            (result, source_ml_temporary_index8) =
                                (result_2, source_ml_temporary_index8_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustIntegerRangeRowsLoop12Result {
    MlTemporaryPlanRustIntegerRangeRowsLoop12Done(Vec<f64>),
    MlTemporaryPlanRustIntegerRangeRowsLoop12Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_integer_range_rows_loop12(
    kinds: &[self::ml_string::Text],
    lowers: &[self::ml_string::Text],
    uppers: &[self::ml_string::Text],
    lower: &self::ml_string::Text,
    upper: &self::ml_string::Text,
    mut rows: Vec<f64>,
    mut row: f64,
) -> self::MlTemporaryPlanRustIntegerRangeRowsLoop12Result {
    loop {
        return if row < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, row);
                if (kind == (self::ml_string::Text::from_units(vec![119, 105, 108, 100])))
                    || (kind == (self::ml_string::Text::from_units(vec![98, 105, 110, 100])))
                {
                    {
                        let rows_2 = {
                            let (ml_append_right, ml_append_left) = (vec![row], rows);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let row_2 = row + 1f64;
                            {
                                (rows, row) = (rows_2, row_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    if (kind == (self::ml_string::Text::from_units(vec![108, 105, 116])))
                        || (kind
                            == (self::ml_string::Text::from_units(vec![
                                105, 110, 116, 101, 114, 118, 97, 108,
                            ])))
                    {
                        if ((self::compare_rust_integer_range_decimals(
                            &(self::read_required_text_fact(lowers, row)),
                            lower,
                        )) <= 0f64)
                            && ((self::compare_rust_integer_range_decimals(
                                &(self::read_required_text_fact(uppers, row)),
                                upper,
                            )) >= 0f64)
                        {
                            {
                                let rows_3 = {
                                    let (ml_append_right, ml_append_left) = (vec![row], rows);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let row_3 = row + 1f64;
                                    {
                                        (rows, row) = (rows_3, row_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let row_4 = row + 1f64;
                                {
                                    row = row_4;
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        self::MlTemporaryPlanRustIntegerRangeRowsLoop12Result::MlTemporaryPlanRustIntegerRangeRowsLoop12Return(vec![-1f64])
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustIntegerRangeRowsLoop12Result::MlTemporaryPlanRustIntegerRangeRowsLoop12Done(rows)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_integer_range_rows_loop11(
    kinds: &[self::ml_string::Text],
    lowers: &[self::ml_string::Text],
    uppers: &[self::ml_string::Text],
    cuts: &[self::ml_string::Text],
    mut result: Vec<f64>,
    mut cell: f64,
) -> self::MlTemporaryPlanRustIntegerRangeRowsLoop11Result {
    loop {
        return if (cell + 1f64) < (cuts.len() as f64) {
            {
                let lower = self::read_required_text_fact(cuts, cell);
                {
                    let upper = self::read_required_text_fact(cuts, (cell + 1f64));
                    {
                        let rows = Vec::<f64>::new();
                        {
                            let row = 0f64;
                            {
                                let source_ml_source1_s153 =
                                    self::source_ml_temporary_plan_rust_integer_range_rows_loop12(
                                        kinds,
                                        lowers,
                                        uppers,
                                        &(lower),
                                        &(upper),
                                        rows,
                                        row,
                                    );
                                match source_ml_source1_s153 {
                                    self::MlTemporaryPlanRustIntegerRangeRowsLoop12Result::MlTemporaryPlanRustIntegerRangeRowsLoop12Done(rows_2) => {
                                        if (rows_2.len() as f64) == 0f64 {
                                            self::MlTemporaryPlanRustIntegerRangeRowsLoop11Result::MlTemporaryPlanRustIntegerRangeRowsLoop11Return(vec![-2f64, cell])
                                        } else {
                                            {
                                                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(rows_2.len() as f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                                {
                                                    let source_ml_temporary_values8 = rows_2;
                                                    {
                                                        let source_ml_temporary_index8 = self::ml::Big::from_i128(0);
                                                        {
                                                            let result_3 = self::source_ml_temporary_plan_rust_integer_range_rows_loop13(result_2, &(source_ml_temporary_values8), source_ml_temporary_index8);
                                                            {
                                                                let cell_2 = cell + 1f64;
                                                                {
                                                                    (result, cell) = (result_3, cell_2);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustIntegerRangeRowsLoop12Result::MlTemporaryPlanRustIntegerRangeRowsLoop12Return(source_ml_temporary_result13) => {
                                        self::MlTemporaryPlanRustIntegerRangeRowsLoop11Result::MlTemporaryPlanRustIntegerRangeRowsLoop11Return(source_ml_temporary_result13)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustIntegerRangeRowsLoop11Result::MlTemporaryPlanRustIntegerRangeRowsLoop11Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustIntegerRangeRowsLoop10Result {
    MlTemporaryPlanRustIntegerRangeRowsLoop10Done,
    MlTemporaryPlanRustIntegerRangeRowsLoop10Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_integer_range_rows_loop10(
    kinds: &[self::ml_string::Text],
    lowers: &[self::ml_string::Text],
    uppers: &[self::ml_string::Text],
    cuts: &[self::ml_string::Text],
    mut row: f64,
) -> self::MlTemporaryPlanRustIntegerRangeRowsLoop10Result {
    loop {
        return if row < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, row);
                {
                    let lower = self::read_required_text_fact(lowers, row);
                    {
                        let upper = self::read_required_text_fact(uppers, row);
                        if (kind == (self::ml_string::Text::from_units(vec![119, 105, 108, 100])))
                            || (kind
                                == (self::ml_string::Text::from_units(vec![98, 105, 110, 100])))
                        {
                            if (lower != (self::ml_string::Text::from_units(vec![])))
                                || (upper != (self::ml_string::Text::from_units(vec![])))
                            {
                                self::MlTemporaryPlanRustIntegerRangeRowsLoop10Result::MlTemporaryPlanRustIntegerRangeRowsLoop10Return(vec![-1f64])
                            } else {
                                {
                                    let row_2 = row + 1f64;
                                    {
                                        row = row_2;
                                        continue;
                                    }
                                }
                            }
                        } else if (kind == (self::ml_string::Text::from_units(vec![108, 105, 116])))
                            || (kind
                                == (self::ml_string::Text::from_units(vec![
                                    105, 110, 116, 101, 114, 118, 97, 108,
                                ])))
                        {
                            if ((((self::read_rust_integer_range_bounds_error(
                                &(lower),
                                &(upper),
                            )) != (self::ml_string::Text::from_units(vec![])))
                                || !(self::ml_array::includes(
                                    cuts,
                                    &lower.clone(),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                )))
                                || !(self::ml_array::includes(
                                    cuts,
                                    &upper.clone(),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                )))
                                || ((kind
                                    == (self::ml_string::Text::from_units(vec![108, 105, 116])))
                                    && ((self::read_rust_integer_range_successor(&(lower)))
                                        != upper))
                            {
                                self::MlTemporaryPlanRustIntegerRangeRowsLoop10Result::MlTemporaryPlanRustIntegerRangeRowsLoop10Return(vec![-1f64])
                            } else {
                                {
                                    let row_3 = row + 1f64;
                                    {
                                        row = row_3;
                                        continue;
                                    }
                                }
                            }
                        } else {
                            self::MlTemporaryPlanRustIntegerRangeRowsLoop10Result::MlTemporaryPlanRustIntegerRangeRowsLoop10Return(vec![-1f64])
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustIntegerRangeRowsLoop10Result::MlTemporaryPlanRustIntegerRangeRowsLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustIntegerRangeRowsLoop9Result {
    MlTemporaryPlanRustIntegerRangeRowsLoop9Done,
    MlTemporaryPlanRustIntegerRangeRowsLoop9Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_integer_range_rows_loop9(
    cuts: &[self::ml_string::Text],
    mut cell: f64,
) -> self::MlTemporaryPlanRustIntegerRangeRowsLoop9Result {
    loop {
        return if (cell + 1f64) < (cuts.len() as f64) {
            if (self::read_rust_integer_range_bounds_error(
                &(self::read_required_text_fact(cuts, cell)),
                &(self::read_required_text_fact(cuts, (cell + 1f64))),
            )) == (self::ml_string::Text::from_units(vec![]))
            {
                {
                    let cell_2 = cell + 1f64;
                    {
                        cell = cell_2;
                        continue;
                    }
                }
            }
            {
                self::MlTemporaryPlanRustIntegerRangeRowsLoop9Result::MlTemporaryPlanRustIntegerRangeRowsLoop9Return(vec![-1f64])
            }
        } else {
            self::MlTemporaryPlanRustIntegerRangeRowsLoop9Result::MlTemporaryPlanRustIntegerRangeRowsLoop9Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_constant_integer_decimal_loop15(
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
