// Generated from complete js/src/translation/source-construct-signal-rules.js. Do not edit by hand.
use super::MlTemporaryReadSourceConstructSignalsJoin3;
use super::MlTemporaryReadSourceConstructSignalsJoin4;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::source_ml_temporary_read_source_construct_signals_loop5;
use super::source_ml_temporary_read_source_construct_signals_loop6;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_source_construct_signals_loop2(
    source: &self::ml_string::Text,
    mut result: Vec<f64>,
    source_ml_temporary_values1: &[self::ml_string::Text],
    mut source_ml_temporary_index1: self::ml::Big,
) -> Vec<f64> {
    loop {
        return if source_ml_temporary_index1
            < (self::ml::Big::from_u128(source_ml_temporary_values1.len() as u128))
        {
            {
                let needle = self::ml_array::at(
                    source_ml_temporary_values1,
                    source_ml_temporary_index1.to_index(),
                );
                {
                    let count = 0f64;
                    {
                        let first = -1f64;
                        {
                            let line = -1f64;
                            {
                                let source_ml_source1_s25 = if (needle.length()) > 0f64 {
                                    {
                                        let at = {
                                            let source_ml_source1_string_search_operand26 =
                                                source.clone();
                                            {
                                                let source_ml_source1_string_search_operand27 =
                                                    needle.clone();
                                                {
                                                    let source_ml_source1_string_search_text28 =
                                                        source_ml_source1_string_search_operand27;
                                                    {
                                                        let source_ml_source1_string_search_position29 =
                                                            0f64;
                                                        source_ml_source1_string_search_operand26.index_of(&source_ml_source1_string_search_text28, source_ml_source1_string_search_position29)
                                                    }
                                                }
                                            }
                                        };
                                        {
                                            let source_ml_source1_s30 = if at >= 0f64 {
                                                {
                                                    let first_2 = at;
                                                    {
                                                        let line_2 = 1f64;
                                                        {
                                                            let before = 0f64;
                                                            {
                                                                let line_3 = self::source_ml_temporary_read_source_construct_signals_loop6(source, line_2, at, before);
                                                                self::MlTemporaryReadSourceConstructSignalsJoin4::MlTemporaryReadSourceConstructSignalsJoin4Next(first_2, line_3)
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlTemporaryReadSourceConstructSignalsJoin4::MlTemporaryReadSourceConstructSignalsJoin4Next(first, line)
                                            };
                                            match source_ml_source1_s30 {
                                                self::MlTemporaryReadSourceConstructSignalsJoin4::MlTemporaryReadSourceConstructSignalsJoin4Next(first_3, line_4) => {
                                                    {
                                                        let count_2 = self::source_ml_temporary_read_source_construct_signals_loop5(source, &(needle), count, at);
                                                        self::MlTemporaryReadSourceConstructSignalsJoin3::MlTemporaryReadSourceConstructSignalsJoin3Next(count_2, first_3, line_4)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    self::MlTemporaryReadSourceConstructSignalsJoin3::MlTemporaryReadSourceConstructSignalsJoin3Next(count, first, line)
                                };
                                match source_ml_source1_s25 {
                                    self::MlTemporaryReadSourceConstructSignalsJoin3::MlTemporaryReadSourceConstructSignalsJoin3Next(count_3, first_4, line_5) => {
                                        {
                                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![count_3, first_4, line_5], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                            {
                                                let source_ml_temporary_index1_2 = source_ml_temporary_index1.add(&self::ml::Big::from_i128(1));
                                                {
                                                    (result, source_ml_temporary_index1) = (result_2, source_ml_temporary_index1_2);
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
        } else {
            result
        };
    }
}
