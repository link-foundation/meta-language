// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlTranscodeLinkNetworkBinaryJoin21;
use super::MlTranscodeLinkNetworkBinaryJoin22;
use super::MlTranscodeLinkNetworkBinaryJoin24;
use super::MlTranscodeLinkNetworkBinaryJoin27;
use super::MlTranscodeLinkNetworkBinaryLoop8Result;
use super::MlTranscodeLinkNetworkBinaryLoop23Result;
use super::MlTranscodeLinkNetworkBinaryLoop25Result;
use super::MlTranscodeLinkNetworkBinaryLoop26Result;
use super::MlTranscodeLinkNetworkBinaryLoop29Result;
use super::MlTranscodeLinkNetworkBinaryLoop34Result;
use super::MlTranscodeLinkNetworkBinaryLoop36Result;
use super::MlTranscodeLinkNetworkBinaryLoop37Result;
use super::link_network_binary_integer_byte;
use super::link_network_binary_unit;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::ml_transcode_link_network_binary_loop23;
use super::ml_transcode_link_network_binary_loop25;
use super::ml_transcode_link_network_binary_loop26;
use super::ml_transcode_link_network_binary_loop28;
use super::ml_transcode_link_network_binary_loop29;
use super::ml_transcode_link_network_binary_loop34;
use super::ml_transcode_link_network_binary_loop36;
use super::read_link_network_binary_integer;
use super::render_link_network_binary_field;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop37(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut identity: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop37Result {
    loop {
        if index < 8f64 {
            {
                let encoded = self::render_link_network_binary_field(
                    self::link_network_binary_integer_byte(identity, index),
                    binary,
                );
                {
                    let output_2 = {
                        let (ml_append_right, ml_append_left) = (vec![encoded.clone()], output);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let output_length_2 = output_length + encoded.length();
                        {
                            let index_2 = index + 1f64;
                            {
                                (output, output_length, index) =
                                    (output_2, output_length_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTranscodeLinkNetworkBinaryLoop37Result::MlTranscodeLinkNetworkBinaryLoop37Done(
                output,
                output_length,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin20 {
    MlTranscodeLinkNetworkBinaryJoin20Next(f64),
    MlTranscodeLinkNetworkBinaryJoin20Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_transcode_link_network_binary_loop8(
    input: &[f64],
    mut binary: bool,
    mut byte_limit: f64,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut cursor: f64,
    mut version: f64,
    mut next_identity: f64,
    mut count: f64,
    mut identities: Vec<f64>,
    mut greatest_identity: f64,
    mut record: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop8Result {
    loop {
        return if record < count {
            {
                let identity = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if ((identity < 1f64) || (identity >= 9_007_199_254_740_991f64))
                    || ((version == 2f64) && (identity >= next_identity))
                {
                    self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Return(((self::ml_string::Text::from_units(vec![45, 55, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let ml_s37 = if identity > greatest_identity {
                            {
                                let greatest_identity_2 = identity;
                                self::MlTranscodeLinkNetworkBinaryJoin20::MlTranscodeLinkNetworkBinaryJoin20Next(greatest_identity_2)
                            }
                        } else if self::ml_array::includes(
                            &identities,
                            &identity,
                            0f64,
                            false,
                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                        ) {
                            self::MlTranscodeLinkNetworkBinaryJoin20::MlTranscodeLinkNetworkBinaryJoin20Return(((self::ml_string::Text::from_units(vec![45, 56, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                        } else {
                            self::MlTranscodeLinkNetworkBinaryJoin20::MlTranscodeLinkNetworkBinaryJoin20Next(greatest_identity)
                        };
                        match ml_s37 {
                            self::MlTranscodeLinkNetworkBinaryJoin20::MlTranscodeLinkNetworkBinaryJoin20Next(greatest_identity_3) => {
                                {
                                    let identities_2 = { let (ml_append_right, ml_append_left) = (vec![identity], identities); self::ml_array::append(ml_append_left, ml_append_right) };
                                    {
                                        let cursor_2 = cursor + if binary {
                                            8f64
                                        } else {
                                            1f64
                                        };
                                        {
                                            let ml_s39 = if binary {
                                                {
                                                    let encoded = self::render_link_network_binary_field(identity, binary);
                                                    {
                                                        let output_2 = { let (ml_append_right, ml_append_left) = (vec![encoded.clone()], output); self::ml_array::append(ml_append_left, ml_append_right) };
                                                        {
                                                            let output_length_2 = output_length + encoded.length();
                                                            self::MlTranscodeLinkNetworkBinaryJoin21::MlTranscodeLinkNetworkBinaryJoin21Next(output_2, output_length_2)
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let index = 0f64;
                                                    {
                                                        let ml_s40 = self::ml_transcode_link_network_binary_loop37(binary, output, output_length, identity, index);
                                                        match ml_s40 {
                                                            self::MlTranscodeLinkNetworkBinaryLoop37Result::MlTranscodeLinkNetworkBinaryLoop37Done(output_3, output_length_3) => {
                                                                self::MlTranscodeLinkNetworkBinaryJoin21::MlTranscodeLinkNetworkBinaryJoin21Next(output_3, output_length_3)
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            match ml_s39 {
                                                self::MlTranscodeLinkNetworkBinaryJoin21::MlTranscodeLinkNetworkBinaryJoin21Next(output_4, output_length_4) => {
                                                    {
                                                        let references = self::read_link_network_binary_integer(input, cursor_2, 8f64, binary);
                                                        if (references < 0f64) || (references > (input.len() as f64)) {
                                                            self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Return(((self::ml_string::Text::from_units(vec![45, 57, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_2)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                                        } else {
                                                            {
                                                                let cursor_3 = cursor_2 + if binary {
                                                                    8f64
                                                                } else {
                                                                    1f64
                                                                };
                                                                {
                                                                    let ml_s42 = if binary {
                                                                        {
                                                                            let encoded_2 = self::render_link_network_binary_field(references, binary);
                                                                            {
                                                                                let output_5 = { let (ml_append_right, ml_append_left) = (vec![encoded_2.clone()], output_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                {
                                                                                    let output_length_5 = output_length_4 + encoded_2.length();
                                                                                    self::MlTranscodeLinkNetworkBinaryJoin22::MlTranscodeLinkNetworkBinaryJoin22Next(output_5, output_length_5)
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let index_2 = 0f64;
                                                                            {
                                                                                let ml_s43 = self::ml_transcode_link_network_binary_loop36(binary, output_4, output_length_4, references, index_2);
                                                                                match ml_s43 {
                                                                                    self::MlTranscodeLinkNetworkBinaryLoop36Result::MlTranscodeLinkNetworkBinaryLoop36Done(output_6, output_length_6) => {
                                                                                        self::MlTranscodeLinkNetworkBinaryJoin22::MlTranscodeLinkNetworkBinaryJoin22Next(output_6, output_length_6)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    match ml_s42 {
                                                                        self::MlTranscodeLinkNetworkBinaryJoin22::MlTranscodeLinkNetworkBinaryJoin22Next(output_7, output_length_7) => {
                                                                            {
                                                                                let reference = 0f64;
                                                                                {
                                                                                    let ml_s44 = self::ml_transcode_link_network_binary_loop23(input, binary, output_7, output_length_7, cursor_3, references, reference);
                                                                                    match ml_s44 {
                                                                                        self::MlTranscodeLinkNetworkBinaryLoop23Result::MlTranscodeLinkNetworkBinaryLoop23Done(output_8, output_length_8, cursor_4) => {
                                                                                            {
                                                                                                let metadata_end = 0f64;
                                                                                                {
                                                                                                    let metadata = Vec::<self::ml_string::Text>::new();
                                                                                                    {
                                                                                                        let metadata_length = 0f64;
                                                                                                        {
                                                                                                            let length_slot = 0f64;
                                                                                                            {
                                                                                                                let ml_s46 = if binary {
                                                                                                                    {
                                                                                                                        let length = self::read_link_network_binary_integer(input, cursor_4, 8f64, true);
                                                                                                                        if (length < 6f64) || (length > (((input.len() as f64) - cursor_4) - 8f64)) {
                                                                                                                            self::MlTranscodeLinkNetworkBinaryJoin24::MlTranscodeLinkNetworkBinaryJoin24Return(((self::ml_string::Text::from_units(vec![45, 49, 49, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_4)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let cursor_5 = cursor_4 + 8f64;
                                                                                                                                {
                                                                                                                                    let metadata_end_2 = cursor_5 + length;
                                                                                                                                    self::MlTranscodeLinkNetworkBinaryJoin24::MlTranscodeLinkNetworkBinaryJoin24Next(cursor_5, metadata_end_2, metadata, metadata_length)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    {
                                                                                                                        let index_3 = 0f64;
                                                                                                                        {
                                                                                                                            let ml_s47 = self::ml_transcode_link_network_binary_loop34(binary, metadata, metadata_length, index_3);
                                                                                                                            match ml_s47 {
                                                                                                                                self::MlTranscodeLinkNetworkBinaryLoop34Result::MlTranscodeLinkNetworkBinaryLoop34Done(metadata_2, metadata_length_2) => {
                                                                                                                                    self::MlTranscodeLinkNetworkBinaryJoin24::MlTranscodeLinkNetworkBinaryJoin24Next(cursor_4, metadata_end, metadata_2, metadata_length_2)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                };
                                                                                                                match ml_s46 {
                                                                                                                    self::MlTranscodeLinkNetworkBinaryJoin24::MlTranscodeLinkNetworkBinaryJoin24Next(cursor_6, metadata_end_3, metadata_3, metadata_length_3) => {
                                                                                                                        {
                                                                                                                            let metadata_start = metadata_length_3;
                                                                                                                            {
                                                                                                                                let field = 0f64;
                                                                                                                                {
                                                                                                                                    let ml_s48 = self::ml_transcode_link_network_binary_loop25(input, binary, cursor_6, metadata_3, metadata_length_3, field);
                                                                                                                                    match ml_s48 {
                                                                                                                                        self::MlTranscodeLinkNetworkBinaryLoop25Result::MlTranscodeLinkNetworkBinaryLoop25Done(cursor_7, metadata_4, metadata_length_4) => {
                                                                                                                                            {
                                                                                                                                                let text = 0f64;
                                                                                                                                                {
                                                                                                                                                    let ml_s49 = self::ml_transcode_link_network_binary_loop26(input, binary, cursor_7, metadata_4, metadata_length_4, text);
                                                                                                                                                    match ml_s49 {
                                                                                                                                                        self::MlTranscodeLinkNetworkBinaryLoop26Result::MlTranscodeLinkNetworkBinaryLoop26Done(cursor_8, metadata_5, metadata_length_5) => {
                                                                                                                                                            {
                                                                                                                                                                let span = self::read_link_network_binary_integer(input, cursor_8, 1f64, binary);
                                                                                                                                                                if (span != 0f64) && (span != 1f64) {
                                                                                                                                                                    self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Return(((self::ml_string::Text::from_units(vec![45, 49, 55, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_8)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                                                                                                                                                } else {
                                                                                                                                                                    {
                                                                                                                                                                        let cursor_9 = cursor_8 + 1f64;
                                                                                                                                                                        {
                                                                                                                                                                            let encoded_3 = self::render_link_network_binary_field(span, binary);
                                                                                                                                                                            {
                                                                                                                                                                                let metadata_6 = { let (ml_append_right, ml_append_left) = (vec![encoded_3.clone()], metadata_5); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                {
                                                                                                                                                                                    let metadata_length_6 = metadata_length_5 + encoded_3.length();
                                                                                                                                                                                    {
                                                                                                                                                                                        let ml_s51 = if span == 1f64 {
                                                                                                                                                                                            {
                                                                                                                                                                                                let field_2 = 0f64;
                                                                                                                                                                                                {
                                                                                                                                                                                                    let ml_s52 = self::ml_transcode_link_network_binary_loop29(input, binary, cursor_9, metadata_6, metadata_length_6, field_2);
                                                                                                                                                                                                    match ml_s52 {
                                                                                                                                                                                                        self::MlTranscodeLinkNetworkBinaryLoop29Result::MlTranscodeLinkNetworkBinaryLoop29Done(cursor_10, metadata_7, metadata_length_7) => {
                                                                                                                                                                                                            self::MlTranscodeLinkNetworkBinaryJoin27::MlTranscodeLinkNetworkBinaryJoin27Next(cursor_10, metadata_7, metadata_length_7)
                                                                                                                                                                                                        }
                                                                                                                                                                                                        self::MlTranscodeLinkNetworkBinaryLoop29Result::MlTranscodeLinkNetworkBinaryLoop29Return(ml_result29) => {
                                                                                                                                                                                                            self::MlTranscodeLinkNetworkBinaryJoin27::MlTranscodeLinkNetworkBinaryJoin27Return(ml_result29)
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        } else {
                                                                                                                                                                                            self::MlTranscodeLinkNetworkBinaryJoin27::MlTranscodeLinkNetworkBinaryJoin27Next(cursor_9, metadata_6, metadata_length_6)
                                                                                                                                                                                        };
                                                                                                                                                                                        match ml_s51 {
                                                                                                                                                                                            self::MlTranscodeLinkNetworkBinaryJoin27::MlTranscodeLinkNetworkBinaryJoin27Next(cursor_11, metadata_8, metadata_length_8) => {
                                                                                                                                                                                                if binary && (cursor_11 != metadata_end_3) {
                                                                                                                                                                                                    self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Return(((self::ml_string::Text::from_units(vec![45, 49, 57, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_11)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                                                                                                                                                                                } else {
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let record_output = self::ml_string::join(&metadata_8, &self::ml_string::Text::from_units(vec![]));
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let completed_record = record_output.clone();
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let completed_record_3 = if binary {
                                                                                                                                                                                                                    completed_record
                                                                                                                                                                                                                } else {
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let length_2 = (metadata_length_8 - metadata_start) / 2f64;
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            let encoded_length = self::ml_string::Text::from_units(vec![]);
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let index_4 = 0f64;
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    let encoded_length_2 = self::ml_transcode_link_network_binary_loop28(length_2, encoded_length, index_4);
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        ((record_output.slice(0f64, length_slot)).concat(&encoded_length_2)).concat(&record_output.slice((length_slot + 16f64), f64::INFINITY))
                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                };
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let output_9 = { let (ml_append_right, ml_append_left) = (vec![completed_record_3.clone()], output_8); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let output_length_9 = output_length_8 + completed_record_3.length();
                                                                                                                                                                                                                        if output_length_9 > (byte_limit * if binary {
                                                                                                                                                                                                                            20f64
                                                                                                                                                                                                                        } else {
                                                                                                                                                                                                                            2f64
                                                                                                                                                                                                                        }) {
                                                                                                                                                                                                                            self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Return(((self::ml_string::Text::from_units(vec![45, 50, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_11)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                                                                                                                                                                                                        } else {
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let record_2 = record + 1f64;
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    (output, output_length, cursor, identities, greatest_identity, record) = (output_9, output_length_9, cursor_11, identities_2, greatest_identity_3, record_2);
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
                                                                                                                                                                                            self::MlTranscodeLinkNetworkBinaryJoin27::MlTranscodeLinkNetworkBinaryJoin27Return(ml_result28) => {
                                                                                                                                                                                                self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Return(ml_result28)
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
                                                                                                                                                        self::MlTranscodeLinkNetworkBinaryLoop26Result::MlTranscodeLinkNetworkBinaryLoop26Return(ml_result30) => {
                                                                                                                                                            self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Return(ml_result30)
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                        self::MlTranscodeLinkNetworkBinaryLoop25Result::MlTranscodeLinkNetworkBinaryLoop25Return(ml_result33) => {
                                                                                                                                            self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Return(ml_result33)
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlTranscodeLinkNetworkBinaryJoin24::MlTranscodeLinkNetworkBinaryJoin24Return(ml_result33_2) => {
                                                                                                                        self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Return(ml_result33_2)
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::MlTranscodeLinkNetworkBinaryLoop23Result::MlTranscodeLinkNetworkBinaryLoop23Return(ml_result34) => {
                                                                                            self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Return(ml_result34)
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
                                }
                            }
                            self::MlTranscodeLinkNetworkBinaryJoin20::MlTranscodeLinkNetworkBinaryJoin20Return(ml_result37) => {
                                self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Return(ml_result37)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Done(
                output,
                output_length,
                cursor,
                identities,
                greatest_identity,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin7 {
    MlTranscodeLinkNetworkBinaryJoin7Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop38Result {
    MlTranscodeLinkNetworkBinaryLoop38Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop38(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut count: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop38Result {
    loop {
        if index < 8f64 {
            {
                let encoded = self::render_link_network_binary_field(
                    self::link_network_binary_integer_byte(count, index),
                    binary,
                );
                {
                    let output_2 = {
                        let (ml_append_right, ml_append_left) = (vec![encoded.clone()], output);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let output_length_2 = output_length + encoded.length();
                        {
                            let index_2 = index + 1f64;
                            {
                                (output, output_length, index) =
                                    (output_2, output_length_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTranscodeLinkNetworkBinaryLoop38Result::MlTranscodeLinkNetworkBinaryLoop38Done(
                output,
                output_length,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin6 {
    MlTranscodeLinkNetworkBinaryJoin6Next(Vec<self::ml_string::Text>, f64, f64, f64),
    MlTranscodeLinkNetworkBinaryJoin6Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop39Result {
    MlTranscodeLinkNetworkBinaryLoop39Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop39(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut next_identity: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop39Result {
    loop {
        if index < 8f64 {
            {
                let encoded = self::render_link_network_binary_field(
                    self::link_network_binary_integer_byte(next_identity, index),
                    binary,
                );
                {
                    let output_2 = {
                        let (ml_append_right, ml_append_left) = (vec![encoded.clone()], output);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let output_length_2 = output_length + encoded.length();
                        {
                            let index_2 = index + 1f64;
                            {
                                (output, output_length, index) =
                                    (output_2, output_length_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTranscodeLinkNetworkBinaryLoop39Result::MlTranscodeLinkNetworkBinaryLoop39Done(
                output,
                output_length,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin5 {
    MlTranscodeLinkNetworkBinaryJoin5Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin4 {
    MlTranscodeLinkNetworkBinaryJoin4Next(Vec<self::ml_string::Text>, f64, f64, f64),
    MlTranscodeLinkNetworkBinaryJoin4Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTranscodeLinkNetworkBinaryLoop40Result {
    MlTranscodeLinkNetworkBinaryLoop40Done,
    MlTranscodeLinkNetworkBinaryLoop40Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop40(
    input: &[f64],
    header: &[f64],
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop40Result {
    loop {
        return if index < 7f64 {
            if (self::link_network_binary_unit(input, index))
                == (self::link_network_binary_unit(header, index))
            {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlTranscodeLinkNetworkBinaryLoop40Result::MlTranscodeLinkNetworkBinaryLoop40Return(((self::ml_string::Text::from_units(vec![45, 51, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(index)))).concat(&self::ml_string::Text::from_units(vec![44])))
            }
        } else {
            self::MlTranscodeLinkNetworkBinaryLoop40Result::MlTranscodeLinkNetworkBinaryLoop40Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop41Result {
    MlTranscodeLinkNetworkBinaryLoop41Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop41(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    header: &[f64],
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop41Result {
    loop {
        if index < 7f64 {
            {
                let encoded = self::render_link_network_binary_field(
                    self::link_network_binary_unit(header, index),
                    binary,
                );
                {
                    let output_2 = {
                        let (ml_append_right, ml_append_left) = (vec![encoded.clone()], output);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let output_length_2 = output_length + encoded.length();
                        {
                            let index_2 = index + 1f64;
                            {
                                (output, output_length, index) =
                                    (output_2, output_length_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTranscodeLinkNetworkBinaryLoop41Result::MlTranscodeLinkNetworkBinaryLoop41Done(
                output,
                output_length,
            )
        };
    }
}
