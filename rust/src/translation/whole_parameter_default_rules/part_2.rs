// Generated from complete js/src/translation/whole-parameter-default-rules.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Result;
use super::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Result;
use super::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Result;
use super::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Result;
use super::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result;
use super::accept_source_private_identifier;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::source_ml_source1_temporary_accept_java_script_source_binding_name_loop3;
use super::source_ml_source1_temporary_find_java_script_array_formal_pattern_loop4;
use super::source_ml_source1_temporary_find_java_script_pattern_default_expression_end_loop11;
use super::source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop5;
use super::source_ml_source1_temporary_read_source_identifier_prefix_loop2;
use super::source_ml_source1_temporary_read_source_identifier_prefix_ordinal_loop1;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_identifier_prefix_ordinal(name: &self::ml_string::Text) -> f64 {
    if (name.starts_with(&self::ml_string::Text::from_units(vec![
        109, 108, 95, 115, 111, 117, 114, 99, 101,
    ]))) {
        {
            let index = 9f64;
            {
                let ordinal = 0f64;
                if ((({
                    let source_ml_source2_string_unit_operand10 = name.clone();
                    {
                        let source_ml_source2_string_unit_operand11 = index;
                        (std::rc::Rc::new(
                            move |text: self::ml_string::Text, position: f64| -> f64 {
                                {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                        f64::NAN
                                    } else {
                                        ({
                                    let source_ml_source2_string_unit_operand17 = {
                                        let source_ml_source2_string_range_operand12 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand13 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand14 = index_2 + 1f64;
                                                {
                                                    let source_ml_source2_string_range_position15 = source_ml_source2_string_range_operand13;
                                                    {
                                                        let source_ml_source2_string_range_position16 = source_ml_source2_string_range_operand14;
                                                        source_ml_source2_string_range_operand12.slice(source_ml_source2_string_range_position15, source_ml_source2_string_range_position16)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand18 = 0f64;
                                        source_ml_source2_string_unit_operand17.code_point_at(source_ml_source2_string_unit_operand18)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source2_string_unit_operand10,
                            source_ml_source2_string_unit_operand11,
                        )
                    }
                }) < 49f64)
                    || (({
                        let source_ml_source2_string_unit_operand28 = name.clone();
                        {
                            let source_ml_source2_string_unit_operand29 = index;
                            (std::rc::Rc::new(
                                move |text: self::ml_string::Text, position: f64| -> f64 {
                                    {
                                        let index_2 = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                            f64::NAN
                                        } else {
                                            ({
                                    let source_ml_source2_string_unit_operand35 = {
                                        let source_ml_source2_string_range_operand30 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand31 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand32 = index_2 + 1f64;
                                                {
                                                    let source_ml_source2_string_range_position33 = source_ml_source2_string_range_operand31;
                                                    {
                                                        let source_ml_source2_string_range_position34 = source_ml_source2_string_range_operand32;
                                                        source_ml_source2_string_range_operand30.slice(source_ml_source2_string_range_position33, source_ml_source2_string_range_position34)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand36 = 0f64;
                                        source_ml_source2_string_unit_operand35.code_point_at(source_ml_source2_string_unit_operand36)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand28,
                                source_ml_source2_string_unit_operand29,
                            )
                        }
                    }) > 57f64))
                    || (index >= (name.length()))
                {
                    -1f64
                } else {
                    {
                        let source_ml_source2_s37 = self::source_ml_source1_temporary_read_source_identifier_prefix_ordinal_loop1(name, index, ordinal);
                        match source_ml_source2_s37 {
                            self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Done(index_2, ordinal_2) => {
                                {
let ml_product_condition_1 = {
                                    let source_ml_source2_string_unit_operand47 = name.clone();
                                    {
                                        let source_ml_source2_string_unit_operand48 = index_2;
                                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                            let index_3 = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if (index_3 < 0f64) || (index_3 >= (text.length())) {
                                                f64::NAN
                                            } else {
                                                ({
                                                    let source_ml_source2_string_unit_operand54 = {
                                                        let source_ml_source2_string_range_operand49 = text.clone();
                                                        {
                                                            let source_ml_source2_string_range_operand50 = index_3;
                                                            {
                                                                let source_ml_source2_string_range_operand51 = index_3 + 1f64;
                                                                {
                                                                    let source_ml_source2_string_range_position52 = source_ml_source2_string_range_operand50;
                                                                    {
                                                                        let source_ml_source2_string_range_position53 = source_ml_source2_string_range_operand51;
                                                                        source_ml_source2_string_range_operand49.slice(source_ml_source2_string_range_position52, source_ml_source2_string_range_position53)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_string_unit_operand55 = 0f64;
                                                        source_ml_source2_string_unit_operand54.code_point_at(source_ml_source2_string_unit_operand55)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source2_string_unit_operand47, source_ml_source2_string_unit_operand48)
                                    }
                                };
if ml_product_condition_1 {
                                    ordinal_2
                                } else {
                                    -1f64
                                }
}
                            }
                            self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1
                            }
                        }
                    }
                }
            }
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_identifier_prefix(
    private_names: bool,
    ordinals: &[f64],
) -> self::ml_string::Text {
    if private_names {
        {
            let next = 1f64;
            {
                let previous = 0f64;
                {
                    let index = 0f64;
                    {
                        let next_2 =
                            self::source_ml_source1_temporary_read_source_identifier_prefix_loop2(
                                ordinals, next, previous, index,
                            );
                        ((self::ml_string::Text::from_units(vec![
                            109, 108, 95, 115, 111, 117, 114, 99, 101,
                        ]))
                        .concat(&self::ml_string::Text::from_text(
                            &self::ml_number::js_number(next_2),
                        )))
                        .concat(&self::ml_string::Text::from_units(vec![95]))
                    }
                }
            }
        }
    } else {
        self::ml_string::Text::from_units(vec![109, 108, 95])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_source_target_identifier(name: &self::ml_string::Text) -> self::ml_string::Text {
    if self::accept_source_private_identifier(name) {
        (self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 95])).concat(name)
    } else {
        name.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_source_generated_identifier(
    prefix: &self::ml_string::Text,
    role: &self::ml_string::Text,
    ordinal: f64,
) -> self::ml_string::Text {
    {
        assert!(
            !(!(self::ml_math::is_safe_integer(ordinal)) || (ordinal < 0f64)),
            "{}",
            "source identifier generation ordinal exceeds its exact range"
        );

        (prefix.concat(
            &if role.starts_with(&self::ml_string::Text::from_units(vec![109, 108, 95])) {
                {
                    let source_ml_source2_string_range_operand56 = role.clone();
                    {
                        let source_ml_source2_string_range_operand57 = 3f64;
                        {
                            let source_ml_source2_string_range_position58 =
                                source_ml_source2_string_range_operand57;
                            {
                                let source_ml_source2_string_range_position59 = f64::INFINITY;
                                source_ml_source2_string_range_operand56.slice(
                                    source_ml_source2_string_range_position58,
                                    source_ml_source2_string_range_position59,
                                )
                            }
                        }
                    }
                }
            } else {
                role.clone()
            },
        ))
        .concat(&self::ml_string::Text::from_text(
            &self::ml_number::js_number(ordinal),
        ))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_source_cell_parameter(
    prefix: &self::ml_string::Text,
    name: &self::ml_string::Text,
    reference: bool,
) -> self::ml_string::Text {
    if reference {
        (prefix.concat(&self::ml_string::Text::from_units(vec![
            99, 101, 108, 108, 95,
        ])))
        .concat(name)
    } else {
        name.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_source_binding_name(
    kind: &self::ml_string::Text,
    name: &self::ml_string::Text,
    duplicate: bool,
    reserved: &[self::ml_string::Text],
) -> bool {
    if (((*kind)
        != (self::ml_string::Text::from_units(vec![
            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
        ])))
        || ((*name) == (self::ml_string::Text::from_units(vec![]))))
        || duplicate
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s60 =
                    self::source_ml_source1_temporary_accept_java_script_source_binding_name_loop3(
                        name, reserved, index,
                    );
                match source_ml_source2_s60 {
                    self::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Result::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Done => {
                        true
                    }
                    self::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Result::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Return(source_ml_source1_temporary_result3) => {
                        source_ml_source1_temporary_result3
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_optional_array_read(
    language: &self::ml_string::Text,
    optional: bool,
    proven: bool,
    expected_optional: bool,
) -> bool {
    (optional
        || (((*language)
            == (self::ml_string::Text::from_units(vec![
                74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
            ])))
            && !proven))
        || expected_optional
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_optional_array_index_number(
    language: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    element: &self::ml_string::Text,
) -> bool {
    (((*language)
        == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116])))
        && ((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))))
        && (((*element) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))
            || ((*element) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_iterable_form(
    kind: &self::ml_string::Text,
    reference: bool,
) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])) {
        self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])
    } else if (*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])) {
        self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
    } else if reference
        && (((*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116])))
            || ((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112]))))
    {
        self::ml_string::Text::from_units(vec![109, 117, 116, 97, 98, 108, 101])
    } else if (*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
        self::ml_string::Text::from_units(vec![115, 101, 116])
    } else if (*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
        self::ml_string::Text::from_units(vec![109, 97, 112])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_mutation_diagnostic(field: &self::ml_string::Text) -> self::ml_string::Text {
    if (*field) == (self::ml_string::Text::from_units(vec![102, 101, 97, 116, 117, 114, 101])) {
        self::ml_string::Text::from_units(vec![
            114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 109, 117, 116, 97, 116, 105, 111, 110,
        ])
    } else if (*field) == (self::ml_string::Text::from_units(vec![114, 101, 97, 115, 111, 110])) {
        self::ml_string::Text::from_units(vec![
            116, 104, 105, 115, 32, 109, 117, 116, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117,
            105, 114, 101, 115, 32, 105, 116, 115, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101,
            102, 101, 114, 101, 110, 99, 101, 32, 97, 110, 100, 32, 115, 108, 111, 116, 32, 111,
            112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 116, 111, 32, 98, 101, 32, 114, 101,
            112, 114, 101, 115, 101, 110, 116, 101, 100,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_array_read(
    target: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    values: &self::ml_string::Text,
    position: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*target)
        == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))
    {
        ((((self::ml_string::Text::from_units(vec![40])).concat(values))
            .concat(&self::ml_string::Text::from_units(vec![41, 91])))
        .concat(position))
        .concat(&self::ml_string::Text::from_units(vec![93]))
    } else if (*target) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])) {
        {
            let index =
                if (*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])) {
                    ((self::ml_string::Text::from_units(vec![
                        99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 97, 114, 114, 97, 121, 58, 58,
                        110, 117, 109, 98, 101, 114, 95, 105, 110, 100, 101, 120, 40,
                    ]))
                    .concat(position))
                    .concat(&self::ml_string::Text::from_units(vec![41]))
                } else {
                    position.concat(&self::ml_string::Text::from_units(vec![
                        46, 116, 111, 95, 105, 110, 100, 101, 120, 40, 41,
                    ]))
                };
            ((((self::ml_string::Text::from_units(vec![
                99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 97, 114, 114, 97, 121, 58, 58, 111,
                112, 116, 105, 111, 110, 97, 108, 95, 97, 116, 40,
            ]))
            .concat(values))
            .concat(&self::ml_string::Text::from_units(vec![44, 32])))
            .concat(&index))
            .concat(&self::ml_string::Text::from_units(vec![41]))
        }
    } else if (*target) == (self::ml_string::Text::from_units(vec![76, 101, 97, 110])) {
        if (*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])) {
            ((((self::ml_string::Text::from_units(vec![
                40, 108, 101, 116, 32, 109, 108, 95, 105, 110, 100, 101, 120, 32, 58, 61, 32,
            ]))
            .concat(position))
            .concat(&self::ml_string::Text::from_units(vec![
                59, 32, 105, 102, 32, 109, 108, 95, 105, 110, 100, 101, 120, 32, 8805, 32, 48, 32,
                38, 38, 32, 109, 108, 95, 105, 110, 100, 101, 120, 46, 102, 108, 111, 111, 114, 32,
                61, 61, 32, 109, 108, 95, 105, 110, 100, 101, 120, 32, 38, 38, 32, 109, 108, 95,
                105, 110, 100, 101, 120, 32, 60, 32, 57, 48, 48, 55, 49, 57, 57, 50, 53, 52, 55,
                52, 48, 57, 57, 50, 32, 116, 104, 101, 110, 32,
            ])))
            .concat(values))
            .concat(&self::ml_string::Text::from_units(vec![
                91, 109, 108, 95, 105, 110, 100, 101, 120, 46, 116, 111, 85, 73, 110, 116, 54, 52,
                46, 116, 111, 78, 97, 116, 93, 63, 32, 101, 108, 115, 101, 32, 110, 111, 110, 101,
                41,
            ]))
        } else if (*kind) == (self::ml_string::Text::from_units(vec![110, 97, 116])) {
            ((((self::ml_string::Text::from_units(vec![40])).concat(values))
                .concat(&self::ml_string::Text::from_units(vec![91])))
            .concat(position))
            .concat(&self::ml_string::Text::from_units(vec![93, 63, 41]))
        } else {
            ((((self::ml_string::Text::from_units(vec![
                40, 108, 101, 116, 32, 109, 108, 95, 105, 110, 100, 101, 120, 32, 58, 61, 32,
            ]))
            .concat(position))
            .concat(&self::ml_string::Text::from_units(vec![
                59, 32, 105, 102, 32, 109, 108, 95, 105, 110, 100, 101, 120, 32, 60, 32, 48, 32,
                116, 104, 101, 110, 32, 110, 111, 110, 101, 32, 101, 108, 115, 101, 32,
            ])))
            .concat(values))
            .concat(&self::ml_string::Text::from_units(vec![
                91, 109, 108, 95, 105, 110, 100, 101, 120, 46, 116, 111, 78, 97, 116, 93, 63, 41,
            ]))
        }
    } else {
        {
            let index_2 =
                if (*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])) {
                    ((self::ml_string::Text::from_units(vec![
                        40, 109, 108, 95, 102, 108, 111, 97, 116, 95, 105, 110, 100, 101, 120, 32,
                    ]))
                    .concat(position))
                    .concat(&self::ml_string::Text::from_units(vec![41]))
                } else if (*kind) == (self::ml_string::Text::from_units(vec![110, 97, 116])) {
                    ((self::ml_string::Text::from_units(vec![40, 90, 46, 111, 102, 95, 78, 32]))
                        .concat(position))
                    .concat(&self::ml_string::Text::from_units(vec![41]))
                } else {
                    position.clone()
                };
            ((((self::ml_string::Text::from_units(vec![
                40, 108, 101, 116, 32, 109, 108, 95, 105, 110, 100, 101, 120, 32, 58, 61, 32,
            ]))
            .concat(&index_2))
            .concat(&self::ml_string::Text::from_units(vec![
                32, 105, 110, 32, 105, 102, 32, 90, 46, 108, 116, 98, 32, 109, 108, 95, 105, 110,
                100, 101, 120, 32, 48, 32, 116, 104, 101, 110, 32, 78, 111, 110, 101, 32, 101, 108,
                115, 101, 32, 76, 105, 115, 116, 46, 110, 116, 104, 95, 101, 114, 114, 111, 114,
                32,
            ])))
            .concat(values))
            .concat(&self::ml_string::Text::from_units(vec![
                32, 40, 90, 46, 116, 111, 95, 110, 97, 116, 32, 109, 108, 95, 105, 110, 100, 101,
                120, 41, 41,
            ]))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_string_point_iteration(
    target: &self::ml_string::Text,
    utf16: bool,
    value: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*target)
        == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))
    {
        ((self::ml_string::Text::from_units(vec![91, 46, 46, 46])).concat(value))
            .concat(&self::ml_string::Text::from_units(vec![93]))
    } else if (*target) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])) {
        if utf16 {
            value.concat(&self::ml_string::Text::from_units(vec![
                46, 99, 111, 100, 101, 95, 112, 111, 105, 110, 116, 115, 40, 41,
            ]))
        } else {
            value.concat(&self::ml_string::Text::from_units(vec![
                46, 99, 104, 97, 114, 115, 40, 41, 46, 109, 97, 112, 40, 124, 109, 108, 95, 99,
                104, 97, 114, 97, 99, 116, 101, 114, 124, 32, 109, 108, 95, 99, 104, 97, 114, 97,
                99, 116, 101, 114, 46, 116, 111, 95, 115, 116, 114, 105, 110, 103, 40, 41, 41, 46,
                99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 83, 116, 114, 105,
                110, 103, 62, 62, 40, 41,
            ]))
        }
    } else if (*target) == (self::ml_string::Text::from_units(vec![76, 101, 97, 110])) {
        ((self::ml_string::Text::from_units(vec![40, 40])).concat(value)).concat(
            &self::ml_string::Text::from_units(vec![
                46, 116, 111, 76, 105, 115, 116, 46, 109, 97, 112, 32, 40, 102, 117, 110, 32, 109,
                108, 95, 99, 104, 97, 114, 97, 99, 116, 101, 114, 32, 61, 62, 32, 83, 116, 114,
                105, 110, 103, 46, 115, 105, 110, 103, 108, 101, 116, 111, 110, 32, 109, 108, 95,
                99, 104, 97, 114, 97, 99, 116, 101, 114, 41, 41, 46, 116, 111, 65, 114, 114, 97,
                121, 41,
            ]),
        )
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_collection_iteration(
    target: &self::ml_string::Text,
    map: bool,
    operation: &self::ml_string::Text,
    value: &self::ml_string::Text,
    key_type: &self::ml_string::Text,
    value_type: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*target)
        == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))
    {
        ((((self::ml_string::Text::from_units(vec![91, 46, 46, 46, 40])).concat(value))
            .concat(&self::ml_string::Text::from_units(vec![41, 46])))
        .concat(operation))
        .concat(&self::ml_string::Text::from_units(vec![40, 41, 93]))
    } else if (*target) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])) {
        if (*operation)
            == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))
        {
            if map {
                value.clone()
            } else {
                value.concat(&self::ml_string::Text::from_units(vec![
                    46, 105, 116, 101, 114, 40, 41, 46, 109, 97, 112, 40, 124, 109, 108, 95, 118,
                    97, 108, 117, 101, 124, 32, 40, 109, 108, 95, 118, 97, 108, 117, 101, 46, 99,
                    108, 111, 110, 101, 40, 41, 44, 32, 109, 108, 95, 118, 97, 108, 117, 101, 46,
                    99, 108, 111, 110, 101, 40, 41, 41, 41, 46, 99, 111, 108, 108, 101, 99, 116,
                    58, 58, 60, 86, 101, 99, 60, 95, 62, 62, 40, 41,
                ]))
            }
        } else if map {
            ((value.concat(&self::ml_string::Text::from_units(vec![
                46, 105, 116, 101, 114, 40, 41, 46, 109, 97, 112, 40, 124, 109, 108, 95, 112, 97,
                105, 114, 124, 32, 109, 108, 95, 112, 97, 105, 114, 46,
            ])))
            .concat(&if (*operation)
                == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))
            {
                self::ml_string::Text::from_units(vec![48])
            } else {
                self::ml_string::Text::from_units(vec![49])
            }))
            .concat(&self::ml_string::Text::from_units(vec![
                46, 99, 108, 111, 110, 101, 40, 41, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58,
                58, 60, 86, 101, 99, 60, 95, 62, 62, 40, 41,
            ]))
        } else {
            value.clone()
        }
    } else if (*target) == (self::ml_string::Text::from_units(vec![76, 101, 97, 110])) {
        if (*operation)
            == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))
        {
            if map {
                value.clone()
            } else {
                ((self::ml_string::Text::from_units(vec![40])).concat(value)).concat(
                    &self::ml_string::Text::from_units(vec![
                        46, 109, 97, 112, 32, 40, 102, 117, 110, 32, 109, 108, 95, 118, 97, 108,
                        117, 101, 32, 61, 62, 32, 40, 109, 108, 95, 118, 97, 108, 117, 101, 44, 32,
                        109, 108, 95, 118, 97, 108, 117, 101, 41, 41, 41,
                    ]),
                )
            }
        } else if map {
            ((((self::ml_string::Text::from_units(vec![40])).concat(value)).concat(
                &self::ml_string::Text::from_units(vec![
                    46, 109, 97, 112, 32, 40, 102, 117, 110, 32, 109, 108, 95, 112, 97, 105, 114,
                    32, 61, 62, 32, 109, 108, 95, 112, 97, 105, 114, 46,
                ]),
            ))
            .concat(&if (*operation)
                == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))
            {
                self::ml_string::Text::from_units(vec![49])
            } else {
                self::ml_string::Text::from_units(vec![50])
            }))
            .concat(&self::ml_string::Text::from_units(vec![41, 41]))
        } else {
            value.clone()
        }
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))
    {
        if map {
            value.clone()
        } else {
            ((self::ml_string::Text::from_units(vec![
                40, 76, 105, 115, 116, 46, 109, 97, 112, 32, 40, 102, 117, 110, 32, 109, 108, 95,
                118, 97, 108, 117, 101, 32, 61, 62, 32, 40, 109, 108, 95, 118, 97, 108, 117, 101,
                44, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41, 41, 32,
            ]))
            .concat(value))
            .concat(&self::ml_string::Text::from_units(vec![41]))
        }
    } else if map {
        ((((((((self::ml_string::Text::from_units(vec![
            40, 76, 105, 115, 116, 46, 109, 97, 112, 32, 40, 64,
        ]))
        .concat(&if (*operation)
            == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))
        {
            self::ml_string::Text::from_units(vec![102, 115, 116])
        } else {
            self::ml_string::Text::from_units(vec![115, 110, 100])
        }))
        .concat(&self::ml_string::Text::from_units(vec![32])))
        .concat(key_type))
        .concat(&self::ml_string::Text::from_units(vec![32])))
        .concat(value_type))
        .concat(&self::ml_string::Text::from_units(vec![41, 32])))
        .concat(value))
        .concat(&self::ml_string::Text::from_units(vec![41]))
    } else {
        value.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_array_formal_pattern(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
) -> f64 {
    if ((((kinds.len() as f64) != (values.len() as f64)) || (start < 0f64))
        || ((start % 1f64) != 0f64))
        || (start >= (kinds.len() as f64))
    {
        -1f64
    } else {
        {
            let depth = 0f64;
            {
                let binding = true;
                {
                    let index = start;
                    {
                        let source_ml_source2_s61 = self::source_ml_source1_temporary_find_java_script_array_formal_pattern_loop4(kinds, values, depth, binding, index);
                        match source_ml_source2_s61 {
                            self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Result::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Done => {
                                -1f64
                            }
                            self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Result::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Return(source_ml_source1_temporary_result4) => {
                                source_ml_source1_temporary_result4
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
pub fn accept_java_script_array_formal_context(
    context: &self::ml_string::Text,
    asynchronous: bool,
    documentation_parameters: f64,
) -> bool {
    (!asynchronous && (documentation_parameters == 0f64))
        && ((((((((*context)
            == (self::ml_string::Text::from_units(vec![
                111, 114, 100, 105, 110, 97, 114, 121, 32, 110, 97, 109, 101, 100, 32, 102, 117,
                110, 99, 116, 105, 111, 110, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111,
                110,
            ])))
            || ((*context)
                == (self::ml_string::Text::from_units(vec![
                    111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 114, 114, 111, 119, 32, 102,
                    117, 110, 99, 116, 105, 111, 110, 32, 99, 108, 111, 115, 117, 114, 101,
                ]))))
            || ((*context)
                == (self::ml_string::Text::from_units(vec![
                    111, 114, 100, 105, 110, 97, 114, 121, 32, 102, 117, 110, 99, 116, 105, 111,
                    110, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 99, 108, 111,
                    115, 117, 114, 101,
                ]))))
            || ((*context)
                == (self::ml_string::Text::from_units(vec![
                    111, 114, 100, 105, 110, 97, 114, 121, 32, 111, 98, 106, 101, 99, 116, 32, 109,
                    101, 116, 104, 111, 100, 32, 99, 108, 111, 115, 117, 114, 101,
                ]))))
            || ((*context)
                == (self::ml_string::Text::from_units(vec![
                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 115, 116, 97, 110,
                    116, 32, 97, 114, 114, 111, 119, 32, 102, 117, 110, 99, 116, 105, 111, 110,
                ]))))
            || ((*context)
                == (self::ml_string::Text::from_units(vec![
                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 115, 116, 97, 110,
                    116, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 101, 120, 112, 114, 101,
                    115, 115, 105, 111, 110,
                ]))))
            || ((*context)
                == (self::ml_string::Text::from_units(vec![
                    111, 114, 100, 105, 110, 97, 114, 121, 32, 110, 97, 109, 101, 115, 112, 97, 99,
                    101, 32, 111, 98, 106, 101, 99, 116, 32, 109, 101, 116, 104, 111, 100,
                ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_formal_default_scope_boundary(
    context: &self::ml_string::Text,
) -> f64 {
    if (((*context)
        == (self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 114, 114, 111, 119, 32, 102, 117, 110,
            99, 116, 105, 111, 110, 32, 99, 108, 111, 115, 117, 114, 101,
        ])))
        || ((*context)
            == (self::ml_string::Text::from_units(vec![
                111, 114, 100, 105, 110, 97, 114, 121, 32, 102, 117, 110, 99, 116, 105, 111, 110,
                32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 99, 108, 111, 115, 117,
                114, 101,
            ]))))
        || ((*context)
            == (self::ml_string::Text::from_units(vec![
                111, 114, 100, 105, 110, 97, 114, 121, 32, 111, 98, 106, 101, 99, 116, 32, 109,
                101, 116, 104, 111, 100, 32, 99, 108, 111, 115, 117, 114, 101,
            ])))
    {
        1f64
    } else {
        2f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_array_formal_parameters(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_java_script_array_formal_parameter_bindings(kinds, values, start, reserved, false)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_array_formal_parameters_with_defaults(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_java_script_array_formal_parameter_bindings(kinds, values, start, reserved, true)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_array_formal_parameter_bindings(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
    defaults: bool,
) -> Vec<f64> {
    if ((((kinds.len() as f64) != (values.len() as f64)) || (start < 0f64))
        || ((start % 1f64) != 0f64))
        || (start >= (kinds.len() as f64))
    {
        vec![-1f64, start]
    } else {
        {
            let index = start;
            {
                let slot = 0f64;
                {
                    let names = Vec::<self::ml_string::Text>::new();
                    {
                        let instructions = Vec::<f64>::new();
                        {
                            let source_ml_source2_s62 = self::source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop5(kinds, values, reserved, defaults, index, slot, names, instructions);
                            match source_ml_source2_s62 {
                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Done(index_2, slot_2, instructions_2) => {
                                    if index_2 < (kinds.len() as f64) {
                                        {
                                            let source_ml_source2_concat_value63 = vec![(index_2 + 1f64), slot_2];
                                            {
                                                let source_ml_source2_concat_value64 = instructions_2;
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value63), source_ml_source2_concat_value64)
                                            }
                                        }
                                    } else {
                                        vec![-1f64, index_2]
                                    }
                                }
                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Return(source_ml_source1_temporary_result5) => {
                                    source_ml_source1_temporary_result5
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
pub fn find_java_script_formal_default_expression_end(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
) -> f64 {
    self::find_java_script_pattern_default_expression_end(
        kinds,
        values,
        start,
        &(self::ml_string::Text::from_units(vec![93])),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_pattern_default_expression_end(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    closing: &self::ml_string::Text,
) -> f64 {
    if (((*closing) != (self::ml_string::Text::from_units(vec![93])))
        && ((*closing) != (self::ml_string::Text::from_units(vec![125]))))
        || (((((kinds.len() as f64) != (values.len() as f64)) || (start < 0f64))
            || ((start % 1f64) != 0f64))
            || (start >= (kinds.len() as f64)))
    {
        -1f64
    } else {
        {
            let stack = Vec::<self::ml_string::Text>::new();
            {
                let index = start;
                {
                    let source_ml_source2_s65 = self::source_ml_source1_temporary_find_java_script_pattern_default_expression_end_loop11(kinds, values, start, closing, stack, index);
                    match source_ml_source2_s65 {
                            self::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Result::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Done => {
                                -1f64
                            }
                            self::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Result::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Return(source_ml_source1_temporary_result11) => {
                                source_ml_source1_temporary_result11
                            }
                        }
                }
            }
        }
    }
}
