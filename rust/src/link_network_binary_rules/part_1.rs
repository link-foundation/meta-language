// Generated from link-network-binary-rules.js by the checked translator.
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
#[must_use]
// Generated continuations retain the checked lexical state.
#[allow(clippy::too_many_lines)]
pub fn transcode_link_network_binary(
    input: &[f64],
    binary: bool,
    record_limit: f64,
    byte_limit: f64,
) -> super::ml_string::Text {
    {
        let output = super::ml_string::Text::from_units(vec![]);
        if (((!(super::accept_link_network_binary_integer(record_limit))
            || (record_limit < (1f64)))
            || !(super::accept_link_network_binary_integer(byte_limit)))
            || (byte_limit < (16f64)))
        {
            ((super::ml_string::Text::from_units(vec![45, 49, 44])).concat(
                &super::ml_string::Text::from_text(&super::ml_number::js_number(0f64)),
            ))
            .concat(&super::ml_string::Text::from_units(vec![44]))
        } else if ((input.len() as f64) > byte_limit) {
            ((super::ml_string::Text::from_units(vec![45, 50, 44])).concat(
                &super::ml_string::Text::from_text(&super::ml_number::js_number(0f64)),
            ))
            .concat(&super::ml_string::Text::from_units(vec![44]))
        } else {
            {
                let cursor = 0f64;
                {
                    let version = 0f64;
                    {
                        let header = vec![77f64, 76f64, 68f64, 83f64, 78f64, 80f64, 48f64];
                        {
                            let ml_s4 = if binary {
                                {
                                    let index = 0f64;
                                    {
                                        let ml_s5 = super::ml_transcode_link_network_binary_loop32(
                                            input, &header, index,
                                        );
                                        match ml_s5 {
                                                super::MlTranscodeLinkNetworkBinaryLoop32Result::MlTranscodeLinkNetworkBinaryLoop32Done => {
                                                    {
                                                        let version_2 = (super::link_network_binary_unit(input, 7f64) - 48f64);
                                                        {
                                                            let cursor_2 = 8f64;
                                                            super::MlTranscodeLinkNetworkBinaryJoin4::MlTranscodeLinkNetworkBinaryJoin4Next(output, cursor_2, version_2)
                                                        }
                                                    }
                                                }
                                                super::MlTranscodeLinkNetworkBinaryLoop32Result::MlTranscodeLinkNetworkBinaryLoop32Return(ml_result32) => {
                                                    super::MlTranscodeLinkNetworkBinaryJoin4::MlTranscodeLinkNetworkBinaryJoin4Return(ml_result32)
                                                }
                                            }
                                    }
                                }
                            } else {
                                {
                                    let version_3 = super::link_network_binary_unit(input, cursor);
                                    {
                                        let cursor_3 = (cursor + 1f64);
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let output_2 =
                                                    super::ml_transcode_link_network_binary_loop33(
                                                        binary, output, &header, index_2,
                                                    );
                                                {
                                                    let output_3 = output_2.concat(
                                                        &super::render_link_network_binary_field(
                                                            (48f64 + version_3),
                                                            binary,
                                                        ),
                                                    );
                                                    super::MlTranscodeLinkNetworkBinaryJoin4::MlTranscodeLinkNetworkBinaryJoin4Next(output_3, cursor_3, version_3)
                                                }
                                            }
                                        }
                                    }
                                }
                            };
                            match ml_s4 {
                                    super::MlTranscodeLinkNetworkBinaryJoin4::MlTranscodeLinkNetworkBinaryJoin4Next(output_4, cursor_4, version_4) => {
                                        if ((version_4 != (1f64)) && (version_4 != (2f64))) {
                                            ((super::ml_string::Text::from_units(vec![45, 52, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(7f64)))).concat(&super::ml_string::Text::from_units(vec![44]))
                                        } else {
                                            {
                                                let output_6 = if binary {
                                                    {
                                                        output_4.concat(&super::render_link_network_binary_field(version_4, binary))
                                                    }
                                                } else {
                                                    output_4
                                                };
                                                {
                                                    let next_identity = 0f64;
                                                    {
                                                        let ml_s6 = if (version_4 == (2f64)) {
                                                            {
                                                                let next_identity_2 = super::read_link_network_binary_integer(input, cursor_4, 8f64, binary);
                                                                if (next_identity_2 < (1f64)) {
                                                                    super::MlTranscodeLinkNetworkBinaryJoin5::MlTranscodeLinkNetworkBinaryJoin5Return(((super::ml_string::Text::from_units(vec![45, 53, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_4)))).concat(&super::ml_string::Text::from_units(vec![44])))
                                                                } else {
                                                                    {
                                                                        let cursor_5 = (cursor_4 + if binary {
                                                                            8f64
                                                                        } else {
                                                                            1f64
                                                                        });
                                                                        if binary {
                                                                            {
                                                                                let output_7 = output_6.concat(&super::render_link_network_binary_field(next_identity_2, binary));
                                                                                super::MlTranscodeLinkNetworkBinaryJoin5::MlTranscodeLinkNetworkBinaryJoin5Next(output_7, cursor_5, next_identity_2)
                                                                            }
                                                                        } else {
                                                                            {
                                                                                let index_3 = 0f64;
                                                                                {
                                                                                    let output_8 = super::ml_transcode_link_network_binary_loop31(binary, output_6, next_identity_2, index_3);
                                                                                    super::MlTranscodeLinkNetworkBinaryJoin5::MlTranscodeLinkNetworkBinaryJoin5Next(output_8, cursor_5, next_identity_2)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            super::MlTranscodeLinkNetworkBinaryJoin5::MlTranscodeLinkNetworkBinaryJoin5Next(output_6, cursor_4, next_identity)
                                                        };
                                                        match ml_s6 {
                                                            super::MlTranscodeLinkNetworkBinaryJoin5::MlTranscodeLinkNetworkBinaryJoin5Next(output_9, cursor_6, next_identity_3) => {
                                                                {
                                                                    let count = super::read_link_network_binary_integer(input, cursor_6, 8f64, binary);
                                                                    if (((count < (0f64)) || (count > record_limit)) || (count > (input.len() as f64))) {
                                                                        ((super::ml_string::Text::from_units(vec![45, 54, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_6)))).concat(&super::ml_string::Text::from_units(vec![44]))
                                                                    } else {
                                                                        {
                                                                            let cursor_7 = (cursor_6 + if binary {
                                                                                8f64
                                                                            } else {
                                                                                1f64
                                                                            });
                                                                            {
                                                                                let output_12 = if binary {
                                                                                    {
                                                                                        output_9.concat(&super::render_link_network_binary_field(count, binary))
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        let index_4 = 0f64;
                                                                                        {
                                                                                            super::ml_transcode_link_network_binary_loop6(binary, output_9, count, index_4)
                                                                                        }
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let identities = super::ml_string::Text::from_units(vec![44]);
                                                                                    {
                                                                                        let record = 0f64;
                                                                                        {
                                                                                            let ml_s7 = super::ml_transcode_link_network_binary_loop7(input, binary, byte_limit, output_12, cursor_7, version_4, next_identity_3, count, identities, record);
                                                                                            match ml_s7 {
                                                                                                super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Done(output_13, cursor_8, identities_2) => {
                                                                                                    {
                                                                                                        let ml_s9 = if (version_4 == (2f64)) {
                                                                                                            {
                                                                                                                let section = 0f64;
                                                                                                                {
                                                                                                                    let ml_s10 = super::ml_transcode_link_network_binary_loop9(input, binary, record_limit, byte_limit, output_13, cursor_8, identities_2, section);
                                                                                                                    match ml_s10 {
                                                                                                                        super::MlTranscodeLinkNetworkBinaryLoop9Result::MlTranscodeLinkNetworkBinaryLoop9Done(completed_output, cursor_9) => {
                                                                                                                            super::MlTranscodeLinkNetworkBinaryJoin8::MlTranscodeLinkNetworkBinaryJoin8Next(completed_output, cursor_9)
                                                                                                                        }
                                                                                                                        super::MlTranscodeLinkNetworkBinaryLoop9Result::MlTranscodeLinkNetworkBinaryLoop9Return(ml_result9) => {
                                                                                                                            super::MlTranscodeLinkNetworkBinaryJoin8::MlTranscodeLinkNetworkBinaryJoin8Return(ml_result9)
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        } else {
                                                                                                            super::MlTranscodeLinkNetworkBinaryJoin8::MlTranscodeLinkNetworkBinaryJoin8Next(output_13, cursor_8)
                                                                                                        };
                                                                                                        match ml_s9 {
                                                                                                            super::MlTranscodeLinkNetworkBinaryJoin8::MlTranscodeLinkNetworkBinaryJoin8Next(output_15, cursor_10) => {
                                                                                                                if (cursor_10 != (input.len() as f64)) {
                                                                                                                    ((super::ml_string::Text::from_units(vec![45, 50, 50, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_10)))).concat(&super::ml_string::Text::from_units(vec![44]))
                                                                                                                } else if ((output_15.length()) > (byte_limit * if binary {
                                                                                                                        20f64
                                                                                                                    } else {
                                                                                                                        2f64
                                                                                                                    })) {
                                                                                                                        ((super::ml_string::Text::from_units(vec![45, 50, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_10)))).concat(&super::ml_string::Text::from_units(vec![44]))
                                                                                                                    } else {
                                                                                                                        output_15
                                                                                                                    }
                                                                                                            }
                                                                                                            super::MlTranscodeLinkNetworkBinaryJoin8::MlTranscodeLinkNetworkBinaryJoin8Return(ml_result8) => {
                                                                                                                ml_result8
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Return(ml_result16) => {
                                                                                                    ml_result16
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
                                                            super::MlTranscodeLinkNetworkBinaryJoin5::MlTranscodeLinkNetworkBinaryJoin5Return(ml_result30) => {
                                                                ml_result30
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    super::MlTranscodeLinkNetworkBinaryJoin4::MlTranscodeLinkNetworkBinaryJoin4Return(ml_result31) => {
                                        ml_result31
                                    }
                                }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadLinkNetworkBinaryIntegerLoop1Result {
    MlReadLinkNetworkBinaryIntegerLoop1Done(f64),
    MlReadLinkNetworkBinaryIntegerLoop1Return(f64),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_read_link_network_binary_integer_loop1(
    input: &[f64],
    mut cursor: f64,
    mut width: f64,
    mut value: f64,
    mut factor: f64,
    mut index: f64,
) -> super::MlReadLinkNetworkBinaryIntegerLoop1Result {
    loop {
        return if (index < width) {
            {
                let byte = super::link_network_binary_unit(input, (cursor + index));
                if (!(super::accept_link_network_binary_integer(byte)) || (byte > (255f64))) {
                    super::MlReadLinkNetworkBinaryIntegerLoop1Result::MlReadLinkNetworkBinaryIntegerLoop1Return(-1f64)
                } else {
                    {
                        let value_2 = (value + (byte * factor));
                        if (super::accept_link_network_binary_integer(value_2)) {
                            {
                                let factor_2 = (factor * 256f64);
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        (value, factor, index) = (value_2, factor_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }

                        super::MlReadLinkNetworkBinaryIntegerLoop1Result::MlReadLinkNetworkBinaryIntegerLoop1Return(-1f64)
                    }
                }
            }
        } else {
            super::MlReadLinkNetworkBinaryIntegerLoop1Result::MlReadLinkNetworkBinaryIntegerLoop1Done(value)
        };
    }
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_link_network_binary_integer_byte_loop2(
    mut value: f64,
    mut index: f64,
    mut at: f64,
) -> f64 {
    loop {
        return if (at < index) {
            {
                let value_2 = (value / 256f64).floor();
                {
                    let at_2 = (at + 1f64);
                    {
                        (value, at) = (value_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            value
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptLinkNetworkBinaryUnicodeLoop3Result {
    MlAcceptLinkNetworkBinaryUnicodeLoop3Done,
    MlAcceptLinkNetworkBinaryUnicodeLoop3Return(bool),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_accept_link_network_binary_unicode_loop3(
    units: &[f64],
    mut index: f64,
) -> super::MlAcceptLinkNetworkBinaryUnicodeLoop3Result {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let unit = super::link_network_binary_unit(units, index);
                if (!(super::accept_link_network_binary_integer(unit)) || (unit > (65535f64))) {
                    super::MlAcceptLinkNetworkBinaryUnicodeLoop3Result::MlAcceptLinkNetworkBinaryUnicodeLoop3Return(false)
                } else if (55296f64..=56319f64).contains(&unit) {
                    {
                        let index_2 = (index + 1f64);
                        {
                            let low = super::link_network_binary_unit(units, index_2);
                            if ((low < (56320f64)) || (low > (57343f64))) {
                                super::MlAcceptLinkNetworkBinaryUnicodeLoop3Result::MlAcceptLinkNetworkBinaryUnicodeLoop3Return(false)
                            } else {
                                {
                                    let index_3 = (index_2 + 1f64);
                                    {
                                        index = index_3;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else if (56320f64..=57343f64).contains(&unit) {
                    super::MlAcceptLinkNetworkBinaryUnicodeLoop3Result::MlAcceptLinkNetworkBinaryUnicodeLoop3Return(false)
                } else {
                    {
                        let index_4 = (index + 1f64);
                        {
                            index = index_4;
                            continue;
                        }
                    }
                }
            }
        } else {
            super::MlAcceptLinkNetworkBinaryUnicodeLoop3Result::MlAcceptLinkNetworkBinaryUnicodeLoop3Done
        };
    }
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop6(
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut count: f64,
    mut index: f64,
) -> super::ml_string::Text {
    loop {
        return if (index < (8f64)) {
            {
                let output_2 = output.concat(&super::render_link_network_binary_field(
                    super::link_network_binary_integer_byte(count, index),
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
pub enum MlTranscodeLinkNetworkBinaryJoin8 {
    MlTranscodeLinkNetworkBinaryJoin8Next(super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryJoin8Return(super::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop9Result {
    MlTranscodeLinkNetworkBinaryLoop9Done(super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryLoop9Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop10(
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut entries: f64,
    mut index: f64,
) -> super::ml_string::Text {
    loop {
        return if (index < (8f64)) {
            {
                let output_2 = output.concat(&super::render_link_network_binary_field(
                    super::link_network_binary_integer_byte(entries, index),
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
pub enum MlTranscodeLinkNetworkBinaryLoop11Result {
    MlTranscodeLinkNetworkBinaryLoop11Done(super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryLoop11Return(super::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin13 {
    MlTranscodeLinkNetworkBinaryJoin13Next(super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryJoin13Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop14(
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop12Result {
    MlTranscodeLinkNetworkBinaryLoop12Done(super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryLoop12Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop15(
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
pub enum MlTranscodeLinkNetworkBinaryLoop16Result {
    MlTranscodeLinkNetworkBinaryLoop16Done(super::ml_string::Text, f64),
    MlTranscodeLinkNetworkBinaryLoop16Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop16(
    input: &[f64],
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut cursor: f64,
    mut length: f64,
    mut index: f64,
) -> super::MlTranscodeLinkNetworkBinaryLoop16Result {
    loop {
        return if (index < length) {
            {
                let value = super::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if ((value < (0f64)) || (value > (255f64))) {
                    super::MlTranscodeLinkNetworkBinaryLoop16Result::MlTranscodeLinkNetworkBinaryLoop16Return(((super::ml_string::Text::from_units(vec![45, 49, 54, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor)))).concat(&super::ml_string::Text::from_units(vec![44])))
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
            super::MlTranscodeLinkNetworkBinaryLoop16Result::MlTranscodeLinkNetworkBinaryLoop16Done(
                output, cursor,
            )
        };
    }
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop12(
    input: &[f64],
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut cursor: f64,
    mut strings: f64,
    mut text: f64,
) -> super::MlTranscodeLinkNetworkBinaryLoop12Result {
    loop {
        return if (text < strings) {
            {
                let length = super::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if ((length < (0f64))
                    || (length
                        > (((input.len() as f64) - cursor) - if binary { 8f64 } else { 1f64 })))
                {
                    super::MlTranscodeLinkNetworkBinaryLoop12Result::MlTranscodeLinkNetworkBinaryLoop12Return(((super::ml_string::Text::from_units(vec![45, 49, 53, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor)))).concat(&super::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = (cursor + if binary { 8f64 } else { 1f64 });
                        {
                            let output_4 = if binary {
                                {
                                    let output_2 = output.concat(
                                        &super::render_link_network_binary_field(length, binary),
                                    );
                                    output_2.clone()
                                }
                            } else {
                                {
                                    let index = 0f64;
                                    {
                                        let output_3 =
                                            super::ml_transcode_link_network_binary_loop15(
                                                binary,
                                                output.clone(),
                                                length,
                                                index,
                                            );
                                        output_3.clone()
                                    }
                                }
                            };
                            {
                                let index_2 = 0f64;
                                {
                                    let ml_s11 = super::ml_transcode_link_network_binary_loop16(
                                        input,
                                        binary,
                                        output_4.clone(),
                                        cursor_2,
                                        length,
                                        index_2,
                                    );
                                    match ml_s11 {
                                        super::MlTranscodeLinkNetworkBinaryLoop16Result::MlTranscodeLinkNetworkBinaryLoop16Done(output_5, cursor_3) => {
                                            {
                                                let text_2 = (text + 1f64);
                                                {
                                                    (output, cursor, text) = (output_5.clone(), cursor_3, text_2);
                                                    continue;
                                                }
                                            }
                                        }
                                        super::MlTranscodeLinkNetworkBinaryLoop16Result::MlTranscodeLinkNetworkBinaryLoop16Return(ml_result16) => {
                                            super::MlTranscodeLinkNetworkBinaryLoop12Result::MlTranscodeLinkNetworkBinaryLoop12Return(ml_result16)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            super::MlTranscodeLinkNetworkBinaryLoop12Result::MlTranscodeLinkNetworkBinaryLoop12Done(
                output, cursor,
            )
        };
    }
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
// Generated continuations retain the checked lexical state.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn ml_transcode_link_network_binary_loop11(
    input: &[f64],
    mut binary: bool,
    mut byte_limit: f64,
    mut output: super::ml_string::Text,
    mut cursor: f64,
    mut identities: super::ml_string::Text,
    mut section: f64,
    mut entries: f64,
    mut entry: f64,
) -> super::MlTranscodeLinkNetworkBinaryLoop11Result {
    loop {
        return if (entry < entries) {
            {
                let strings = if (section == (0f64)) { 1f64 } else { 3f64 };
                {
                    let text = 0f64;
                    {
                        let ml_s12 = super::ml_transcode_link_network_binary_loop12(
                            input,
                            binary,
                            output.clone(),
                            cursor,
                            strings,
                            text,
                        );
                        match ml_s12 {
                            super::MlTranscodeLinkNetworkBinaryLoop12Result::MlTranscodeLinkNetworkBinaryLoop12Done(output_2, cursor_2) => {
                                {
                                    let ml_s13 = if (section == (0f64)) {
                                        {
                                            let identity = super::read_link_network_binary_integer(input, cursor_2, 8f64, binary);
                                            if (identity < (1f64)) {
                                                super::MlTranscodeLinkNetworkBinaryJoin13::MlTranscodeLinkNetworkBinaryJoin13Return(((super::ml_string::Text::from_units(vec![45, 50, 49, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_2)))).concat(&super::ml_string::Text::from_units(vec![44])))
                                            } else {
                                                {
                                                    let found = identities.contains(&((super::ml_string::Text::from_units(vec![44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(identity)))).concat(&super::ml_string::Text::from_units(vec![44])));
                                                    if found {
                                                        {
                                                            let cursor_3 = (cursor_2 + if binary {
                                                                8f64
                                                            } else {
                                                                1f64
                                                            });
                                                            if binary {
                                                                {
                                                                    let output_3 = output_2.concat(&super::render_link_network_binary_field(identity, binary));
                                                                    super::MlTranscodeLinkNetworkBinaryJoin13::MlTranscodeLinkNetworkBinaryJoin13Next(output_3.clone(), cursor_3)
                                                                }
                                                            } else {
                                                                {
                                                                    let index = 0f64;
                                                                    {
                                                                        let output_4 = super::ml_transcode_link_network_binary_loop14(binary, output_2.clone(), identity, index);
                                                                        super::MlTranscodeLinkNetworkBinaryJoin13::MlTranscodeLinkNetworkBinaryJoin13Next(output_4.clone(), cursor_3)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        super::MlTranscodeLinkNetworkBinaryJoin13::MlTranscodeLinkNetworkBinaryJoin13Return(((super::ml_string::Text::from_units(vec![45, 50, 49, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_2)))).concat(&super::ml_string::Text::from_units(vec![44])))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        super::MlTranscodeLinkNetworkBinaryJoin13::MlTranscodeLinkNetworkBinaryJoin13Next(output_2.clone(), cursor_2)
                                    };
                                    match ml_s13 {
                                        super::MlTranscodeLinkNetworkBinaryJoin13::MlTranscodeLinkNetworkBinaryJoin13Next(output_5, cursor_4) => {
                                            if ((output_5.length()) > (byte_limit * if binary {
                                                20f64
                                            } else {
                                                2f64
                                            })) {
                                                super::MlTranscodeLinkNetworkBinaryLoop11Result::MlTranscodeLinkNetworkBinaryLoop11Return(((super::ml_string::Text::from_units(vec![45, 50, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_4)))).concat(&super::ml_string::Text::from_units(vec![44])))
                                            } else {
                                                {
                                                    let entry_2 = (entry + 1f64);
                                                    {
                                                        (output, cursor, entry) = (output_5.clone(), cursor_4, entry_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        super::MlTranscodeLinkNetworkBinaryJoin13::MlTranscodeLinkNetworkBinaryJoin13Return(ml_result13) => {
                                            super::MlTranscodeLinkNetworkBinaryLoop11Result::MlTranscodeLinkNetworkBinaryLoop11Return(ml_result13)
                                        }
                                    }
                                }
                            }
                            super::MlTranscodeLinkNetworkBinaryLoop12Result::MlTranscodeLinkNetworkBinaryLoop12Return(ml_result14) => {
                                super::MlTranscodeLinkNetworkBinaryLoop11Result::MlTranscodeLinkNetworkBinaryLoop11Return(ml_result14)
                            }
                        }
                    }
                }
            }
        } else {
            super::MlTranscodeLinkNetworkBinaryLoop11Result::MlTranscodeLinkNetworkBinaryLoop11Done(
                output, cursor,
            )
        };
    }
}
