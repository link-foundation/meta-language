// Generated from complete js/src/grammar-scanner-character-class-rules.js. Do not edit by hand.
use super::MlTemporaryRenderGrammarScannerCharacterClassMembersLoop5Result;
use super::ml;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_render_grammar_scanner_character_class_members_loop5(
    code_points: &[f64],
    mut previous: f64,
    mut selected: f64,
    mut minimum: f64,
    source_ml_temporary_values3: &[f64],
    mut source_ml_temporary_index3: self::ml::Big,
) -> self::MlTemporaryRenderGrammarScannerCharacterClassMembersLoop5Result {
    loop {
        if source_ml_temporary_index3
            < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))
        {
            {
                let member = self::ml_array::at(
                    source_ml_temporary_values3,
                    source_ml_temporary_index3.to_index(),
                );
                {
                    let scalar = self::read_required_number_fact(code_points, member);
                    if (previous < scalar) && (scalar < minimum) {
                        {
                            let selected_2 = member;
                            {
                                let minimum_2 = scalar;
                                {
                                    let source_ml_temporary_index3_2 = source_ml_temporary_index3
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (selected, minimum, source_ml_temporary_index3) =
                                            (selected_2, minimum_2, source_ml_temporary_index3_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_temporary_index3_3 =
                                source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index3 = source_ml_temporary_index3_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryRenderGrammarScannerCharacterClassMembersLoop5Result::MlTemporaryRenderGrammarScannerCharacterClassMembersLoop5Done(selected, minimum)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_render_grammar_scanner_character_class_members_loop4(
    members: &[f64],
    code_points: &[f64],
    mut ordered: Vec<f64>,
    mut previous: f64,
    mut position: f64,
) -> Vec<f64> {
    loop {
        return if position < (members.len() as f64) {
            {
                let selected = -1f64;
                {
                    let minimum = 1_114_112f64;
                    {
                        let source_ml_temporary_values3 = members.to_vec();
                        {
                            let source_ml_temporary_index3 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_s3 = self::source_ml_temporary_render_grammar_scanner_character_class_members_loop5(code_points, previous, selected, minimum, &(source_ml_temporary_values3), source_ml_temporary_index3);
                                match source_ml_source1_s3 {
                                    self::MlTemporaryRenderGrammarScannerCharacterClassMembersLoop5Result::MlTemporaryRenderGrammarScannerCharacterClassMembersLoop5Done(selected_2, minimum_2) => {
                                        {
                                            let ordered_2 = { let (ml_append_right, ml_append_left) = (vec![selected_2], ordered); self::ml_array::append(ml_append_left, ml_append_right) };
                                            {
                                                let previous_2 = minimum_2;
                                                {
                                                    let position_2 = position + 1f64;
                                                    {
                                                        (ordered, previous, position) = (ordered_2, previous_2, position_2);
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
        } else {
            ordered
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_render_grammar_scanner_character_class_members_loop7(
    code_points: &[f64],
    ordered: &[f64],
    mut last: f64,
) -> f64 {
    loop {
        if ((last + 1f64) < (ordered.len() as f64))
            && ((self::read_required_number_fact(
                code_points,
                self::read_required_number_fact(ordered, (last + 1f64)),
            )) == (self::read_required_number_fact(
                code_points,
                self::read_required_number_fact(ordered, last),
            ) + 1f64))
        {
            {
                let last_2 = last + 1f64;
                {
                    last = last_2;
                    continue;
                }
            }
        }
        return { last };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_render_grammar_scanner_character_class_members_loop6(
    spellings: &[String],
    code_points: &[f64],
    ordered: &[f64],
    mut text: String,
    mut first: f64,
) -> String {
    loop {
        if first < (ordered.len() as f64) {
            {
                let last = first;
                {
                    let last_2 = self::source_ml_temporary_render_grammar_scanner_character_class_members_loop7(code_points, ordered, last);
                    {
                        let lower = self::read_required_text_fact(
                            spellings,
                            self::read_required_number_fact(ordered, first),
                        );
                        {
                            let upper = self::read_required_text_fact(
                                spellings,
                                self::read_required_number_fact(ordered, last_2),
                            );
                            {
                                let text_2 = format!(
                                    "{text}{}",
                                    if first == last_2 {
                                        format!(
                                            "{}{}",
                                            (format!("{}{}", (String::from(" (char ")), lower)),
                                            (String::from(")"))
                                        )
                                    } else {
                                        format!(
                                            "{}{}",
                                            (format!(
                                                "{}{}",
                                                (format!(
                                                    "{}{}",
                                                    (format!(
                                                        "{}{}",
                                                        (String::from(" (range ")),
                                                        lower
                                                    )),
                                                    (String::from(" "))
                                                )),
                                                upper
                                            )),
                                            (String::from(")"))
                                        )
                                    }
                                );
                                {
                                    let first_2 = last_2 + 1f64;
                                    {
                                        (text, first) = (text_2, first_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { text };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderGrammarScannerCharacterClassDefinitionLoop8Result {
    MlTemporaryRenderGrammarScannerCharacterClassDefinitionLoop8Done(
        Vec<String>,
        String,
        String,
        String,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin10 {
    MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin10Next(Vec<String>, String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin9 {
    MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin9Next(Vec<String>, String),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_render_grammar_scanner_character_class_definition_loop8(
    modes: &[String],
    tokens: &[String],
    next_modes: &[String],
    code_points: &[f64],
    plan: &[f64],
    matchers: &[String],
    mut declared_modes: Vec<String>,
    mut declared_tokens: Vec<String>,
    mut declarations: String,
    mut literals: String,
    mut characters: String,
    mut position: f64,
) -> self::MlTemporaryRenderGrammarScannerCharacterClassDefinitionLoop8Result {
    loop {
        return if position < (plan.len() as f64) {
            {
                let matcher = self::read_required_text_fact(matchers, position);
                if (matcher).is_empty() {
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
                        let index = self::read_required_number_fact(plan, position);
                        {
                            let mode = self::read_required_text_fact(modes, index);
                            {
                                let token = self::read_required_text_fact(tokens, index);
                                {
                                    let next = self::read_required_text_fact(next_modes, index);
                                    {
                                        let source_ml_source1_s4 = if (self::ml_array::includes(
                                            &declared_modes,
                                            &mode.clone(),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        )) {
                                            self::MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin9::MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin9Next(declared_modes, declarations.clone())
                                        } else {
                                            {
                                                let declared_modes_2 = {
                                                    let (ml_append_right, ml_append_left) =
                                                        (vec![mode.clone()], declared_modes);
                                                    self::ml_array::append(
                                                        ml_append_left,
                                                        ml_append_right,
                                                    )
                                                };
                                                {
                                                    let declarations_2 =
                                                        format!("{declarations}(mode {mode})\n");
                                                    self::MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin9::MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin9Next(declared_modes_2, declarations_2)
                                                }
                                            }
                                        };
                                        match source_ml_source1_s4 {
                                            self::MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin9::MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin9Next(declared_modes_3, declarations_3) => {
                                                {
                                                    let source_ml_source1_s5 = if (self::ml_array::includes(&declared_modes_3, &next.clone(), 0f64, false, |a, b| a == b)) {
                                                        self::MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin10::MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin10Next(declared_modes_3, declarations_3.clone())
                                                    } else {
                                                        {
                                                            let declared_modes_4 = { let (ml_append_right, ml_append_left) = (vec![next.clone()], declared_modes_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                            {
                                                                let declarations_4 = format!("{declarations_3}(mode {next})\n");
                                                                self::MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin10::MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin10Next(declared_modes_4, declarations_4)
                                                            }
                                                        }
                                                    };
                                                    match source_ml_source1_s5 {
                                                        self::MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin10::MlTemporaryRenderGrammarScannerCharacterClassDefinitionJoin10Next(declared_modes_5, declarations_5) => {
                                                            {
                                                                let declared_tokens_3 = if (self::ml_array::includes(&declared_tokens, &token.clone(), 0f64, false, |a, b| a == b)) {
                                                                    declared_tokens
                                                                } else {
                                                                    {
                                                                        { let (ml_append_right, ml_append_left) = (vec![token.clone()], declared_tokens); self::ml_array::append(ml_append_left, ml_append_right) }
                                                                    }
                                                                };
                                                                {
                                                                    let operation = format!("(if (all (valid {token}) (equal mode (text {mode})) (next {matcher})) (then (consume {matcher}) (setMode {next}) (emit {token}))) ");
                                                                    if (self::read_required_number_fact(code_points, index)) < 0f64 {
                                                                        {
                                                                            let literals_2 = format!("{literals}{operation}");
                                                                            {
                                                                                let position_3 = position + 1f64;
                                                                                {
                                                                                    (declared_modes, declared_tokens, declarations, literals, position) = (declared_modes_5, declared_tokens_3, declarations_5.clone(), literals_2, position_3);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
{
                                                                        {
                                                                            let characters_2 = format!("{characters}{operation}");
                                                                            {
                                                                                let position_4 = position + 1f64;
                                                                                {
                                                                                    (declared_modes, declared_tokens, declarations, characters, position) = (declared_modes_5, declared_tokens_3, declarations_5.clone(), characters_2, position_4);
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
        } else {
            self::MlTemporaryRenderGrammarScannerCharacterClassDefinitionLoop8Result::MlTemporaryRenderGrammarScannerCharacterClassDefinitionLoop8Done(declared_tokens, declarations.clone(), literals.clone(), characters.clone())
        };
    }
}
