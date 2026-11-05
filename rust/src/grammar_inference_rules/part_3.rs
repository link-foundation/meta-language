// Generated from complete js/src/grammar-inference-rules.js. Do not edit by hand.
use super::MlInferRegularGrammarTableLoop18Result;
use super::find_inference_representative;
use super::find_inference_transition;
use super::ml;
use super::ml_array;
use super::ml_infer_regular_grammar_table_loop18;
use super::ml_math;
use super::read_inference_blue_states;
use super::read_inference_cell;
use super::replace_inference_cell;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_infer_regular_grammar_table_loop17(mut strategy: f64, mut pair_limit: f64, mut attempt_limit: f64, mut table: Vec<f64>, mut attempts: f64, mut bound: f64) -> Vec<f64> {
    loop {
        return {
            let blue = self::read_inference_blue_states(&(table));
            if (blue.len() as f64) == 0f64 {
                table
            } else {
                {
                    let best = Vec::<f64>::new();
                    {
                        let best_evidence = -1f64;
                        {
                            let index = 0f64;
                            {
                                let ml_s18 = self::ml_infer_regular_grammar_table_loop18(strategy, pair_limit, attempt_limit, &(table), attempts, bound, &(blue), best, best_evidence, index);
                                match ml_s18 {
                                    self::MlInferRegularGrammarTableLoop18Result::MlInferRegularGrammarTableLoop18Done(attempts_2, best_2) => {
                                        if (best_2.len() as f64) > 0f64 {
                                            {
                                                let table_2 = best_2;
                                                {
                                                    (table, attempts) = (table_2, attempts_2);
                                                    continue;
                                                }
                                            }
                                        }
{
                                            {
                                                let table_3 = self::replace_inference_cell(&(table), (self::read_inference_cell(&(blue), 0f64) + 6f64), 1f64);
                                                {
                                                    (table, attempts) = (table_3, attempts_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlInferRegularGrammarTableLoop18Result::MlInferRegularGrammarTableLoop18Return(ml_result18) => {
                                        ml_result18
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptInferredSymbolSequenceLoop23Result {
    MlAcceptInferredSymbolSequenceLoop23Done(f64),
    MlAcceptInferredSymbolSequenceLoop23Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_inferred_symbol_sequence_loop23(table: &[f64], mut state: f64, ml_values22: &[f64], mut ml_index22: self::ml::Big) -> self::MlAcceptInferredSymbolSequenceLoop23Result {
    loop {
        return if ml_index22 < (self::ml::Big::from_u128(ml_values22.len() as u128)) {
            {
                let symbol = self::ml_array::at(ml_values22, ml_index22.to_index());
                {
                    let edge = self::find_inference_transition(table, state, symbol);
                    if edge < 0f64 {
                        self::MlAcceptInferredSymbolSequenceLoop23Result::MlAcceptInferredSymbolSequenceLoop23Return(false)
                    } else {
                        {
                            let state_2 = self::find_inference_representative(table, self::read_inference_cell(table, (edge + 3f64)));
                            {
                                let ml_index22_2 = ml_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (state, ml_index22) = (state_2, ml_index22_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptInferredSymbolSequenceLoop23Result::MlAcceptInferredSymbolSequenceLoop23Done(state)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptInferenceTableShapeLoop26Result {
    MlAcceptInferenceTableShapeLoop26Done,
    MlAcceptInferenceTableShapeLoop26Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptInferenceTableShapeLoop28Result {
    MlAcceptInferenceTableShapeLoop28Done,
    MlAcceptInferenceTableShapeLoop28Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_inference_table_shape_loop28(table: &[f64], mut at: f64, mut source: f64, mut symbol: f64, mut other: f64) -> self::MlAcceptInferenceTableShapeLoop28Result {
    loop {
        return if other < at {
            if (((self::read_inference_cell(table, other)) == 1f64) && ((self::read_inference_cell(table, (other + 1f64))) == source)) && ((self::read_inference_cell(table, (other + 2f64))) == symbol) {
                self::MlAcceptInferenceTableShapeLoop28Result::MlAcceptInferenceTableShapeLoop28Return(false)
            } else {
                {
                    let other_2 = other + 8f64;
                    {
                        other = other_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlAcceptInferenceTableShapeLoop28Result::MlAcceptInferenceTableShapeLoop28Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptInferenceTableShapeJoin27 {
    MlAcceptInferenceTableShapeJoin27Next,
    MlAcceptInferenceTableShapeJoin27Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptInferenceTableShapeLoop29Result {
    MlAcceptInferenceTableShapeLoop29Done(bool),
    MlAcceptInferenceTableShapeLoop29Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_inference_table_shape_loop29(table: &[f64], mut current: f64, mut complete: bool, mut count: f64) -> self::MlAcceptInferenceTableShapeLoop29Result {
    loop {
        return if count < (table.len() as f64) {
            {
                let next = self::read_inference_cell(table, (current + 1f64));
                if (((next < 0f64) || (next >= (table.len() as f64))) || ((next % 8f64) != 0f64)) || ((self::read_inference_cell(table, next)) != 0f64) {
                    self::MlAcceptInferenceTableShapeLoop29Result::MlAcceptInferenceTableShapeLoop29Return(false)
                } else if (next).partial_cmp(&(current)) == Some(std::cmp::Ordering::Equal) {
                        {
                            let complete_2 = true;
                            self::MlAcceptInferenceTableShapeLoop29Result::MlAcceptInferenceTableShapeLoop29Done(complete_2)
                        }
                    } else {
                        {
                            let current_2 = next;
                            {
                                let count_2 = count + 8f64;
                                {
                                    (current, count) = (current_2, count_2);
                                    continue;
                                }
                            }
                        }
                    }
            }
        } else {
            self::MlAcceptInferenceTableShapeLoop29Result::MlAcceptInferenceTableShapeLoop29Done(complete)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_inference_table_shape_loop26(table: &[f64], mut alphabet_size: f64, mut at: f64) -> self::MlAcceptInferenceTableShapeLoop26Result {
    loop {
        return if at < (table.len() as f64) {
            {
                let kind = self::read_inference_cell(table, at);
                if (((kind != -1f64) && (kind != 0f64)) && (kind != 1f64)) && (kind != 2f64) {
                    self::MlAcceptInferenceTableShapeLoop26Result::MlAcceptInferenceTableShapeLoop26Return(false)
                } else {
                    {
                        let ml_s20 = if kind == 0f64 {
                            {
                                let parent = self::read_inference_cell(table, (at + 1f64));
                                if (((parent < 0f64) || (parent >= (table.len() as f64))) || ((parent % 8f64) != 0f64)) || ((self::read_inference_cell(table, parent)) != 0f64) {
                                    self::MlAcceptInferenceTableShapeJoin27::MlAcceptInferenceTableShapeJoin27Return(false)
                                } else {
                                    {
                                        let accept = self::read_inference_cell(table, (at + 2f64));
                                        {
                                            let reject = self::read_inference_cell(table, (at + 3f64));
                                            {
                                                let red = self::read_inference_cell(table, (at + 6f64));
                                                if (
 ((((accept != 0f64) && (accept != 1f64)) || ((reject != 0f64) && (reject != 1f64))) || ((accept + reject) > 1f64)) || ((red != 0f64) && (red != 1f64)) 
) || (
 (((self::read_inference_cell(table, (at + 4f64))) < 0f64) || ((self::read_inference_cell(table, (at + 5f64))) < 0f64)) || ((self::read_inference_cell(table, (at + 7f64))) < 0f64) 
) {
                                                        self::MlAcceptInferenceTableShapeJoin27::MlAcceptInferenceTableShapeJoin27Return(false)
                                                    } else {
                                                        {
                                                            let current = at;
                                                            {
                                                                let complete = false;
                                                                {
                                                                    let count = 0f64;
                                                                    {
                                                                        let ml_s21 = self::ml_accept_inference_table_shape_loop29(table, current, complete, count);
                                                                        match ml_s21 {
                                                                            self::MlAcceptInferenceTableShapeLoop29Result::MlAcceptInferenceTableShapeLoop29Done(complete_2) => {
                                                                                if complete_2 {
                                                                                    self::MlAcceptInferenceTableShapeJoin27::MlAcceptInferenceTableShapeJoin27Next
                                                                                } else {
                                                                                    self::MlAcceptInferenceTableShapeJoin27::MlAcceptInferenceTableShapeJoin27Return(false)
                                                                                }
                                                                            }
                                                                            self::MlAcceptInferenceTableShapeLoop29Result::MlAcceptInferenceTableShapeLoop29Return(ml_result29) => {
                                                                                self::MlAcceptInferenceTableShapeJoin27::MlAcceptInferenceTableShapeJoin27Return(ml_result29)
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
                            self::MlAcceptInferenceTableShapeJoin27::MlAcceptInferenceTableShapeJoin27Next
                        };
                        match ml_s20 {
                            self::MlAcceptInferenceTableShapeJoin27::MlAcceptInferenceTableShapeJoin27Next => {
                                if (kind == 1f64) || (kind == 2f64) {
                                    {
                                        let source = self::read_inference_cell(table, (at + 1f64));
                                        {
                                            let target = self::read_inference_cell(table, (at + if kind == 1f64 {
                                                3f64
                                            } else {
                                                2f64
                                            }));
                                            if (((((((source < 0f64) || (target < 0f64)) || (source >= (table.len() as f64))) || (target >= (table.len() as f64))) || ((source % 8f64) != 0f64)) || ((target % 8f64) != 0f64)) || ((self::read_inference_cell(table, source)) != 0f64)) || ((self::read_inference_cell(table, target)) != 0f64) {
                                                self::MlAcceptInferenceTableShapeLoop26Result::MlAcceptInferenceTableShapeLoop26Return(false)
                                            } else if kind == 1f64 {
                                                    {
                                                        let symbol = self::read_inference_cell(table, (at + 2f64));
                                                        if ((((symbol < 0f64) || (symbol >= alphabet_size)) || ((self::read_inference_cell(table, (at + 4f64))) < 0f64)) || ((self::read_inference_cell(table, (source + 1f64))) != source)) || ((self::read_inference_cell(table, (target + 1f64))) != target) {
                                                            self::MlAcceptInferenceTableShapeLoop26Result::MlAcceptInferenceTableShapeLoop26Return(false)
                                                        } else {
                                                            {
                                                                let other = 0f64;
                                                                {
                                                                    let ml_s22 = self::ml_accept_inference_table_shape_loop28(table, at, source, symbol, other);
                                                                    match ml_s22 {
                                                                        self::MlAcceptInferenceTableShapeLoop28Result::MlAcceptInferenceTableShapeLoop28Done => {
                                                                            {
                                                                                let at_2 = at + 8f64;
                                                                                {
                                                                                    at = at_2;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlAcceptInferenceTableShapeLoop28Result::MlAcceptInferenceTableShapeLoop28Return(ml_result28) => {
                                                                            self::MlAcceptInferenceTableShapeLoop26Result::MlAcceptInferenceTableShapeLoop26Return(ml_result28)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else if (self::read_inference_cell(table, (at + 3f64))) < 0f64 {
                                                        self::MlAcceptInferenceTableShapeLoop26Result::MlAcceptInferenceTableShapeLoop26Return(false)
                                                    } else {
                                                        {
                                                            let at_3 = at + 8f64;
                                                            {
                                                                at = at_3;
                                                                continue;
                                                            }
                                                        }
                                                    }
                                        }
                                    }
                                } else {
                                    {
                                        let at_4 = at + 8f64;
                                        {
                                            at = at_4;
                                            continue;
                                        }
                                    }
                                }
                            }
                            self::MlAcceptInferenceTableShapeJoin27::MlAcceptInferenceTableShapeJoin27Return(ml_result28_2) => {
                                self::MlAcceptInferenceTableShapeLoop26Result::MlAcceptInferenceTableShapeLoop26Return(ml_result28_2)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptInferenceTableShapeLoop26Result::MlAcceptInferenceTableShapeLoop26Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptInferenceTableShapeLoop25Result {
    MlAcceptInferenceTableShapeLoop25Done,
    MlAcceptInferenceTableShapeLoop25Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_inference_table_shape_loop25(ml_values24: &[f64], mut ml_index24: self::ml::Big) -> self::MlAcceptInferenceTableShapeLoop25Result {
    loop {
        return if ml_index24 < (self::ml::Big::from_u128(ml_values24.len() as u128)) {
            {
                let value = self::ml_array::at(ml_values24, ml_index24.to_index());
                if !(self::ml_math::is_safe_integer(value)) || (value < -1f64) {
                    self::MlAcceptInferenceTableShapeLoop25Result::MlAcceptInferenceTableShapeLoop25Return(false)
                } else {
                    {
                        let ml_index24_2 = ml_index24.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index24 = ml_index24_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlAcceptInferenceTableShapeLoop25Result::MlAcceptInferenceTableShapeLoop25Done
        };
    }
}
