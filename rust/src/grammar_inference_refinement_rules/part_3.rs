// Generated from complete js/src/grammar-inference-refinement-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Result;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_regular_grammar_refinement_identifier_loop17(
    identifier: &str,
    existing: &[String],
    mut index: f64,
) -> self::MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Result {
    loop {
        return if index < (existing.len() as f64) {
            if (self::read_required_text_fact(existing, index)) == identifier {
                self::MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Result::MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Return(false)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Result::MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Done
        };
    }
}
