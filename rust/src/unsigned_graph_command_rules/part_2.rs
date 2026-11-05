// Generated from complete js/src/unsigned-graph-command-rules.js. Do not edit by hand.
use super::MlTemporaryPlanUnsignedGraphCommandLoop13Result;
use super::MlTemporaryPlanUnsignedTypedGraphRootLoop12Result;
use super::MlTemporaryReadUnsignedGraphLinkEndLoop10Result;
use super::MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Result;
use super::accept_unsigned_source_command_identity;
use super::accept_unsigned_source_target;
use super::ml_math;
use super::ml_string;
use super::normalize_unsigned_graph_identity;
use super::read_unsigned_graph_identity_end;
use super::skip_unsigned_graph_whitespace;
use super::source_ml_temporary_plan_unsigned_graph_command_loop13;
use super::source_ml_temporary_plan_unsigned_typed_graph_root_loop12;
use super::source_ml_temporary_read_unsigned_graph_link_end_loop10;
use super::source_ml_temporary_read_unsigned_graph_text_surface_error_loop11;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unsigned_graph_link_end(source: &self::ml_string::Text, at: f64) -> f64 {
    {
        let ml_product_condition_2 = {
            let source_ml_source1_string_unit_operand36 = source.clone();
            {
                let source_ml_source1_string_unit_operand37 = at;
                (std::rc::Rc::new(
                    move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text {
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
                                    let source_ml_source1_string_range_operand38 = text.clone();
                                    {
                                        let source_ml_source1_string_range_operand39 = index;
                                        {
                                            let source_ml_source1_string_range_operand40 =
                                                index + 1f64;
                                            {
                                                let source_ml_source1_string_range_position41 =
                                                    source_ml_source1_string_range_operand39;
                                                {
                                                    let source_ml_source1_string_range_position42 =
                                                        source_ml_source1_string_range_operand40;
                                                    source_ml_source1_string_range_operand38.slice(
                                                        source_ml_source1_string_range_position41,
                                                        source_ml_source1_string_range_position42,
                                                    )
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                ))(
                    source_ml_source1_string_unit_operand36,
                    source_ml_source1_string_unit_operand37,
                )
            }
        };
        if ml_product_condition_2 {
            -1f64
        } else {
            {
                let end = self::read_unsigned_graph_identity_end(
                    source,
                    self::skip_unsigned_graph_whitespace(source, (at + 1f64)),
                );
                if end < 0f64 {
                    -1f64
                } else {
                    {
                        let cursor = self::skip_unsigned_graph_whitespace(source, end);
                        {
                            let ml_product_condition_1 = {
                                let source_ml_source1_string_unit_operand50 = source.clone();
                                {
                                    let source_ml_source1_string_unit_operand51 = cursor;
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
                                        let source_ml_source1_string_range_operand52 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand53 = index;
                                            {
                                                let source_ml_source1_string_range_operand54 = index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position55 = source_ml_source1_string_range_operand53;
                                                    {
                                                        let source_ml_source1_string_range_position56 = source_ml_source1_string_range_operand54;
                                                        source_ml_source1_string_range_operand52.slice(source_ml_source1_string_range_position55, source_ml_source1_string_range_position56)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } }))(source_ml_source1_string_unit_operand50, source_ml_source1_string_unit_operand51)
                                }
                            };
                            if ml_product_condition_1 {
                                -1f64
                            } else {
                                {
                                    let cursor_2 = cursor + 1f64;
                                    {
                                        let source_ml_source1_s57 = self::source_ml_temporary_read_unsigned_graph_link_end_loop10(source, cursor_2);
                                        match source_ml_source1_s57 {
                                    self::MlTemporaryReadUnsignedGraphLinkEndLoop10Result::MlTemporaryReadUnsignedGraphLinkEndLoop10Done => {
                                        -1f64
                                    }
                                    self::MlTemporaryReadUnsignedGraphLinkEndLoop10Result::MlTemporaryReadUnsignedGraphLinkEndLoop10Return(source_ml_temporary_result10) => {
                                        source_ml_temporary_result10
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unsigned_graph_text_surface_error(
    source: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let at = self::skip_unsigned_graph_whitespace(source, 0f64);
        {
            let source_ml_source1_s58 =
                self::source_ml_temporary_read_unsigned_graph_text_surface_error_loop11(source, at);
            match source_ml_source1_s58 {
                self::MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Result::MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Done => {
                    self::ml_string::Text::from_units(vec![])
                }
                self::MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Result::MlTemporaryReadUnsignedGraphTextSurfaceErrorLoop11Return(source_ml_temporary_result11) => {
                    source_ml_temporary_result11
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unsigned_graph_read_bounds_error(
    bytes: f64,
    records: f64,
    byte_limit: f64,
    record_limit: f64,
) -> self::ml_string::Text {
    if ((!(self::ml_math::is_safe_integer(byte_limit)) || (byte_limit < 33f64))
        || !(self::ml_math::is_safe_integer(record_limit)))
        || (record_limit < 1f64)
    {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 115, 97, 118, 101, 100, 32, 103, 114, 97, 112,
            104, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 111, 117, 110, 100, 115,
        ])
    } else if ((!(self::ml_math::is_safe_integer(bytes)) || (bytes < 0f64))
        || !(self::ml_math::is_safe_integer(records)))
        || (records < 0f64)
    {
        self::ml_string::Text::from_units(vec![
            115, 97, 118, 101, 100, 32, 103, 114, 97, 112, 104, 32, 105, 110, 112, 117, 116, 32,
            110, 101, 101, 100, 115, 32, 101, 120, 97, 99, 116, 32, 110, 111, 110, 110, 101, 103,
            97, 116, 105, 118, 101, 32, 98, 121, 116, 101, 32, 97, 110, 100, 32, 114, 101, 99, 111,
            114, 100, 32, 99, 111, 117, 110, 116, 115,
        ])
    } else if bytes > byte_limit {
        self::ml_string::Text::from_units(vec![
            115, 97, 118, 101, 100, 32, 103, 114, 97, 112, 104, 32, 105, 110, 112, 117, 116, 32,
            101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 98, 121, 116, 101, 32, 108,
            105, 109, 105, 116,
        ])
    } else if records > record_limit {
        self::ml_string::Text::from_units(vec![
            115, 97, 118, 101, 100, 32, 103, 114, 97, 112, 104, 32, 105, 110, 112, 117, 116, 32,
            101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 99, 111, 114, 100,
            32, 108, 105, 109, 105, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unsigned_typed_root_bounds_error(records: f64, limit: f64) -> self::ml_string::Text {
    if (((!(self::ml_math::is_safe_integer(limit)) || (limit < 1f64))
        || !(self::ml_math::is_safe_integer(records)))
        || (records < 0f64))
        || (records > limit)
    {
        self::ml_string::Text::from_units(vec![
            116, 121, 112, 101, 100, 32, 114, 111, 111, 116, 32, 102, 97, 99, 116, 115, 32, 101,
            120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 114, 101, 99, 111, 114, 100,
            32, 108, 105, 109, 105, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_unsigned_typed_graph_root(
    ids: &[self::ml_string::Text],
    types: &[self::ml_string::Text],
    languages: &[self::ml_string::Text],
    terms: &[self::ml_string::Text],
    definitions: &[self::ml_string::Text],
    requested: &self::ml_string::Text,
    limit: f64,
) -> Vec<self::ml_string::Text> {
    {
        let bounds = self::read_unsigned_typed_root_bounds_error((ids.len() as f64), limit);
        {
            let ml_library_inequality_2 = bounds != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                vec![
                    self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                    bounds,
                ]
            } else if ((((types.len() as f64) != (ids.len() as f64))
                || ((languages.len() as f64) != (ids.len() as f64)))
                || ((terms.len() as f64) != (ids.len() as f64)))
                || ((definitions.len() as f64) != (ids.len() as f64))
            {
                vec![
                    self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                    self::ml_string::Text::from_units(vec![
                        116, 121, 112, 101, 100, 32, 114, 111, 111, 116, 32, 102, 97, 99, 116, 115,
                        32, 110, 101, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97,
                        108, 105, 103, 110, 101, 100, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32,
                        99, 111, 108, 117, 109, 110, 115,
                    ]),
                ]
            } else if ((*requested) != (self::ml_string::Text::from_units(vec![])))
                && !(self::accept_unsigned_source_command_identity(requested))
            {
                vec![
                    self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                    self::ml_string::Text::from_units(vec![
                        97, 32, 116, 121, 112, 101, 100, 32, 103, 114, 97, 112, 104, 32, 114, 111,
                        111, 116, 32, 110, 101, 101, 100, 115, 32, 97, 110, 32, 101, 120, 97, 99,
                        116, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105,
                        103, 110, 101, 100, 32, 105, 100, 101, 110, 116, 105, 116, 121,
                    ]),
                ]
            } else {
                {
                    let selected = if (*requested) == (self::ml_string::Text::from_units(vec![])) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::normalize_unsigned_graph_identity(requested)
                    };
                    {
                        let found = self::ml_string::Text::from_units(vec![]);
                        {
                            let count = 0f64;
                            {
                                let at = 0f64;
                                {
                                    let source_ml_source1_s59 = self::source_ml_temporary_plan_unsigned_typed_graph_root_loop12(ids, types, languages, terms, definitions, &(selected), found, count, at);
                                    match source_ml_source1_s59 {
                                            self::MlTemporaryPlanUnsignedTypedGraphRootLoop12Result::MlTemporaryPlanUnsignedTypedGraphRootLoop12Done(found_2, count_2) => {
                                                {
let ml_library_inequality_1 = selected != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                    if found_2 == (self::ml_string::Text::from_units(vec![])) {
                                                        vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 108, 105, 110, 107, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 116, 121, 112, 101, 100, 32, 112, 114, 111, 103, 114, 97, 109, 32, 114, 111, 111, 116])]
                                                    } else {
                                                        vec![self::ml_string::Text::from_units(vec![114, 111, 111, 116]), found_2]
                                                    }
                                                } else if count_2 == 1f64 {
                                                        vec![self::ml_string::Text::from_units(vec![114, 111, 111, 116]), found_2]
                                                    } else if count_2 == 0f64 {
                                                            vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 103, 114, 97, 112, 104, 32, 110, 101, 101, 100, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 99, 104, 101, 109, 97, 45, 49, 32, 116, 121, 112, 101, 100, 32, 112, 114, 111, 103, 114, 97, 109, 32, 114, 111, 111, 116])]
                                                        } else {
                                                            vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 103, 114, 97, 112, 104, 32, 119, 105, 116, 104, 32, 109, 117, 108, 116, 105, 112, 108, 101, 32, 116, 121, 112, 101, 100, 32, 112, 114, 111, 103, 114, 97, 109, 32, 114, 111, 111, 116, 115, 32, 110, 101, 101, 100, 115, 32, 97, 110, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 111, 111, 116])]
                                                        }
}
                                            }
                                            self::MlTemporaryPlanUnsignedTypedGraphRootLoop12Result::MlTemporaryPlanUnsignedTypedGraphRootLoop12Return(source_ml_temporary_result12) => {
                                                source_ml_temporary_result12
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
pub fn read_unsigned_graph_command_usage() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        117, 115, 97, 103, 101, 58, 32, 109, 101, 116, 97, 45, 108, 97, 110, 103, 117, 97, 103,
        101, 32, 101, 120, 97, 99, 116, 45, 103, 114, 97, 112, 104, 32, 60, 102, 105, 108, 101, 62,
        32, 91, 45, 45, 105, 110, 112, 117, 116, 45, 98, 105, 110, 97, 114, 121, 93, 32, 91, 45,
        45, 98, 105, 110, 97, 114, 121, 32, 124, 32, 45, 45, 116, 111, 32, 60, 74, 97, 118, 97, 83,
        99, 114, 105, 112, 116, 124, 82, 117, 115, 116, 124, 76, 101, 97, 110, 124, 82, 111, 99,
        113, 62, 93, 32, 91, 45, 45, 114, 111, 111, 116, 32, 60, 100, 101, 99, 105, 109, 97, 108,
        62, 93, 32, 91, 45, 45, 114, 101, 98, 97, 115, 101, 32, 60, 100, 101, 99, 105, 109, 97,
        108, 62, 93, 10, 82, 101, 97, 100, 115, 32, 101, 120, 97, 99, 116, 32, 76, 105, 110, 107,
        115, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115, 32, 119, 105, 116, 104, 111, 117, 116,
        32, 114, 101, 112, 97, 114, 115, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 59, 32,
        101, 109, 105, 115, 115, 105, 111, 110, 32, 99, 104, 101, 99, 107, 115, 32, 116, 104, 101,
        32, 99, 117, 114, 114, 101, 110, 116, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 116,
        121, 112, 101, 100, 32, 103, 114, 97, 112, 104, 46, 10,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_unsigned_graph_command(
    file: &self::ml_string::Text,
    root: &self::ml_string::Text,
    first: &self::ml_string::Text,
    target: &self::ml_string::Text,
    input: bool,
    binary: bool,
) -> Vec<self::ml_string::Text> {
    if ((((*file) == (self::ml_string::Text::from_units(vec![])))
        || (((*root) != (self::ml_string::Text::from_units(vec![])))
            && ((*target) == (self::ml_string::Text::from_units(vec![])))))
        || (binary && ((*target) != (self::ml_string::Text::from_units(vec![])))))
        || (((*target) != (self::ml_string::Text::from_units(vec![])))
            && !(self::accept_unsigned_source_target(target)))
    {
        vec![self::ml_string::Text::from_units(vec![
            101, 114, 114, 111, 114,
        ])]
    } else {
        vec![
            file.clone(),
            if input {
                self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121])
            } else {
                self::ml_string::Text::from_units(vec![116, 101, 120, 116])
            },
            if binary {
                self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121])
            } else {
                self::ml_string::Text::from_units(vec![116, 101, 120, 116])
            },
            target.clone(),
            root.clone(),
            first.clone(),
        ]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_unsigned_graph_command(args: &[self::ml_string::Text]) -> Vec<self::ml_string::Text> {
    {
        let file = self::ml_string::Text::from_units(vec![]);
        {
            let root = self::ml_string::Text::from_units(vec![]);
            {
                let first = self::ml_string::Text::from_units(vec![]);
                {
                    let target = self::ml_string::Text::from_units(vec![]);
                    {
                        let input = false;
                        {
                            let binary = false;
                            {
                                let ended = false;
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_source1_s60 = self::source_ml_temporary_plan_unsigned_graph_command_loop13(args, file, root, first, target, input, binary, ended, at);
                                        match source_ml_source1_s60 {
                                            self::MlTemporaryPlanUnsignedGraphCommandLoop13Result::MlTemporaryPlanUnsignedGraphCommandLoop13Done(file_2, root_2, first_2, target_2, input_2, binary_2) => {
                                                self::finish_unsigned_graph_command(&(file_2), &(root_2), &(first_2), &(target_2), input_2, binary_2)
                                            }
                                            self::MlTemporaryPlanUnsignedGraphCommandLoop13Result::MlTemporaryPlanUnsignedGraphCommandLoop13Return(source_ml_temporary_result13) => {
                                                source_ml_temporary_result13
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Result {
    MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Done,
    MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_unsigned_source_command_identity_loop2(
    digits: &self::ml_string::Text,
    maximum: &self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Result {
    loop {
        return if at < 20f64 {
            {
                let actual = {
                    let source_ml_source1_string_unit_operand61 = digits.clone();
                    {
                        let source_ml_source1_string_unit_operand62 = at;
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
                                    let source_ml_source1_string_unit_operand68 = {
                                        let source_ml_source1_string_range_operand63 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand64 = index;
                                            {
                                                let source_ml_source1_string_range_operand65 = index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position66 = source_ml_source1_string_range_operand64;
                                                    {
                                                        let source_ml_source1_string_range_position67 = source_ml_source1_string_range_operand65;
                                                        source_ml_source1_string_range_operand63.slice(source_ml_source1_string_range_position66, source_ml_source1_string_range_position67)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand69 = 0f64;
                                        source_ml_source1_string_unit_operand68.code_point_at(source_ml_source1_string_unit_operand69)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand61,
                            source_ml_source1_string_unit_operand62,
                        )
                    }
                };
                {
                    let limit = {
                        let source_ml_source1_string_unit_operand70 = maximum.clone();
                        {
                            let source_ml_source1_string_unit_operand71 = at;
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
                                        let source_ml_source1_string_unit_operand77 = {
                                            let source_ml_source1_string_range_operand72 = text.clone();
                                            {
                                                let source_ml_source1_string_range_operand73 = index;
                                                {
                                                    let source_ml_source1_string_range_operand74 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position75 = source_ml_source1_string_range_operand73;
                                                        {
                                                            let source_ml_source1_string_range_position76 = source_ml_source1_string_range_operand74;
                                                            source_ml_source1_string_range_operand72.slice(source_ml_source1_string_range_position75, source_ml_source1_string_range_position76)
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                        {
                                            let source_ml_source1_string_unit_operand78 = 0f64;
                                            source_ml_source1_string_unit_operand77.code_point_at(source_ml_source1_string_unit_operand78)
                                        }
                                    }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source1_string_unit_operand70,
                                source_ml_source1_string_unit_operand71,
                            )
                        }
                    };
                    {
                        let ml_library_inequality_1 = actual != limit;
                        if ml_library_inequality_1 {
                            self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Result::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Return(actual < limit)
                        } else {
                            {
                                let at_2 = at + 1f64;
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Result::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Result {
    MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Done(f64),
    MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_unsigned_source_command_identity_loop1(
    value: &self::ml_string::Text,
    mut first: f64,
    mut at: f64,
) -> self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Result {
    loop {
        return if at < (value.length()) {
            {
                let code = {
                    let source_ml_source1_string_unit_operand79 = value.clone();
                    {
                        let source_ml_source1_string_unit_operand80 = at;
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
                                    let source_ml_source1_string_unit_operand86 = {
                                        let source_ml_source1_string_range_operand81 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand82 = index;
                                            {
                                                let source_ml_source1_string_range_operand83 = index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position84 = source_ml_source1_string_range_operand82;
                                                    {
                                                        let source_ml_source1_string_range_position85 = source_ml_source1_string_range_operand83;
                                                        source_ml_source1_string_range_operand81.slice(source_ml_source1_string_range_position84, source_ml_source1_string_range_position85)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand87 = 0f64;
                                        source_ml_source1_string_unit_operand86.code_point_at(source_ml_source1_string_unit_operand87)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand79,
                            source_ml_source1_string_unit_operand80,
                        )
                    }
                };
                if (code < 48f64) || (code > 57f64) {
                    self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Result::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Return(false)
                } else {
                    if (first == at) && (code == 48f64) {
                        {
                            let first_2 = first + 1f64;
                            {
                                let at_2 = at + 1f64;
                                {
                                    (first, at) = (first_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let at_3 = at + 1f64;
                            {
                                at = at_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Result::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Done(first)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_normalize_unsigned_graph_identity_loop5(
    value: &self::ml_string::Text,
    mut at: f64,
) -> f64 {
    loop {
        if ((at + 1f64) < (value.length()))
            && (({
                let source_ml_source1_string_unit_operand96 = value.clone();
                {
                    let source_ml_source1_string_unit_operand97 = at;
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
                                        let source_ml_source1_string_range_operand98 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand99 = index;
                                            {
                                                let source_ml_source1_string_range_operand100 =
                                                    index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position101 =
                                                        source_ml_source1_string_range_operand99;
                                                    {
                                                        let source_ml_source1_string_range_position102 = source_ml_source1_string_range_operand100;
                                                        source_ml_source1_string_range_operand98.slice(source_ml_source1_string_range_position101, source_ml_source1_string_range_position102)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    ))(
                        source_ml_source1_string_unit_operand96,
                        source_ml_source1_string_unit_operand97,
                    )
                }
            }) == (self::ml_string::Text::from_units(vec![48])))
        {
            {
                let at_2 = at + 1f64;
                {
                    at = at_2;
                    continue;
                }
            }
        }
        return { at };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareUnsignedGraphIdentitiesLoop6Result {
    MlTemporaryCompareUnsignedGraphIdentitiesLoop6Done,
    MlTemporaryCompareUnsignedGraphIdentitiesLoop6Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_unsigned_graph_identities_loop6(
    first: &self::ml_string::Text,
    second: &self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryCompareUnsignedGraphIdentitiesLoop6Result {
    loop {
        return if at < (first.length()) {
            {
                let a = {
                    let source_ml_source1_string_unit_operand103 = first.clone();
                    {
                        let source_ml_source1_string_unit_operand104 = at;
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
                                    let source_ml_source1_string_unit_operand110 = {
                                        let source_ml_source1_string_range_operand105 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand106 = index;
                                            {
                                                let source_ml_source1_string_range_operand107 = index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position108 = source_ml_source1_string_range_operand106;
                                                    {
                                                        let source_ml_source1_string_range_position109 = source_ml_source1_string_range_operand107;
                                                        source_ml_source1_string_range_operand105.slice(source_ml_source1_string_range_position108, source_ml_source1_string_range_position109)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand111 = 0f64;
                                        source_ml_source1_string_unit_operand110.code_point_at(source_ml_source1_string_unit_operand111)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand103,
                            source_ml_source1_string_unit_operand104,
                        )
                    }
                };
                {
                    let b = {
                        let source_ml_source1_string_unit_operand112 = second.clone();
                        {
                            let source_ml_source1_string_unit_operand113 = at;
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
                                        let source_ml_source1_string_unit_operand119 = {
                                            let source_ml_source1_string_range_operand114 = text.clone();
                                            {
                                                let source_ml_source1_string_range_operand115 = index;
                                                {
                                                    let source_ml_source1_string_range_operand116 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position117 = source_ml_source1_string_range_operand115;
                                                        {
                                                            let source_ml_source1_string_range_position118 = source_ml_source1_string_range_operand116;
                                                            source_ml_source1_string_range_operand114.slice(source_ml_source1_string_range_position117, source_ml_source1_string_range_position118)
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                        {
                                            let source_ml_source1_string_unit_operand120 = 0f64;
                                            source_ml_source1_string_unit_operand119.code_point_at(source_ml_source1_string_unit_operand120)
                                        }
                                    }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source1_string_unit_operand112,
                                source_ml_source1_string_unit_operand113,
                            )
                        }
                    };
                    {
                        let ml_library_inequality_1 = a != b;
                        if ml_library_inequality_1 {
                            self::MlTemporaryCompareUnsignedGraphIdentitiesLoop6Result::MlTemporaryCompareUnsignedGraphIdentitiesLoop6Return(if a < b {
                            -1f64
                        } else {
                            1f64
                        })
                        } else {
                            {
                                let at_2 = at + 1f64;
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompareUnsignedGraphIdentitiesLoop6Result::MlTemporaryCompareUnsignedGraphIdentitiesLoop6Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_skip_unsigned_graph_whitespace_loop7(
    source: &self::ml_string::Text,
    mut at: f64,
) -> f64 {
    loop {
        return if at < (source.length()) {
            {
                let code = {
                    let source_ml_source1_string_unit_operand121 = source.clone();
                    {
                        let source_ml_source1_string_unit_operand122 = at;
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
                                    let source_ml_source1_string_unit_operand128 = {
                                        let source_ml_source1_string_range_operand123 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand124 = index;
                                            {
                                                let source_ml_source1_string_range_operand125 = index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position126 = source_ml_source1_string_range_operand124;
                                                    {
                                                        let source_ml_source1_string_range_position127 = source_ml_source1_string_range_operand125;
                                                        source_ml_source1_string_range_operand123.slice(source_ml_source1_string_range_position126, source_ml_source1_string_range_position127)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand129 = 0f64;
                                        source_ml_source1_string_unit_operand128.code_point_at(source_ml_source1_string_unit_operand129)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand121,
                            source_ml_source1_string_unit_operand122,
                        )
                    }
                };
                if (((code != 9f64) && (code != 10f64)) && (code != 13f64)) && (code != 32f64) {
                    at
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            at
        };
    }
}
