// Generated from complete js/src/translation/rust-type-alias-rules.js. Do not edit by hand.
use super::group_after;
use super::ml_array;
use super::ml_number;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::read_rust_primitive_try_from_mapper_owner;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_type_alias_nominal_access(
    kind: &str,
    wrappers: f64,
    suffix: f64,
    role: &str,
    syntax: &str,
) -> bool {
    ((kind == "named") && (wrappers == 0f64))
        && (((suffix > 0f64) || (role == "implementation owner"))
            || ((role == "braced record") && (syntax == "named")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_namespace_disposition(alias: bool, owner: &str) -> String {
    if alias {
        String::from("alias")
    } else if ((owner == "data") || (owner == "module")) || (owner == "externalCrate") {
        String::from("block")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_value_constructor_error(
    local: bool,
    value: bool,
    alias: bool,
) -> String {
    if (!local && !value) && alias {
        String::from("a source type alias has no independent value constructor")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_type_alias_primitive_member_owner(
    kind: &str,
    bits: &str,
    signed: bool,
    owner_kind: &str,
    owner: &str,
    children: f64,
    proven: bool,
    layers: f64,
    suffix: f64,
) -> String {
    if suffix == 1f64 {
        self::read_rust_primitive_try_from_mapper_owner(
            kind, bits, signed, owner_kind, owner, children, proven, layers,
        )
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryGroupAfterLoop1Result {
    MlTemporaryGroupAfterLoop1Done,
    MlTemporaryGroupAfterLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_group_after_loop1(
    kinds: &[String],
    values: &[String],
    mut stack: Vec<String>,
    mut at: f64,
) -> self::MlTemporaryGroupAfterLoop1Result {
    loop {
        return if at < (values.len() as f64) {
            if (self::read_required_text_fact(kinds, at)) != "punct" {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                {
                    let value = self::read_required_text_fact(values, at);
                    if ((value == "{") || (value == "[")) || (value == "(") {
                        {
                            let stack_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![if value == "{" {
                                        String::from("}")
                                    } else if value == "[" {
                                        String::from("]")
                                    } else {
                                        String::from(")")
                                    }],
                                    stack,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let at_3 = at + 1f64;
                                {
                                    (stack, at) = (stack_2, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if ((value == "}") || (value == "]")) || (value == ")") {
                            if value
                                == (self::read_required_text_fact(
                                    &(stack),
                                    ((stack.len() as f64) - 1f64),
                                ))
                            {
                                {
                                    let stack_3 = self::ml_array::slice(&stack, 0f64, -1f64);
                                    if (stack_3.len() as f64) == 0f64 {
                                        self::MlTemporaryGroupAfterLoop1Result::MlTemporaryGroupAfterLoop1Return(at + 1f64)
                                    } else {
                                        {
                                            let at_4 = at + 1f64;
                                            {
                                                (stack, at) = (stack_3, at_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlTemporaryGroupAfterLoop1Result::MlTemporaryGroupAfterLoop1Return(-1f64)
                            }
                        } else {
                            {
                                let at_5 = at + 1f64;
                                {
                                    at = at_5;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryGroupAfterLoop1Result::MlTemporaryGroupAfterLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result {
    MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Done(Vec<String>),
    MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(Vec<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result {
    MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Done(f64),
    MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Return(Vec<String>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_type_alias_declaration_plan_loop3(
    kinds: &[String],
    values: &[String],
    mut end: f64,
) -> self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result {
    loop {
        return if end < (values.len() as f64) {
            {
                let boundary = self::read_required_text_fact(values, end);
                if ((self::read_required_text_fact(kinds, end)) == "punct") && (boundary == ";") {
                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Done(end)
                } else if ((self::read_required_text_fact(kinds, end)) == "punct")
                    && (((boundary == "{") || (boundary == "[")) || (boundary == "("))
                {
                    {
                        let after = self::group_after(kinds, values, end);
                        if after < 0f64 {
                            self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Return(vec![String::from("error"), String::from("unclosed source type alias group")])
                        } else {
                            {
                                let end_2 = after;
                                {
                                    end = end_2;
                                    continue;
                                }
                            }
                        }
                    }
                } else if ((self::read_required_text_fact(kinds, end)) == "eof")
                    || (((self::read_required_text_fact(kinds, end)) == "punct")
                        && (boundary == "}"))
                {
                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Return(vec![String::from("error"), String::from("unclosed source type alias declaration")])
                } else {
                    {
                        let end_3 = end + 1f64;
                        {
                            end = end_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Done(end)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_type_alias_declaration_plan_loop2(
    kinds: &[String],
    values: &[String],
    mut result: Vec<String>,
    mut names: Vec<String>,
    mut owners: Vec<String>,
    mut closes: Vec<f64>,
    mut position: f64,
) -> self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result {
    loop {
        return if position < (values.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, position);
                {
                    let value = self::read_required_text_fact(values, position);
                    if ((closes.len() as f64) > 0f64)
                        && (position
                            == (self::read_required_number_fact(
                                &(closes),
                                ((closes.len() as f64) - 1f64),
                            )))
                    {
                        {
                            let closes_2 = self::ml_array::slice(&closes, 0f64, -1f64);
                            {
                                let owners_2 = self::ml_array::slice(&owners, 0f64, -1f64);
                                {
                                    let position_2 = position + 1f64;
                                    {
                                        (owners, closes, position) =
                                            (owners_2, closes_2, position_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    {
                        if ((((kind == "keyword") && (value == "mod"))
                            && ((position + 2f64) < (values.len() as f64)))
                            && ((self::read_required_text_fact(kinds, (position + 1f64)))
                                == "identifier"))
                            && ((self::read_required_text_fact(values, (position + 2f64))) == "{")
                        {
                            {
                                let after = self::group_after(kinds, values, (position + 2f64));
                                if after < 0f64 {
                                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(result)
                                } else {
                                    {
                                        let owners_3 = {
                                            let (ml_append_right, ml_append_left) = (
                                                vec![self::read_required_text_fact(
                                                    values,
                                                    (position + 1f64),
                                                )],
                                                owners,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let closes_3 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![(after - 1f64)], closes);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let position_3 = position + 3f64;
                                                {
                                                    (owners, closes, position) =
                                                        (owners_3, closes_3, position_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if (kind == "keyword") && (value == "type") {
                            if (((position + 3f64) >= (values.len() as f64))
                                || ((self::read_required_text_fact(kinds, (position + 1f64)))
                                    != "identifier"))
                                || ((self::read_required_text_fact(values, (position + 2f64)))
                                    != "=")
                            {
                                self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(vec![String::from("error"), String::from("generic, constrained or malformed type aliases require their complete parameter interpretation")])
                            } else {
                                {
                                    let name = (self::ml_array::append(
                                        owners.clone(),
                                        vec![self::read_required_text_fact(
                                            values,
                                            (position + 1f64),
                                        )],
                                    ))
                                    .join((String::from(".")).as_str());
                                    if self::ml_array::includes(
                                        &names,
                                        &name.clone(),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    ) {
                                        self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(vec![String::from("error"), String::from("duplicate source type alias")])
                                    } else {
                                        {
                                            let names_2 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![name.clone()], names);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let end = position + 3f64;
                                                {
                                                    let source_ml_source1_s39 = self::source_ml_temporary_read_rust_type_alias_declaration_plan_loop3(kinds, values, end);
                                                    match source_ml_source1_s39 {
                                                            self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Done(end_2) => {
                                                                if (end_2 >= (values.len() as f64)) || (end_2 == (position + 3f64)) {
                                                                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(vec![String::from("error"), String::from("empty or unclosed source type alias declaration")])
                                                                } else {
                                                                    {
                                                                        let result_2 = { let (ml_append_right, ml_append_left) = (vec![name.clone(), {
                                                                            let source_ml_source1_string_argument40 = position;
                                                                            self::ml_number::js_number(source_ml_source1_string_argument40)
                                                                        }, {
                                                                            let source_ml_source1_string_argument41 = position + 3f64;
                                                                            self::ml_number::js_number(source_ml_source1_string_argument41)
                                                                        }, {
                                                                            let source_ml_source1_string_argument42 = end_2;
                                                                            self::ml_number::js_number(source_ml_source1_string_argument42)
                                                                        }, {
                                                                            let source_ml_source1_string_argument43 = end_2 + 1f64;
                                                                            self::ml_number::js_number(source_ml_source1_string_argument43)
                                                                        }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        if (names_2.len() as f64) > 10_000f64 {
                                                                            self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(vec![String::from("error"), String::from("source type alias declaration limit exceeded")])
                                                                        } else {
                                                                            {
                                                                                let position_4 = end_2 + 1f64;
                                                                                {
                                                                                    (result, names, position) = (result_2, names_2, position_4);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Return(source_ml_temporary_result3) => {
                                                                self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(source_ml_temporary_result3)
                                                            }
                                                        }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if (kind == "punct")
                            && (((value == "{") || (value == "[")) || (value == "("))
                        {
                            {
                                let after_2 = self::group_after(kinds, values, position);
                                if after_2 < 0f64 {
                                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(result)
                                } else {
                                    {
                                        let position_5 = after_2;
                                        {
                                            position = position_5;
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let position_6 = position + 1f64;
                                {
                                    position = position_6;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustTypeAliasLookupKeysJoin4 {
    MlTemporaryReadRustTypeAliasLookupKeysJoin4Next(Vec<String>, Vec<String>),
    MlTemporaryReadRustTypeAliasLookupKeysJoin4Return(Vec<String>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_type_alias_lookup_keys_loop5(
    names: &[String],
    mut count: f64,
) -> f64 {
    loop {
        if ((self::ml_array::optional_at(names, self::ml_array::number_index(count)))
            .unwrap_or_else(|| String::new()))
            == "super"
        {
            {
                let count_2 = count + 1f64;
                {
                    count = count_2;
                    continue;
                }
            }
        }
        return { count };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_type_alias_access_prefix_lengths_loop6(
    mut result: Vec<f64>,
    mut length: f64,
) -> Vec<f64> {
    loop {
        if length > 0f64 {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (vec![length], result);
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let length_2 = length - 1f64;
                    {
                        (result, length) = (result_2, length_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}
