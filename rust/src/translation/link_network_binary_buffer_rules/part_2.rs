// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlTemporaryTranscodeLinkNetworkBinaryJoin4;
use super::MlTemporaryTranscodeLinkNetworkBinaryJoin5;
use super::MlTemporaryTranscodeLinkNetworkBinaryJoin6;
use super::MlTemporaryTranscodeLinkNetworkBinaryJoin7;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop8Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop38Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop39Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop40Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop41Result;
use super::accept_link_network_binary_integer;
use super::link_network_binary_integer_byte;
use super::link_network_binary_unit;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_link_network_binary_integer;
use super::render_link_network_binary_field;
use super::source_ml_temporary_transcode_link_network_binary_loop8;
use super::source_ml_temporary_transcode_link_network_binary_loop10;
use super::source_ml_temporary_transcode_link_network_binary_loop38;
use super::source_ml_temporary_transcode_link_network_binary_loop39;
use super::source_ml_temporary_transcode_link_network_binary_loop40;
use super::source_ml_temporary_transcode_link_network_binary_loop41;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn transcode_link_network_binary(
    input: &[f64],
    binary: bool,
    record_limit: f64,
    byte_limit: f64,
) -> self::ml_string::Text {
    {
        let output = Vec::<self::ml_string::Text>::new();
        {
            let output_length = 0f64;
            if ((!(self::accept_link_network_binary_integer(record_limit))
                || (record_limit < 1f64))
                || !(self::accept_link_network_binary_integer(byte_limit)))
                || (byte_limit < 16f64)
            {
                ((self::ml_string::Text::from_units(vec![45, 49, 44])).concat(
                    &self::ml_string::Text::from_text(&self::ml_number::js_number(0f64)),
                ))
                .concat(&self::ml_string::Text::from_units(vec![44]))
            } else if (input.len() as f64) > byte_limit {
                ((self::ml_string::Text::from_units(vec![45, 50, 44])).concat(
                    &self::ml_string::Text::from_text(&self::ml_number::js_number(0f64)),
                ))
                .concat(&self::ml_string::Text::from_units(vec![44]))
            } else {
                {
                    let cursor = 0f64;
                    {
                        let version = 0f64;
                        {
                            let header = vec![77f64, 76f64, 68f64, 83f64, 78f64, 80f64, 48f64];
                            {
                                let source_ml_source1_s5 = if binary {
                                    {
                                        let index = 0f64;
                                        {
                                            let source_ml_source1_s6 = self::source_ml_temporary_transcode_link_network_binary_loop40(input, &(header), index);
                                            match source_ml_source1_s6 {
                                                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop40Result::MlTemporaryTranscodeLinkNetworkBinaryLoop40Done => {
                                                        {
                                                            let version_2 = self::link_network_binary_unit(input, 7f64) - 48f64;
                                                            {
                                                                let cursor_2 = 8f64;
                                                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin4::MlTemporaryTranscodeLinkNetworkBinaryJoin4Next(output, output_length, cursor_2, version_2)
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop40Result::MlTemporaryTranscodeLinkNetworkBinaryLoop40Return(source_ml_temporary_result40) => {
                                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin4::MlTemporaryTranscodeLinkNetworkBinaryJoin4Return(source_ml_temporary_result40)
                                                    }
                                                }
                                        }
                                    }
                                } else {
                                    {
                                        let version_3 =
                                            self::link_network_binary_unit(input, cursor);
                                        {
                                            let cursor_3 = cursor + 1f64;
                                            {
                                                let index_2 = 0f64;
                                                {
                                                    let source_ml_source1_s7 = self::source_ml_temporary_transcode_link_network_binary_loop41(binary, output, output_length, &(header), index_2);
                                                    match source_ml_source1_s7 {
                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop41Result::MlTemporaryTranscodeLinkNetworkBinaryLoop41Done(output_2, output_length_2) => {
                                                                {
                                                                    let encoded = self::render_link_network_binary_field((48f64 + version_3), binary);
                                                                    {
                                                                        let output_3 = { let (ml_append_right, ml_append_left) = (vec![encoded.clone()], output_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        {
                                                                            let output_length_3 = output_length_2 + encoded.length();
                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin4::MlTemporaryTranscodeLinkNetworkBinaryJoin4Next(output_3, output_length_3, cursor_3, version_3)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                }
                                            }
                                        }
                                    }
                                };
                                match source_ml_source1_s5 {
                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin4::MlTemporaryTranscodeLinkNetworkBinaryJoin4Next(output_4, output_length_4, cursor_4, version_4) => {
                                            if (version_4 != 1f64) && (version_4 != 2f64) {
                                                ((self::ml_string::Text::from_units(vec![45, 52, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(7f64)))).concat(&self::ml_string::Text::from_units(vec![44]))
                                            } else {
                                                {
                                                    let source_ml_source1_s8 = if binary {
                                                        {
                                                            let encoded_2 = self::render_link_network_binary_field(version_4, binary);
                                                            {
                                                                let output_5 = { let (ml_append_right, ml_append_left) = (vec![encoded_2.clone()], output_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let output_length_5 = output_length_4 + encoded_2.length();
                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin5::MlTemporaryTranscodeLinkNetworkBinaryJoin5Next(output_5, output_length_5)
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin5::MlTemporaryTranscodeLinkNetworkBinaryJoin5Next(output_4, output_length_4)
                                                    };
                                                    match source_ml_source1_s8 {
                                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin5::MlTemporaryTranscodeLinkNetworkBinaryJoin5Next(output_6, output_length_6) => {
                                                            {
                                                                let next_identity = 0f64;
                                                                {
                                                                    let source_ml_source1_s10 = if version_4 == 2f64 {
                                                                        {
                                                                            let next_identity_2 = self::read_link_network_binary_integer(input, cursor_4, 8f64, binary);
                                                                            if next_identity_2 < 1f64 {
                                                                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin6::MlTemporaryTranscodeLinkNetworkBinaryJoin6Return(((self::ml_string::Text::from_units(vec![45, 53, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_4)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                                                            } else {
                                                                                {
                                                                                    let cursor_5 = cursor_4 + if binary {
                                                                                        8f64
                                                                                    } else {
                                                                                        1f64
                                                                                    };
                                                                                    if binary {
                                                                                        {
                                                                                            let encoded_3 = self::render_link_network_binary_field(next_identity_2, binary);
                                                                                            {
                                                                                                let output_7 = { let (ml_append_right, ml_append_left) = (vec![encoded_3.clone()], output_6); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                {
                                                                                                    let output_length_7 = output_length_6 + encoded_3.length();
                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin6::MlTemporaryTranscodeLinkNetworkBinaryJoin6Next(output_7, output_length_7, cursor_5, next_identity_2)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let index_3 = 0f64;
                                                                                            {
                                                                                                let source_ml_source1_s11 = self::source_ml_temporary_transcode_link_network_binary_loop39(binary, output_6, output_length_6, next_identity_2, index_3);
                                                                                                match source_ml_source1_s11 {
                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop39Result::MlTemporaryTranscodeLinkNetworkBinaryLoop39Done(output_8, output_length_8) => {
                                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin6::MlTemporaryTranscodeLinkNetworkBinaryJoin6Next(output_8, output_length_8, cursor_5, next_identity_2)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin6::MlTemporaryTranscodeLinkNetworkBinaryJoin6Next(output_6, output_length_6, cursor_4, next_identity)
                                                                    };
                                                                    match source_ml_source1_s10 {
                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin6::MlTemporaryTranscodeLinkNetworkBinaryJoin6Next(output_9, output_length_9, cursor_6, next_identity_3) => {
                                                                            {
                                                                                let count = self::read_link_network_binary_integer(input, cursor_6, 8f64, binary);
                                                                                if ((count < 0f64) || (count > record_limit)) || (count > (input.len() as f64)) {
                                                                                    ((self::ml_string::Text::from_units(vec![45, 54, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_6)))).concat(&self::ml_string::Text::from_units(vec![44]))
                                                                                } else {
                                                                                    {
                                                                                        let cursor_7 = cursor_6 + if binary {
                                                                                            8f64
                                                                                        } else {
                                                                                            1f64
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_s13 = if binary {
                                                                                                {
                                                                                                    let encoded_4 = self::render_link_network_binary_field(count, binary);
                                                                                                    {
                                                                                                        let output_10 = { let (ml_append_right, ml_append_left) = (vec![encoded_4.clone()], output_9); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                        {
                                                                                                            let output_length_10 = output_length_9 + encoded_4.length();
                                                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin7::MlTemporaryTranscodeLinkNetworkBinaryJoin7Next(output_10, output_length_10)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                {
                                                                                                    let index_4 = 0f64;
                                                                                                    {
                                                                                                        let source_ml_source1_s14 = self::source_ml_temporary_transcode_link_network_binary_loop38(binary, output_9, output_length_9, count, index_4);
                                                                                                        match source_ml_source1_s14 {
                                                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop38Result::MlTemporaryTranscodeLinkNetworkBinaryLoop38Done(output_11, output_length_11) => {
                                                                                                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin7::MlTemporaryTranscodeLinkNetworkBinaryJoin7Next(output_11, output_length_11)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            match source_ml_source1_s13 {
                                                                                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin7::MlTemporaryTranscodeLinkNetworkBinaryJoin7Next(output_12, output_length_12) => {
                                                                                                    {
                                                                                                        let identities = Vec::<f64>::new();
                                                                                                        {
                                                                                                            let greatest_identity = 0f64;
                                                                                                            {
                                                                                                                let record = 0f64;
                                                                                                                {
                                                                                                                    let record_iteration_result = self::source_ml_temporary_transcode_link_network_binary_loop8(input, binary, byte_limit, output_12, output_length_12, cursor_7, version_4, next_identity_3, count, identities, greatest_identity, record);
                                                                                                                    match record_iteration_result {
                                                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop8Result::MlTemporaryTranscodeLinkNetworkBinaryLoop8Done(output_13, output_length_13, cursor_8, identities_2, greatest_identity_2) => {
                                                                                                                            {
                                                                                                                                let source_ml_source1_s17 = if version_4 == 2f64 {
                                                                                                                                    {
                                                                                                                                        let section = 0f64;
                                                                                                                                        {
                                                                                                                                            let metadata_iteration_result = self::source_ml_temporary_transcode_link_network_binary_loop10(input, binary, record_limit, byte_limit, output_13, output_length_13, cursor_8, &(identities_2), greatest_identity_2, section);
                                                                                                                                            match metadata_iteration_result {
                                                                                                                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop10Result::MlTemporaryTranscodeLinkNetworkBinaryLoop10Done(metadata_output, metadata_output_length, cursor_9) => {
                                                                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin9::MlTemporaryTranscodeLinkNetworkBinaryJoin9Next(metadata_output, metadata_output_length, cursor_9)
                                                                                                                                                }
                                                                                                                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop10Result::MlTemporaryTranscodeLinkNetworkBinaryLoop10Return(source_ml_temporary_result10) => {
                                                                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin9::MlTemporaryTranscodeLinkNetworkBinaryJoin9Return(source_ml_temporary_result10)
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                } else {
                                                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin9::MlTemporaryTranscodeLinkNetworkBinaryJoin9Next(output_13, output_length_13, cursor_8)
                                                                                                                                };
                                                                                                                                match source_ml_source1_s17 {
                                                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin9::MlTemporaryTranscodeLinkNetworkBinaryJoin9Next(output_15, output_length_15, cursor_10) => {
                                                                                                                                        {
let ml_library_inequality_1 = cursor_10 != (input.len() as f64) ;
if ml_library_inequality_1 {
                                                                                                                                            ((self::ml_string::Text::from_units(vec![45, 50, 50, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_10)))).concat(&self::ml_string::Text::from_units(vec![44]))
                                                                                                                                        } else if output_length_15 > (byte_limit * if binary {
                                                                                                                                                20f64
                                                                                                                                            } else {
                                                                                                                                                2f64
                                                                                                                                            }) {
                                                                                                                                                ((self::ml_string::Text::from_units(vec![45, 50, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_10)))).concat(&self::ml_string::Text::from_units(vec![44]))
                                                                                                                                            } else {
                                                                                                                                                self::ml_string::join(&output_15, &self::ml_string::Text::from_units(vec![]))
                                                                                                                                            }
}
                                                                                                                                    }
                                                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin9::MlTemporaryTranscodeLinkNetworkBinaryJoin9Return(source_ml_temporary_result9) => {
                                                                                                                                        source_ml_temporary_result9
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop8Result::MlTemporaryTranscodeLinkNetworkBinaryLoop8Return(source_ml_temporary_result19) => {
                                                                                                                            source_ml_temporary_result19
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
                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin6::MlTemporaryTranscodeLinkNetworkBinaryJoin6Return(source_ml_temporary_result38) => {
                                                                            source_ml_temporary_result38
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin4::MlTemporaryTranscodeLinkNetworkBinaryJoin4Return(source_ml_temporary_result39) => {
                                            source_ml_temporary_result39
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadLinkNetworkBinaryIntegerLoop1Result {
    MlTemporaryReadLinkNetworkBinaryIntegerLoop1Done(f64),
    MlTemporaryReadLinkNetworkBinaryIntegerLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_link_network_binary_integer_loop1(
    input: &[f64],
    mut cursor: f64,
    mut width: f64,
    mut value: f64,
    mut factor: f64,
    mut index: f64,
) -> self::MlTemporaryReadLinkNetworkBinaryIntegerLoop1Result {
    loop {
        return if index < width {
            {
                let byte = self::link_network_binary_unit(input, (cursor + index));
                if !(self::accept_link_network_binary_integer(byte)) || (byte > 255f64) {
                    self::MlTemporaryReadLinkNetworkBinaryIntegerLoop1Result::MlTemporaryReadLinkNetworkBinaryIntegerLoop1Return(-1f64)
                } else {
                    {
                        let value_2 = value + (byte * factor);
                        if (self::accept_link_network_binary_integer(value_2)) {
                            {
                                let factor_2 = factor * 256f64;
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (value, factor, index) = (value_2, factor_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            self::MlTemporaryReadLinkNetworkBinaryIntegerLoop1Result::MlTemporaryReadLinkNetworkBinaryIntegerLoop1Return(-1f64)
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadLinkNetworkBinaryIntegerLoop1Result::MlTemporaryReadLinkNetworkBinaryIntegerLoop1Done(value)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_link_network_binary_integer_byte_loop2(
    mut value: f64,
    mut index: f64,
    mut at: f64,
) -> f64 {
    loop {
        if at < index {
            {
                let value_2 = (value / 256f64).floor();
                {
                    let at_2 = at + 1f64;
                    {
                        (value, at) = (value_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { value };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptLinkNetworkBinaryUnicodeLoop3Result {
    MlTemporaryAcceptLinkNetworkBinaryUnicodeLoop3Done,
    MlTemporaryAcceptLinkNetworkBinaryUnicodeLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_link_network_binary_unicode_loop3(
    units: &[f64],
    mut index: f64,
) -> self::MlTemporaryAcceptLinkNetworkBinaryUnicodeLoop3Result {
    loop {
        return if index < (units.len() as f64) {
            {
                let unit = self::link_network_binary_unit(units, index);
                if !(self::accept_link_network_binary_integer(unit)) || (unit > 65_535f64) {
                    self::MlTemporaryAcceptLinkNetworkBinaryUnicodeLoop3Result::MlTemporaryAcceptLinkNetworkBinaryUnicodeLoop3Return(false)
                } else if (55_296f64..=56_319f64).contains(&unit) {
                    {
                        let index_2 = index + 1f64;
                        {
                            let low = self::link_network_binary_unit(units, index_2);
                            if (low < 56_320f64) || (low > 57_343f64) {
                                self::MlTemporaryAcceptLinkNetworkBinaryUnicodeLoop3Result::MlTemporaryAcceptLinkNetworkBinaryUnicodeLoop3Return(false)
                            } else {
                                {
                                    let index_3 = index_2 + 1f64;
                                    {
                                        index = index_3;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else if (56_320f64..=57_343f64).contains(&unit) {
                    self::MlTemporaryAcceptLinkNetworkBinaryUnicodeLoop3Result::MlTemporaryAcceptLinkNetworkBinaryUnicodeLoop3Return(false)
                } else {
                    {
                        let index_4 = index + 1f64;
                        {
                            index = index_4;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptLinkNetworkBinaryUnicodeLoop3Result::MlTemporaryAcceptLinkNetworkBinaryUnicodeLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin9 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin9Next(Vec<self::ml_string::Text>, f64, f64),
    MlTemporaryTranscodeLinkNetworkBinaryJoin9Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop10Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop10Done(Vec<self::ml_string::Text>, f64, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop10Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop12Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop12Done(Vec<self::ml_string::Text>, f64, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop12Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin14 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin14Next(Vec<self::ml_string::Text>, f64, f64),
    MlTemporaryTranscodeLinkNetworkBinaryJoin14Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop15Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop15Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop15(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut identity: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop15Result {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop15Result::MlTemporaryTranscodeLinkNetworkBinaryLoop15Done(output, output_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop13Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop13Done(Vec<self::ml_string::Text>, f64, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop13Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop17Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop17Done(Vec<self::ml_string::Text>, f64, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop17Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop17(
    input: &[f64],
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut cursor: f64,
    mut length: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop17Result {
    loop {
        return if index < length {
            {
                let value = self::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if (value < 0f64) || (value > 255f64) {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop17Result::MlTemporaryTranscodeLinkNetworkBinaryLoop17Return(((self::ml_string::Text::from_units(vec![45, 49, 54, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = cursor + 1f64;
                        {
                            let encoded = self::render_link_network_binary_field(value, binary);
                            {
                                let output_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![encoded.clone()], output);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let output_length_2 = output_length + encoded.length();
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (output, output_length, cursor, index) =
                                                (output_2, output_length_2, cursor_2, index_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop17Result::MlTemporaryTranscodeLinkNetworkBinaryLoop17Done(output, output_length, cursor)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin16 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin16Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop18Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop18Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop18(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut length: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop18Result {
    loop {
        if index < 8f64 {
            {
                let encoded = self::render_link_network_binary_field(
                    self::link_network_binary_integer_byte(length, index),
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop18Result::MlTemporaryTranscodeLinkNetworkBinaryLoop18Done(output, output_length)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop13(
    input: &[f64],
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut cursor: f64,
    mut strings: f64,
    mut text: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop13Result {
    loop {
        return if text < strings {
            {
                let length = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if (length < 0f64)
                    || (length
                        > (((input.len() as f64) - cursor) - if binary { 8f64 } else { 1f64 }))
                {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop13Result::MlTemporaryTranscodeLinkNetworkBinaryLoop13Return(((self::ml_string::Text::from_units(vec![45, 49, 53, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = cursor + if binary { 8f64 } else { 1f64 };
                        {
                            let source_ml_source1_s20 = if binary {
                                {
                                    let encoded =
                                        self::render_link_network_binary_field(length, binary);
                                    {
                                        let output_2 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![encoded.clone()], output);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let output_length_2 = output_length + encoded.length();
                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin16::MlTemporaryTranscodeLinkNetworkBinaryJoin16Next(output_2, output_length_2)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index = 0f64;
                                    {
                                        let source_ml_source1_s21 = self::source_ml_temporary_transcode_link_network_binary_loop18(binary, output, output_length, length, index);
                                        match source_ml_source1_s21 {
                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop18Result::MlTemporaryTranscodeLinkNetworkBinaryLoop18Done(output_3, output_length_3) => {
                                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin16::MlTemporaryTranscodeLinkNetworkBinaryJoin16Next(output_3, output_length_3)
                                            }
                                        }
                                    }
                                }
                            };
                            match source_ml_source1_s20 {
                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin16::MlTemporaryTranscodeLinkNetworkBinaryJoin16Next(output_4, output_length_4) => {
                                    {
                                        let index_2 = 0f64;
                                        {
                                            let source_ml_source1_s22 = self::source_ml_temporary_transcode_link_network_binary_loop17(input, binary, output_4, output_length_4, cursor_2, length, index_2);
                                            match source_ml_source1_s22 {
                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop17Result::MlTemporaryTranscodeLinkNetworkBinaryLoop17Done(output_5, output_length_5, cursor_3) => {
                                                    {
                                                        let text_2 = text + 1f64;
                                                        {
                                                            (output, output_length, cursor, text) = (output_5, output_length_5, cursor_3, text_2);
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop17Result::MlTemporaryTranscodeLinkNetworkBinaryLoop17Return(source_ml_temporary_result17) => {
                                                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop13Result::MlTemporaryTranscodeLinkNetworkBinaryLoop13Return(source_ml_temporary_result17)
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop13Result::MlTemporaryTranscodeLinkNetworkBinaryLoop13Done(output, output_length, cursor)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_transcode_link_network_binary_loop12(
    input: &[f64],
    mut binary: bool,
    mut byte_limit: f64,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut cursor: f64,
    identities: &[f64],
    mut greatest_identity: f64,
    mut section: f64,
    mut entries: f64,
    mut entry: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop12Result {
    loop {
        return if entry < entries {
            {
                let strings = if section == 0f64 { 1f64 } else { 3f64 };
                {
                    let text = 0f64;
                    {
                        let source_ml_source1_s23 =
                            self::source_ml_temporary_transcode_link_network_binary_loop13(
                                input,
                                binary,
                                output,
                                output_length,
                                cursor,
                                strings,
                                text,
                            );
                        match source_ml_source1_s23 {
                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop13Result::MlTemporaryTranscodeLinkNetworkBinaryLoop13Done(output_2, output_length_2, cursor_2) => {
                                {
                                    let source_ml_source1_s25 = if section == 0f64 {
                                        {
                                            let identity = self::read_link_network_binary_integer(input, cursor_2, 8f64, binary);
                                            if identity < 1f64 {
                                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin14::MlTemporaryTranscodeLinkNetworkBinaryJoin14Return(((self::ml_string::Text::from_units(vec![45, 50, 49, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_2)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                            } else {
                                                {
                                                    let found = (identity <= greatest_identity) && self::ml_array::includes(identities, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()));
                                                    if found {
                                                        {
                                                            let cursor_3 = cursor_2 + if binary {
                                                                8f64
                                                            } else {
                                                                1f64
                                                            };
                                                            if binary {
                                                                {
                                                                    let encoded = self::render_link_network_binary_field(identity, binary);
                                                                    {
                                                                        let output_3 = { let (ml_append_right, ml_append_left) = (vec![encoded.clone()], output_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        {
                                                                            let output_length_3 = output_length_2 + encoded.length();
                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin14::MlTemporaryTranscodeLinkNetworkBinaryJoin14Next(output_3, output_length_3, cursor_3)
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    let index = 0f64;
                                                                    {
                                                                        let source_ml_source1_s26 = self::source_ml_temporary_transcode_link_network_binary_loop15(binary, output_2, output_length_2, identity, index);
                                                                        match source_ml_source1_s26 {
                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop15Result::MlTemporaryTranscodeLinkNetworkBinaryLoop15Done(output_4, output_length_4) => {
                                                                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin14::MlTemporaryTranscodeLinkNetworkBinaryJoin14Next(output_4, output_length_4, cursor_3)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin14::MlTemporaryTranscodeLinkNetworkBinaryJoin14Return(((self::ml_string::Text::from_units(vec![45, 50, 49, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_2)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin14::MlTemporaryTranscodeLinkNetworkBinaryJoin14Next(output_2, output_length_2, cursor_2)
                                    };
                                    match source_ml_source1_s25 {
                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin14::MlTemporaryTranscodeLinkNetworkBinaryJoin14Next(output_5, output_length_5, cursor_4) => {
                                            if output_length_5 > (byte_limit * if binary {
                                                20f64
                                            } else {
                                                2f64
                                            }) {
                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop12Result::MlTemporaryTranscodeLinkNetworkBinaryLoop12Return(((self::ml_string::Text::from_units(vec![45, 50, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_4)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                            } else {
                                                {
                                                    let entry_2 = entry + 1f64;
                                                    {
                                                        (output, output_length, cursor, entry) = (output_5, output_length_5, cursor_4, entry_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin14::MlTemporaryTranscodeLinkNetworkBinaryJoin14Return(source_ml_temporary_result14) => {
                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop12Result::MlTemporaryTranscodeLinkNetworkBinaryLoop12Return(source_ml_temporary_result14.clone())
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop13Result::MlTemporaryTranscodeLinkNetworkBinaryLoop13Return(source_ml_temporary_result15) => {
                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop12Result::MlTemporaryTranscodeLinkNetworkBinaryLoop12Return(source_ml_temporary_result15.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop12Result::MlTemporaryTranscodeLinkNetworkBinaryLoop12Done(output, output_length, cursor)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin11 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin11Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop19Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop19Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop19(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut entries: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop19Result {
    loop {
        if index < 8f64 {
            {
                let encoded = self::render_link_network_binary_field(
                    self::link_network_binary_integer_byte(entries, index),
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop19Result::MlTemporaryTranscodeLinkNetworkBinaryLoop19Done(output, output_length)
        };
    }
}
