// Generated from complete js/src/grammar-scanner-mode-stack-rules.js. Do not edit by hand.
use super::MlTemporaryRenderGrammarScannerModeStackDefinitionLoop4Result;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_render_grammar_scanner_mode_stack_definition_loop4(
    modes: &[String],
    spellings: &[String],
    tokens: &[String],
    effects: &[String],
    code_points: &[f64],
    plan: &[f64],
    matchers: &[String],
    mut declared_tokens: Vec<String>,
    mut literals: String,
    mut characters: String,
    mut position: f64,
) -> self::MlTemporaryRenderGrammarScannerModeStackDefinitionLoop4Result {
    loop {
        if position < (plan.len() as f64) {
            {
                let index = self::read_required_number_fact(plan, position);
                {
                    let matcher = if (matchers.len() as f64) == 0f64 {
                        format!(
                            "(literal {})",
                            self::read_required_text_fact(spellings, index)
                        )
                    } else {
                        self::read_required_text_fact(matchers, position)
                    };
                    if matcher == "" {
                        {
                            let position_2 = position + 1f64;
                            {
                                position = position_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let mode = self::read_required_text_fact(modes, index);
                            {
                                let token = self::read_required_text_fact(tokens, index);
                                {
                                    let effect = self::read_required_text_fact(effects, index);
                                    {
                                        let declared_tokens_3 = if (self::ml_array::includes(
                                            &declared_tokens,
                                            &token.clone(),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        )) {
                                            declared_tokens
                                        } else {
                                            {
                                                {
                                                    let (ml_append_right, ml_append_left) =
                                                        (vec![token.clone()], declared_tokens);
                                                    self::ml_array::append(
                                                        ml_append_left,
                                                        ml_append_right,
                                                    )
                                                }
                                            }
                                        };
                                        {
                                            let operation = format!(
                                                "(if (all (valid {token}) (equal mode (text {mode})) (next {matcher})) (then (consume {matcher}) {}(emit {token}))) ",
                                                if effect == "keep" {
                                                    String::new()
                                                } else {
                                                    format!("{}{}", effect, (String::from(" ")))
                                                }
                                            );
                                            if (self::read_required_number_fact(code_points, index))
                                                < 0f64
                                            {
                                                {
                                                    let literals_2 =
                                                        format!("{literals}{operation}");
                                                    {
                                                        let position_3 = position + 1f64;
                                                        {
                                                            (declared_tokens, literals, position) = (
                                                                declared_tokens_3,
                                                                literals_2,
                                                                position_3,
                                                            );
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                            {
                                                {
                                                    let characters_2 =
                                                        format!("{characters}{operation}");
                                                    {
                                                        let position_4 = position + 1f64;
                                                        {
                                                            (
                                                                declared_tokens,
                                                                characters,
                                                                position,
                                                            ) = (
                                                                declared_tokens_3,
                                                                characters_2,
                                                                position_4,
                                                            );
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
        return {
            self::MlTemporaryRenderGrammarScannerModeStackDefinitionLoop4Result::MlTemporaryRenderGrammarScannerModeStackDefinitionLoop4Done(declared_tokens, literals.clone(), characters.clone())
        };
    }
}
