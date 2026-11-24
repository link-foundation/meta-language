// Generated from complete js/src/translation/rust-contextual-closure-rules.js. Do not edit by hand.
use super::MlTemporaryFindRustContextualClosureParenthesisCountLoop7Result;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_rust_contextual_closure_parenthesis_count_loop7(
    syntax: &[String],
    mut index: f64,
) -> self::MlTemporaryFindRustContextualClosureParenthesisCountLoop7Result {
    loop {
        return if index < (syntax.len() as f64) {
            {
                let token = self::read_required_text_fact(syntax, index);
                if token == "(" {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlTemporaryFindRustContextualClosureParenthesisCountLoop7Result::MlTemporaryFindRustContextualClosureParenthesisCountLoop7Return(if (index > 0f64) && (((token == "|") || (token == "||")) || (token == "move")) {
                        index
                    } else {
                        -1f64
                    })
                }
            }
        } else {
            self::MlTemporaryFindRustContextualClosureParenthesisCountLoop7Result::MlTemporaryFindRustContextualClosureParenthesisCountLoop7Done
        };
    }
}
