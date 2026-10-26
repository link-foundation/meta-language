// Generated from complete js/src/rust-project-source-rules.js. Do not edit by hand.
use super::MlTemporaryReadRustProjectModulePlanJoin3;
use super::MlTemporaryReadRustProjectModulePlanJoin4;
use super::group_end;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_project_module_plan_loop2(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut plan: Vec<f64>,
    mut parents: Vec<f64>,
    mut closes: Vec<f64>,
    mut position: f64,
    mut parent: f64,
) -> Vec<f64> {
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
                                let parent_2 = self::read_required_number_fact(
                                    &(parents),
                                    ((parents.len() as f64) - 1f64),
                                );
                                {
                                    let parents_2 = self::ml_array::slice(&parents, 0f64, -1f64);
                                    {
                                        let position_2 = position + 1f64;
                                        {
                                            (parents, closes, position, parent) =
                                                (parents_2, closes_2, position_2, parent_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    {
                        if (kind
                            == (self::ml_string::Text::from_units(vec![
                                107, 101, 121, 119, 111, 114, 100,
                            ])))
                            && (value == (self::ml_string::Text::from_units(vec![109, 111, 100])))
                        {
                            if ((position + 2f64) >= (values.len() as f64))
                                || ((self::read_required_text_fact(kinds, (position + 1f64)))
                                    != (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                    ])))
                            {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        105, 110, 118, 97, 108, 105, 100, 32, 82, 117, 115, 116,
                                        32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100,
                                        117, 108, 101, 32, 110, 97, 109, 101,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            } else {
                                {
                                    let delimiter =
                                        self::read_required_text_fact(values, (position + 2f64));
                                    if ((self::read_required_text_fact(kinds, (position + 2f64)))
                                        != (self::ml_string::Text::from_units(vec![
                                            112, 117, 110, 99, 116,
                                        ])))
                                        || ((delimiter
                                            != (self::ml_string::Text::from_units(vec![59])))
                                            && (delimiter
                                                != (self::ml_string::Text::from_units(vec![123]))))
                                    {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    105, 110, 118, 97, 108, 105, 100, 32, 82, 117,
                                                    115, 116, 32, 112, 114, 111, 106, 101, 99, 116,
                                                    32, 109, 111, 100, 117, 108, 101, 32, 98, 111,
                                                    117, 110, 100, 97, 114, 121,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    } else {
                                        {
                                            let after = if delimiter
                                                == (self::ml_string::Text::from_units(vec![59]))
                                            {
                                                position + 3f64
                                            } else {
                                                self::group_end(kinds, values, (position + 2f64))
                                            };
                                            {
                                                let record = (plan.len() as f64) / 5f64;
                                                {
                                                    let plan_2 = {
                                                        let (ml_append_right, ml_append_left) = (
                                                            vec![
                                                                position,
                                                                (position + 1f64),
                                                                (position + 2f64),
                                                                after,
                                                                parent,
                                                            ],
                                                            plan,
                                                        );
                                                        self::ml_array::append(
                                                            ml_append_left,
                                                            ml_append_right,
                                                        )
                                                    };
                                                    {
                                                        let source_ml_source1_s35 = if delimiter
                                                            == (self::ml_string::Text::from_units(
                                                                vec![123],
                                                            )) {
                                                            {
                                                                let parents_3 = {
                                                                    let (
                                                                        ml_append_right,
                                                                        ml_append_left,
                                                                    ) = (vec![parent], parents);
                                                                    self::ml_array::append(
                                                                        ml_append_left,
                                                                        ml_append_right,
                                                                    )
                                                                };
                                                                {
                                                                    let closes_3 = {
                                                                        let (
                                                                            ml_append_right,
                                                                            ml_append_left,
                                                                        ) = (
                                                                            vec![(after - 1f64)],
                                                                            closes,
                                                                        );
                                                                        self::ml_array::append(
                                                                            ml_append_left,
                                                                            ml_append_right,
                                                                        )
                                                                    };
                                                                    {
                                                                        let parent_3 = record;
                                                                        {
                                                                            let position_3 =
                                                                                position + 3f64;
                                                                            self::MlTemporaryReadRustProjectModulePlanJoin3::MlTemporaryReadRustProjectModulePlanJoin3Next(parents_3, closes_3, position_3, parent_3)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let position_4 = after;
                                                                self::MlTemporaryReadRustProjectModulePlanJoin3::MlTemporaryReadRustProjectModulePlanJoin3Next(parents, closes, position_4, parent)
                                                            }
                                                        };
                                                        match source_ml_source1_s35 {
                                                            self::MlTemporaryReadRustProjectModulePlanJoin3::MlTemporaryReadRustProjectModulePlanJoin3Next(parents_4, closes_4, position_5, parent_4) => {
                                                                {
                                                                    (plan, parents, closes, position, parent) = (plan_2, parents_4, closes_4, position_5, parent_4);
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
                        } else if (kind
                            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                            && (((value == (self::ml_string::Text::from_units(vec![123])))
                                || (value == (self::ml_string::Text::from_units(vec![91]))))
                                || (value == (self::ml_string::Text::from_units(vec![40]))))
                        {
                            {
                                let source_ml_source1_s36 = if ((value
                                    == (self::ml_string::Text::from_units(vec![91])))
                                    && (position > 0f64))
                                    && (((self::read_required_text_fact(
                                        values,
                                        (position - 1f64),
                                    )) == (self::ml_string::Text::from_units(vec![35])))
                                        || (((position > 1f64)
                                            && ((self::read_required_text_fact(
                                                values,
                                                (position - 1f64),
                                            )) == (self::ml_string::Text::from_units(vec![
                                                33,
                                            ]))))
                                            && ((self::read_required_text_fact(
                                                values,
                                                (position - 2f64),
                                            )) == (self::ml_string::Text::from_units(vec![
                                                35,
                                            ]))))) {
                                    {
                                        let attribute = if (position + 1f64) < (values.len() as f64)
                                        {
                                            self::read_required_text_fact(values, (position + 1f64))
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        };
                                        if ((attribute
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 102, 103,
                                            ])))
                                            || (attribute
                                                == (self::ml_string::Text::from_units(vec![
                                                    99, 102, 103, 95, 97, 116, 116, 114,
                                                ]))))
                                            || (attribute
                                                == (self::ml_string::Text::from_units(vec![
                                                    109, 97, 99, 114, 111, 95, 117, 115, 101,
                                                ])))
                                        {
                                            (self::ml_source_exception::SourceException {
                                                class_name: String::from("TypeError"),
                                                message_units: (self::ml_string::Text::from_units(
                                                    vec![
                                                        82, 117, 115, 116, 32, 112, 114, 111, 106,
                                                        101, 99, 116, 32, 109, 111, 100, 117, 108,
                                                        101, 32, 97, 116, 116, 114, 105, 98, 117,
                                                        116, 101, 115, 32, 110, 101, 101, 100, 32,
                                                        97, 110, 32, 101, 120, 112, 108, 105, 99,
                                                        105, 116, 32, 99, 111, 110, 102, 105, 103,
                                                        117, 114, 97, 116, 105, 111, 110, 32, 105,
                                                        110, 116, 101, 114, 112, 114, 101, 116, 97,
                                                        116, 105, 111, 110,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        } else {
                                            self::MlTemporaryReadRustProjectModulePlanJoin4::MlTemporaryReadRustProjectModulePlanJoin4Next
                                        }
                                    }
                                } else {
                                    self::MlTemporaryReadRustProjectModulePlanJoin4::MlTemporaryReadRustProjectModulePlanJoin4Next
                                };
                                match source_ml_source1_s36 {
                                        self::MlTemporaryReadRustProjectModulePlanJoin4::MlTemporaryReadRustProjectModulePlanJoin4Next => {
                                            {
                                                let position_6 = self::group_end(kinds, values, position);
                                                {
                                                    position = position_6;
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                            }
                        } else if (kind
                            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                            && (((value == (self::ml_string::Text::from_units(vec![125])))
                                || (value == (self::ml_string::Text::from_units(vec![93]))))
                                || (value == (self::ml_string::Text::from_units(vec![41]))))
                        {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    117, 110, 101, 120, 112, 101, 99, 116, 101, 100, 32, 82, 117,
                                    115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 99, 108,
                                    111, 115, 105, 110, 103, 32, 103, 114, 111, 117, 112,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        } else {
                            {
                                let position_7 = position + 1f64;
                                {
                                    position = position_7;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            plan
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustProjectSourcePathLoop6Result {
    MlTemporaryAcceptRustProjectSourcePathLoop6Done,
    MlTemporaryAcceptRustProjectSourcePathLoop6Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_rust_project_source_path_loop6(
    source_ml_temporary_values5: &[self::ml_string::Text],
    mut source_ml_temporary_index5: self::ml::Big,
) -> self::MlTemporaryAcceptRustProjectSourcePathLoop6Result {
    loop {
        return if source_ml_temporary_index5
            < (self::ml::Big::from_u128(source_ml_temporary_values5.len() as u128))
        {
            {
                let part = self::ml_array::at(
                    source_ml_temporary_values5,
                    source_ml_temporary_index5.to_index(),
                );
                if (!(!part.0.is_empty())
                    || (part == (self::ml_string::Text::from_units(vec![46]))))
                    || (part == (self::ml_string::Text::from_units(vec![46, 46])))
                {
                    self::MlTemporaryAcceptRustProjectSourcePathLoop6Result::MlTemporaryAcceptRustProjectSourcePathLoop6Return(false)
                } else {
                    {
                        let source_ml_temporary_index5_2 =
                            source_ml_temporary_index5.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index5 = source_ml_temporary_index5_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustProjectSourcePathLoop6Result::MlTemporaryAcceptRustProjectSourcePathLoop6Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_project_module_path_attributes_loop8(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut owner: f64,
) -> f64 {
    loop {
        if (((owner + 1f64) < (values.len() as f64))
            && ((self::read_required_text_fact(values, owner))
                == (self::ml_string::Text::from_units(vec![35]))))
            && ((self::read_required_text_fact(values, (owner + 1f64)))
                == (self::ml_string::Text::from_units(vec![91])))
        {
            {
                let owner_2 = self::group_end(kinds, values, (owner + 1f64));
                {
                    owner = owner_2;
                    continue;
                }
            }
        }
        return { owner };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_project_module_path_attributes_loop9(
    plan: &[f64],
    mut owner: f64,
    mut record: f64,
    mut at: f64,
) -> f64 {
    loop {
        if at < (plan.len() as f64) {
            if (self::read_required_number_fact(plan, at)) == owner {
                {
                    let record_2 = at / 5f64;
                    {
                        let at_2 = at + 5f64;
                        {
                            (record, at) = (record_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let at_3 = at + 5f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        }
        return { record };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_project_module_path_attributes_loop10(
    plan: &[f64],
    mut position: f64,
    mut inline: bool,
    mut at: f64,
) -> bool {
    loop {
        if at < (plan.len() as f64) {
            if (self::read_required_number_fact(plan, (at + 2f64))) == position {
                {
                    let inline_2 = true;
                    {
                        let at_2 = at + 5f64;
                        {
                            (inline, at) = (inline_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let at_3 = at + 5f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        }
        return { inline };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_project_module_path_attributes_loop7(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    plan: &[f64],
    mut selected: Vec<f64>,
    mut position: f64,
) -> Vec<f64> {
    loop {
        if position < (values.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, position);
                {
                    let value = self::read_required_text_fact(values, position);
                    if ((kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && (value == (self::ml_string::Text::from_units(vec![35]))))
                        && ((position + 1f64) < (values.len() as f64))
                    {
                        {
                            let inner = (self::read_required_text_fact(values, (position + 1f64)))
                                == (self::ml_string::Text::from_units(vec![33]));
                            {
                                let open = position + if inner { 2f64 } else { 1f64 };
                                if ((open >= (values.len() as f64))
                                    || ((self::read_required_text_fact(kinds, open))
                                        != (self::ml_string::Text::from_units(vec![
                                            112, 117, 110, 99, 116,
                                        ]))))
                                    || ((self::read_required_text_fact(values, open))
                                        != (self::ml_string::Text::from_units(vec![91])))
                                {
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
                                        let after = self::group_end(kinds, values, open);
                                        {
                                            let selected_3 = if ((open + 1f64)
                                                < (values.len() as f64))
                                                && ((self::read_required_text_fact(
                                                    values,
                                                    (open + 1f64),
                                                )) == (self::ml_string::Text::from_units(
                                                    vec![112, 97, 116, 104],
                                                ))) {
                                                if ((inner || (after != (open + 5f64)))
                                                    || ((self::read_required_text_fact(
                                                        values,
                                                        (open + 2f64),
                                                    )) != (self::ml_string::Text::from_units(
                                                        vec![61],
                                                    ))))
                                                    || ((self::read_required_text_fact(
                                                        kinds,
                                                        (open + 3f64),
                                                    )) != (self::ml_string::Text::from_units(
                                                        vec![115, 116, 114, 105, 110, 103],
                                                    )))
                                                {
                                                    (self::ml_source_exception::SourceException {
                                                        class_name: String::from("TypeError"),
                                                        message_units:
                                                            (self::ml_string::Text::from_units(
                                                                vec![
                                                                    82, 117, 115, 116, 32, 112,
                                                                    114, 111, 106, 101, 99, 116,
                                                                    32, 112, 97, 116, 104, 32, 97,
                                                                    116, 116, 114, 105, 98, 117,
                                                                    116, 101, 32, 110, 101, 101,
                                                                    100, 115, 32, 97, 110, 32, 111,
                                                                    117, 116, 101, 114, 32, 108,
                                                                    105, 116, 101, 114, 97, 108,
                                                                    32, 109, 111, 100, 117, 108,
                                                                    101, 32, 112, 97, 116, 104,
                                                                ],
                                                            ))
                                                            .0,
                                                        message_present: true,
                                                    })
                                                    .raise()
                                                } else {
                                                    {
                                                        let owner = after;
                                                        {
                                                            let owner_2 = self::source_ml_temporary_read_rust_project_module_path_attributes_loop8(kinds, values, owner);
                                                            {
                                                                let owner_5 = if (owner_2 < (values.len() as f64)) && ((self::read_required_text_fact(values, owner_2)) == (self::ml_string::Text::from_units(vec![112, 117, 98]))) {
                                                                    {
                                                                        let owner_3 = owner_2 + 1f64;
                                                                        if (owner_3 < (values.len() as f64)) && ((self::read_required_text_fact(values, owner_3)) == (self::ml_string::Text::from_units(vec![40]))) {
                                                                            {
                                                                                self::group_end(kinds, values, owner_3)
                                                                            }
                                                                        } else {
                                                                            owner_3
                                                                        }
                                                                    }
                                                                } else {
                                                                    owner_2
                                                                };
                                                                {
                                                                    let owner_7 = if (owner_5 < (values.len() as f64)) && ((self::read_required_text_fact(values, owner_5)) == (self::ml_string::Text::from_units(vec![117, 110, 115, 97, 102, 101]))) {
                                                                        {
                                                                            owner_5 + 1f64
                                                                        }
                                                                    } else {
                                                                        owner_5
                                                                    };
                                                                    {
                                                                        let record = -1f64;
                                                                        {
                                                                            let at = 0f64;
                                                                            {
                                                                                let record_2 = self::source_ml_temporary_read_rust_project_module_path_attributes_loop9(plan, owner_7, record, at);
                                                                                if record_2 < 0f64 {
                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 112, 97, 116, 104, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 109, 111, 100, 117, 108, 101, 32, 111, 119, 110, 101, 114])).0, message_present: true }).raise()
                                                                                } else if ((selected.len() as f64) > 0f64) && ((self::read_required_number_fact(&(selected), ((selected.len() as f64) - 4f64))) == record_2) {
                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100, 117, 108, 101, 32, 112, 97, 116, 104, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115])).0, message_present: true }).raise()
                                                                                    } else {
                                                                                        {
                                                                                            { let (ml_append_right, ml_append_left) = (vec![record_2, (open + 3f64), position, after], selected); self::ml_array::append(ml_append_left, ml_append_right) }
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
                                                selected
                                            };
                                            {
                                                let position_3 = after;
                                                {
                                                    (selected, position) = (selected_3, position_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    {
                        if (kind
                            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                            && (((value == (self::ml_string::Text::from_units(vec![123])))
                                || (value == (self::ml_string::Text::from_units(vec![91]))))
                                || (value == (self::ml_string::Text::from_units(vec![40]))))
                        {
                            {
                                let inline = false;
                                {
                                    let inline_3 = if value
                                        == (self::ml_string::Text::from_units(vec![123]))
                                    {
                                        {
                                            let at_2 = 0f64;
                                            {
                                                self::source_ml_temporary_read_rust_project_module_path_attributes_loop10(plan, position, inline, at_2)
                                            }
                                        }
                                    } else {
                                        inline
                                    };
                                    {
                                        let position_4 = if inline_3 {
                                            position + 1f64
                                        } else {
                                            self::group_end(kinds, values, position)
                                        };
                                        {
                                            position = position_4;
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        {
                            {
                                let position_5 = position + 1f64;
                                {
                                    position = position_5;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { selected };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_project_module_path_attributes_loop11(
    plan: &[f64],
    selected: &[f64],
    mut attributes: Vec<f64>,
    mut selected_at: f64,
    mut record: f64,
) -> Vec<f64> {
    loop {
        if (record * 5f64) < (plan.len() as f64) {
            if (selected_at < (selected.len() as f64))
                && ((self::read_required_number_fact(selected, selected_at)) == record)
            {
                {
                    let attributes_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![
                                self::read_required_number_fact(selected, (selected_at + 1f64)),
                                self::read_required_number_fact(selected, (selected_at + 2f64)),
                                self::read_required_number_fact(selected, (selected_at + 3f64)),
                            ],
                            attributes,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let selected_at_2 = selected_at + 4f64;
                        {
                            let record_2 = record + 1f64;
                            {
                                (attributes, selected_at, record) =
                                    (attributes_2, selected_at_2, record_2);
                                continue;
                            }
                        }
                    }
                }
            }
            {
                {
                    let attributes_3 = {
                        let (ml_append_right, ml_append_left) =
                            (vec![-1f64, -1f64, -1f64], attributes);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let record_3 = record + 1f64;
                        {
                            (attributes, record) = (attributes_3, record_3);
                            continue;
                        }
                    }
                }
            }
        }
        return { attributes };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_resolve_rust_project_literal_path_loop13(
    mut result: Vec<self::ml_string::Text>,
    source_ml_temporary_values12: &[self::ml_string::Text],
    mut source_ml_temporary_index12: self::ml::Big,
) -> Vec<self::ml_string::Text> {
    loop {
        return if source_ml_temporary_index12
            < (self::ml::Big::from_u128(source_ml_temporary_values12.len() as u128))
        {
            {
                let part = self::ml_array::at(
                    source_ml_temporary_values12,
                    source_ml_temporary_index12.to_index(),
                );
                if (part == (self::ml_string::Text::from_units(vec![])))
                    || (part == (self::ml_string::Text::from_units(vec![46])))
                {
                    {
                        let source_ml_temporary_index12_2 =
                            source_ml_temporary_index12.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index12 = source_ml_temporary_index12_2;
                            continue;
                        }
                    }
                }
                {
                    if part == (self::ml_string::Text::from_units(vec![46, 46])) {
                        if (result.len() as f64) == 0f64 {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32,
                                    108, 105, 116, 101, 114, 97, 108, 32, 112, 97, 116, 104, 32,
                                    101, 115, 99, 97, 112, 101, 115, 32, 115, 117, 112, 112, 108,
                                    105, 101, 100, 32, 105, 110, 118, 101, 110, 116, 111, 114, 121,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        } else {
                            {
                                let result_2 = self::ml_array::slice(&result, 0f64, -1f64);
                                {
                                    let source_ml_temporary_index12_3 = source_ml_temporary_index12
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_temporary_index12) =
                                            (result_2, source_ml_temporary_index12_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = {
                                let (ml_append_right, ml_append_left) =
                                    (vec![part.clone()], result);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let source_ml_temporary_index12_4 =
                                    source_ml_temporary_index12.add(&self::ml::Big::from_i128(1));
                                {
                                    (result, source_ml_temporary_index12) =
                                        (result_3, source_ml_temporary_index12_4);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_module_source_context_loop15(
    plan: &[f64],
    mut parent: f64,
    mut ancestors: Vec<f64>,
) -> Vec<f64> {
    loop {
        if parent >= 0f64 {
            {
                let ancestors_2 = self::ml_array::append(vec![parent], ancestors);
                {
                    let parent_2 = self::read_required_number_fact(plan, ((parent * 5f64) + 4f64));
                    {
                        (parent, ancestors) = (parent_2, ancestors_2);
                        continue;
                    }
                }
            }
        }
        return { ancestors };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryModuleSourceContextLoop16Result {
    MlTemporaryModuleSourceContextLoop16Done(self::ml_string::Text, self::ml_string::Text),
}
