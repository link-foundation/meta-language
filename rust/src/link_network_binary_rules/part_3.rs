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
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn ml_transcode_link_network_binary_loop7(
    input: &[f64],
    mut binary: bool,
    mut byte_limit: f64,
    mut output: super::ml_string::Text,
    mut cursor: f64,
    mut version: f64,
    mut next_identity: f64,
    mut count: f64,
    mut identities: super::ml_string::Text,
    mut record: f64,
) -> super::MlTranscodeLinkNetworkBinaryLoop7Result {
    loop {
        return if (record < count) {
            {
                let identity = super::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if (((identity < (1f64)) || (identity >= (9_007_199_254_740_991f64)))
                    || ((version == (2f64)) && (identity >= next_identity)))
                {
                    super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Return(((super::ml_string::Text::from_units(vec![45, 55, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor)))).concat(&super::ml_string::Text::from_units(vec![44])))
                } else {
                    if identities.contains(
                        &((super::ml_string::Text::from_units(vec![44])).concat(
                            &super::ml_string::Text::from_text(&super::ml_number::js_number(
                                identity,
                            )),
                        ))
                        .concat(&super::ml_string::Text::from_units(vec![44])),
                    ) {
                        super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Return(((super::ml_string::Text::from_units(vec![45, 56, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor)))).concat(&super::ml_string::Text::from_units(vec![44])))
                    } else {
                        {
                            let identities_2 = identities.concat(
                                &(super::ml_string::Text::from_text(&super::ml_number::js_number(
                                    identity,
                                )))
                                .concat(&super::ml_string::Text::from_units(vec![44])),
                            );
                            {
                                let cursor_2 = (cursor + if binary { 8f64 } else { 1f64 });
                                {
                                    let output_4 = if binary {
                                        {
                                            let output_2 = output.concat(
                                                &super::render_link_network_binary_field(
                                                    identity, binary,
                                                ),
                                            );
                                            output_2.clone()
                                        }
                                    } else {
                                        {
                                            let index = 0f64;
                                            {
                                                let output_3 =
                                                    super::ml_transcode_link_network_binary_loop17(
                                                        binary,
                                                        output.clone(),
                                                        identity,
                                                        index,
                                                    );
                                                output_3.clone()
                                            }
                                        }
                                    };
                                    {
                                        let references = super::read_link_network_binary_integer(
                                            input, cursor_2, 8f64, binary,
                                        );
                                        if ((references < (0f64))
                                            || (references > (input.len() as f64)))
                                        {
                                            super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Return(((super::ml_string::Text::from_units(vec![45, 57, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_2)))).concat(&super::ml_string::Text::from_units(vec![44])))
                                        } else {
                                            {
                                                let cursor_3 =
                                                    (cursor_2 + if binary { 8f64 } else { 1f64 });
                                                {
                                                    let output_7 = if binary {
                                                        {
                                                            let output_5 = output_4.concat(&super::render_link_network_binary_field(references, binary));
                                                            output_5.clone()
                                                        }
                                                    } else {
                                                        {
                                                            let index_2 = 0f64;
                                                            {
                                                                let output_6 = super::ml_transcode_link_network_binary_loop18(binary, output_4.clone(), references, index_2);
                                                                output_6.clone()
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let reference = 0f64;
                                                        {
                                                            let ml_s16 = super::ml_transcode_link_network_binary_loop19(input, binary, output_7.clone(), cursor_3, references, reference);
                                                            match ml_s16.clone() {
                                                                super::MlTranscodeLinkNetworkBinaryLoop19Result::MlTranscodeLinkNetworkBinaryLoop19Done(output_8, cursor_4) => {
                                                                    {
                                                                        let metadata_end = 0f64;
                                                                        {
                                                                            let length_slot = output_8.length();
                                                                            {
                                                                                let ml_s17 = if binary {
                                                                                    {
                                                                                        let length = super::read_link_network_binary_integer(input, cursor_4, 8f64, true);
                                                                                        if ((length < (6f64)) || (length > ((((input.len() as f64) - cursor_4) - 8f64)))) {
                                                                                            super::MlTranscodeLinkNetworkBinaryJoin20::MlTranscodeLinkNetworkBinaryJoin20Return(((super::ml_string::Text::from_units(vec![45, 49, 49, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_4)))).concat(&super::ml_string::Text::from_units(vec![44])))
                                                                                        } else {
                                                                                            {
                                                                                                let cursor_5 = (cursor_4 + 8f64);
                                                                                                {
                                                                                                    let metadata_end_2 = (cursor_5 + length);
                                                                                                    super::MlTranscodeLinkNetworkBinaryJoin20::MlTranscodeLinkNetworkBinaryJoin20Next(output_8.clone(), cursor_5, metadata_end_2)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        let index_3 = 0f64;
                                                                                        {
                                                                                            let output_9 = super::ml_transcode_link_network_binary_loop29(binary, output_8.clone(), index_3);
                                                                                            super::MlTranscodeLinkNetworkBinaryJoin20::MlTranscodeLinkNetworkBinaryJoin20Next(output_9.clone(), cursor_4, metadata_end)
                                                                                        }
                                                                                    }
                                                                                };
                                                                                match ml_s17.clone() {
                                                                                    super::MlTranscodeLinkNetworkBinaryJoin20::MlTranscodeLinkNetworkBinaryJoin20Next(output_10, cursor_6, metadata_end_3) => {
                                                                                        {
                                                                                            let metadata_start = output_10.length();
                                                                                            {
                                                                                                let field = 0f64;
                                                                                                {
                                                                                                    let ml_s18 = super::ml_transcode_link_network_binary_loop21(input, binary, output_10.clone(), cursor_6, field);
                                                                                                    match ml_s18.clone() {
                                                                                                        super::MlTranscodeLinkNetworkBinaryLoop21Result::MlTranscodeLinkNetworkBinaryLoop21Done(output_11, cursor_7) => {
                                                                                                            {
                                                                                                                let text = 0f64;
                                                                                                                {
                                                                                                                    let ml_s19 = super::ml_transcode_link_network_binary_loop22(input, binary, output_11.clone(), cursor_7, text);
                                                                                                                    match ml_s19.clone() {
                                                                                                                        super::MlTranscodeLinkNetworkBinaryLoop22Result::MlTranscodeLinkNetworkBinaryLoop22Done(output_12, cursor_8) => {
                                                                                                                            {
                                                                                                                                let span = super::read_link_network_binary_integer(input, cursor_8, 1f64, binary);
                                                                                                                                if ((span != (0f64)) && (span != (1f64))) {
                                                                                                                                    super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Return(((super::ml_string::Text::from_units(vec![45, 49, 55, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_8)))).concat(&super::ml_string::Text::from_units(vec![44])))
                                                                                                                                } else {
                                                                                                                                    {
                                                                                                                                        let cursor_9 = (cursor_8 + 1f64);
                                                                                                                                        {
                                                                                                                                            let output_13 = output_12.concat(&super::render_link_network_binary_field(span, binary));
                                                                                                                                            {
                                                                                                                                                let ml_s21 = if (span == (1f64)) {
                                                                                                                                                    {
                                                                                                                                                        let field_2 = 0f64;
                                                                                                                                                        {
                                                                                                                                                            let ml_s22 = super::ml_transcode_link_network_binary_loop25(input, binary, output_13.clone(), cursor_9, field_2);
                                                                                                                                                            match ml_s22.clone() {
                                                                                                                                                                super::MlTranscodeLinkNetworkBinaryLoop25Result::MlTranscodeLinkNetworkBinaryLoop25Done(output_14, cursor_10) => {
                                                                                                                                                                    super::MlTranscodeLinkNetworkBinaryJoin23::MlTranscodeLinkNetworkBinaryJoin23Next(output_14.clone(), cursor_10)
                                                                                                                                                                }
                                                                                                                                                                super::MlTranscodeLinkNetworkBinaryLoop25Result::MlTranscodeLinkNetworkBinaryLoop25Return(ml_result25) => {
                                                                                                                                                                    super::MlTranscodeLinkNetworkBinaryJoin23::MlTranscodeLinkNetworkBinaryJoin23Return(ml_result25.clone())
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                } else {
                                                                                                                                                    super::MlTranscodeLinkNetworkBinaryJoin23::MlTranscodeLinkNetworkBinaryJoin23Next(output_13.clone(), cursor_9)
                                                                                                                                                };
                                                                                                                                                match ml_s21.clone() {
                                                                                                                                                    super::MlTranscodeLinkNetworkBinaryJoin23::MlTranscodeLinkNetworkBinaryJoin23Next(output_15, cursor_11) => {
                                                                                                                                                        if (binary && (cursor_11 != metadata_end_3)) {
                                                                                                                                                            super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Return(((super::ml_string::Text::from_units(vec![45, 49, 57, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_11)))).concat(&super::ml_string::Text::from_units(vec![44])))
                                                                                                                                                        } else {
                                                                                                                                                            {
                                                                                                                                                                let output_17 = if !binary {
                                                                                                                                                                    {
                                                                                                                                                                        let length_2 = ((output_15.length() - metadata_start) / 2f64);
                                                                                                                                                                        {
                                                                                                                                                                            let encoded_length = super::ml_string::Text::from_units(vec![]);
                                                                                                                                                                            {
                                                                                                                                                                                let index_4 = 0f64;
                                                                                                                                                                                {
                                                                                                                                                                                    let encoded_length_2 = super::ml_transcode_link_network_binary_loop24(length_2, encoded_length.clone(), index_4);
                                                                                                                                                                                    {
                                                                                                                                                                                        let output_16 = ((output_15.slice(0f64, length_slot)).concat(&encoded_length_2)).concat(&output_15.slice((length_slot + 16f64), f64::INFINITY));
                                                                                                                                                                                        output_16.clone()
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    output_15.clone()
                                                                                                                                                                };
                                                                                                                                                                if ((output_17.length()) > ((byte_limit * if binary {
                                                                                                                                                                    20f64
                                                                                                                                                                } else {
                                                                                                                                                                    2f64
                                                                                                                                                                }))) {
                                                                                                                                                                    super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Return(((super::ml_string::Text::from_units(vec![45, 50, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(cursor_11)))).concat(&super::ml_string::Text::from_units(vec![44])))
                                                                                                                                                                } else {
                                                                                                                                                                    {
                                                                                                                                                                        let record_2 = (record + 1f64);
                                                                                                                                                                        {
                                                                                                                                                                            (output, cursor, identities, record) = (output_17.clone(), cursor_11, identities_2.clone(), record_2);
                                                                                                                                                                            continue;
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                    super::MlTranscodeLinkNetworkBinaryJoin23::MlTranscodeLinkNetworkBinaryJoin23Return(ml_result24) => {
                                                                                                                                                        super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Return(ml_result24.clone())
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        super::MlTranscodeLinkNetworkBinaryLoop22Result::MlTranscodeLinkNetworkBinaryLoop22Return(ml_result26) => {
                                                                                                                            super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Return(ml_result26.clone())
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        super::MlTranscodeLinkNetworkBinaryLoop21Result::MlTranscodeLinkNetworkBinaryLoop21Return(ml_result28) => {
                                                                                                            super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Return(ml_result28.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    super::MlTranscodeLinkNetworkBinaryJoin20::MlTranscodeLinkNetworkBinaryJoin20Return(ml_result28_2) => {
                                                                                        super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Return(ml_result28_2.clone())
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                super::MlTranscodeLinkNetworkBinaryLoop19Result::MlTranscodeLinkNetworkBinaryLoop19Return(ml_result29) => {
                                                                    super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Return(ml_result29.clone())
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
                }
            }
        } else {
            super::MlTranscodeLinkNetworkBinaryLoop7Result::MlTranscodeLinkNetworkBinaryLoop7Done(
                output.clone(),
                cursor,
                identities.clone(),
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin5 {
    MlTranscodeLinkNetworkBinaryJoin5Next(super::ml_string::Text, f64, f64),
    MlTranscodeLinkNetworkBinaryJoin5Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop31(
    mut binary: bool,
    mut output: super::ml_string::Text,
    mut next_identity: f64,
    mut index: f64,
) -> super::ml_string::Text {
    loop {
        return if (index < (8f64)) {
            {
                let output_2 = output.concat(&super::render_link_network_binary_field(
                    super::link_network_binary_integer_byte(next_identity, index),
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
            output.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin4 {
    MlTranscodeLinkNetworkBinaryJoin4Next(super::ml_string::Text, f64, f64),
    MlTranscodeLinkNetworkBinaryJoin4Return(super::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTranscodeLinkNetworkBinaryLoop32Result {
    MlTranscodeLinkNetworkBinaryLoop32Done,
    MlTranscodeLinkNetworkBinaryLoop32Return(super::ml_string::Text),
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop32(
    input: &[f64],
    header: &[f64],
    mut index: f64,
) -> super::MlTranscodeLinkNetworkBinaryLoop32Result {
    loop {
        return if (index < (7f64)) {
            if ((super::link_network_binary_unit(input, index))
                != (super::link_network_binary_unit(header, index)))
            {
                super::MlTranscodeLinkNetworkBinaryLoop32Result::MlTranscodeLinkNetworkBinaryLoop32Return(((super::ml_string::Text::from_units(vec![45, 51, 44])).concat(&super::ml_string::Text::from_text(&super::ml_number::js_number(index)))).concat(&super::ml_string::Text::from_units(vec![44])))
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            super::MlTranscodeLinkNetworkBinaryLoop32Result::MlTranscodeLinkNetworkBinaryLoop32Done
        };
    }
}

/// Execute the checked shared binary-network framing algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop33(
    mut binary: bool,
    mut output: super::ml_string::Text,
    header: &[f64],
    mut index: f64,
) -> super::ml_string::Text {
    loop {
        return if (index < (7f64)) {
            {
                let output_2 = output.concat(&super::render_link_network_binary_field(
                    super::link_network_binary_unit(header, index),
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
            output.clone()
        };
    }
}
