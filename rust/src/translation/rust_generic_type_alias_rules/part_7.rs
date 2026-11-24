// Generated from complete js/src/translation/rust-generic-type-alias-rules.js. Do not edit by hand.
use super::MlSource1TemporaryGroupAfterLoop30Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Result;
use super::accept_rust_generic_type_alias_header_standard_sum_origin;
use super::accept_rust_retained_function_owner;
use super::accept_rust_source_array_element;
use super::accept_rust_source_array_owner;
use super::accept_rust_source_signature_primitive;
use super::accept_rust_type_alias_empty_vector_member;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::ml_utf16_text;
use super::read_required_text_fact;
use super::read_rust_function_pointer_header_type_error;
use super::read_rust_function_pointer_owned_domain_error;
use super::read_rust_function_pointer_type_error;
use super::read_rust_standard_sum_source_argument_count;
use super::read_rust_standard_sum_source_constructor_arity;
use super::read_rust_standard_sum_source_depth_limit;
use super::read_rust_type_alias_expansion_error;
use super::source_ml_source1_temporary_group_after_loop30;
use super::source_ml_source1_temporary_read_rust_generic_type_alias_declaration_plan_loop32;
use super::source_ml_source1_temporary_read_rust_generic_type_alias_expansion_error_loop37;
use super::source_ml_source1_temporary_read_rust_generic_type_alias_unused_parameter_error_loop29;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_unused_parameter_error(
    parameters: &[self::ml_string::Text],
    used: &[self::ml_string::Text],
) -> self::ml_string::Text {
    {
        let at = 0f64;
        {
            let source_ml_source2_s62 = self::source_ml_source1_temporary_read_rust_generic_type_alias_unused_parameter_error_loop29(parameters, used, at);
            match source_ml_source2_s62 {
                self::MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Result::MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Done => {
                    self::ml_string::Text::from_units(vec![])
                }
                self::MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Result::MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Return(source_ml_source1_temporary_result29) => {
                    source_ml_source1_temporary_result29
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_generic_type_alias_concrete_template(
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    children: f64,
    proven: bool,
) -> bool {
    ((proven && (wrappers == 0f64)) && (children == 0f64))
        && if (*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])) {
            ((*owner_kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))
                && ((*owner) == (self::ml_string::Text::from_units(vec![])))
        } else {
            (((*owner_kind)
                == (self::ml_string::Text::from_units(vec![
                    112, 114, 105, 109, 105, 116, 105, 118, 101,
                ])))
                && self::ml_array::includes(
                    &vec![
                        self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]),
                        self::ml_string::Text::from_units(vec![98, 111, 111, 108]),
                        self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]),
                    ],
                    kind,
                    0f64,
                    false,
                    |a, b| a == b,
                ))
                && self::accept_rust_source_signature_primitive(kind, owner, bits, signed)
        }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_generic_type_alias_concrete_callable_template(
    ordinary: bool,
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner_kind: &self::ml_string::Text,
    owner_paths: f64,
    children: f64,
    parameters: f64,
    parameter_copies: &[bool],
    result_copy: bool,
    result_typed: bool,
    proven: bool,
    copy: bool,
    closure: bool,
) -> bool {
    (((((((wrappers == 0f64) && !closure) && result_typed)
        && ((parameter_copies.len() as f64) == parameters))
        && self::accept_rust_retained_function_owner(
            &(self::ml_string::Text::from_units(vec![
                82, 117, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116,
                105, 111, 110, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 50,
            ])),
            owner_kind,
            kind,
            owner_paths,
            children,
            parameters,
            false,
        ))
        && proven)
        && copy)
        && ((self::read_rust_function_pointer_type_error(ordinary, parameter_copies, result_copy))
            == (self::ml_string::Text::from_units(vec![])))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_generic_type_alias_owned_callable_template(
    ordinary: bool,
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner_kind: &self::ml_string::Text,
    owner_paths: f64,
    children: f64,
    parameters: f64,
    roles: &[self::ml_string::Text],
    result_role: &self::ml_string::Text,
    result_typed: bool,
    proven: bool,
    copy: bool,
    closure: bool,
    minor: f64,
) -> bool {
    (((((((wrappers == 0f64) && !closure) && result_typed)
        && ((roles.len() as f64) == parameters))
        && self::accept_rust_retained_function_owner(
            &(self::ml_string::Text::from_units(vec![
                82, 117, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116,
                105, 111, 110, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 50,
            ])),
            owner_kind,
            kind,
            owner_paths,
            children,
            parameters,
            false,
        ))
        && proven)
        && copy)
        && ((self::read_rust_function_pointer_owned_domain_error(
            ordinary,
            roles,
            result_role,
            minor,
        )) == (self::ml_string::Text::from_units(vec![])))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_generic_type_alias_header_callable_template(
    ordinary: bool,
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner_kind: &self::ml_string::Text,
    owner_paths: f64,
    children: f64,
    parameters: f64,
    parameter_copies: &[bool],
    result_copy: bool,
    result_typed: bool,
    header: bool,
    pointer_header: bool,
    closure: bool,
) -> bool {
    ((((((header && pointer_header) && (wrappers == 0f64)) && !closure) && result_typed)
        && ((parameter_copies.len() as f64) == parameters))
        && self::accept_rust_retained_function_owner(
            &(self::ml_string::Text::from_units(vec![
                82, 117, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116,
                105, 111, 110, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 50,
            ])),
            owner_kind,
            kind,
            owner_paths,
            children,
            parameters,
            false,
        ))
        && ((self::read_rust_function_pointer_header_type_error(
            ordinary,
            parameter_copies,
            result_copy,
            header,
        )) == (self::ml_string::Text::from_units(vec![])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_generic_type_alias_named_template(
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    children: f64,
    proven: bool,
    actual_data_owner: &self::ml_string::Text,
) -> bool {
    (((((((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100])))
        && (wrappers == 0f64))
        && ((*owner_kind)
            == (self::ml_string::Text::from_units(vec![
                100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110,
            ]))))
        && (children == 0f64))
        && proven)
        && ((*actual_data_owner) != (self::ml_string::Text::from_units(vec![]))))
        && ((*owner)
            == (self::ml_string::join(
                &{
                    let source_ml_source2_string_unit_operand67 = actual_data_owner.clone();
                    {
                        let source_ml_source2_string_unit_operand68 =
                            self::ml_string::Text::from_units(vec![46]);
                        {
                            let source_ml_source2_string_split_limit69 = 4_294_967_295f64;
                            {
                                let source_ml_source2_string_split_separator70 =
                                    source_ml_source2_string_unit_operand68;
                                (|ml_text: &self::ml_string::Text,
                                  ml_separator: Option<&self::ml_string::Text>,
                                  ml_limit: f64| {
                                    self::ml_utf16_text::split_utf16_text(
                                        &(ml_text
                                            .0
                                            .iter()
                                            .map(|ml_unit| f64::from(*ml_unit))
                                            .collect::<Vec<f64>>()),
                                        &(ml_separator
                                            .map(|ml_value| {
                                                ml_value
                                                    .0
                                                    .iter()
                                                    .map(|ml_unit| f64::from(*ml_unit))
                                                    .collect::<Vec<f64>>()
                                            })
                                            .unwrap_or_default()),
                                        ml_limit,
                                        ml_separator.is_none(),
                                    )
                                    .into_iter()
                                    .map(|ml_units| {
                                        self::ml_string::Text::from_units(
                                            ml_units
                                                .into_iter()
                                                .map(|ml_unit| ml_unit as u16)
                                                .collect(),
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                })(
                                    &(source_ml_source2_string_unit_operand67),
                                    Some(&source_ml_source2_string_split_separator70),
                                    source_ml_source2_string_split_limit69,
                                )
                            }
                        }
                    }
                },
                &self::ml_string::Text::from_units(vec![58, 58]),
            )))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_standard_sum_owner(
    owner: &self::ml_string::Text,
    constructor: &self::ml_string::Text,
    count: f64,
) -> self::ml_string::Text {
    if (((*constructor) == (self::ml_string::Text::from_units(vec![])))
        && (count == (self::read_rust_standard_sum_source_argument_count(owner))))
        && (count > 0f64)
    {
        owner.clone()
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_generic_type_alias_standard_sum_leaf(
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner: &self::ml_string::Text,
    count: f64,
    proven: bool,
    actual_proven: bool,
    actual_copy: bool,
    actual_wrappers: f64,
) -> bool {
    (((((((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100])))
        && (wrappers == 0f64))
        && proven)
        && actual_proven)
        && actual_copy)
        && (actual_wrappers == 0f64))
        && ((self::read_rust_generic_type_alias_standard_sum_owner(
            owner,
            &(self::ml_string::Text::from_units(vec![])),
            count,
        )) != (self::ml_string::Text::from_units(vec![])))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_generic_type_alias_owned_standard_sum_leaf(
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner: &self::ml_string::Text,
    count: f64,
    proven: bool,
    actual_proven: bool,
    actual_wrappers: f64,
    profile: &self::ml_string::Text,
) -> bool {
    (((((((*profile) == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100])))
        && ((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100]))))
        && (wrappers == 0f64))
        && proven)
        && actual_proven)
        && (actual_wrappers == 0f64))
        && ((self::read_rust_generic_type_alias_standard_sum_owner(
            owner,
            &(self::ml_string::Text::from_units(vec![])),
            count,
        )) != (self::ml_string::Text::from_units(vec![])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_standard_sum_depth_error(depth: f64) -> self::ml_string::Text {
    if depth > (self::read_rust_standard_sum_source_depth_limit()) {
        self::ml_string::Text::from_units(vec![
            103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 115, 116, 97, 110,
            100, 97, 114, 100, 32, 115, 117, 109, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105,
            116, 115, 32, 115, 104, 97, 114, 101, 100, 32, 100, 101, 112, 116, 104, 32, 98, 111,
            117, 110, 100,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_vector_owner(
    path: &self::ml_string::Text,
    shadowed: bool,
) -> self::ml_string::Text {
    if self::accept_rust_source_array_owner(path, shadowed) {
        self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 118, 101, 99, 58, 58, 86, 101, 99,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_generic_type_alias_vector_arguments(
    owner: &self::ml_string::Text,
    count: f64,
) -> bool {
    ((*owner)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 118, 101, 99, 58, 58, 86, 101, 99,
        ])))
        && (count == 1f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_generic_type_alias_owned_string_template(
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    children: f64,
    proven: bool,
    text_model: &self::ml_string::Text,
) -> bool {
    (((((((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
        && (wrappers == 0f64))
        && ((*owner_kind)
            == (self::ml_string::Text::from_units(vec![108, 105, 98, 114, 97, 114, 121]))))
        && ((*owner)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 115, 116, 114, 105, 110, 103, 58, 58, 83, 116, 114, 105,
                110, 103,
            ]))))
        && (children == 0f64))
        && proven)
        && ((*text_model)
            == (self::ml_string::Text::from_units(vec![
                111, 119, 110, 101, 100, 32, 115, 116, 114, 105, 110, 103,
            ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_generic_type_alias_vector_element_error(
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    children: f64,
    proven: bool,
    text_model: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let scalar = self::accept_rust_generic_type_alias_concrete_template(
            kind, wrappers, owner_kind, owner, bits, signed, children, proven,
        );
        {
            let string = self::accept_rust_generic_type_alias_owned_string_template(
                kind, wrappers, owner_kind, owner, children, proven, text_model,
            );
            if self::accept_rust_source_array_element(kind, (wrappers != 0f64))
                && (scalar || string)
            {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![
                    103, 101, 110, 101, 114, 105, 99, 32, 118, 101, 99, 116, 111, 114, 32, 97, 108,
                    105, 97, 115, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 110, 101, 101,
                    100, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 105, 114, 101, 99,
                    116, 32, 97, 100, 109, 105, 116, 116, 101, 100, 32, 115, 99, 97, 108, 97, 114,
                    44, 32, 85, 110, 105, 116, 32, 111, 114, 32, 111, 119, 110, 101, 100, 32, 83,
                    116, 114, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 111, 114, 105,
                    103, 105, 110,
                ])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_generic_type_alias_constructor_plan(
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    children: f64,
    element: &self::ml_string::Text,
    model: &self::ml_string::Text,
    proven: bool,
    actual_sum_owner: &self::ml_string::Text,
    actual_arguments: f64,
    actual_proven: bool,
    actual_copy: bool,
    member: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    if self::accept_rust_type_alias_empty_vector_member(
        kind, owner_kind, owner, children, element, model, proven, wrappers, member,
    ) {
        vec![
            self::ml_string::Text::from_units(vec![
                118, 101, 99, 116, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111,
                114,
            ]),
            self::ml_string::Text::from_units(vec![48]),
        ]
    } else if (actual_copy && proven) && actual_proven {
        self::standard_sum_constructor_shape(
            kind,
            wrappers,
            owner_kind,
            owner,
            children,
            actual_sum_owner,
            actual_arguments,
            member,
        )
    } else {
        Vec::<self::ml_string::Text>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn standard_sum_constructor_shape(
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    children: f64,
    actual_sum_owner: &self::ml_string::Text,
    actual_arguments: f64,
    member: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    {
        let arity = self::read_rust_standard_sum_source_constructor_arity(actual_sum_owner, member);
        if (((((((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100])))
            && (wrappers == 0f64))
            && ((*owner_kind)
                == (self::ml_string::Text::from_units(vec![108, 105, 98, 114, 97, 114, 121]))))
            && ((*owner) == (*actual_sum_owner)))
            && ((children).partial_cmp(&(actual_arguments)) == Some(std::cmp::Ordering::Equal)))
            && (actual_arguments
                == (self::read_rust_standard_sum_source_argument_count(actual_sum_owner))))
            && (arity >= 0f64)
        {
            vec![
                self::ml_string::Text::from_units(vec![
                    115, 117, 109, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                ]),
                {
                    let source_ml_source2_string_argument71 = arity;
                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                        source_ml_source2_string_argument71,
                    ))
                },
            ]
        } else {
            Vec::<self::ml_string::Text>::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_generic_type_alias_owned_constructor_plan(
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    children: f64,
    proven: bool,
    actual_sum_owner: &self::ml_string::Text,
    actual_arguments: f64,
    actual_proven: bool,
    profile: &self::ml_string::Text,
    member: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    if (((*profile) == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100])))
        && proven)
        && actual_proven
    {
        self::standard_sum_constructor_shape(
            kind,
            wrappers,
            owner_kind,
            owner,
            children,
            actual_sum_owner,
            actual_arguments,
            member,
        )
    } else {
        Vec::<self::ml_string::Text>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_generic_type_alias_header_constructor_plan(
    header: bool,
    provisional: bool,
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    children: f64,
    actual_owner: &self::ml_string::Text,
    actual_arguments: f64,
    actual_wrappers: f64,
    member: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    if self::accept_rust_generic_type_alias_header_standard_sum_origin(
        header,
        kind,
        wrappers,
        owner_kind,
        owner,
        children,
        actual_owner,
        actual_arguments,
        actual_wrappers,
        provisional,
    ) {
        self::standard_sum_constructor_shape(
            kind,
            wrappers,
            owner_kind,
            owner,
            children,
            actual_owner,
            actual_arguments,
            member,
        )
    } else {
        Vec::<self::ml_string::Text>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn group_after(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    position: f64,
) -> f64 {
    {
        let open = self::read_required_text_fact(values, position);
        {
            let stack = vec![if open == (self::ml_string::Text::from_units(vec![123])) {
                self::ml_string::Text::from_units(vec![125])
            } else if open == (self::ml_string::Text::from_units(vec![91])) {
                self::ml_string::Text::from_units(vec![93])
            } else {
                self::ml_string::Text::from_units(vec![41])
            }];
            {
                let at = position + 1f64;
                {
                    let source_ml_source2_s72 =
                        self::source_ml_source1_temporary_group_after_loop30(
                            kinds, values, stack, at,
                        );
                    match source_ml_source2_s72 {
                        self::MlSource1TemporaryGroupAfterLoop30Result::MlSource1TemporaryGroupAfterLoop30Done => {
                            -1f64
                        }
                        self::MlSource1TemporaryGroupAfterLoop30Result::MlSource1TemporaryGroupAfterLoop30Return(source_ml_source1_temporary_result30) => {
                            source_ml_source1_temporary_result30
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_declaration_plan(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    raws: &[self::ml_string::Text],
) -> Vec<self::ml_string::Text> {
    if ((kinds.len() as f64) != (values.len() as f64))
        || ((raws.len() as f64) != (values.len() as f64))
    {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                117, 110, 97, 108, 105, 103, 110, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32,
                116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32, 99, 111, 108, 117, 109, 110,
                115,
            ]),
        ]
    } else {
        {
            let result = vec![self::ml_string::Text::from_units(vec![
                97, 108, 105, 97, 115, 101, 115,
            ])];
            {
                let names = Vec::<self::ml_string::Text>::new();
                {
                    let owners = Vec::<self::ml_string::Text>::new();
                    {
                        let closes = Vec::<self::ml_string::Text>::new();
                        {
                            let position = 0f64;
                            {
                                let source_ml_source2_s73 = self::source_ml_source1_temporary_read_rust_generic_type_alias_declaration_plan_loop32(kinds, values, raws, result, names, owners, closes, position);
                                match source_ml_source2_s73 {
                                    self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Done(result_2) => {
                                        result_2
                                    }
                                    self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Return(source_ml_source1_temporary_result32) => {
                                        source_ml_source1_temporary_result32
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_expansion_error(
    declaration: &self::ml_string::Text,
    declarations: &[self::ml_string::Text],
    expansions: f64,
) -> self::ml_string::Text {
    if (*declaration) == (self::ml_string::Text::from_units(vec![])) {
        self::ml_string::Text::from_units(vec![
            103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 101, 120, 112, 97,
            110, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108,
            32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114,
            115,
        ])
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source2_s74 = self::source_ml_source1_temporary_read_rust_generic_type_alias_expansion_error_loop37(declarations, at);
                match source_ml_source2_s74 {
                    self::MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Result::MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Done => {
                        if (expansions < 0f64) || ((expansions % 1f64) != 0f64) {
                            self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 99, 111, 117, 110, 116])
                        } else {
                            self::read_rust_type_alias_expansion_error((declarations.len() as f64), expansions, self::ml_array::includes(declarations, declaration, 0f64, false, |a, b| a == b))
                        }
                    }
                    self::MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Result::MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Return(source_ml_source1_temporary_result37) => {
                        source_ml_source1_temporary_result37.clone()
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDependencyFunction1Loop1Result {
    MlSource1TemporaryDependencyFunction1Loop1Done,
    MlSource1TemporaryDependencyFunction1Loop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_dependency_function1_loop1(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut stack: Vec<self::ml_string::Text>,
    mut at: f64,
) -> self::MlSource1TemporaryDependencyFunction1Loop1Result {
    loop {
        return if at < (values.len() as f64) {
            if (self::read_required_text_fact(kinds, at))
                != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))
            {
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
                    if ((value == (self::ml_string::Text::from_units(vec![123])))
                        || (value == (self::ml_string::Text::from_units(vec![91]))))
                        || (value == (self::ml_string::Text::from_units(vec![40])))
                    {
                        {
                            let stack_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![
                                        if value == (self::ml_string::Text::from_units(vec![123])) {
                                            self::ml_string::Text::from_units(vec![125])
                                        } else if value
                                            == (self::ml_string::Text::from_units(vec![91]))
                                        {
                                            self::ml_string::Text::from_units(vec![93])
                                        } else {
                                            self::ml_string::Text::from_units(vec![41])
                                        },
                                    ],
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
                        if ((value == (self::ml_string::Text::from_units(vec![125])))
                            || (value == (self::ml_string::Text::from_units(vec![93]))))
                            || (value == (self::ml_string::Text::from_units(vec![41])))
                        {
                            if value
                                == (self::read_required_text_fact(
                                    &(stack),
                                    ((stack.len() as f64) - 1f64),
                                ))
                            {
                                {
                                    let stack_3 = self::ml_array::slice(&stack, 0f64, -1f64);
                                    if (stack_3.len() as f64) == 0f64 {
                                        self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Return(at + 1f64)
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
                                self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Return(-1f64)
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
            self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result {
    MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Done(Vec<self::ml_string::Text>),
    MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Return(Vec<self::ml_string::Text>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Result {
    MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Done(f64),
    MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Return(Vec<self::ml_string::Text>),
}
