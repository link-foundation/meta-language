// Generated from complete js/src/grammar-inference-rules.js. Do not edit by hand.
use super::MlBuildInferencePrefixTreeJoin5;
use super::MlBuildInferencePrefixTreeLoop3Result;
use super::MlBuildInferencePrefixTreeLoop4Result;
use super::accept_inference_distributions;
use super::accept_inference_proportion;
use super::find_inference_representative;
use super::find_inference_transition;
use super::fold_inference_states;
use super::ml_array;
use super::ml_math;
use super::read_inference_cell;
use super::replace_inference_cell;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(clippy::too_many_arguments, reason = "checked source arguments and captured continuation state retain their original order")]
pub fn ml_build_inference_prefix_tree_loop4(samples: &[f64], mut alphabet_size: f64, mut state_limit: f64, mut table: Vec<f64>, mut position: f64, mut states: f64, mut label: f64, mut length: f64, mut state: f64, mut index: f64) -> self::MlBuildInferencePrefixTreeLoop4Result {
    loop {
        return if index < length {
            {
                let symbol = self::read_inference_cell(samples, (position + index));
                if ((symbol < 0f64) || (symbol >= alphabet_size)) || ((symbol.floor()) != symbol) {
                    self::MlBuildInferencePrefixTreeLoop4Result::MlBuildInferencePrefixTreeLoop4Return(vec![-2f64])
                } else {
                    {
                        let edge = self::find_inference_transition(&(table), state, symbol);
                        {
                            let ml_s11 = if edge < 0f64 {
                                if states >= state_limit {
                                    self::MlBuildInferencePrefixTreeJoin5::MlBuildInferencePrefixTreeJoin5Return(vec![-3f64])
                                } else {
                                    {
                                        let next = table.len() as f64;
                                        {
                                            let table_2 = { let (ml_append_right, ml_append_left) = (vec![0f64, next, 0f64, 0f64, 0f64, 0f64, 0f64, states], table); self::ml_array::append(ml_append_left, ml_append_right) };
                                            {
                                                let edge_2 = table_2.len() as f64;
                                                {
                                                    let table_3 = { let (ml_append_right, ml_append_left) = (vec![1f64, state, symbol, next, 0f64, 0f64, 0f64, 0f64], table_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                    {
                                                        let states_2 = states + 1f64;
                                                        self::MlBuildInferencePrefixTreeJoin5::MlBuildInferencePrefixTreeJoin5Next(table_3, states_2, edge_2)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlBuildInferencePrefixTreeJoin5::MlBuildInferencePrefixTreeJoin5Next(table, states, edge)
                            };
                            match ml_s11 {
                                self::MlBuildInferencePrefixTreeJoin5::MlBuildInferencePrefixTreeJoin5Next(table_4, states_3, edge_3) => {
                                    {
                                        let table_6 = if label == 1f64 {
                                            {
                                                self::replace_inference_cell(&(table_4), (edge_3 + 4f64), (self::read_inference_cell(&(table_4), (edge_3 + 4f64)) + 1f64))
                                            }
                                        } else {
                                            table_4
                                        };
                                        {
                                            let state_2 = self::read_inference_cell(&(table_6), (edge_3 + 3f64));
                                            if label == 1f64 {
                                                {
                                                    let table_7 = self::replace_inference_cell(&(table_6), (state_2 + 4f64), (self::read_inference_cell(&(table_6), (state_2 + 4f64)) + 1f64));
                                                    {
                                                        let index_2 = index + 1f64;
                                                        {
                                                            (table, states, state, index) = (table_7, states_3, state_2, index_2);
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
{
                                                {
                                                    let index_3 = index + 1f64;
                                                    {
                                                        (table, states, state, index) = (table_6, states_3, state_2, index_3);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlBuildInferencePrefixTreeJoin5::MlBuildInferencePrefixTreeJoin5Return(ml_result5) => {
                                    self::MlBuildInferencePrefixTreeLoop4Result::MlBuildInferencePrefixTreeLoop4Return(ml_result5)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlBuildInferencePrefixTreeLoop4Result::MlBuildInferencePrefixTreeLoop4Done(table, states, state)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_build_inference_prefix_tree_loop3(samples: &[f64], mut alphabet_size: f64, mut state_limit: f64, mut table: Vec<f64>, mut position: f64, mut states: f64) -> self::MlBuildInferencePrefixTreeLoop3Result {
    loop {
        return if position < (samples.len() as f64) {
            {
                let label = self::read_inference_cell(samples, position);
                {
                    let length = self::read_inference_cell(samples, (position + 1f64));
                    if ((((label != 0f64) && (label != 1f64)) || (length < 0f64)) || ((length.floor()) != length)) || (((position + 2f64) + length) > (samples.len() as f64)) {
                        self::MlBuildInferencePrefixTreeLoop3Result::MlBuildInferencePrefixTreeLoop3Return(vec![-1f64])
                    } else {
                        {
                            let position_2 = position + 2f64;
                            {
                                let state = 0f64;
                                {
                                    let table_3 = if label == 1f64 {
                                        {
                                            self::replace_inference_cell(&(table), 4f64, (self::read_inference_cell(&(table), 4f64) + 1f64))
                                        }
                                    } else {
                                        table
                                    };
                                    {
                                        let index = 0f64;
                                        {
                                            let ml_s12 = self::ml_build_inference_prefix_tree_loop4(samples, alphabet_size, state_limit, table_3, position_2, states, label, length, state, index);
                                            match ml_s12 {
                                                self::MlBuildInferencePrefixTreeLoop4Result::MlBuildInferencePrefixTreeLoop4Done(table_4, states_2, state_2) => {
                                                    {
                                                        let table_5 = self::replace_inference_cell(&(table_4), (state_2 + if label == 1f64 {
                                                            2f64
                                                        } else {
                                                            3f64
                                                        }), 1f64);
                                                        {
                                                            let table_7 = if label == 1f64 {
                                                                {
                                                                    self::replace_inference_cell(&(table_5), (state_2 + 5f64), (self::read_inference_cell(&(table_5), (state_2 + 5f64)) + 1f64))
                                                                }
                                                            } else {
                                                                table_5
                                                            };
                                                            if ((self::read_inference_cell(&(table_7), (state_2 + 2f64))) == 1f64) && ((self::read_inference_cell(&(table_7), (state_2 + 3f64))) == 1f64) {
                                                                self::MlBuildInferencePrefixTreeLoop3Result::MlBuildInferencePrefixTreeLoop3Return(vec![-4f64])
                                                            } else {
                                                                {
                                                                    let position_3 = position_2 + length;
                                                                    {
                                                                        (table, position, states) = (table_7, position_3, states_2);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlBuildInferencePrefixTreeLoop4Result::MlBuildInferencePrefixTreeLoop4Return(ml_result4) => {
                                                    self::MlBuildInferencePrefixTreeLoop3Result::MlBuildInferencePrefixTreeLoop3Return(ml_result4)
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
            self::MlBuildInferencePrefixTreeLoop3Result::MlBuildInferencePrefixTreeLoop3Done(table)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_approximate_inference_logarithm_loop8(mut square: f64, mut term: f64, mut total: f64, mut index: f64) -> f64 {
    loop {
        if index < 64f64 {
            {
                let total_2 = total + (term / ((2f64 * index) + 1f64));
                {
                    let term_2 = term * square;
                    {
                        let index_2 = index + 1f64;
                        {
                            (term, total, index) = (term_2, total_2, index_2);
                            continue;
                        }
                    }
                }
            }
        }
return {
            total
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlApproximateInferenceLogarithmLoop7Result {
    MlApproximateInferenceLogarithmLoop7Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_approximate_inference_logarithm_loop7(mut scaled: f64, mut exponent: f64) -> self::MlApproximateInferenceLogarithmLoop7Result {
    loop {
        if scaled < 1f64 {
            {
                let scaled_2 = scaled * 2f64;
                {
                    let exponent_2 = exponent - 1f64;
                    {
                        (scaled, exponent) = (scaled_2, exponent_2);
                        continue;
                    }
                }
            }
        }
return {
            self::MlApproximateInferenceLogarithmLoop7Result::MlApproximateInferenceLogarithmLoop7Done(scaled, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlApproximateInferenceLogarithmLoop6Result {
    MlApproximateInferenceLogarithmLoop6Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_approximate_inference_logarithm_loop6(mut scaled: f64, mut exponent: f64) -> self::MlApproximateInferenceLogarithmLoop6Result {
    loop {
        if scaled > 2f64 {
            {
                let scaled_2 = scaled / 2f64;
                {
                    let exponent_2 = exponent + 1f64;
                    {
                        (scaled, exponent) = (scaled_2, exponent_2);
                        continue;
                    }
                }
            }
        }
return {
            self::MlApproximateInferenceLogarithmLoop6Result::MlApproximateInferenceLogarithmLoop6Done(scaled, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptInferenceDistributionsLoop9Result {
    MlAcceptInferenceDistributionsLoop9Done,
    MlAcceptInferenceDistributionsLoop9Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptInferenceDistributionsLoop10Result {
    MlAcceptInferenceDistributionsLoop10Done(Vec<f64>),
    MlAcceptInferenceDistributionsLoop10Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(clippy::too_many_arguments, reason = "checked source arguments and captured continuation state retain their original order")]
pub fn ml_accept_inference_distributions_loop10(table: &[f64], mut bound: f64, mut pending: Vec<f64>, mut left: f64, mut right: f64, mut left_count: f64, mut right_count: f64, mut at: f64) -> self::MlAcceptInferenceDistributionsLoop10Result {
    loop {
        return if at < (table.len() as f64) {
            if (self::read_inference_cell(table, at)) != 1f64 {
                {
                    let at_2 = at + 8f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
{
                {
                    let owner = self::read_inference_cell(table, (at + 1f64));
                    if (owner != left) && (owner != right) {
                        {
                            let at_3 = at + 8f64;
                            {
                                at = at_3;
                                continue;
                            }
                        }
                    }
{
                        {
                            let symbol = self::read_inference_cell(table, (at + 2f64));
                            {
                                let a = self::find_inference_transition(table, left, symbol);
                                {
                                    let b = self::find_inference_transition(table, right, symbol);
                                    if (self::accept_inference_proportion(if a < 0f64 {
                                        0f64
                                    } else {
                                        self::read_inference_cell(table, (a + 4f64))
                                    }, left_count, if b < 0f64 {
                                        0f64
                                    } else {
                                        self::read_inference_cell(table, (b + 4f64))
                                    }, right_count, bound)) {
                                        if (a >= 0f64) && (b >= 0f64) {
                                            {
                                                let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_inference_cell(table, (a + 3f64)), self::read_inference_cell(table, (b + 3f64))], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                                                {
                                                    let at_4 = at + 8f64;
                                                    {
                                                        (pending, at) = (pending_2, at_4);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
{
                                            {
                                                let at_5 = at + 8f64;
                                                {
                                                    at = at_5;
                                                    continue;
                                                }
                                            }
                                        }
                                    }
{
                                        self::MlAcceptInferenceDistributionsLoop10Result::MlAcceptInferenceDistributionsLoop10Return(0f64)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptInferenceDistributionsLoop10Result::MlAcceptInferenceDistributionsLoop10Done(pending)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_inference_distributions_loop9(table: &[f64], mut bound: f64, mut pair_limit: f64, mut pending: Vec<f64>, mut seen: Vec<f64>, mut position: f64) -> self::MlAcceptInferenceDistributionsLoop9Result {
    loop {
        return if position < (pending.len() as f64) {
            if (position / 2f64) >= pair_limit {
                self::MlAcceptInferenceDistributionsLoop9Result::MlAcceptInferenceDistributionsLoop9Return(-5f64)
            } else {
                {
                    let left = self::find_inference_representative(table, self::read_inference_cell(&(pending), position));
                    {
                        let right = self::find_inference_representative(table, self::read_inference_cell(&(pending), (position + 1f64)));
                        {
                            let position_2 = position + 2f64;
                            {
                                let key = (self::ml_math::min(left, right) * (table.len() as f64)) + self::ml_math::max(left, right);
                                if (left == right) || self::ml_array::includes(&seen, &key, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) {
                                    {
                                        position = position_2;
                                        continue;
                                    }
                                }
{
                                    {
                                        let seen_2 = { let (ml_append_right, ml_append_left) = (vec![key], seen); self::ml_array::append(ml_append_left, ml_append_right) };
                                        {
                                            let left_count = self::read_inference_cell(table, (left + 4f64));
                                            {
                                                let right_count = self::read_inference_cell(table, (right + 4f64));
                                                if (self::accept_inference_proportion(self::read_inference_cell(table, (left + 5f64)), left_count, self::read_inference_cell(table, (right + 5f64)), right_count, bound)) {
                                                    {
                                                        let at = 0f64;
                                                        {
                                                            let ml_s13 = self::ml_accept_inference_distributions_loop10(table, bound, pending, left, right, left_count, right_count, at);
                                                            match ml_s13 {
                                                                self::MlAcceptInferenceDistributionsLoop10Result::MlAcceptInferenceDistributionsLoop10Done(pending_2) => {
                                                                    {
                                                                        (pending, seen, position) = (pending_2, seen_2, position_2);
                                                                        continue;
                                                                    }
                                                                }
                                                                self::MlAcceptInferenceDistributionsLoop10Result::MlAcceptInferenceDistributionsLoop10Return(ml_result10) => {
                                                                    self::MlAcceptInferenceDistributionsLoop9Result::MlAcceptInferenceDistributionsLoop9Return(ml_result10)
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    self::MlAcceptInferenceDistributionsLoop9Result::MlAcceptInferenceDistributionsLoop9Return(0f64)
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
            self::MlAcceptInferenceDistributionsLoop9Result::MlAcceptInferenceDistributionsLoop9Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_fold_inference_states_loop12(mut table: Vec<f64>, mut edge: f64) -> Vec<f64> {
    loop {
        if edge < (table.len() as f64) {
            if (self::read_inference_cell(&(table), edge)) == 1f64 {
                {
                    let table_2 = self::replace_inference_cell(&(table), (edge + 3f64), self::find_inference_representative(&(table), self::read_inference_cell(&(table), (edge + 3f64))));
                    {
                        let edge_2 = edge + 8f64;
                        {
                            (table, edge) = (table_2, edge_2);
                            continue;
                        }
                    }
                }
            }
{
                {
                    let edge_3 = edge + 8f64;
                    {
                        edge = edge_3;
                        continue;
                    }
                }
            }
        }
return {
            table
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFoldInferenceStatesLoop11Result {
    MlFoldInferenceStatesLoop11Done(Vec<f64>, f64),
    MlFoldInferenceStatesLoop11Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_fold_inference_states_loop13(mut table: Vec<f64>, mut left: f64, mut right: f64, mut field: f64) -> Vec<f64> {
    loop {
        if field <= 5f64 {
            {
                let table_2 = self::replace_inference_cell(&(table), (left + field), (self::read_inference_cell(&(table), (left + field)) + self::read_inference_cell(&(table), (right + field))));
                {
                    let field_2 = field + 1f64;
                    {
                        (table, field) = (table_2, field_2);
                        continue;
                    }
                }
            }
        }
return {
            table
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFoldInferenceStatesLoop14Result {
    MlFoldInferenceStatesLoop14Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_fold_inference_states_loop14(mut table: Vec<f64>, mut pending: Vec<f64>, mut left: f64, mut right: f64, mut edge: f64) -> self::MlFoldInferenceStatesLoop14Result {
    loop {
        if edge < (table.len() as f64) {
            if ((self::read_inference_cell(&(table), edge)) != 1f64) || ((self::read_inference_cell(&(table), (edge + 1f64))) != right) {
                {
                    let edge_2 = edge + 8f64;
                    {
                        edge = edge_2;
                        continue;
                    }
                }
            }
{
                {
                    let symbol = self::read_inference_cell(&(table), (edge + 2f64));
                    {
                        let other = self::find_inference_transition(&(table), left, symbol);
                        if other < 0f64 {
                            {
                                let table_2 = self::replace_inference_cell(&(table), (edge + 1f64), left);
                                {
                                    let edge_3 = edge + 8f64;
                                    {
                                        (table, edge) = (table_2, edge_3);
                                        continue;
                                    }
                                }
                            }
                        }
{
                            {
                                let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_inference_cell(&(table), (other + 3f64)), self::read_inference_cell(&(table), (edge + 3f64))], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let table_3 = self::replace_inference_cell(&(table), (other + 4f64), (self::read_inference_cell(&(table), (other + 4f64)) + self::read_inference_cell(&(table), (edge + 4f64))));
                                    {
                                        let table_4 = self::replace_inference_cell(&(table_3), edge, -1f64);
                                        {
                                            let edge_4 = edge + 8f64;
                                            {
                                                (table, pending, edge) = (table_4, pending_2, edge_4);
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
return {
            self::MlFoldInferenceStatesLoop14Result::MlFoldInferenceStatesLoop14Done(table, pending)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_fold_inference_states_loop11(mut pair_limit: f64, mut table: Vec<f64>, mut evidence: f64, mut position: f64, mut pending: Vec<f64>) -> self::MlFoldInferenceStatesLoop11Result {
    loop {
        return if position < (pending.len() as f64) {
            if (position / 2f64) >= pair_limit {
                self::MlFoldInferenceStatesLoop11Result::MlFoldInferenceStatesLoop11Return(vec![-5f64])
            } else {
                {
                    let left = self::find_inference_representative(&(table), self::read_inference_cell(&(pending), position));
                    {
                        let right = self::find_inference_representative(&(table), self::read_inference_cell(&(pending), (position + 1f64)));
                        {
                            let position_2 = position + 2f64;
                            if left == right {
                                {
                                    position = position_2;
                                    continue;
                                }
                            }
{
                                {
                                    let accept_left = self::read_inference_cell(&(table), (left + 2f64));
                                    {
                                        let accept_right = self::read_inference_cell(&(table), (right + 2f64));
                                        {
                                            let reject_left = self::read_inference_cell(&(table), (left + 3f64));
                                            {
                                                let reject_right = self::read_inference_cell(&(table), (right + 3f64));
                                                if ((accept_left == 1f64) && (reject_right == 1f64)) || ((reject_left == 1f64) && (accept_right == 1f64)) {
                                                    self::MlFoldInferenceStatesLoop11Result::MlFoldInferenceStatesLoop11Return(Vec::<f64>::new())
                                                } else {
                                                    {
                                                        let evidence_3 = if (accept_left == 1f64) && (accept_right == 1f64) {
                                                            {
                                                                evidence + 1f64
                                                            }
                                                        } else {
                                                            evidence
                                                        };
                                                        {
                                                            let evidence_5 = if (reject_left == 1f64) && (reject_right == 1f64) {
                                                                {
                                                                    evidence_3 + 1f64
                                                                }
                                                            } else {
                                                                evidence_3
                                                            };
                                                            {
                                                                let table_2 = self::replace_inference_cell(&(table), (right + 1f64), left);
                                                                {
                                                                    let table_3 = self::replace_inference_cell(&(table_2), (left + 2f64), self::ml_math::max(accept_left, accept_right));
                                                                    {
                                                                        let table_4 = self::replace_inference_cell(&(table_3), (left + 3f64), self::ml_math::max(reject_left, reject_right));
                                                                        {
                                                                            let field = 4f64;
                                                                            {
                                                                                let table_5 = self::ml_fold_inference_states_loop13(table_4, left, right, field);
                                                                                {
                                                                                    let table_6 = self::replace_inference_cell(&(table_5), (left + 6f64), self::ml_math::max(self::read_inference_cell(&(table_5), (left + 6f64)), self::read_inference_cell(&(table_5), (right + 6f64))));
                                                                                    {
                                                                                        let edge = 0f64;
                                                                                        {
                                                                                            let ml_s14 = self::ml_fold_inference_states_loop14(table_6, pending, left, right, edge);
                                                                                            match ml_s14 {
                                                                                                self::MlFoldInferenceStatesLoop14Result::MlFoldInferenceStatesLoop14Done(table_7, pending_2) => {
                                                                                                    {
                                                                                                        (table, evidence, position, pending) = (table_7, evidence_5, position_2, pending_2);
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
                            }
                        }
                    }
                }
            }
        } else {
            self::MlFoldInferenceStatesLoop11Result::MlFoldInferenceStatesLoop11Done(table, evidence)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inference_blue_states_loop15(table: &[f64], mut next: Vec<f64>, mut edge: f64) -> Vec<f64> {
    loop {
        if edge < (table.len() as f64) {
            if (self::read_inference_cell(table, edge)) != 1f64 {
                {
                    let edge_2 = edge + 8f64;
                    {
                        edge = edge_2;
                        continue;
                    }
                }
            }
{
                {
                    let source = self::find_inference_representative(table, self::read_inference_cell(table, (edge + 1f64)));
                    {
                        let target = self::find_inference_representative(table, self::read_inference_cell(table, (edge + 3f64)));
                        if (((self::read_inference_cell(table, (source + 6f64))) == 1f64) && ((self::read_inference_cell(table, (target + 6f64))) == 0f64)) && !(self::ml_array::includes(&next, &target, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                            {
                                let next_2 = { let (ml_append_right, ml_append_left) = (vec![target], next); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let edge_3 = edge + 8f64;
                                    {
                                        (next, edge) = (next_2, edge_3);
                                        continue;
                                    }
                                }
                            }
                        }
{
                            {
                                let edge_4 = edge + 8f64;
                                {
                                    edge = edge_4;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
return {
            next
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inference_blue_states_loop16(table: &[f64], next: &[f64], mut result: Vec<f64>, mut state: f64) -> Vec<f64> {
    loop {
        if state < (table.len() as f64) {
            if self::ml_array::includes(next, &state, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) {
                {
                    let result_2 = { let (ml_append_right, ml_append_left) = (vec![state], result); self::ml_array::append(ml_append_left, ml_append_right) };
                    {
                        let state_2 = state + 8f64;
                        {
                            (result, state) = (result_2, state_2);
                            continue;
                        }
                    }
                }
            }
{
                {
                    let state_3 = state + 8f64;
                    {
                        state = state_3;
                        continue;
                    }
                }
            }
        }
return {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlInferRegularGrammarTableLoop18Result {
    MlInferRegularGrammarTableLoop18Done(f64, Vec<f64>),
    MlInferRegularGrammarTableLoop18Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlInferRegularGrammarTableLoop19Result {
    MlInferRegularGrammarTableLoop19Done(f64, Vec<f64>, f64),
    MlInferRegularGrammarTableLoop19Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlInferRegularGrammarTableJoin21 {
    MlInferRegularGrammarTableJoin21Next(Vec<f64>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlInferRegularGrammarTableJoin20 {
    MlInferRegularGrammarTableJoin20Next,
    MlInferRegularGrammarTableJoin20Return(Vec<f64>),
    MlInferRegularGrammarTableJoin20Continue,
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(clippy::too_many_arguments, reason = "checked source arguments and captured continuation state retain their original order")]
pub fn ml_infer_regular_grammar_table_loop19(mut strategy: f64, mut pair_limit: f64, mut attempt_limit: f64, table: &[f64], mut attempts: f64, mut bound: f64, mut best: Vec<f64>, mut best_evidence: f64, mut source: f64, mut target: f64) -> self::MlInferRegularGrammarTableLoop19Result {
    loop {
        return if target < (table.len() as f64) {
            if (((self::read_inference_cell(table, target)) != 0f64) || ((self::read_inference_cell(table, (target + 6f64))) != 1f64)) || ((self::find_inference_representative(table, target)) != target) {
                {
                    let target_2 = target + 8f64;
                    {
                        target = target_2;
                        continue;
                    }
                }
            }
{
                {
                    let attempts_2 = attempts + 1f64;
                    if attempts_2 > attempt_limit {
                        self::MlInferRegularGrammarTableLoop19Result::MlInferRegularGrammarTableLoop19Return(vec![-6f64])
                    } else {
                        {
                            let ml_s15 = if strategy == 2f64 {
                                {
                                    let compatible = self::accept_inference_distributions(table, target, source, bound, pair_limit);
                                    if compatible < 0f64 {
                                        self::MlInferRegularGrammarTableJoin20::MlInferRegularGrammarTableJoin20Return(vec![compatible])
                                    } else if compatible == 0f64 {
                                            self::MlInferRegularGrammarTableJoin20::MlInferRegularGrammarTableJoin20Continue
                                        } else {
                                            self::MlInferRegularGrammarTableJoin20::MlInferRegularGrammarTableJoin20Next
                                        }
                                }
                            } else {
                                self::MlInferRegularGrammarTableJoin20::MlInferRegularGrammarTableJoin20Next
                            };
                            match ml_s15 {
                                self::MlInferRegularGrammarTableJoin20::MlInferRegularGrammarTableJoin20Next => {
                                    {
                                        let candidate = self::fold_inference_states(table, target, source, pair_limit);
                                        if (candidate.len() as f64) == 0f64 {
                                            {
                                                let target_3 = target + 8f64;
                                                {
                                                    (attempts, target) = (attempts_2, target_3);
                                                    continue;
                                                }
                                            }
                                        }
{
                                            if (self::read_inference_cell(&(candidate), 0f64)) < 0f64 {
                                                self::MlInferRegularGrammarTableLoop19Result::MlInferRegularGrammarTableLoop19Return(candidate)
                                            } else {
                                                {
                                                    let score = self::read_inference_cell(&(candidate), ((candidate.len() as f64) - 5f64));
                                                    {
                                                        let ml_s16 = if score > best_evidence {
                                                            {
                                                                let best_2 = candidate;
                                                                {
                                                                    let best_evidence_2 = score;
                                                                    self::MlInferRegularGrammarTableJoin21::MlInferRegularGrammarTableJoin21Next(best_2, best_evidence_2)
                                                                }
                                                            }
                                                        } else {
                                                            self::MlInferRegularGrammarTableJoin21::MlInferRegularGrammarTableJoin21Next(best, best_evidence)
                                                        };
                                                        match ml_s16 {
                                                            self::MlInferRegularGrammarTableJoin21::MlInferRegularGrammarTableJoin21Next(best_3, best_evidence_3) => {
                                                                {
let ml_library_inequality_1 = strategy != 1f64 ;
if ml_library_inequality_1 {
                                                                    self::MlInferRegularGrammarTableLoop19Result::MlInferRegularGrammarTableLoop19Done(attempts_2, best_3, best_evidence_3)
                                                                } else {
                                                                    {
                                                                        let target_4 = target + 8f64;
                                                                        {
                                                                            (attempts, best, best_evidence, target) = (attempts_2, best_3, best_evidence_3, target_4);
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
                                self::MlInferRegularGrammarTableJoin20::MlInferRegularGrammarTableJoin20Return(ml_result21) => {
                                    self::MlInferRegularGrammarTableLoop19Result::MlInferRegularGrammarTableLoop19Return(ml_result21)
                                }
                                self::MlInferRegularGrammarTableJoin20::MlInferRegularGrammarTableJoin20Continue => {
                                    {
                                        let target_5 = target + 8f64;
                                        {
                                            (attempts, target) = (attempts_2, target_5);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlInferRegularGrammarTableLoop19Result::MlInferRegularGrammarTableLoop19Done(attempts, best, best_evidence)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(clippy::too_many_arguments, reason = "checked source arguments and captured continuation state retain their original order")]
pub fn ml_infer_regular_grammar_table_loop18(mut strategy: f64, mut pair_limit: f64, mut attempt_limit: f64, table: &[f64], mut attempts: f64, mut bound: f64, blue: &[f64], mut best: Vec<f64>, mut best_evidence: f64, mut index: f64) -> self::MlInferRegularGrammarTableLoop18Result {
    loop {
        return if index < (blue.len() as f64) {
            if (strategy != 1f64) && (index > 0f64) {
                self::MlInferRegularGrammarTableLoop18Result::MlInferRegularGrammarTableLoop18Done(attempts, best)
            } else {
                {
                    let source = self::read_inference_cell(blue, index);
                    {
                        let target = 0f64;
                        {
                            let ml_s17 = self::ml_infer_regular_grammar_table_loop19(strategy, pair_limit, attempt_limit, table, attempts, bound, best, best_evidence, source, target);
                            match ml_s17 {
                                self::MlInferRegularGrammarTableLoop19Result::MlInferRegularGrammarTableLoop19Done(attempts_2, best_2, best_evidence_2) => {
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (attempts, best, best_evidence, index) = (attempts_2, best_2, best_evidence_2, index_2);
                                            continue;
                                        }
                                    }
                                }
                                self::MlInferRegularGrammarTableLoop19Result::MlInferRegularGrammarTableLoop19Return(ml_result19) => {
                                    self::MlInferRegularGrammarTableLoop18Result::MlInferRegularGrammarTableLoop18Return(ml_result19)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlInferRegularGrammarTableLoop18Result::MlInferRegularGrammarTableLoop18Done(attempts, best)
        };
    }
}
