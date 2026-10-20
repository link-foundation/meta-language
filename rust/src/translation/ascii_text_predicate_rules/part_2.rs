// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::accept_ascii_character_method;
use super::accept_rust_const_function_name;
use super::accept_rust_const_function_name_character;
use super::ml_array;
use super::ml_math;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_ascii_text_query(
    method: &self::ml_string::Text,
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
) -> Vec<f64> {
    if ((((kinds.len() as f64) != (values.len() as f64)) || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64))
    {
        Vec::<f64>::new()
    } else if (((((*method)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 97, 114, 116, 115, 95, 119, 105, 116, 104,
        ])))
        && match (
            self::ml_array::optional_at(kinds, self::ml_array::number_index(start)),
            self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(values, self::ml_array::number_index(start)),
            self::ml_string::Text::from_units(vec![40]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(kinds, self::ml_array::number_index(start + 1f64)),
            self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(values, self::ml_array::number_index(start + 1f64)),
            self::ml_string::Text::from_units(vec![124]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        }
    {
        vec![1f64, (start + 1f64)]
    } else if (((((((((((*method)
        == (self::ml_string::Text::from_units(vec![99, 104, 97, 114, 115])))
        && match (
            self::ml_array::optional_at(kinds, self::ml_array::number_index(start)),
            self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(values, self::ml_array::number_index(start)),
            self::ml_string::Text::from_units(vec![40]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(kinds, self::ml_array::number_index(start + 1f64)),
            self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(values, self::ml_array::number_index(start + 1f64)),
            self::ml_string::Text::from_units(vec![41]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(kinds, self::ml_array::number_index(start + 2f64)),
            self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(values, self::ml_array::number_index(start + 2f64)),
            self::ml_string::Text::from_units(vec![46]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(kinds, self::ml_array::number_index(start + 3f64)),
            self::ml_string::Text::from_units(vec![
                105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
            ]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(values, self::ml_array::number_index(start + 3f64)),
            self::ml_string::Text::from_units(vec![97, 108, 108]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(kinds, self::ml_array::number_index(start + 4f64)),
            self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(values, self::ml_array::number_index(start + 4f64)),
            self::ml_string::Text::from_units(vec![40]),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        }
    {
        vec![2f64, (start + 5f64)]
    } else {
        Vec::<f64>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_ascii_character_callback(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    characters: &[f64],
    start: f64,
) -> Vec<f64> {
    if (((((kinds.len() as f64) != (values.len() as f64))
        || ((kinds.len() as f64) != (characters.len() as f64)))
        || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64))
    {
        Vec::<f64>::new()
    } else if !(match (
        self::ml_array::optional_at(kinds, self::ml_array::number_index(start)),
        self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]),
    ) {
        (Some(ml_left), ml_right) => ml_left == ml_right,
        _ => false,
    }) || !(match (
        self::ml_array::optional_at(values, self::ml_array::number_index(start)),
        self::ml_string::Text::from_units(vec![124]),
    ) {
        (Some(ml_left), ml_right) => ml_left == ml_right,
        _ => false,
    }) {
        Vec::<f64>::new()
    } else {
        {
            let at = start + 1f64;
            if (match (
                self::ml_array::optional_at(kinds, self::ml_array::number_index(at)),
                self::ml_string::Text::from_units(vec![
                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                ]),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }) {
                {
                    let parameter =
                        (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if (self::accept_rust_const_function_name(&(parameter))) {
                        {
                            let index = 0f64;
                            {
                                let ml_s1 = self::ml_plan_rust_ascii_character_callback_loop1(
                                    &(parameter),
                                    index,
                                );
                                match ml_s1 {
                                        self::MlPlanRustAsciiCharacterCallbackLoop1Result::MlPlanRustAsciiCharacterCallbackLoop1Done => {
                                            {
                                                let at_2 = at + 1f64;
                                                {
                                                    let annotation = -1f64;
                                                    {
                                                        let ml_s2 = if match (self::ml_array::optional_at(kinds, self::ml_array::number_index(at_2)), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } && match (self::ml_array::optional_at(values, self::ml_array::number_index(at_2)), self::ml_string::Text::from_units(vec![58])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } {
                                                            {
                                                                let at_3 = at_2 + 1f64;
                                                                {
                                                                    let annotation_2 = at_3;
                                                                    if !(match (self::ml_array::optional_at(kinds, self::ml_array::number_index(at_3)), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) || !(match (self::ml_array::optional_at(values, self::ml_array::number_index(at_3)), self::ml_string::Text::from_units(vec![99, 104, 97, 114])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) {
                                                                        self::MlPlanRustAsciiCharacterCallbackJoin2::MlPlanRustAsciiCharacterCallbackJoin2Return(Vec::<f64>::new())
                                                                    } else {
                                                                        {
                                                                            let at_4 = at_3 + 1f64;
                                                                            self::MlPlanRustAsciiCharacterCallbackJoin2::MlPlanRustAsciiCharacterCallbackJoin2Next(at_4, annotation_2)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlPlanRustAsciiCharacterCallbackJoin2::MlPlanRustAsciiCharacterCallbackJoin2Next(at_2, annotation)
                                                        };
                                                        match ml_s2 {
                                                            self::MlPlanRustAsciiCharacterCallbackJoin2::MlPlanRustAsciiCharacterCallbackJoin2Next(at_5, annotation_3) => {
                                                                if !(match (self::ml_array::optional_at(kinds, self::ml_array::number_index(at_5)), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) || !(match (self::ml_array::optional_at(values, self::ml_array::number_index(at_5)), self::ml_string::Text::from_units(vec![124])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) {
                                                                    Vec::<f64>::new()
                                                                } else {
                                                                    {
                                                                        let at_6 = at_5 + 1f64;
                                                                        {
                                                                            let methods = Vec::<self::ml_string::Text>::new();
                                                                            {
                                                                                let literals = Vec::<f64>::new();
                                                                                {
                                                                                    let ml_s3 = self::ml_plan_rust_ascii_character_callback_loop3(kinds, values, characters, at_6, &(parameter), annotation_3, methods, literals);
                                                                                    match ml_s3 {
                                                                                        self::MlPlanRustAsciiCharacterCallbackLoop3Result::MlPlanRustAsciiCharacterCallbackLoop3Done => {
                                                                                            Vec::<f64>::new()
                                                                                        }
                                                                                        self::MlPlanRustAsciiCharacterCallbackLoop3Result::MlPlanRustAsciiCharacterCallbackLoop3Return(ml_result3) => {
                                                                                            ml_result3
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlPlanRustAsciiCharacterCallbackJoin2::MlPlanRustAsciiCharacterCallbackJoin2Return(ml_result6) => {
                                                                ml_result6
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlPlanRustAsciiCharacterCallbackLoop1Result::MlPlanRustAsciiCharacterCallbackLoop1Return(ml_result6_2) => {
                                            ml_result6_2
                                        }
                                    }
                            }
                        }
                    } else {
                        Vec::<f64>::new()
                    }
                }
            } else {
                Vec::<f64>::new()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustAsciiCharacterCallbackLoop3Result {
    MlPlanRustAsciiCharacterCallbackLoop3Done,
    MlPlanRustAsciiCharacterCallbackLoop3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_ascii_character_callback_loop6(
    methods: &[self::ml_string::Text],
    literals: &[f64],
    mut scalar: f64,
    mut accepted: bool,
    mut index: f64,
) -> bool {
    loop {
        return if index < (methods.len() as f64) {
            {
                let method =
                    (self::ml_array::optional_at(methods, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if if method
                    == (self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108]))
                {
                    scalar
                        == ((self::ml_array::optional_at(
                            literals,
                            self::ml_array::number_index(index),
                        ))
                        .unwrap_or_else(|| -1f64))
                } else {
                    self::accept_ascii_character_method(&(method), scalar)
                } {
                    {
                        let accepted_2 = true;
                        {
                            let index_2 = index + 1f64;
                            {
                                (accepted, index) = (accepted_2, index_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let index_3 = index + 1f64;
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            accepted
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_ascii_character_callback_loop5(
    methods: &[self::ml_string::Text],
    literals: &[f64],
    mut result: Vec<f64>,
    mut scalar: f64,
) -> Vec<f64> {
    loop {
        if scalar < 128f64 {
            {
                let accepted = false;
                {
                    let index = 0f64;
                    {
                        let accepted_2 = self::ml_plan_rust_ascii_character_callback_loop6(
                            methods, literals, scalar, accepted, index,
                        );
                        if accepted_2 {
                            {
                                let result_2 = {
                                    let (ml_append_right, ml_append_left) = (vec![scalar], result);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let scalar_2 = scalar + 1f64;
                                    {
                                        (result, scalar) = (result_2, scalar_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let scalar_3 = scalar + 1f64;
                                {
                                    scalar = scalar_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustAsciiCharacterCallbackJoin4 {
    MlPlanRustAsciiCharacterCallbackJoin4Next(f64, Vec<self::ml_string::Text>, Vec<f64>),
    MlPlanRustAsciiCharacterCallbackJoin4Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_rust_ascii_character_callback_loop3(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    characters: &[f64],
    mut at: f64,
    parameter: &self::ml_string::Text,
    mut annotation: f64,
    mut methods: Vec<self::ml_string::Text>,
    mut literals: Vec<f64>,
) -> self::MlPlanRustAsciiCharacterCallbackLoop3Result {
    loop {
        return if at < (kinds.len() as f64) {
            if !(match (
                self::ml_array::optional_at(kinds, self::ml_array::number_index(at)),
                self::ml_string::Text::from_units(vec![
                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                ]),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }) || !(match (
                self::ml_array::optional_at(values, self::ml_array::number_index(at)),
                parameter.clone(),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }) {
                self::MlPlanRustAsciiCharacterCallbackLoop3Result::MlPlanRustAsciiCharacterCallbackLoop3Return(Vec::<f64>::new())
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        let ml_s4 = if match (
                            self::ml_array::optional_at(kinds, self::ml_array::number_index(at_2)),
                            self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } && match (
                            self::ml_array::optional_at(values, self::ml_array::number_index(at_2)),
                            self::ml_string::Text::from_units(vec![46]),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } {
                            {
                                let at_3 = at_2 + 1f64;
                                {
                                    let method = (self::ml_array::optional_at(
                                        values,
                                        self::ml_array::number_index(at_3),
                                    ))
                                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                    if !(match (
                                        self::ml_array::optional_at(
                                            kinds,
                                            self::ml_array::number_index(at_3),
                                        ),
                                        self::ml_string::Text::from_units(vec![
                                            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                        ]),
                                    ) {
                                        (Some(ml_left), ml_right) => ml_left == ml_right,
                                        _ => false,
                                    }) || (((((method
                                        != (self::ml_string::Text::from_units(vec![
                                            105, 115, 95, 97, 115, 99, 105, 105, 95, 97, 108, 112,
                                            104, 97, 98, 101, 116, 105, 99,
                                        ])))
                                        && (method
                                            != (self::ml_string::Text::from_units(vec![
                                                105, 115, 95, 97, 115, 99, 105, 105, 95, 97, 108,
                                                112, 104, 97, 110, 117, 109, 101, 114, 105, 99,
                                            ]))))
                                        && (method
                                            != (self::ml_string::Text::from_units(vec![
                                                105, 115, 95, 97, 115, 99, 105, 105, 95, 117, 112,
                                                112, 101, 114, 99, 97, 115, 101,
                                            ]))))
                                        && (method
                                            != (self::ml_string::Text::from_units(vec![
                                                105, 115, 95, 97, 115, 99, 105, 105, 95, 108, 111,
                                                119, 101, 114, 99, 97, 115, 101,
                                            ]))))
                                        && (method
                                            != (self::ml_string::Text::from_units(vec![
                                                105, 115, 95, 97, 115, 99, 105, 105, 95, 100, 105,
                                                103, 105, 116,
                                            ]))))
                                    {
                                        self::MlPlanRustAsciiCharacterCallbackJoin4::MlPlanRustAsciiCharacterCallbackJoin4Return(Vec::<f64>::new())
                                    } else {
                                        {
                                            let at_4 = at_3 + 1f64;
                                            if !(match (
                                                self::ml_array::optional_at(
                                                    kinds,
                                                    self::ml_array::number_index(at_4),
                                                ),
                                                self::ml_string::Text::from_units(vec![
                                                    112, 117, 110, 99, 116,
                                                ]),
                                            ) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            }) || !(match (
                                                self::ml_array::optional_at(
                                                    values,
                                                    self::ml_array::number_index(at_4),
                                                ),
                                                self::ml_string::Text::from_units(vec![40]),
                                            ) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            }) {
                                                self::MlPlanRustAsciiCharacterCallbackJoin4::MlPlanRustAsciiCharacterCallbackJoin4Return(Vec::<f64>::new())
                                            } else {
                                                {
                                                    let at_5 = at_4 + 1f64;
                                                    if !(match (
                                                        self::ml_array::optional_at(
                                                            kinds,
                                                            self::ml_array::number_index(at_5),
                                                        ),
                                                        self::ml_string::Text::from_units(vec![
                                                            112, 117, 110, 99, 116,
                                                        ]),
                                                    ) {
                                                        (Some(ml_left), ml_right) => {
                                                            ml_left == ml_right
                                                        }
                                                        _ => false,
                                                    }) || !(match (
                                                        self::ml_array::optional_at(
                                                            values,
                                                            self::ml_array::number_index(at_5),
                                                        ),
                                                        self::ml_string::Text::from_units(vec![41]),
                                                    ) {
                                                        (Some(ml_left), ml_right) => {
                                                            ml_left == ml_right
                                                        }
                                                        _ => false,
                                                    }) {
                                                        self::MlPlanRustAsciiCharacterCallbackJoin4::MlPlanRustAsciiCharacterCallbackJoin4Return(Vec::<f64>::new())
                                                    } else {
                                                        {
                                                            let at_6 = at_5 + 1f64;
                                                            {
                                                                let methods_2 = {
                                                                    let (
                                                                        ml_append_right,
                                                                        ml_append_left,
                                                                    ) = (
                                                                        vec![method.clone()],
                                                                        methods,
                                                                    );
                                                                    self::ml_array::append(
                                                                        ml_append_left,
                                                                        ml_append_right,
                                                                    )
                                                                };
                                                                {
                                                                    let literals_2 = {
                                                                        let (
                                                                            ml_append_right,
                                                                            ml_append_left,
                                                                        ) = (vec![-1f64], literals);
                                                                        self::ml_array::append(
                                                                            ml_append_left,
                                                                            ml_append_right,
                                                                        )
                                                                    };
                                                                    self::MlPlanRustAsciiCharacterCallbackJoin4::MlPlanRustAsciiCharacterCallbackJoin4Next(at_6, methods_2, literals_2)
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
                        } else if !(match (
                            self::ml_array::optional_at(kinds, self::ml_array::number_index(at_2)),
                            self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }) || !(match (
                            self::ml_array::optional_at(values, self::ml_array::number_index(at_2)),
                            self::ml_string::Text::from_units(vec![61, 61]),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }) {
                            self::MlPlanRustAsciiCharacterCallbackJoin4::MlPlanRustAsciiCharacterCallbackJoin4Return(Vec::<f64>::new())
                        } else {
                            {
                                let at_7 = at_2 + 1f64;
                                {
                                    let scalar = (self::ml_array::optional_at(
                                        characters,
                                        self::ml_array::number_index(at_7),
                                    ))
                                    .unwrap_or_else(|| -1f64);
                                    if ((!(match (
                                        self::ml_array::optional_at(
                                            kinds,
                                            self::ml_array::number_index(at_7),
                                        ),
                                        self::ml_string::Text::from_units(vec![99, 104, 97, 114]),
                                    ) {
                                        (Some(ml_left), ml_right) => ml_left == ml_right,
                                        _ => false,
                                    }) || !(self::ml_math::is_integer(scalar)))
                                        || (scalar < 0f64))
                                        || (scalar > 127f64)
                                    {
                                        self::MlPlanRustAsciiCharacterCallbackJoin4::MlPlanRustAsciiCharacterCallbackJoin4Return(Vec::<f64>::new())
                                    } else {
                                        {
                                            let methods_3 = {
                                                let (ml_append_right, ml_append_left) = (
                                                    vec![self::ml_string::Text::from_units(vec![
                                                        108, 105, 116, 101, 114, 97, 108,
                                                    ])],
                                                    methods,
                                                );
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let literals_3 = {
                                                    let (ml_append_right, ml_append_left) =
                                                        (vec![scalar], literals);
                                                    self::ml_array::append(
                                                        ml_append_left,
                                                        ml_append_right,
                                                    )
                                                };
                                                {
                                                    let at_8 = at_7 + 1f64;
                                                    self::MlPlanRustAsciiCharacterCallbackJoin4::MlPlanRustAsciiCharacterCallbackJoin4Next(at_8, methods_3, literals_3)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        match ml_s4 {
                            self::MlPlanRustAsciiCharacterCallbackJoin4::MlPlanRustAsciiCharacterCallbackJoin4Next(at_9, methods_4, literals_4) => {
                                if match (self::ml_array::optional_at(kinds, self::ml_array::number_index(at_9)), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } && match (self::ml_array::optional_at(values, self::ml_array::number_index(at_9)), self::ml_string::Text::from_units(vec![124, 124])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } {
                                    {
                                        let at_10 = at_9 + 1f64;
                                        {
                                            (at, methods, literals) = (at_10, methods_4, literals_4);
                                            continue;
                                        }
                                    }
                                }
{
                                    if !(match (self::ml_array::optional_at(kinds, self::ml_array::number_index(at_9)), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) || !(match (self::ml_array::optional_at(values, self::ml_array::number_index(at_9)), self::ml_string::Text::from_units(vec![41])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) {
                                        self::MlPlanRustAsciiCharacterCallbackLoop3Result::MlPlanRustAsciiCharacterCallbackLoop3Return(Vec::<f64>::new())
                                    } else {
                                        {
                                            let result = vec![at_9, annotation];
                                            {
                                                let scalar_2 = 0f64;
                                                {
                                                    let result_2 = self::ml_plan_rust_ascii_character_callback_loop5(&(methods_4), &(literals_4), result, scalar_2);
                                                    self::MlPlanRustAsciiCharacterCallbackLoop3Result::MlPlanRustAsciiCharacterCallbackLoop3Return(result_2)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlPlanRustAsciiCharacterCallbackJoin4::MlPlanRustAsciiCharacterCallbackJoin4Return(ml_result6) => {
                                self::MlPlanRustAsciiCharacterCallbackLoop3Result::MlPlanRustAsciiCharacterCallbackLoop3Return(ml_result6)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlPlanRustAsciiCharacterCallbackLoop3Result::MlPlanRustAsciiCharacterCallbackLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustAsciiCharacterCallbackJoin2 {
    MlPlanRustAsciiCharacterCallbackJoin2Next(f64, f64),
    MlPlanRustAsciiCharacterCallbackJoin2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustAsciiCharacterCallbackLoop1Result {
    MlPlanRustAsciiCharacterCallbackLoop1Done,
    MlPlanRustAsciiCharacterCallbackLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_ascii_character_callback_loop1(
    parameter: &self::ml_string::Text,
    mut index: f64,
) -> self::MlPlanRustAsciiCharacterCallbackLoop1Result {
    loop {
        return if index < (parameter.length()) {
            if (self::accept_rust_const_function_name_character(
                (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 {
                    {
                        let index_2 = if position.is_nan() {
                            0f64
                        } else {
                            position.trunc()
                        };
                        if (index_2 < 0f64) || (index_2 >= (text.length())) {
                            f64::NAN
                        } else {
                            ((text.slice(index_2, (index_2 + 1f64))).code_point_at(0f64))
                                .clone()
                                .unwrap_or_else(|| f64::NAN)
                        }
                    }
                }))(parameter.clone(), index),
                (index == 0f64),
            )) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlPlanRustAsciiCharacterCallbackLoop1Result::MlPlanRustAsciiCharacterCallbackLoop1Return(Vec::<f64>::new())
            }
        } else {
            self::MlPlanRustAsciiCharacterCallbackLoop1Result::MlPlanRustAsciiCharacterCallbackLoop1Done
        };
    }
}
