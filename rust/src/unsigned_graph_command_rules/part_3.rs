// Generated from complete js/src/unsigned-graph-command-rules.js. Do not edit by hand.
use super::accept_unsigned_graph_root_marker;
use super::accept_unsigned_source_command_identity;
use super::compare_unsigned_graph_identities;
use super::ml_array;
use super::ml_string;
use super::normalize_unsigned_graph_identity;
use super::read_unsigned_graph_identity_end;
use super::read_unsigned_graph_link_end;
use super::read_unsigned_graph_metadata_end;
use super::skip_unsigned_graph_whitespace;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_unsigned_graph_identity_end_loop8(
    source: &self::ml_string::Text,
    mut end: f64,
) -> f64 {
    loop {
        return if end < (source.length()) {
            {
                let code = {
                    let source_ml_source1_string_unit_operand130 = source.clone();
                    {
                        let source_ml_source1_string_unit_operand131 = end;
                        (std::rc::Rc::new(
                            move |text: self::ml_string::Text, position: f64| -> f64 {
                                {
                                    let index = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index < 0f64) || (index >= (text.length())) {
                                        f64::NAN
                                    } else {
                                        ({
                                    let source_ml_source1_string_unit_operand137 = {
                                        let source_ml_source1_string_range_operand132 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand133 = index;
                                            {
                                                let source_ml_source1_string_range_operand134 = index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position135 = source_ml_source1_string_range_operand133;
                                                    {
                                                        let source_ml_source1_string_range_position136 = source_ml_source1_string_range_operand134;
                                                        source_ml_source1_string_range_operand132.slice(source_ml_source1_string_range_position135, source_ml_source1_string_range_position136)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand138 = 0f64;
                                        source_ml_source1_string_unit_operand137.code_point_at(source_ml_source1_string_unit_operand138)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand130,
                            source_ml_source1_string_unit_operand131,
                        )
                    }
                };
                if (code < 48f64) || (code > 57f64) {
                    end
                } else {
                    {
                        let end_2 = end + 1f64;
                        {
                            end = end_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            end
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadUnsignedGraphMetadataEndLoop9Result {
    MlTemporaryReadUnsignedGraphMetadataEndLoop9Done,
    MlTemporaryReadUnsignedGraphMetadataEndLoop9Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_unsigned_graph_metadata_end_loop9(
    source: &self::ml_string::Text,
    mut depth: f64,
    mut index: f64,
) -> self::MlTemporaryReadUnsignedGraphMetadataEndLoop9Result {
    loop {
        return if index < (source.length()) {
            {
                let code = {
                    let source_ml_source1_string_unit_operand139 = source.clone();
                    {
                        let source_ml_source1_string_unit_operand140 = index;
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
                                    let source_ml_source1_string_unit_operand146 = {
                                        let source_ml_source1_string_range_operand141 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand142 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand143 = index_2 + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position144 = source_ml_source1_string_range_operand142;
                                                    {
                                                        let source_ml_source1_string_range_position145 = source_ml_source1_string_range_operand143;
                                                        source_ml_source1_string_range_operand141.slice(source_ml_source1_string_range_position144, source_ml_source1_string_range_position145)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand147 = 0f64;
                                        source_ml_source1_string_unit_operand146.code_point_at(source_ml_source1_string_unit_operand147)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand139,
                            source_ml_source1_string_unit_operand140,
                        )
                    }
                };
                if code == 40f64 {
                    {
                        let depth_2 = depth + 1f64;
                        if depth_2 > 64f64 {
                            self::MlTemporaryReadUnsignedGraphMetadataEndLoop9Result::MlTemporaryReadUnsignedGraphMetadataEndLoop9Return(-1f64)
                        } else {
                            {
                                let index_2 = index + 1f64;
                                {
                                    (depth, index) = (depth_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                } else if code == 41f64 {
                    {
                        let depth_3 = depth - 1f64;
                        if depth_3 == 0f64 {
                            self::MlTemporaryReadUnsignedGraphMetadataEndLoop9Result::MlTemporaryReadUnsignedGraphMetadataEndLoop9Return(index + 1f64)
                        } else {
                            {
                                let index_3 = index + 1f64;
                                {
                                    (depth, index) = (depth_3, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    if ((((((((((((code == 9f64) || (code == 10f64)) || (code == 13f64))
                        || (code == 32f64))
                        || (code == 37f64))
                        || (code == 45f64))
                        || (code == 46f64))
                        || (code == 58f64))
                        || (code == 95f64))
                        || ((48f64..=57f64).contains(&code)))
                        || ((65f64..=90f64).contains(&code)))
                        || ((97f64..=122f64).contains(&code)))
                    {
                        {
                            let index_4 = index + 1f64;
                            {
                                index = index_4;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadUnsignedGraphMetadataEndLoop9Result::MlTemporaryReadUnsignedGraphMetadataEndLoop9Return(-1f64)
                    }
                }
            }
        } else {
            self::MlTemporaryReadUnsignedGraphMetadataEndLoop9Result::MlTemporaryReadUnsignedGraphMetadataEndLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadUnsignedGraphLinkEndLoop10Result {
    MlTemporaryReadUnsignedGraphLinkEndLoop10Done,
    MlTemporaryReadUnsignedGraphLinkEndLoop10Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_unsigned_graph_link_end_loop10(
    source: &self::ml_string::Text,
    mut cursor: f64,
) -> self::MlTemporaryReadUnsignedGraphLinkEndLoop10Result {
    loop {
        return if cursor < (source.length()) {
            {
                let cursor_2 = self::skip_unsigned_graph_whitespace(source, cursor);
                {
                    let token = {
                        let source_ml_source1_string_unit_operand148 = source.clone();
                        {
                            let source_ml_source1_string_unit_operand149 = cursor_2;
                            (std::rc::Rc::new(
                                move |text: self::ml_string::Text,
                                      position: f64|
                                      -> self::ml_string::Text {
                                    {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if (index < 0f64) || (index >= (text.length())) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            {
                                                let source_ml_source1_string_range_operand150 =
                                                    text.clone();
                                                {
                                                    let source_ml_source1_string_range_operand151 =
                                                        index;
                                                    {
                                                        let source_ml_source1_string_range_operand152 =
                                                            index + 1f64;
                                                        {
                                                            let source_ml_source1_string_range_position153 = source_ml_source1_string_range_operand151;
                                                            {
                                                                let source_ml_source1_string_range_position154 = source_ml_source1_string_range_operand152;
                                                                source_ml_source1_string_range_operand150.slice(source_ml_source1_string_range_position153, source_ml_source1_string_range_position154)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                            ))(
                                source_ml_source1_string_unit_operand148,
                                source_ml_source1_string_unit_operand149,
                            )
                        }
                    };
                    if token == (self::ml_string::Text::from_units(vec![41])) {
                        self::MlTemporaryReadUnsignedGraphLinkEndLoop10Result::MlTemporaryReadUnsignedGraphLinkEndLoop10Return(cursor_2 + 1f64)
                    } else if token == (self::ml_string::Text::from_units(vec![40])) {
                        {
                            let meta = self::read_unsigned_graph_metadata_end(source, cursor_2);
                            if meta < 0f64 {
                                self::MlTemporaryReadUnsignedGraphLinkEndLoop10Result::MlTemporaryReadUnsignedGraphLinkEndLoop10Return(-1f64)
                            } else {
                                {
                                    let close = self::skip_unsigned_graph_whitespace(source, meta);
                                    self::MlTemporaryReadUnsignedGraphLinkEndLoop10Result::MlTemporaryReadUnsignedGraphLinkEndLoop10Return({
let ml_product_condition_1 = {
                                            let source_ml_source1_string_unit_operand162 = source.clone();
                                            {
                                                let source_ml_source1_string_unit_operand163 = close;
                                                (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                                    let index = if position.is_nan() {
                                                        0f64
                                                    } else {
                                                        position.trunc()
                                                    };
                                                    if (index < 0f64) || (index >= (text.length())) {
                                                        self::ml_string::Text::from_units(vec![])
                                                    } else {
                                                        {
                                                            let source_ml_source1_string_range_operand164 = text.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand165 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand166 = index + 1f64;
                                                                    {
                                                                        let source_ml_source1_string_range_position167 = source_ml_source1_string_range_operand165;
                                                                        {
                                                                            let source_ml_source1_string_range_position168 = source_ml_source1_string_range_operand166;
                                                                            source_ml_source1_string_range_operand164.slice(source_ml_source1_string_range_position167, source_ml_source1_string_range_position168)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } }))(source_ml_source1_string_unit_operand162, source_ml_source1_string_unit_operand163)
                                            }
                                        };
if ml_product_condition_1 {
                                            close + 1f64
                                        } else {
                                            -1f64
                                        }
})
                                }
                            }
                        }
                    } else {
                        {
                            let next = self::read_unsigned_graph_identity_end(source, cursor_2);
                            if next < 0f64 {
                                self::MlTemporaryReadUnsignedGraphLinkEndLoop10Result::MlTemporaryReadUnsignedGraphLinkEndLoop10Return(-1f64)
                            } else {
                                {
                                    let cursor_3 = next;
                                    {
                                        cursor = cursor_3;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadUnsignedGraphLinkEndLoop10Result::MlTemporaryReadUnsignedGraphLinkEndLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Result {
    MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Done,
    MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_unsigned_graph_text_surface_error_loop11(
    source: &self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Result {
    loop {
        return if at < (source.length()) {
            {
                let end = self::read_unsigned_graph_link_end(source, at);
                if end < 0f64 {
                    self::MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Result::MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Return(self::ml_string::Text::from_units(vec![115, 97, 118, 101, 100, 32, 103, 114, 97, 112, 104, 32, 116, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 100, 101, 110, 116, 105, 102, 105, 101, 100, 32, 102, 108, 97, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 108, 105, 110, 107, 115, 32, 119, 105, 116, 104, 32, 99, 97, 110, 111, 110, 105, 99, 97, 108, 32, 109, 101, 116, 97, 100, 97, 116, 97]))
                } else {
                    {
                        let at_2 = self::skip_unsigned_graph_whitespace(source, end);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Result::MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanUnsignedTypedGraphRootLoop12Result {
    MlTemporaryPlanUnsignedTypedGraphRootLoop12Done(self::ml_string::Text, f64),
    MlTemporaryPlanUnsignedTypedGraphRootLoop12Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_unsigned_typed_graph_root_loop12(
    ids: &[self::ml_string::Text],
    types: &[self::ml_string::Text],
    languages: &[self::ml_string::Text],
    terms: &[self::ml_string::Text],
    definitions: &[self::ml_string::Text],
    selected: &self::ml_string::Text,
    mut found: self::ml_string::Text,
    mut count: f64,
    mut at: f64,
) -> self::MlTemporaryPlanUnsignedTypedGraphRootLoop12Result {
    loop {
        return if at < (ids.len() as f64) {
            {
                let id = (self::ml_array::optional_at(ids, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if (!(self::accept_unsigned_source_command_identity(&(id)))
                    || ((self::normalize_unsigned_graph_identity(&(id))) != id))
                    || ((at > 0f64)
                        && ((self::compare_unsigned_graph_identities(
                            &((self::ml_array::optional_at(
                                ids,
                                self::ml_array::number_index(at - 1f64),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))),
                            &(id),
                        )) >= 0f64))
                {
                    self::MlTemporaryPlanUnsignedTypedGraphRootLoop12Result::MlTemporaryPlanUnsignedTypedGraphRootLoop12Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 114, 111, 111, 116, 32, 102, 97, 99, 116, 115, 32, 110, 101, 101, 100, 32, 99, 97, 110, 111, 110, 105, 99, 97, 108, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115])])
                } else {
                    if !(self::accept_unsigned_graph_root_marker(
                        &((self::ml_array::optional_at(types, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))),
                        &((self::ml_array::optional_at(
                            languages,
                            self::ml_array::number_index(at),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))),
                        &((self::ml_array::optional_at(terms, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))),
                        &((self::ml_array::optional_at(
                            definitions,
                            self::ml_array::number_index(at),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))),
                    )) {
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
                            let found_3 = if ((*selected)
                                != (self::ml_string::Text::from_units(vec![])))
                                && (id == (*selected))
                            {
                                { id.clone() }
                            } else {
                                found.clone()
                            };
                            if (*selected) == (self::ml_string::Text::from_units(vec![])) {
                                {
                                    let found_4 = id.clone();
                                    {
                                        let count_2 = count + 1f64;
                                        {
                                            let at_3 = at + 1f64;
                                            {
                                                (found, count, at) = (found_4, count_2, at_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let at_4 = at + 1f64;
                                    {
                                        (found, at) = (found_3, at_4);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanUnsignedTypedGraphRootLoop12Result::MlTemporaryPlanUnsignedTypedGraphRootLoop12Done(found.clone(), count)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanUnsignedGraphCommandLoop13Result {
    MlTemporaryPlanUnsignedGraphCommandLoop13Done(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        bool,
        bool,
    ),
    MlTemporaryPlanUnsignedGraphCommandLoop13Return(Vec<self::ml_string::Text>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanUnsignedGraphCommandJoin14 {
    MlTemporaryPlanUnsignedGraphCommandJoin14Next(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    MlTemporaryPlanUnsignedGraphCommandJoin14Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_unsigned_graph_command_loop13(
    args: &[self::ml_string::Text],
    mut file: self::ml_string::Text,
    mut root: self::ml_string::Text,
    mut first: self::ml_string::Text,
    mut target: self::ml_string::Text,
    mut input: bool,
    mut binary: bool,
    mut ended: bool,
    mut at: f64,
) -> self::MlTemporaryPlanUnsignedGraphCommandLoop13Result {
    loop {
        return if at < (args.len() as f64) {
            {
                let arg = (self::ml_array::optional_at(args, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if !ended
                    && (((arg
                        == (self::ml_string::Text::from_units(vec![45, 45, 104, 101, 108, 112])))
                        || (arg == (self::ml_string::Text::from_units(vec![45, 104]))))
                        || (arg == (self::ml_string::Text::from_units(vec![104, 101, 108, 112]))))
                {
                    self::MlTemporaryPlanUnsignedGraphCommandLoop13Result::MlTemporaryPlanUnsignedGraphCommandLoop13Return(vec![self::ml_string::Text::from_units(vec![104, 101, 108, 112])])
                } else {
                    if !ended && (arg == (self::ml_string::Text::from_units(vec![45, 45]))) {
                        {
                            let ended_2 = true;
                            {
                                let at_2 = at + 1f64;
                                {
                                    (ended, at) = (ended_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if !ended
                            && (arg
                                == (self::ml_string::Text::from_units(vec![
                                    45, 45, 105, 110, 112, 117, 116, 45, 98, 105, 110, 97, 114, 121,
                                ])))
                        {
                            if input {
                                self::MlTemporaryPlanUnsignedGraphCommandLoop13Result::MlTemporaryPlanUnsignedGraphCommandLoop13Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                            } else {
                                {
                                    let input_2 = true;
                                    {
                                        let at_3 = at + 1f64;
                                        {
                                            (input, at) = (input_2, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if !ended
                            && (arg
                                == (self::ml_string::Text::from_units(vec![
                                    45, 45, 98, 105, 110, 97, 114, 121,
                                ])))
                        {
                            if binary {
                                self::MlTemporaryPlanUnsignedGraphCommandLoop13Result::MlTemporaryPlanUnsignedGraphCommandLoop13Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                            } else {
                                {
                                    let binary_2 = true;
                                    {
                                        let at_4 = at + 1f64;
                                        {
                                            (binary, at) = (binary_2, at_4);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if !ended
                            && (((arg
                                == (self::ml_string::Text::from_units(vec![
                                    45, 45, 114, 111, 111, 116,
                                ])))
                                || (arg
                                    == (self::ml_string::Text::from_units(vec![
                                        45, 45, 114, 101, 98, 97, 115, 101,
                                    ]))))
                                || (arg
                                    == (self::ml_string::Text::from_units(vec![45, 45, 116, 111]))))
                        {
                            {
                                let at_5 = at + 1f64;
                                {
                                    let value = (self::ml_array::optional_at(
                                        args,
                                        self::ml_array::number_index(at_5),
                                    ))
                                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                    if (value == (self::ml_string::Text::from_units(vec![])))
                                        || value.starts_with(&self::ml_string::Text::from_units(
                                            vec![45],
                                        ))
                                    {
                                        self::MlTemporaryPlanUnsignedGraphCommandLoop13Result::MlTemporaryPlanUnsignedGraphCommandLoop13Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                                    } else {
                                        {
                                            let source_ml_source1_s169 = if arg == (self::ml_string::Text::from_units(vec![45, 45, 116, 111])) {
                                                        {
let ml_library_inequality_3 = target != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_3 {
                                                            self::MlTemporaryPlanUnsignedGraphCommandJoin14::MlTemporaryPlanUnsignedGraphCommandJoin14Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                                                        } else {
                                                            {
                                                                let target_2 = value.clone();
                                                                self::MlTemporaryPlanUnsignedGraphCommandJoin14::MlTemporaryPlanUnsignedGraphCommandJoin14Next(root.clone(), first.clone(), target_2)
                                                            }
                                                        }
}
                                                    } else if (self::accept_unsigned_source_command_identity(&(value))) {if arg == (self::ml_string::Text::from_units(vec![45, 45, 114, 111, 111, 116])) {
                                                                {
let ml_library_inequality_2 = root != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_2 {
                                                                    self::MlTemporaryPlanUnsignedGraphCommandJoin14::MlTemporaryPlanUnsignedGraphCommandJoin14Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                                                                } else {
                                                                    {
                                                                        let root_2 = value.clone();
                                                                        self::MlTemporaryPlanUnsignedGraphCommandJoin14::MlTemporaryPlanUnsignedGraphCommandJoin14Next(root_2, first.clone(), target.clone())
                                                                    }
                                                                }
}
                                                            } else {
let ml_library_inequality_1 = first != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                                    self::MlTemporaryPlanUnsignedGraphCommandJoin14::MlTemporaryPlanUnsignedGraphCommandJoin14Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                                                                } else {
                                                                    {
                                                                        let first_2 = value.clone();
                                                                        self::MlTemporaryPlanUnsignedGraphCommandJoin14::MlTemporaryPlanUnsignedGraphCommandJoin14Next(root.clone(), first_2, target.clone())
                                                                    }
                                                                }
}} else {
                                                            self::MlTemporaryPlanUnsignedGraphCommandJoin14::MlTemporaryPlanUnsignedGraphCommandJoin14Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                                                        };
                                            match source_ml_source1_s169 {
                                                        self::MlTemporaryPlanUnsignedGraphCommandJoin14::MlTemporaryPlanUnsignedGraphCommandJoin14Next(root_3, first_3, target_3) => {
                                                            {
                                                                let at_6 = at_5 + 1f64;
                                                                {
                                                                    (root, first, target, at) = (root_3.clone(), first_3.clone(), target_3.clone(), at_6);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::MlTemporaryPlanUnsignedGraphCommandJoin14::MlTemporaryPlanUnsignedGraphCommandJoin14Return(source_ml_temporary_result14) => {
                                                            self::MlTemporaryPlanUnsignedGraphCommandLoop13Result::MlTemporaryPlanUnsignedGraphCommandLoop13Return(source_ml_temporary_result14)
                                                        }
                                                    }
                                        }
                                    }
                                }
                            }
                        } else if ((file != (self::ml_string::Text::from_units(vec![])))
                            || (arg == (self::ml_string::Text::from_units(vec![]))))
                            || (!ended
                                && arg.starts_with(&self::ml_string::Text::from_units(vec![45])))
                        {
                            self::MlTemporaryPlanUnsignedGraphCommandLoop13Result::MlTemporaryPlanUnsignedGraphCommandLoop13Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                        } else {
                            {
                                let file_2 = arg.clone();
                                {
                                    let at_7 = at + 1f64;
                                    {
                                        (file, at) = (file_2, at_7);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanUnsignedGraphCommandLoop13Result::MlTemporaryPlanUnsignedGraphCommandLoop13Done(file.clone(), root.clone(), first.clone(), target.clone(), input, binary)
        };
    }
}
