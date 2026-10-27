// Generated from complete js/src/translation/rust-primitive-try-from-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustIntegerBitsJoin4;
use super::MlTemporaryPlanRustIntegerBitsLoop1Result;
use super::ml_array;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_integer_bits_loop1(
    mut bits: f64,
    mut reverse: bool,
    mut trailing: bool,
    mut leading: bool,
    mut ones: bool,
    mut plan: Vec<f64>,
    mut bindings: Vec<f64>,
    mut current: f64,
    mut count: f64,
    mut active: f64,
    mut index: f64,
    mut bit: f64,
) -> self::MlTemporaryPlanRustIntegerBitsLoop1Result {
    loop {
        return if bit < bits {
            {
                let at = index;
                {
                    let next_current = 3f64 + (bit * 3f64);
                    {
                        let next_count = 4f64 + (bit * 3f64);
                        {
                            let next_active = 5f64 + (bit * 3f64);
                            {
                                let plan_2 = {
                                    let source_ml_source1_concat_value66 = plan;
                                    {
                                        let source_ml_source1_concat_value67 = vec![
                                            8f64,
                                            current,
                                            0f64,
                                            0f64,
                                            0f64,
                                            if reverse { 8f64 } else { 9f64 },
                                            count,
                                            0f64,
                                            0f64,
                                            0f64,
                                            10f64,
                                            active,
                                            0f64,
                                            0f64,
                                            0f64,
                                            12f64,
                                            at,
                                            3f64,
                                            0f64,
                                            0f64,
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source1_concat_value66,
                                            ),
                                            source_ml_source1_concat_value67,
                                        )
                                    }
                                };
                                {
                                    let index_2 = index + 4f64;
                                    {
                                        let next_count_term = 0f64;
                                        {
                                            let next_active_term = 7f64;
                                            {
                                                let source_ml_source1_s76 = if reverse {
                                                    {
                                                        let plan_3 = {
                                                            let source_ml_source1_concat_value77 =
                                                                plan_2;
                                                            {
                                                                let source_ml_source1_concat_value78 = vec![
                                                                    16f64,
                                                                    (at + 1f64),
                                                                    3f64,
                                                                    0f64,
                                                                    0f64,
                                                                    15f64,
                                                                    (at + 4f64),
                                                                    (at + 3f64),
                                                                    0f64,
                                                                    0f64,
                                                                ];
                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value77), source_ml_source1_concat_value78)
                                                            }
                                                        };
                                                        {
                                                            let next_count_term_2 = at + 5f64;
                                                            {
                                                                let index_3 = index_2 + 2f64;
                                                                self::MlTemporaryPlanRustIntegerBitsJoin4::MlTemporaryPlanRustIntegerBitsJoin4Next(plan_3, index_3, next_count_term_2, next_active_term)
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let match_ = index_2;
                                                        {
                                                            let plan_4 = {
                                                                let source_ml_source1_concat_value79 =
                                                                    plan_2;
                                                                {
                                                                    let source_ml_source1_concat_value80 = vec![
                                                                        11f64,
                                                                        (at + 3f64),
                                                                        if ones {
                                                                            2f64
                                                                        } else {
                                                                            1f64
                                                                        },
                                                                        0f64,
                                                                        0f64,
                                                                        14f64,
                                                                        (at + 1f64),
                                                                        5f64,
                                                                        0f64,
                                                                        0f64,
                                                                    ];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value79), source_ml_source1_concat_value80)
                                                                }
                                                            };
                                                            {
                                                                let index_4 = index_2 + 2f64;
                                                                if trailing {
                                                                    {
                                                                        let next_flag = index_4;
                                                                        {
                                                                            let plan_5 = {
                                                                                let source_ml_source1_concat_value81 =
                                                                                    plan_4;
                                                                                {
                                                                                    let source_ml_source1_concat_value82 = vec![
                                                                                        17f64,
                                                                                        (at + 2f64),
                                                                                        match_,
                                                                                        6f64,
                                                                                        0f64,
                                                                                        17f64,
                                                                                        next_flag,
                                                                                        (match_
                                                                                            + 1f64),
                                                                                        (at + 1f64),
                                                                                        0f64,
                                                                                    ];
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value81), source_ml_source1_concat_value82)
                                                                                }
                                                                            };
                                                                            {
                                                                                let next_active_term_2 =
                                                                                    next_flag;
                                                                                {
                                                                                    let next_count_term_3 =
                                                                                        next_flag
                                                                                            + 1f64;
                                                                                    {
                                                                                        let index_5 = index_4 + 2f64;
                                                                                        self::MlTemporaryPlanRustIntegerBitsJoin4::MlTemporaryPlanRustIntegerBitsJoin4Next(plan_5, index_5, next_count_term_3, next_active_term_2)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    {
                                                                        let next_count_term_4 =
                                                                            index_4;
                                                                        {
                                                                            let plan_6 = {
                                                                                let source_ml_source1_concat_value83 =
                                                                                    plan_4;
                                                                                {
                                                                                    let source_ml_source1_concat_value84 = vec![
                                                                                        17f64,
                                                                                        match_,
                                                                                        (match_
                                                                                            + 1f64),
                                                                                        if leading {
                                                                                            4f64
                                                                                        } else {
                                                                                            at + 1f64
                                                                                        },
                                                                                        0f64,
                                                                                    ];
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value83), source_ml_source1_concat_value84)
                                                                                }
                                                                            };
                                                                            {
                                                                                let index_6 =
                                                                                    index_4 + 1f64;
                                                                                self::MlTemporaryPlanRustIntegerBitsJoin4::MlTemporaryPlanRustIntegerBitsJoin4Next(plan_6, index_6, next_count_term_4, next_active_term)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                match source_ml_source1_s76 {
                                                    self::MlTemporaryPlanRustIntegerBitsJoin4::MlTemporaryPlanRustIntegerBitsJoin4Next(plan_7, index_7, next_count_term_5, next_active_term_3) => {
                                                        {
                                                            let quotient = index_7;
                                                            {
                                                                let plan_8 = {
                                                                    let source_ml_source1_concat_value85 = plan_7;
                                                                    {
                                                                        let source_ml_source1_concat_value86 = vec![13f64, at, 3f64, 0f64, 0f64];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value85), source_ml_source1_concat_value86)
                                                                    }
                                                                };
                                                                {
                                                                    let index_8 = index_7 + 1f64;
                                                                    {
                                                                        let bindings_2 = {
                                                                            let source_ml_source1_concat_value87 = bindings;
                                                                            {
                                                                                let source_ml_source1_concat_value88 = vec![next_current, quotient, next_count, next_count_term_5];
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value87), source_ml_source1_concat_value88)
                                                                            }
                                                                        };
                                                                        {
                                                                            let bindings_4 = if trailing {
                                                                                {
                                                                                    {
                                                                                        let source_ml_source1_concat_value89 = bindings_2;
                                                                                        {
                                                                                            let source_ml_source1_concat_value90 = vec![next_active, next_active_term_3];
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value89), source_ml_source1_concat_value90)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                bindings_2
                                                                            };
                                                                            {
                                                                                let current_2 = next_current;
                                                                                {
                                                                                    let count_2 = next_count;
                                                                                    if trailing {
                                                                                        {
                                                                                            let active_2 = next_active;
                                                                                            {
                                                                                                let bit_2 = bit + 1f64;
                                                                                                {
                                                                                                    (plan, bindings, current, count, active, index, bit) = (plan_8, bindings_4, current_2, count_2, active_2, index_8, bit_2);
                                                                                                    continue;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
{
                                                                                        {
                                                                                            let bit_3 = bit + 1f64;
                                                                                            {
                                                                                                (plan, bindings, current, count, index, bit) = (plan_8, bindings_4, current_2, count_2, index_8, bit_3);
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
        } else {
            self::MlTemporaryPlanRustIntegerBitsLoop1Result::MlTemporaryPlanRustIntegerBitsLoop1Done(
                plan, bindings, count, index,
            )
        };
    }
}
