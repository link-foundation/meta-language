// Generated from the complete shared grammar-lexer-rules.js and recorded source closure.
// Do not edit by hand.
use super::*;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_lexer_lazy_normalization_loop41(
    kinds: &[self::ml_string::Text],
    labels: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    inner: &[f64],
    edge_sources: &[f64],
    edge_targets: &[f64],
    mut count: f64,
    mut captures: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let captures_3 = if ((self::read_required_text_fact(kinds, index))
                    == (self::ml_string::Text::from_units(vec![99, 97, 112, 116, 117, 114, 101])))
                    && ((self::read_required_text_fact(labels, index))
                        == (self::ml_string::Text::from_units(vec![
                            110, 111, 110, 95, 103, 114, 101, 101, 100, 121,
                        ]))) {
                    {
                        let delimiter = -1f64;
                        {
                            let repeat = self::read_required_number_fact(inner, index);
                            {
                                let delimiter_3 = if ((repeat >= 0f64) && (repeat < count))
                                    && ((self::read_required_text_fact(kinds, repeat))
                                        == (self::ml_string::Text::from_units(vec![
                                            114, 101, 112, 101, 97, 116, 48,
                                        ]))) {
                                    {
                                        let atom = self::read_required_number_fact(inner, repeat);
                                        if ((atom >= 0f64) && (atom < count))
                                            && ((self::read_required_text_fact(kinds, atom))
                                                == (self::ml_string::Text::from_units(vec![
                                                    97, 110, 121,
                                                ])))
                                        {
                                            {
                                                let owner = 0f64;
                                                {
                                                    self::ml_read_grammar_lexer_lazy_normalization_loop42(kinds, texts, edge_sources, edge_targets, count, index, delimiter, owner)
                                                }
                                            }
                                        } else {
                                            delimiter
                                        }
                                    }
                                } else {
                                    delimiter
                                };
                                {
                                    {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![index, delimiter_3], captures);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    }
                                }
                            }
                        }
                    }
                } else {
                    captures
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (captures, index) = (captures_3, index_2);
                        continue;
                    }
                }
            }
        }
        return { captures };
    }
}
