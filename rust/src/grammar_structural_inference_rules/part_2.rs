// Generated from complete js/src/grammar-structural-inference-rules.js. Do not edit by hand.
use super::MlTemporaryInferGrammarStructuralProductionsLoop9Result;
use super::MlTemporaryInferGrammarStructuralProductionsLoop10Result;
use super::ml;
use super::ml_array;
use super::read_grammar_structural_productions;
use super::read_required_number_fact;
use super::test_same_grammar_structural_production;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_infer_grammar_structural_productions_loop10(
    productions: &[Vec<f64>],
    mut accepted: bool,
    source_ml_temporary_values7: &[Vec<f64>],
    mut source_ml_temporary_index7: self::ml::Big,
) -> self::MlTemporaryInferGrammarStructuralProductionsLoop10Result {
    loop {
        return if source_ml_temporary_index7
            < (self::ml::Big::from_u128(source_ml_temporary_values7.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values7,
                    source_ml_temporary_index7.to_index(),
                );
                {
                    let group = self::read_required_number_fact(&(row), 0f64);
                    if group == -2f64 {
                        self::MlTemporaryInferGrammarStructuralProductionsLoop10Result::MlTemporaryInferGrammarStructuralProductionsLoop10Return(vec![row])
                    } else {
                        if (group < 0f64)
                            || !({
                                let ml_values1 = productions.to_vec();
                                let ml_callback1 = {
                                    let row = row.clone();
                                    std::rc::Rc::new(move |previous: Vec<f64>| -> bool {
                                        self::test_same_grammar_structural_production(
                                            &(previous),
                                            &(row),
                                        )
                                    })
                                };
                                ml_values1
                                    .iter()
                                    .cloned()
                                    .enumerate()
                                    .any(|(ml_index1, ml_value1)| ml_callback1(ml_value1))
                            })
                        {
                            {
                                let accepted_2 = false;
                                {
                                    let source_ml_temporary_index7_2 = source_ml_temporary_index7
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (accepted, source_ml_temporary_index7) =
                                            (accepted_2, source_ml_temporary_index7_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let source_ml_temporary_index7_3 =
                                    source_ml_temporary_index7.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_temporary_index7 = source_ml_temporary_index7_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryInferGrammarStructuralProductionsLoop10Result::MlTemporaryInferGrammarStructuralProductionsLoop10Done(accepted)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_infer_grammar_structural_productions_loop9(
    opening_groups: &[f64],
    closing_groups: &[f64],
    mut depth_limit: f64,
    productions: &[Vec<f64>],
    source_ml_temporary_values6: &[Vec<f64>],
    mut source_ml_temporary_index6: self::ml::Big,
) -> self::MlTemporaryInferGrammarStructuralProductionsLoop9Result {
    loop {
        return if source_ml_temporary_index6
            < (self::ml::Big::from_u128(source_ml_temporary_values6.len() as u128))
        {
            {
                let tokens = self::ml_array::at(
                    source_ml_temporary_values6,
                    source_ml_temporary_index6.to_index(),
                );
                {
                    let rows = self::read_grammar_structural_productions(
                        &(tokens),
                        opening_groups,
                        closing_groups,
                        depth_limit,
                    );
                    {
                        let accepted = true;
                        {
                            let source_ml_temporary_values7 = rows;
                            {
                                let source_ml_temporary_index7 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_s5 = self::source_ml_temporary_infer_grammar_structural_productions_loop10(productions, accepted, &(source_ml_temporary_values7), source_ml_temporary_index7);
                                    match source_ml_source1_s5 {
                                        self::MlTemporaryInferGrammarStructuralProductionsLoop10Result::MlTemporaryInferGrammarStructuralProductionsLoop10Done(accepted_2) => {
                                            if accepted_2 {
                                                self::MlTemporaryInferGrammarStructuralProductionsLoop9Result::MlTemporaryInferGrammarStructuralProductionsLoop9Return(vec![vec![-4f64]])
                                            } else {
                                                {
                                                    let source_ml_temporary_index6_2 = source_ml_temporary_index6.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index6 = source_ml_temporary_index6_2;
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryInferGrammarStructuralProductionsLoop10Result::MlTemporaryInferGrammarStructuralProductionsLoop10Return(source_ml_temporary_result10) => {
                                            self::MlTemporaryInferGrammarStructuralProductionsLoop9Result::MlTemporaryInferGrammarStructuralProductionsLoop9Return(source_ml_temporary_result10)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryInferGrammarStructuralProductionsLoop9Result::MlTemporaryInferGrammarStructuralProductionsLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInferGrammarStructuralProductionsLoop8Result {
    MlTemporaryInferGrammarStructuralProductionsLoop8Done(Vec<Vec<f64>>),
    MlTemporaryInferGrammarStructuralProductionsLoop8Return(Vec<Vec<f64>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInferGrammarStructuralProductionsLoop11Result {
    MlTemporaryInferGrammarStructuralProductionsLoop11Done(Vec<Vec<f64>>),
    MlTemporaryInferGrammarStructuralProductionsLoop11Return(Vec<Vec<f64>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_infer_grammar_structural_productions_loop11(
    mut production_limit: f64,
    mut productions: Vec<Vec<f64>>,
    source_ml_temporary_values5: &[Vec<f64>],
    mut source_ml_temporary_index5: self::ml::Big,
) -> self::MlTemporaryInferGrammarStructuralProductionsLoop11Result {
    loop {
        return if source_ml_temporary_index5
            < (self::ml::Big::from_u128(source_ml_temporary_values5.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values5,
                    source_ml_temporary_index5.to_index(),
                );
                {
                    let group = self::read_required_number_fact(&(row), 0f64);
                    if group < 0f64 {
                        self::MlTemporaryInferGrammarStructuralProductionsLoop11Result::MlTemporaryInferGrammarStructuralProductionsLoop11Return(vec![row])
                    } else {
                        {
                            let ml_product_condition_1 = {
                                let ml_values2 = productions.clone();
                                let ml_callback2 = {
                                    let row = row.clone();
                                    std::rc::Rc::new(move |previous: Vec<f64>| -> bool {
                                        self::test_same_grammar_structural_production(
                                            &(previous),
                                            &(row),
                                        )
                                    })
                                };
                                ml_values2
                                    .iter()
                                    .cloned()
                                    .enumerate()
                                    .any(|(ml_index2, ml_value2)| ml_callback2(ml_value2))
                            };
                            if ml_product_condition_1 {
                                {
                                    let source_ml_temporary_index5_3 = source_ml_temporary_index5
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index5 = source_ml_temporary_index5_3;
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if (productions.len() as f64) >= production_limit {
                                self::MlTemporaryInferGrammarStructuralProductionsLoop11Result::MlTemporaryInferGrammarStructuralProductionsLoop11Return(vec![vec![-3f64]])
                            } else {
                                {
                                    let productions_2 = {
                                        let source_ml_source1_concat_value6 = productions;
                                        {
                                            let source_ml_source1_concat_value7 = vec![row];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<Vec<f64>>::new(),
                                                    source_ml_source1_concat_value6,
                                                ),
                                                source_ml_source1_concat_value7,
                                            )
                                        }
                                    };
                                    {
                                        let source_ml_temporary_index5_2 =
                                            source_ml_temporary_index5
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (productions, source_ml_temporary_index5) =
                                                (productions_2, source_ml_temporary_index5_2);
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
            self::MlTemporaryInferGrammarStructuralProductionsLoop11Result::MlTemporaryInferGrammarStructuralProductionsLoop11Done(productions)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_infer_grammar_structural_productions_loop8(
    opening_groups: &[f64],
    closing_groups: &[f64],
    mut depth_limit: f64,
    mut production_limit: f64,
    mut productions: Vec<Vec<f64>>,
    source_ml_temporary_values4: &[Vec<f64>],
    mut source_ml_temporary_index4: self::ml::Big,
) -> self::MlTemporaryInferGrammarStructuralProductionsLoop8Result {
    loop {
        return if source_ml_temporary_index4
            < (self::ml::Big::from_u128(source_ml_temporary_values4.len() as u128))
        {
            {
                let tokens = self::ml_array::at(
                    source_ml_temporary_values4,
                    source_ml_temporary_index4.to_index(),
                );
                {
                    let rows = self::read_grammar_structural_productions(
                        &(tokens),
                        opening_groups,
                        closing_groups,
                        depth_limit,
                    );
                    {
                        let source_ml_temporary_values5 = rows;
                        {
                            let source_ml_temporary_index5 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_s8 = self::source_ml_temporary_infer_grammar_structural_productions_loop11(production_limit, productions, &(source_ml_temporary_values5), source_ml_temporary_index5);
                                match source_ml_source1_s8 {
                                    self::MlTemporaryInferGrammarStructuralProductionsLoop11Result::MlTemporaryInferGrammarStructuralProductionsLoop11Done(productions_2) => {
                                        {
                                            let source_ml_temporary_index4_2 = source_ml_temporary_index4.add(&self::ml::Big::from_i128(1));
                                            {
                                                (productions, source_ml_temporary_index4) = (productions_2, source_ml_temporary_index4_2);
                                                continue;
                                            }
                                        }
                                    }
                                    self::MlTemporaryInferGrammarStructuralProductionsLoop11Result::MlTemporaryInferGrammarStructuralProductionsLoop11Return(source_ml_temporary_result11) => {
                                        self::MlTemporaryInferGrammarStructuralProductionsLoop8Result::MlTemporaryInferGrammarStructuralProductionsLoop8Return(source_ml_temporary_result11)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryInferGrammarStructuralProductionsLoop8Result::MlTemporaryInferGrammarStructuralProductionsLoop8Done(productions)
        };
    }
}
