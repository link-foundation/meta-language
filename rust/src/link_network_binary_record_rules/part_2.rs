// Generated from link-network-binary-record-rules.js and authenticated shared primitives by the checked translator.
// Do not edit by hand.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons,
    clippy::float_cmp,
    clippy::suboptimal_flops,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
/// Execute the checked shared binary-network framing algorithm.
// Generated continuations retain the checked lexical state.
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn ml_transcode_link_network_binary_loop9(
    input: &[f64],
    mut binary: bool,
    mut record_limit: f64,
    mut byte_limit: f64,
    mut output: super::ml_string::Text,
    mut cursor: f64,
    mut identities: super::ml_string::Text,
    mut greatest_identity: f64,
    mut section: f64,
) -> super::MlTranscodeLinkNetworkBinaryLoop9Result {
    loop {
        return if section < 2f64 {
            {
                let entries = super::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if ((entries < 0f64) || (entries > record_limit))
                    || (entries > (input.len() as f64))
                {
                    super::MlTranscodeLinkNetworkBinaryLoop9Result::MlTranscodeLinkNetworkBinaryLoop9Return(((super::ml_string::Text::from_units(vec![45, 50, 48, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor)))).concat(&super::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = cursor + if binary { 8f64 } else { 1f64 };
                        {
                            let output_4 = if binary {
                                {
                                    let output_2 = output.concat(
                                        &super::render_link_network_binary_field(entries, binary),
                                    );
                                    output_2.clone()
                                }
                            } else {
                                {
                                    let index = 0f64;
                                    {
                                        let output_3 =
                                            super::ml_transcode_link_network_binary_loop10(
                                                binary,
                                                output.clone(),
                                                entries,
                                                index,
                                            );
                                        output_3.clone()
                                    }
                                }
                            };
                            {
                                let entry = 0f64;
                                {
                                    let ml_s14 = super::ml_transcode_link_network_binary_loop11(
                                        input,
                                        binary,
                                        byte_limit,
                                        output_4.clone(),
                                        cursor_2,
                                        identities.clone(),
                                        greatest_identity,
                                        section,
                                        entries,
                                        entry,
                                    );
                                    match ml_s14 {
                                        super::MlTranscodeLinkNetworkBinaryLoop11Result::MlTranscodeLinkNetworkBinaryLoop11Done(output_5, cursor_3) => {
                                            {
                                                let section_2 = section + 1f64;
                                                {
                                                    (output, cursor, section) = (output_5.clone(), cursor_3, section_2);
                                                    continue;
                                                }
                                            }
                                        }
                                        super::MlTranscodeLinkNetworkBinaryLoop11Result::MlTranscodeLinkNetworkBinaryLoop11Return(ml_result11) => {
                                            super::MlTranscodeLinkNetworkBinaryLoop9Result::MlTranscodeLinkNetworkBinaryLoop9Return(ml_result11)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            super::MlTranscodeLinkNetworkBinaryLoop9Result::MlTranscodeLinkNetworkBinaryLoop9Done(
                output, cursor,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop7Result {
    MlTranscodeLinkNetworkBinaryLoop7Done(super::ml_string::Text, f64, super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryLoop7Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop18(
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut identity: f64,
    mut index: f64,
) -> super::ml_string::Text {
    loop {
        return if (index < (8f64)) {
            {
                let output_2 = output.concat(&super::render_link_network_binary_field(
                    super::link_network_binary_integer_byte(identity, index),
                    binary,
                ));
                {
                    let index_2 = (index + 1f64);
                    {
                        (output, index) = (output_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            output
        };
    }
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop19(
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut references: f64,
    mut index: f64,
) -> super::ml_string::Text {
    loop {
        return if (index < (8f64)) {
            {
                let output_2 = output.concat(&super::render_link_network_binary_field(
                    super::link_network_binary_integer_byte(references, index),
                    binary,
                ));
                {
                    let index_2 = (index + 1f64);
                    {
                        (output, index) = (output_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            output
        };
    }
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop25(
    mut length: f64,
    mut encoded_length: super::ml_string::Text,
    mut index: f64,
) -> super::ml_string::Text {
    loop {
        return if (index < (8f64)) {
            {
                let encoded_length_2 =
                    encoded_length.concat(&super::render_link_network_binary_field(
                        super::link_network_binary_integer_byte(length, index),
                        false,
                    ));
                {
                    let index_2 = (index + 1f64);
                    {
                        (encoded_length, index) = (encoded_length_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            encoded_length
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin24 {
    MlTranscodeLinkNetworkBinaryJoin24Next(super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryJoin24Return(super::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop26Result {
    MlTranscodeLinkNetworkBinaryLoop26Done(super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryLoop26Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop27(
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut value: f64,
    mut index: f64,
) -> super::ml_string::Text {
    loop {
        return if (index < (8f64)) {
            {
                let output_2 = output.concat(&super::render_link_network_binary_field(
                    super::link_network_binary_integer_byte(value, index),
                    binary,
                ));
                {
                    let index_2 = (index + 1f64);
                    {
                        (output, index) = (output_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            output
        };
    }
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop26(
    input: &[f64],
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut cursor: f64,
    mut field: f64,
) -> super::MlTranscodeLinkNetworkBinaryLoop26Result {
    loop {
        return if (field < (6f64)) {
            {
                let value = super::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if (value < (0f64)) {
                    super::MlTranscodeLinkNetworkBinaryLoop26Result::MlTranscodeLinkNetworkBinaryLoop26Return(((super::ml_string::Text::from_units(vec![45, 49, 56, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor)))).concat(&super::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = (cursor + if binary { 8f64 } else { 1f64 });
                        if binary {
                            {
                                let output_2 = output.concat(
                                    &super::render_link_network_binary_field(value, binary),
                                );
                                {
                                    let field_2 = (field + 1f64);
                                    {
                                        (output, cursor, field) =
                                            (output_2.clone(), cursor_2, field_2);
                                        continue;
                                    }
                                }
                            }
                        }

                        {
                            let index = 0f64;
                            {
                                let output_3 = super::ml_transcode_link_network_binary_loop27(
                                    binary,
                                    output.clone(),
                                    value,
                                    index,
                                );
                                {
                                    let field_3 = (field + 1f64);
                                    {
                                        (output, cursor, field) =
                                            (output_3.clone(), cursor_2, field_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            super::MlTranscodeLinkNetworkBinaryLoop26Result::MlTranscodeLinkNetworkBinaryLoop26Done(
                output, cursor,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop23Result {
    MlTranscodeLinkNetworkBinaryLoop23Done(super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryLoop23Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop28(
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut length: f64,
    mut index: f64,
) -> super::ml_string::Text {
    loop {
        return if (index < (8f64)) {
            {
                let output_2 = output.concat(&super::render_link_network_binary_field(
                    super::link_network_binary_integer_byte(length, index),
                    binary,
                ));
                {
                    let index_2 = (index + 1f64);
                    {
                        (output, index) = (output_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            output
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop29Result {
    MlTranscodeLinkNetworkBinaryLoop29Done(super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryLoop29Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop29(
    input: &[f64],
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut cursor: f64,
    mut length: f64,
    mut index: f64,
) -> super::MlTranscodeLinkNetworkBinaryLoop29Result {
    loop {
        return if (index < length) {
            {
                let value = super::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if ((value < (0f64)) || (value > (255f64))) {
                    super::MlTranscodeLinkNetworkBinaryLoop29Result::MlTranscodeLinkNetworkBinaryLoop29Return(((super::ml_string::Text::from_units(vec![45, 49, 54, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor)))).concat(&super::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = (cursor + 1f64);
                        {
                            let output_2 = output
                                .concat(&super::render_link_network_binary_field(value, binary));
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (output, cursor, index) = (output_2.clone(), cursor_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            super::MlTranscodeLinkNetworkBinaryLoop29Result::MlTranscodeLinkNetworkBinaryLoop29Done(
                output, cursor,
            )
        };
    }
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop23(
    input: &[f64],
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut cursor: f64,
    mut text: f64,
) -> super::MlTranscodeLinkNetworkBinaryLoop23Result {
    loop {
        return if (text < (3f64)) {
            {
                let present = super::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if ((present != (0f64)) && (present != (1f64))) {
                    super::MlTranscodeLinkNetworkBinaryLoop23Result::MlTranscodeLinkNetworkBinaryLoop23Return(((super::ml_string::Text::from_units(vec![45, 49, 52, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor)))).concat(&super::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = (cursor + 1f64);
                        {
                            let output_2 = output
                                .concat(&super::render_link_network_binary_field(present, binary));
                            if (present == (1f64)) {
                                {
                                    let length = super::read_link_network_binary_integer(
                                        input, cursor_2, 8f64, binary,
                                    );
                                    if ((length < (0f64))
                                        || (length
                                            > (((input.len() as f64) - cursor_2)
                                                - if binary { 8f64 } else { 1f64 })))
                                    {
                                        super::MlTranscodeLinkNetworkBinaryLoop23Result::MlTranscodeLinkNetworkBinaryLoop23Return(((super::ml_string::Text::from_units(vec![45, 49, 53, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_2)))).concat(&super::ml_string::Text::from_units(vec![44])))
                                    } else {
                                        {
                                            let cursor_3 =
                                                (cursor_2 + if binary { 8f64 } else { 1f64 });
                                            {
                                                let output_5 = if binary {
                                                    {
                                                        let output_3 = output_2.concat(&super::render_link_network_binary_field(length, binary));
                                                        output_3.clone()
                                                    }
                                                } else {
                                                    {
                                                        let index = 0f64;
                                                        {
                                                            let output_4 = super::ml_transcode_link_network_binary_loop28(binary, output_2.clone(), length, index);
                                                            output_4.clone()
                                                        }
                                                    }
                                                };
                                                {
                                                    let index_2 = 0f64;
                                                    {
                                                        let ml_s15 = super::ml_transcode_link_network_binary_loop29(input, binary, output_5.clone(), cursor_3, length, index_2);
                                                        match ml_s15 {
                                                            super::MlTranscodeLinkNetworkBinaryLoop29Result::MlTranscodeLinkNetworkBinaryLoop29Done(output_6, cursor_4) => {
                                                                {
                                                                    let text_2 = (text + 1f64);
                                                                    {
                                                                        (output, cursor, text) = (output_6.clone(), cursor_4, text_2);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            super::MlTranscodeLinkNetworkBinaryLoop29Result::MlTranscodeLinkNetworkBinaryLoop29Return(ml_result29) => {
                                                                super::MlTranscodeLinkNetworkBinaryLoop23Result::MlTranscodeLinkNetworkBinaryLoop23Return(ml_result29)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let text_3 = (text + 1f64);
                                    {
                                        (output, cursor, text) =
                                            (output_2.clone(), cursor_2, text_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            super::MlTranscodeLinkNetworkBinaryLoop23Result::MlTranscodeLinkNetworkBinaryLoop23Done(
                output, cursor,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop22Result {
    MlTranscodeLinkNetworkBinaryLoop22Done(super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryLoop22Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop22(
    input: &[f64],
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut cursor: f64,
    mut field: f64,
) -> super::MlTranscodeLinkNetworkBinaryLoop22Result {
    loop {
        return if (field < (6f64)) {
            {
                let value = super::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if ((value < (0f64)) || (value > (255f64))) {
                    super::MlTranscodeLinkNetworkBinaryLoop22Result::MlTranscodeLinkNetworkBinaryLoop22Return(((super::ml_string::Text::from_units(vec![45, 49, 50, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor)))).concat(&super::ml_string::Text::from_units(vec![44])))
                } else if ((((((field == (0f64)) && (value != (1f64)))
                    || ((field == (1f64)) && (value > (15f64))))
                    || (((field == (2f64)) || (field == (4f64))) && (value > (1f64))))
                    || ((field == (5f64)) && (value != (0f64))))
                    || ((field == (3f64))
                        && (((value > (15f64))
                            || (((value % 2f64) == (1f64))
                                && (((value / 2f64).floor() % 2f64) != (1f64))))
                            || ((((value / 4f64).floor() % 2f64) == (1f64))
                                && (((value / 2f64).floor() % 2f64) != (1f64))))))
                {
                    super::MlTranscodeLinkNetworkBinaryLoop22Result::MlTranscodeLinkNetworkBinaryLoop22Return(((super::ml_string::Text::from_units(vec![45, 49, 51, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor)))).concat(&super::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = (cursor + 1f64);
                        {
                            let output_2 = output
                                .concat(&super::render_link_network_binary_field(value, binary));
                            {
                                let field_2 = (field + 1f64);
                                {
                                    (output, cursor, field) = (output_2.clone(), cursor_2, field_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            super::MlTranscodeLinkNetworkBinaryLoop22Result::MlTranscodeLinkNetworkBinaryLoop22Done(
                output, cursor,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin21 {
    MlTranscodeLinkNetworkBinaryJoin21Next(super::ml_string::Text, f64, f64),
    MlTranscodeLinkNetworkBinaryJoin21Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop30(
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut index: f64,
) -> super::ml_string::Text {
    loop {
        return if (index < (8f64)) {
            {
                let output_2 =
                    output.concat(&super::render_link_network_binary_field(0f64, binary));
                {
                    let index_2 = (index + 1f64);
                    {
                        (output, index) = (output_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            output
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop20Result {
    MlTranscodeLinkNetworkBinaryLoop20Done(super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryLoop20Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop31(
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut target: f64,
    mut index: f64,
) -> super::ml_string::Text {
    loop {
        return if (index < (8f64)) {
            {
                let output_2 = output.concat(&super::render_link_network_binary_field(
                    super::link_network_binary_integer_byte(target, index),
                    binary,
                ));
                {
                    let index_2 = (index + 1f64);
                    {
                        (output, index) = (output_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            output
        };
    }
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop20(
    input: &[f64],
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut cursor: f64,
    mut references: f64,
    mut reference: f64,
) -> super::MlTranscodeLinkNetworkBinaryLoop20Result {
    loop {
        return if (reference < references) {
            {
                let target = super::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if (target < (1f64)) {
                    super::MlTranscodeLinkNetworkBinaryLoop20Result::MlTranscodeLinkNetworkBinaryLoop20Return(((super::ml_string::Text::from_units(vec![45, 49, 48, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor)))).concat(&super::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = (cursor + if binary { 8f64 } else { 1f64 });
                        if binary {
                            {
                                let output_2 = output.concat(
                                    &super::render_link_network_binary_field(target, binary),
                                );
                                {
                                    let reference_2 = (reference + 1f64);
                                    {
                                        (output, cursor, reference) =
                                            (output_2.clone(), cursor_2, reference_2);
                                        continue;
                                    }
                                }
                            }
                        }

                        {
                            let index = 0f64;
                            {
                                let output_3 = super::ml_transcode_link_network_binary_loop31(
                                    binary,
                                    output.clone(),
                                    target,
                                    index,
                                );
                                {
                                    let reference_3 = (reference + 1f64);
                                    {
                                        (output, cursor, reference) =
                                            (output_3.clone(), cursor_2, reference_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            super::MlTranscodeLinkNetworkBinaryLoop20Result::MlTranscodeLinkNetworkBinaryLoop20Done(
                output, cursor,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin17 {
    MlTranscodeLinkNetworkBinaryJoin17Next(f64),
    MlTranscodeLinkNetworkBinaryJoin17Return(super::ml_string::Text),
}
