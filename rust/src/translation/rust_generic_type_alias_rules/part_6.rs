// Generated from complete js/src/translation/rust-generic-type-alias-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Result;
use super::accept_rust_function_pointer_safe_source;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::read_required_text_fact;
use super::read_rust_function_pointer_parameter_limit;
use super::read_rust_generic_type_alias_standard_sum_owner;
use super::source_ml_source1_temporary_parameter_directory_loop16;
use super::source_ml_source1_temporary_read_rust_function_pointer_type_error_loop13;
use super::source_ml_source1_temporary_read_rust_generic_type_alias_binding_plan_loop27;
use super::source_ml_source1_temporary_read_rust_generic_type_alias_binding_plan_loop28;
use super::source_ml_source1_temporary_read_rust_generic_type_alias_default_argument_plan_loop24;
use super::source_ml_source1_temporary_read_rust_generic_type_alias_default_substitution_plan_loop23;
use super::source_ml_source1_temporary_read_rust_generic_type_alias_header_substitution_plan_loop26;
use super::source_ml_source1_temporary_read_rust_generic_type_alias_parameter_directory_loop17;
use super::source_ml_source1_temporary_read_rust_generic_type_alias_parameter_plan_loop14;
use super::source_ml_source1_temporary_read_rust_generic_type_alias_substitution_plan_loop25;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_function_pointer_type_error(
    ordinary: bool,
    parameters: &[bool],
    result: bool,
) -> self::ml_string::Text {
    if ordinary {
        if (parameters.len() as f64) > (self::read_rust_function_pointer_parameter_limit()) {
            self::ml_string::Text::from_units(vec![
                116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 111, 105, 110,
                116, 101, 114, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 101, 120, 99,
                101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 111, 117, 114, 99, 101, 32, 98,
                117, 100, 103, 101, 116,
            ])
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source2_s45 = self::source_ml_source1_temporary_read_rust_function_pointer_type_error_loop13(parameters, index);
                    match source_ml_source2_s45 {
                        self::MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Result::MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Done => {
                            if result {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 112, 111, 105, 110, 116, 101, 114, 32, 114, 101, 115, 117, 108, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 114, 101, 99, 116, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 67, 111, 112, 121, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 115])
                            }
                        }
                        self::MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Result::MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Return(source_ml_source1_temporary_result13) => {
                            source_ml_source1_temporary_result13
                        }
                    }
                }
            }
        }
    } else {
        self::ml_string::Text::from_units(vec![
            102, 117, 110, 99, 116, 105, 111, 110, 32, 112, 111, 105, 110, 116, 101, 114, 115, 32,
            114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 115, 101, 112, 97,
            114, 97, 116, 101, 32, 97, 99, 116, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32,
            112, 114, 111, 102, 105, 108, 101,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_function_pointer_header_type_error(
    ordinary: bool,
    parameters: &[bool],
    result: bool,
    header: bool,
) -> self::ml_string::Text {
    if (header && ordinary)
        && ((parameters.len() as f64) <= (self::read_rust_function_pointer_parameter_limit()))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::read_rust_function_pointer_type_error(ordinary, parameters, result)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_function_pointer_header_copy(
    kind: &self::ml_string::Text,
    layers: f64,
    header: bool,
) -> bool {
    (((*kind) == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))
        && (layers == 0f64))
        && header
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_function_pointer_coercion_error(
    pointer: bool,
    actual: &self::ml_string::Text,
    closure: bool,
    captures: f64,
    proven: bool,
) -> self::ml_string::Text {
    if pointer {
        if ((*actual)
            != (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))
            || !proven
        {
            self::ml_string::Text::from_units(vec![
                97, 32, 112, 111, 105, 110, 116, 101, 114, 32, 97, 114, 103, 117, 109, 101, 110,
                116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 115, 111, 117, 114, 99, 101, 32, 112,
                114, 111, 111, 102,
            ])
        } else if closure && (captures != 0f64) {
            self::ml_string::Text::from_units(vec![
                99, 97, 112, 116, 117, 114, 105, 110, 103, 32, 99, 108, 111, 115, 117, 114, 101,
                115, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 101, 114, 99, 101, 32, 116, 111,
                32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 112, 111, 105, 110, 116, 101, 114,
                115,
            ])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_function_pointer_observation_error(
    kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])) {
        self::ml_string::Text::from_units(vec![
            99, 97, 108, 108, 97, 98, 108, 101, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32,
            97, 100, 100, 114, 101, 115, 115, 32, 97, 110, 100, 32, 110, 117, 109, 101, 114, 105,
            99, 32, 99, 97, 115, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104,
            101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 114,
            101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_retained_source_signature_model(model: &self::ml_string::Text) -> bool {
    (*model)
        == (self::ml_string::Text::from_units(vec![
            82, 117, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105,
            111, 110, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 50,
        ]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_rust_retained_source_signature_model(
    model: &self::ml_string::Text,
    retain: bool,
) -> self::ml_string::Text {
    {
        let ml_library_inequality_1 = (*model) != (self::ml_string::Text::from_units(vec![]));
        if ml_library_inequality_1 {
            model.clone()
        } else if retain {
            self::ml_string::Text::from_units(vec![
                82, 117, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116,
                105, 111, 110, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 50,
            ])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_retained_function_owner(
    model: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    type_: &self::ml_string::Text,
    paths: f64,
    children: f64,
    parameters: f64,
    reference: bool,
) -> bool {
    (((((((self::accept_rust_retained_source_signature_model(model)
        && ((*owner)
            == (self::ml_string::Text::from_units(vec![
                102, 117, 110, 99, 116, 105, 111, 110,
            ]))))
        && ((*type_)
            == (self::ml_string::Text::from_units(vec![
                102, 117, 110, 99, 116, 105, 111, 110,
            ]))))
        && (paths == 0f64))
        && self::ml_math::is_integer(parameters))
        && (parameters >= 0f64))
        && (parameters <= 100_000f64))
        && (children == (parameters + 1f64)))
        && !reference
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_parameter_name(name: &self::ml_string::Text, raw_: &self::ml_string::Text) -> bool {
    if (((*name) == (self::ml_string::Text::from_units(vec![])))
        || ((*name) == (self::ml_string::Text::from_units(vec![95]))))
        || self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![83, 101, 108, 102]),
                self::ml_string::Text::from_units(vec![115, 101, 108, 102]),
                self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]),
                self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101]),
            ],
            name,
            0f64,
            false,
            |a, b| a == b,
        )
    {
        false
    } else if raw_.starts_with(&self::ml_string::Text::from_units(vec![114, 35])) {
        true
    } else if (*raw_) == (self::ml_string::Text::from_units(vec![])) {
        false
    } else {
        !(self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![97, 115]),
                self::ml_string::Text::from_units(vec![97, 115, 121, 110, 99]),
                self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116]),
                self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107]),
                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]),
                self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101]),
                self::ml_string::Text::from_units(vec![100, 121, 110]),
                self::ml_string::Text::from_units(vec![101, 108, 115, 101]),
                self::ml_string::Text::from_units(vec![101, 110, 117, 109]),
                self::ml_string::Text::from_units(vec![101, 120, 116, 101, 114, 110]),
                self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101]),
                self::ml_string::Text::from_units(vec![102, 110]),
                self::ml_string::Text::from_units(vec![102, 111, 114]),
                self::ml_string::Text::from_units(vec![105, 102]),
                self::ml_string::Text::from_units(vec![105, 109, 112, 108]),
                self::ml_string::Text::from_units(vec![105, 110]),
                self::ml_string::Text::from_units(vec![108, 101, 116]),
                self::ml_string::Text::from_units(vec![108, 111, 111, 112]),
                self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104]),
                self::ml_string::Text::from_units(vec![109, 111, 100]),
                self::ml_string::Text::from_units(vec![109, 111, 118, 101]),
                self::ml_string::Text::from_units(vec![109, 117, 116]),
                self::ml_string::Text::from_units(vec![112, 117, 98]),
                self::ml_string::Text::from_units(vec![114, 101, 102]),
                self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]),
                self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99]),
                self::ml_string::Text::from_units(vec![115, 116, 114, 117, 99, 116]),
                self::ml_string::Text::from_units(vec![116, 114, 97, 105, 116]),
                self::ml_string::Text::from_units(vec![116, 114, 117, 101]),
                self::ml_string::Text::from_units(vec![116, 121, 112, 101]),
                self::ml_string::Text::from_units(vec![117, 110, 115, 97, 102, 101]),
                self::ml_string::Text::from_units(vec![117, 115, 101]),
                self::ml_string::Text::from_units(vec![119, 104, 101, 114, 101]),
                self::ml_string::Text::from_units(vec![119, 104, 105, 108, 101]),
                self::ml_string::Text::from_units(vec![97, 98, 115, 116, 114, 97, 99, 116]),
                self::ml_string::Text::from_units(vec![98, 101, 99, 111, 109, 101]),
                self::ml_string::Text::from_units(vec![98, 111, 120]),
                self::ml_string::Text::from_units(vec![100, 111]),
                self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108]),
                self::ml_string::Text::from_units(vec![103, 101, 110]),
                self::ml_string::Text::from_units(vec![109, 97, 99, 114, 111]),
                self::ml_string::Text::from_units(vec![111, 118, 101, 114, 114, 105, 100, 101]),
                self::ml_string::Text::from_units(vec![112, 114, 105, 118]),
                self::ml_string::Text::from_units(vec![116, 114, 121]),
                self::ml_string::Text::from_units(vec![116, 121, 112, 101, 111, 102]),
                self::ml_string::Text::from_units(vec![117, 110, 115, 105, 122, 101, 100]),
                self::ml_string::Text::from_units(vec![118, 105, 114, 116, 117, 97, 108]),
                self::ml_string::Text::from_units(vec![121, 105, 101, 108, 100]),
            ],
            name,
            0f64,
            false,
            |a, b| a == b,
        ))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_parameter_plan(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    raws: &[self::ml_string::Text],
    position: f64,
) -> Vec<self::ml_string::Text> {
    if ((kinds.len() as f64) != (values.len() as f64))
        || ((raws.len() as f64) != (values.len() as f64))
    {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                117, 110, 97, 108, 105, 103, 110, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99,
                32, 97, 108, 105, 97, 115, 32, 116, 111, 107, 101, 110, 32, 99, 111, 108, 117, 109,
                110, 115,
            ]),
        ]
    } else if ((position < 0f64) || ((position % 1f64) != 0f64))
        || (position >= (values.len() as f64))
    {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                109, 105, 115, 115, 105, 110, 103, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97,
                108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 108, 105,
                115, 116,
            ]),
        ]
    } else if ((self::read_required_text_fact(kinds, position))
        != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
        || ((self::read_required_text_fact(values, position))
            != (self::ml_string::Text::from_units(vec![60])))
    {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                109, 105, 115, 115, 105, 110, 103, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97,
                108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 108, 105,
                115, 116,
            ]),
        ]
    } else {
        {
            let names = Vec::<self::ml_string::Text>::new();
            {
                let at = position + 1f64;
                {
                    let source_ml_source2_s46 = self::source_ml_source1_temporary_read_rust_generic_type_alias_parameter_plan_loop14(kinds, values, raws, names, at);
                    match source_ml_source2_s46 {
                                self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Done => {
                                    vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 108, 105, 115, 116])]
                                }
                                self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Return(source_ml_source1_temporary_result14) => {
                                    source_ml_source1_temporary_result14
                                }
                            }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn parameter_directory(
    names: &[self::ml_string::Text],
    starts: &[self::ml_string::Text],
    ends: &[self::ml_string::Text],
    offsets: &[self::ml_string::Text],
    close: f64,
    remainder: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    if ((*remainder) != (self::ml_string::Text::from_units(vec![])))
        && ((*remainder) != (self::ml_string::Text::from_units(vec![61])))
    {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                109, 97, 108, 102, 111, 114, 109, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99,
                32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 100,
                101, 108, 105, 109, 105, 116, 101, 114,
            ]),
        ]
    } else {
        {
            let result = vec![
                self::ml_string::Text::from_units(vec![
                    112, 97, 114, 97, 109, 101, 116, 101, 114, 115,
                ]),
                {
                    let source_ml_source2_string_argument47 = close;
                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                        source_ml_source2_string_argument47,
                    ))
                },
                remainder.clone(),
                {
                    let source_ml_source2_string_argument48 = names.len() as f64;
                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                        source_ml_source2_string_argument48,
                    ))
                },
            ];
            {
                let at = 0f64;
                {
                    self::source_ml_source1_temporary_parameter_directory_loop16(
                        names, starts, ends, offsets, result, at,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_parameter_directory(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    raws: &[self::ml_string::Text],
    position: f64,
) -> Vec<self::ml_string::Text> {
    if ((kinds.len() as f64) != (values.len() as f64))
        || ((raws.len() as f64) != (values.len() as f64))
    {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                117, 110, 97, 108, 105, 103, 110, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99,
                32, 97, 108, 105, 97, 115, 32, 116, 111, 107, 101, 110, 32, 99, 111, 108, 117, 109,
                110, 115,
            ]),
        ]
    } else if ((((position < 0f64) || ((position % 1f64) != 0f64))
        || (position >= (values.len() as f64)))
        || ((self::read_required_text_fact(kinds, position))
            != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))))
        || ((self::read_required_text_fact(values, position))
            != (self::ml_string::Text::from_units(vec![60])))
    {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                109, 105, 115, 115, 105, 110, 103, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97,
                108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 108, 105,
                115, 116,
            ]),
        ]
    } else {
        {
            let names = Vec::<self::ml_string::Text>::new();
            {
                let starts = Vec::<self::ml_string::Text>::new();
                {
                    let ends = Vec::<self::ml_string::Text>::new();
                    {
                        let offsets = Vec::<self::ml_string::Text>::new();
                        {
                            let at = position + 1f64;
                            {
                                let default_seen = false;
                                {
                                    let source_ml_source2_s49 = self::source_ml_source1_temporary_read_rust_generic_type_alias_parameter_directory_loop17(kinds, values, raws, names, starts, ends, offsets, at, default_seen);
                                    match source_ml_source2_s49 {
                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Done => {
                                                vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 108, 105, 115, 116])]
                                            }
                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Return(source_ml_source1_temporary_result17) => {
                                                source_ml_source1_temporary_result17
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_default_substitution_plan(
    parameters: &[self::ml_string::Text],
    position: f64,
    count: f64,
    occurrences: &[self::ml_string::Text],
) -> Vec<self::ml_string::Text> {
    {
        let directory = self::read_rust_generic_type_alias_binding_plan(
            parameters,
            (parameters.len() as f64),
            &(Vec::<self::ml_string::Text>::new()),
        );
        if {
            let source_ml_source2_comparison_operand50 =
                self::ml_array::optional_at(&directory, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand51 =
                    self::ml_string::Text::from_units(vec![
                        115, 117, 98, 115, 116, 105, 116, 117, 116, 105, 111, 110,
                    ]);
                !(match (
                    source_ml_source2_comparison_operand50,
                    source_ml_source2_comparison_operand51,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })
            }
        } {
            directory
        } else if (((position < 0f64) || ((position % 1f64) != 0f64))
            || (position >= (parameters.len() as f64)))
            || (count != position)
        {
            vec![
                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                self::ml_string::Text::from_units(vec![
                    103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 100, 101, 102,
                    97, 117, 108, 116, 32, 110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121,
                    32, 101, 97, 114, 108, 105, 101, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112,
                    97, 114, 97, 109, 101, 116, 101, 114, 32, 98, 105, 110, 100, 105, 110, 103,
                ]),
            ]
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s52 = self::source_ml_source1_temporary_read_rust_generic_type_alias_default_substitution_plan_loop23(parameters, position, occurrences, at);
                    match source_ml_source2_s52 {
                            self::MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Result::MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Done => {
                                if position == 0f64 {
                                    vec![self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 105, 116, 117, 116, 105, 111, 110])]
                                } else {
                                    self::read_rust_generic_type_alias_binding_plan(&(self::ml_array::slice(parameters, 0f64, position)), count, occurrences)
                                }
                            }
                            self::MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Result::MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Return(source_ml_source1_temporary_result23) => {
                                source_ml_source1_temporary_result23
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_default_argument_plan(
    defaults: &[bool],
    count: f64,
    explicit: bool,
) -> Vec<self::ml_string::Text> {
    if ((defaults.len() as f64) == 0f64) || ((defaults.len() as f64) > 64f64) {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 110, 101, 101,
                100, 115, 32, 97, 32, 98, 111, 117, 110, 100, 101, 100, 32, 101, 120, 112, 108,
                105, 99, 105, 116, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 100, 105,
                114, 101, 99, 116, 111, 114, 121,
            ]),
        ]
    } else if ((count < 0f64) || ((count % 1f64) != 0f64)) || (count > (defaults.len() as f64)) {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 97, 114, 103, 117,
                109, 101, 110, 116, 32, 97, 114, 105, 116, 121, 32, 100, 105, 102, 102, 101, 114,
                115, 32, 102, 114, 111, 109, 32, 105, 116, 115, 32, 100, 101, 99, 108, 97, 114, 97,
                116, 105, 111, 110,
            ]),
        ]
    } else {
        {
            let result = vec![self::ml_string::Text::from_units(vec![
                97, 114, 103, 117, 109, 101, 110, 116, 115,
            ])];
            {
                let at = count;
                {
                    let source_ml_source2_s53 = self::source_ml_source1_temporary_read_rust_generic_type_alias_default_argument_plan_loop24(defaults, explicit, result, at);
                    match source_ml_source2_s53 {
                            self::MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Result::MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Done(result_2) => {
                                result_2
                            }
                            self::MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Result::MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Return(source_ml_source1_temporary_result24) => {
                                source_ml_source1_temporary_result24
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_generic_type_alias_concrete_identifier(
    parameters: &[self::ml_string::Text],
    name: &self::ml_string::Text,
    qualified: bool,
) -> bool {
    qualified || !(self::ml_array::includes(parameters, name, 0f64, false, |a, b| a == b))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_default_access_head_limit() -> f64 {
    16f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_default_owner_phase(
    collecting: bool,
    discovery: bool,
) -> self::ml_string::Text {
    if collecting && discovery {
        self::ml_string::Text::from_units(vec![100, 101, 102, 101, 114])
    } else {
        self::ml_string::Text::from_units(vec![112, 114, 101, 112, 97, 114, 101])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_generic_type_alias_function_template_head(
    value: &self::ml_string::Text,
    raw_: &self::ml_string::Text,
) -> bool {
    self::accept_rust_function_pointer_safe_source(value, raw_)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_generic_type_alias_function_parameter_name(
    value: &self::ml_string::Text,
    raw_: &self::ml_string::Text,
) -> bool {
    if (*value) == (self::ml_string::Text::from_units(vec![95])) {
        (*raw_) == (self::ml_string::Text::from_units(vec![95]))
    } else {
        self::accept_parameter_name(value, raw_)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_function_template_error(count: f64) -> self::ml_string::Text {
    if !(self::ml_math::is_integer(count)) || (count < 0f64) {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 102, 117,
            110, 99, 116, 105, 111, 110, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 112, 97,
            114, 97, 109, 101, 116, 101, 114, 32, 99, 111, 117, 110, 116,
        ])
    } else if count > (self::read_rust_function_pointer_parameter_limit()) {
        self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 111, 105, 110, 116,
            101, 114, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 101, 120, 99, 101, 101,
            100, 115, 32, 105, 116, 115, 32, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103,
            101, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_substitution_plan(
    parameters: &[self::ml_string::Text],
    arguments_proven: &[bool],
    occurrences: &[self::ml_string::Text],
) -> Vec<self::ml_string::Text> {
    {
        let binding = self::read_rust_generic_type_alias_binding_plan(
            parameters,
            (arguments_proven.len() as f64),
            occurrences,
        );
        if {
            let source_ml_source2_comparison_operand54 =
                self::ml_array::optional_at(&binding, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand55 =
                    self::ml_string::Text::from_units(vec![
                        115, 117, 98, 115, 116, 105, 116, 117, 116, 105, 111, 110,
                    ]);
                !(match (
                    source_ml_source2_comparison_operand54,
                    source_ml_source2_comparison_operand55,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })
            }
        } {
            binding
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s56 = self::source_ml_source1_temporary_read_rust_generic_type_alias_substitution_plan_loop25(arguments_proven, at);
                    match source_ml_source2_s56 {
                        self::MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Result::MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Done => {
                            binding
                        }
                        self::MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Result::MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Return(source_ml_source1_temporary_result25) => {
                            source_ml_source1_temporary_result25
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_header_substitution_plan(
    parameters: &[self::ml_string::Text],
    header: bool,
    arguments_typed: &[bool],
    arguments_proven: &[bool],
    arguments_provisional: &[bool],
    occurrences: &[self::ml_string::Text],
) -> Vec<self::ml_string::Text> {
    if (!header || ((arguments_typed.len() as f64) != (arguments_proven.len() as f64)))
        || ((arguments_typed.len() as f64) != (arguments_provisional.len() as f64))
    {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 97, 114, 103, 117,
                109, 101, 110, 116, 115, 32, 110, 101, 101, 100, 32, 99, 111, 109, 112, 108, 101,
                116, 101, 32, 99, 104, 101, 99, 107, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32,
                111, 114, 105, 103, 105, 110, 115,
            ]),
        ]
    } else {
        {
            let plan = self::read_rust_generic_type_alias_binding_plan(
                parameters,
                (arguments_typed.len() as f64),
                occurrences,
            );
            if {
                let source_ml_source2_comparison_operand57 =
                    self::ml_array::optional_at(&plan, self::ml_array::number_index(0f64));
                {
                    let source_ml_source2_comparison_operand58 =
                        self::ml_string::Text::from_units(vec![
                            115, 117, 98, 115, 116, 105, 116, 117, 116, 105, 111, 110,
                        ]);
                    !(match (
                        source_ml_source2_comparison_operand57,
                        source_ml_source2_comparison_operand58,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                plan
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s59 = self::source_ml_source1_temporary_read_rust_generic_type_alias_header_substitution_plan_loop26(arguments_typed, arguments_proven, arguments_provisional, at);
                        match source_ml_source2_s59 {
                            self::MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Result::MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Done => {
                                plan
                            }
                            self::MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Result::MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Return(source_ml_source1_temporary_result26) => {
                                source_ml_source1_temporary_result26
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_generic_type_alias_header_standard_sum_origin(
    header: bool,
    kind: &self::ml_string::Text,
    wrappers: f64,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    children: f64,
    actual_owner: &self::ml_string::Text,
    actual_arguments: f64,
    actual_wrappers: f64,
    provisional: bool,
) -> bool {
    (((((((header && provisional)
        && ((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100]))))
        && (wrappers == 0f64))
        && (actual_wrappers == 0f64))
        && ((*owner_kind)
            == (self::ml_string::Text::from_units(vec![108, 105, 98, 114, 97, 114, 121]))))
        && ((*owner) == (*actual_owner)))
        && ((children).partial_cmp(&(actual_arguments)) == Some(std::cmp::Ordering::Equal)))
        && ((self::read_rust_generic_type_alias_standard_sum_owner(
            actual_owner,
            &(self::ml_string::Text::from_units(vec![])),
            actual_arguments,
        )) != (self::ml_string::Text::from_units(vec![])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_binding_plan(
    parameters: &[self::ml_string::Text],
    count: f64,
    occurrences: &[self::ml_string::Text],
) -> Vec<self::ml_string::Text> {
    if ((parameters.len() as f64) == 0f64) || ((parameters.len() as f64) > 64f64) {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 110, 101, 101,
                100, 115, 32, 97, 32, 98, 111, 117, 110, 100, 101, 100, 32, 101, 120, 112, 108,
                105, 99, 105, 116, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 100, 105,
                114, 101, 99, 116, 111, 114, 121,
            ]),
        ]
    } else {
        let ml_library_inequality_1 = (parameters.len() as f64) != count;
        if ml_library_inequality_1 {
            vec![
                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                self::ml_string::Text::from_units(vec![
                    103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 97, 114, 103,
                    117, 109, 101, 110, 116, 32, 97, 114, 105, 116, 121, 32, 100, 105, 102, 102,
                    101, 114, 115, 32, 102, 114, 111, 109, 32, 105, 116, 115, 32, 100, 101, 99,
                    108, 97, 114, 97, 116, 105, 111, 110,
                ]),
            ]
        } else if (occurrences.len() as f64) > 262_144f64 {
            vec![
                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                self::ml_string::Text::from_units(vec![
                    103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 111, 99, 99,
                    117, 114, 114, 101, 110, 99, 101, 32, 108, 105, 109, 105, 116, 32, 101, 120,
                    99, 101, 101, 100, 101, 100,
                ]),
            ]
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s60 = self::source_ml_source1_temporary_read_rust_generic_type_alias_binding_plan_loop27(parameters, at);
                    match source_ml_source2_s60 {
                            self::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Result::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Done => {
                                {
                                    let plan = vec![self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 105, 116, 117, 116, 105, 111, 110])];
                                    {
                                        let at_2 = 0f64;
                                        {
                                            let source_ml_source2_s61 = self::source_ml_source1_temporary_read_rust_generic_type_alias_binding_plan_loop28(parameters, occurrences, plan, at_2);
                                            match source_ml_source2_s61 {
                                                self::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Result::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Done(plan_2) => {
                                                    plan_2
                                                }
                                                self::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Result::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Return(source_ml_source1_temporary_result28) => {
                                                    source_ml_source1_temporary_result28
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Result::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Return(source_ml_source1_temporary_result28_2) => {
                                source_ml_source1_temporary_result28_2
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_generic_type_alias_template_depth_error(depth: f64) -> self::ml_string::Text {
    if depth >= 128f64 {
        self::ml_string::Text::from_units(vec![
            103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 116, 101, 109, 112,
            108, 97, 116, 101, 32, 100, 101, 112, 116, 104, 32, 108, 105, 109, 105, 116, 32, 101,
            120, 99, 101, 101, 100, 101, 100,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}
