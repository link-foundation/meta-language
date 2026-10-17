// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlTranscodeLinkNetworkBinaryJoin4;
use super::MlTranscodeLinkNetworkBinaryJoin5;
use super::MlTranscodeLinkNetworkBinaryJoin6;
use super::MlTranscodeLinkNetworkBinaryJoin7;
use super::MlTranscodeLinkNetworkBinaryLoop8Result;
use super::MlTranscodeLinkNetworkBinaryLoop38Result;
use super::MlTranscodeLinkNetworkBinaryLoop39Result;
use super::MlTranscodeLinkNetworkBinaryLoop40Result;
use super::MlTranscodeLinkNetworkBinaryLoop41Result;
use super::accept_link_network_binary_integer;
use super::link_network_binary_integer_byte;
use super::link_network_binary_unit;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::ml_transcode_link_network_binary_loop8;
use super::ml_transcode_link_network_binary_loop10;
use super::ml_transcode_link_network_binary_loop38;
use super::ml_transcode_link_network_binary_loop39;
use super::ml_transcode_link_network_binary_loop40;
use super::ml_transcode_link_network_binary_loop41;
use super::read_link_network_binary_integer;
use super::render_link_network_binary_field;

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
                                let ml_s5 = if binary {
                                    {
                                        let index = 0f64;
                                        {
                                            let ml_s6 =
                                                self::ml_transcode_link_network_binary_loop40(
                                                    input,
                                                    &(header),
                                                    index,
                                                );
                                            match ml_s6 {
                                                    self::MlTranscodeLinkNetworkBinaryLoop40Result::MlTranscodeLinkNetworkBinaryLoop40Done => {
                                                        {
                                                            let version_2 = self::link_network_binary_unit(input, 7f64) - 48f64;
                                                            {
                                                                let cursor_2 = 8f64;
                                                                self::MlTranscodeLinkNetworkBinaryJoin4::MlTranscodeLinkNetworkBinaryJoin4Next(output, output_length, cursor_2, version_2)
                                                            }
                                                        }
                                                    }
                                                    self::MlTranscodeLinkNetworkBinaryLoop40Result::MlTranscodeLinkNetworkBinaryLoop40Return(ml_result40) => {
                                                        self::MlTranscodeLinkNetworkBinaryJoin4::MlTranscodeLinkNetworkBinaryJoin4Return(ml_result40)
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
                                                    let ml_s7 = self::ml_transcode_link_network_binary_loop41(binary, output, output_length, &(header), index_2);
                                                    match ml_s7 {
                                                            self::MlTranscodeLinkNetworkBinaryLoop41Result::MlTranscodeLinkNetworkBinaryLoop41Done(output_2, output_length_2) => {
                                                                {
                                                                    let encoded = self::render_link_network_binary_field((48f64 + version_3), binary);
                                                                    {
                                                                        let output_3 = { let (ml_append_right, ml_append_left) = (vec![encoded.clone()], output_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        {
                                                                            let output_length_3 = output_length_2 + encoded.length();
                                                                            self::MlTranscodeLinkNetworkBinaryJoin4::MlTranscodeLinkNetworkBinaryJoin4Next(output_3, output_length_3, cursor_3, version_3)
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
                                match ml_s5 {
                                        self::MlTranscodeLinkNetworkBinaryJoin4::MlTranscodeLinkNetworkBinaryJoin4Next(output_4, output_length_4, cursor_4, version_4) => {
                                            if (version_4 != 1f64) && (version_4 != 2f64) {
                                                ((self::ml_string::Text::from_units(vec![45, 52, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(7f64)))).concat(&self::ml_string::Text::from_units(vec![44]))
                                            } else {
                                                {
                                                    let ml_s8 = if binary {
                                                        {
                                                            let encoded_2 = self::render_link_network_binary_field(version_4, binary);
                                                            {
                                                                let output_5 = { let (ml_append_right, ml_append_left) = (vec![encoded_2.clone()], output_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let output_length_5 = output_length_4 + encoded_2.length();
                                                                    self::MlTranscodeLinkNetworkBinaryJoin5::MlTranscodeLinkNetworkBinaryJoin5Next(output_5, output_length_5)
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlTranscodeLinkNetworkBinaryJoin5::MlTranscodeLinkNetworkBinaryJoin5Next(output_4, output_length_4)
                                                    };
                                                    match ml_s8 {
                                                        self::MlTranscodeLinkNetworkBinaryJoin5::MlTranscodeLinkNetworkBinaryJoin5Next(output_6, output_length_6) => {
                                                            {
                                                                let next_identity = 0f64;
                                                                {
                                                                    let ml_s10 = if version_4 == 2f64 {
                                                                        {
                                                                            let next_identity_2 = self::read_link_network_binary_integer(input, cursor_4, 8f64, binary);
                                                                            if next_identity_2 < 1f64 {
                                                                                self::MlTranscodeLinkNetworkBinaryJoin6::MlTranscodeLinkNetworkBinaryJoin6Return(((self::ml_string::Text::from_units(vec![45, 53, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_4)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                                                                                                    self::MlTranscodeLinkNetworkBinaryJoin6::MlTranscodeLinkNetworkBinaryJoin6Next(output_7, output_length_7, cursor_5, next_identity_2)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let index_3 = 0f64;
                                                                                            {
                                                                                                let ml_s11 = self::ml_transcode_link_network_binary_loop39(binary, output_6, output_length_6, next_identity_2, index_3);
                                                                                                match ml_s11 {
                                                                                                    self::MlTranscodeLinkNetworkBinaryLoop39Result::MlTranscodeLinkNetworkBinaryLoop39Done(output_8, output_length_8) => {
                                                                                                        self::MlTranscodeLinkNetworkBinaryJoin6::MlTranscodeLinkNetworkBinaryJoin6Next(output_8, output_length_8, cursor_5, next_identity_2)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        self::MlTranscodeLinkNetworkBinaryJoin6::MlTranscodeLinkNetworkBinaryJoin6Next(output_6, output_length_6, cursor_4, next_identity)
                                                                    };
                                                                    match ml_s10 {
                                                                        self::MlTranscodeLinkNetworkBinaryJoin6::MlTranscodeLinkNetworkBinaryJoin6Next(output_9, output_length_9, cursor_6, next_identity_3) => {
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
                                                                                            let ml_s13 = if binary {
                                                                                                {
                                                                                                    let encoded_4 = self::render_link_network_binary_field(count, binary);
                                                                                                    {
                                                                                                        let output_10 = { let (ml_append_right, ml_append_left) = (vec![encoded_4.clone()], output_9); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                        {
                                                                                                            let output_length_10 = output_length_9 + encoded_4.length();
                                                                                                            self::MlTranscodeLinkNetworkBinaryJoin7::MlTranscodeLinkNetworkBinaryJoin7Next(output_10, output_length_10)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                {
                                                                                                    let index_4 = 0f64;
                                                                                                    {
                                                                                                        let ml_s14 = self::ml_transcode_link_network_binary_loop38(binary, output_9, output_length_9, count, index_4);
                                                                                                        match ml_s14 {
                                                                                                            self::MlTranscodeLinkNetworkBinaryLoop38Result::MlTranscodeLinkNetworkBinaryLoop38Done(output_11, output_length_11) => {
                                                                                                                self::MlTranscodeLinkNetworkBinaryJoin7::MlTranscodeLinkNetworkBinaryJoin7Next(output_11, output_length_11)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            match ml_s13 {
                                                                                                self::MlTranscodeLinkNetworkBinaryJoin7::MlTranscodeLinkNetworkBinaryJoin7Next(output_12, output_length_12) => {
                                                                                                    {
                                                                                                        let identities = Vec::<f64>::new();
                                                                                                        {
                                                                                                            let greatest_identity = 0f64;
                                                                                                            {
                                                                                                                let record = 0f64;
                                                                                                                {
                                                                                                                    let ml_s15 = self::ml_transcode_link_network_binary_loop8(input, binary, byte_limit, output_12, output_length_12, cursor_7, version_4, next_identity_3, count, identities, greatest_identity, record);
                                                                                                                    match ml_s15 {
                                                                                                                        self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Done(output_13, output_length_13, cursor_8, identities_2, greatest_identity_2) => {
                                                                                                                            {
                                                                                                                                let ml_s17 = if version_4 == 2f64 {
                                                                                                                                    {
                                                                                                                                        let section = 0f64;
                                                                                                                                        {
                                                                                                                                            let ml_s18 = self::ml_transcode_link_network_binary_loop10(input, binary, record_limit, byte_limit, output_13, output_length_13, cursor_8, &(identities_2), greatest_identity_2, section);
                                                                                                                                            match ml_s18 {
                                                                                                                                                self::MlTranscodeLinkNetworkBinaryLoop10Result::MlTranscodeLinkNetworkBinaryLoop10Done(output_14, output_length_14, cursor_9) => {
                                                                                                                                                    self::MlTranscodeLinkNetworkBinaryJoin9::MlTranscodeLinkNetworkBinaryJoin9Next(output_14, output_length_14, cursor_9)
                                                                                                                                                }
                                                                                                                                                self::MlTranscodeLinkNetworkBinaryLoop10Result::MlTranscodeLinkNetworkBinaryLoop10Return(ml_result10) => {
                                                                                                                                                    self::MlTranscodeLinkNetworkBinaryJoin9::MlTranscodeLinkNetworkBinaryJoin9Return(ml_result10)
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                } else {
                                                                                                                                    self::MlTranscodeLinkNetworkBinaryJoin9::MlTranscodeLinkNetworkBinaryJoin9Next(output_13, output_length_13, cursor_8)
                                                                                                                                };
                                                                                                                                match ml_s17 {
                                                                                                                                    self::MlTranscodeLinkNetworkBinaryJoin9::MlTranscodeLinkNetworkBinaryJoin9Next(output_15, output_length_15, cursor_10) => {
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
                                                                                                                                    self::MlTranscodeLinkNetworkBinaryJoin9::MlTranscodeLinkNetworkBinaryJoin9Return(ml_result9) => {
                                                                                                                                        ml_result9
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlTranscodeLinkNetworkBinaryLoop8Result::MlTranscodeLinkNetworkBinaryLoop8Return(ml_result19) => {
                                                                                                                            ml_result19
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
                                                                        self::MlTranscodeLinkNetworkBinaryJoin6::MlTranscodeLinkNetworkBinaryJoin6Return(ml_result38) => {
                                                                            ml_result38
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTranscodeLinkNetworkBinaryJoin4::MlTranscodeLinkNetworkBinaryJoin4Return(ml_result39) => {
                                            ml_result39
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
pub enum MlReadLinkNetworkBinaryIntegerLoop1Result {
    MlReadLinkNetworkBinaryIntegerLoop1Done(f64),
    MlReadLinkNetworkBinaryIntegerLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_link_network_binary_integer_loop1(
    input: &[f64],
    mut cursor: f64,
    mut width: f64,
    mut value: f64,
    mut factor: f64,
    mut index: f64,
) -> self::MlReadLinkNetworkBinaryIntegerLoop1Result {
    loop {
        return if index < width {
            {
                let byte = self::link_network_binary_unit(input, (cursor + index));
                if !(self::accept_link_network_binary_integer(byte)) || (byte > 255f64) {
                    self::MlReadLinkNetworkBinaryIntegerLoop1Result::MlReadLinkNetworkBinaryIntegerLoop1Return(-1f64)
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
                            self::MlReadLinkNetworkBinaryIntegerLoop1Result::MlReadLinkNetworkBinaryIntegerLoop1Return(-1f64)
                        }
                    }
                }
            }
        } else {
            self::MlReadLinkNetworkBinaryIntegerLoop1Result::MlReadLinkNetworkBinaryIntegerLoop1Done(
                value,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_link_network_binary_integer_byte_loop2(
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
pub enum MlAcceptLinkNetworkBinaryUnicodeLoop3Result {
    MlAcceptLinkNetworkBinaryUnicodeLoop3Done,
    MlAcceptLinkNetworkBinaryUnicodeLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_link_network_binary_unicode_loop3(
    units: &[f64],
    mut index: f64,
) -> self::MlAcceptLinkNetworkBinaryUnicodeLoop3Result {
    loop {
        return if index < (units.len() as f64) {
            {
                let unit = self::link_network_binary_unit(units, index);
                if !(self::accept_link_network_binary_integer(unit)) || (unit > 65_535f64) {
                    self::MlAcceptLinkNetworkBinaryUnicodeLoop3Result::MlAcceptLinkNetworkBinaryUnicodeLoop3Return(false)
                } else if (55_296f64..=56_319f64).contains(&unit) {
                    {
                        let index_2 = index + 1f64;
                        {
                            let low = self::link_network_binary_unit(units, index_2);
                            if (low < 56_320f64) || (low > 57_343f64) {
                                self::MlAcceptLinkNetworkBinaryUnicodeLoop3Result::MlAcceptLinkNetworkBinaryUnicodeLoop3Return(false)
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
                    self::MlAcceptLinkNetworkBinaryUnicodeLoop3Result::MlAcceptLinkNetworkBinaryUnicodeLoop3Return(false)
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
            self::MlAcceptLinkNetworkBinaryUnicodeLoop3Result::MlAcceptLinkNetworkBinaryUnicodeLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin9 {
    MlTranscodeLinkNetworkBinaryJoin9Next(Vec<self::ml_string::Text>, f64, f64),
    MlTranscodeLinkNetworkBinaryJoin9Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop10Result {
    MlTranscodeLinkNetworkBinaryLoop10Done(Vec<self::ml_string::Text>, f64, f64),
    MlTranscodeLinkNetworkBinaryLoop10Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop12Result {
    MlTranscodeLinkNetworkBinaryLoop12Done(Vec<self::ml_string::Text>, f64, f64),
    MlTranscodeLinkNetworkBinaryLoop12Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin14 {
    MlTranscodeLinkNetworkBinaryJoin14Next(Vec<self::ml_string::Text>, f64, f64),
    MlTranscodeLinkNetworkBinaryJoin14Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop15Result {
    MlTranscodeLinkNetworkBinaryLoop15Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop15(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut identity: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop15Result {
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
            self::MlTranscodeLinkNetworkBinaryLoop15Result::MlTranscodeLinkNetworkBinaryLoop15Done(
                output,
                output_length,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop13Result {
    MlTranscodeLinkNetworkBinaryLoop13Done(Vec<self::ml_string::Text>, f64, f64),
    MlTranscodeLinkNetworkBinaryLoop13Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop17Result {
    MlTranscodeLinkNetworkBinaryLoop17Done(Vec<self::ml_string::Text>, f64, f64),
    MlTranscodeLinkNetworkBinaryLoop17Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop17(
    input: &[f64],
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut cursor: f64,
    mut length: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop17Result {
    loop {
        return if index < length {
            {
                let value = self::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if (value < 0f64) || (value > 255f64) {
                    self::MlTranscodeLinkNetworkBinaryLoop17Result::MlTranscodeLinkNetworkBinaryLoop17Return(((self::ml_string::Text::from_units(vec![45, 49, 54, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
            self::MlTranscodeLinkNetworkBinaryLoop17Result::MlTranscodeLinkNetworkBinaryLoop17Done(
                output,
                output_length,
                cursor,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin16 {
    MlTranscodeLinkNetworkBinaryJoin16Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop18Result {
    MlTranscodeLinkNetworkBinaryLoop18Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop18(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut length: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop18Result {
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
            self::MlTranscodeLinkNetworkBinaryLoop18Result::MlTranscodeLinkNetworkBinaryLoop18Done(
                output,
                output_length,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop13(
    input: &[f64],
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut cursor: f64,
    mut strings: f64,
    mut text: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop13Result {
    loop {
        return if text < strings {
            {
                let length = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if (length < 0f64)
                    || (length
                        > (((input.len() as f64) - cursor) - if binary { 8f64 } else { 1f64 }))
                {
                    self::MlTranscodeLinkNetworkBinaryLoop13Result::MlTranscodeLinkNetworkBinaryLoop13Return(((self::ml_string::Text::from_units(vec![45, 49, 53, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = cursor + if binary { 8f64 } else { 1f64 };
                        {
                            let ml_s20 = if binary {
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
                                            self::MlTranscodeLinkNetworkBinaryJoin16::MlTranscodeLinkNetworkBinaryJoin16Next(output_2, output_length_2)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index = 0f64;
                                    {
                                        let ml_s21 = self::ml_transcode_link_network_binary_loop18(
                                            binary,
                                            output,
                                            output_length,
                                            length,
                                            index,
                                        );
                                        match ml_s21 {
                                            self::MlTranscodeLinkNetworkBinaryLoop18Result::MlTranscodeLinkNetworkBinaryLoop18Done(output_3, output_length_3) => {
                                                self::MlTranscodeLinkNetworkBinaryJoin16::MlTranscodeLinkNetworkBinaryJoin16Next(output_3, output_length_3)
                                            }
                                        }
                                    }
                                }
                            };
                            match ml_s20 {
                                self::MlTranscodeLinkNetworkBinaryJoin16::MlTranscodeLinkNetworkBinaryJoin16Next(output_4, output_length_4) => {
                                    {
                                        let index_2 = 0f64;
                                        {
                                            let ml_s22 = self::ml_transcode_link_network_binary_loop17(input, binary, output_4, output_length_4, cursor_2, length, index_2);
                                            match ml_s22 {
                                                self::MlTranscodeLinkNetworkBinaryLoop17Result::MlTranscodeLinkNetworkBinaryLoop17Done(output_5, output_length_5, cursor_3) => {
                                                    {
                                                        let text_2 = text + 1f64;
                                                        {
                                                            (output, output_length, cursor, text) = (output_5, output_length_5, cursor_3, text_2);
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlTranscodeLinkNetworkBinaryLoop17Result::MlTranscodeLinkNetworkBinaryLoop17Return(ml_result17) => {
                                                    self::MlTranscodeLinkNetworkBinaryLoop13Result::MlTranscodeLinkNetworkBinaryLoop13Return(ml_result17)
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
            self::MlTranscodeLinkNetworkBinaryLoop13Result::MlTranscodeLinkNetworkBinaryLoop13Done(
                output,
                output_length,
                cursor,
            )
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
pub fn ml_transcode_link_network_binary_loop12(
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
) -> self::MlTranscodeLinkNetworkBinaryLoop12Result {
    loop {
        return if entry < entries {
            {
                let strings = if section == 0f64 { 1f64 } else { 3f64 };
                {
                    let text = 0f64;
                    {
                        let ml_s23 = self::ml_transcode_link_network_binary_loop13(
                            input,
                            binary,
                            output,
                            output_length,
                            cursor,
                            strings,
                            text,
                        );
                        match ml_s23 {
                            self::MlTranscodeLinkNetworkBinaryLoop13Result::MlTranscodeLinkNetworkBinaryLoop13Done(output_2, output_length_2, cursor_2) => {
                                {
                                    let ml_s25 = if section == 0f64 {
                                        {
                                            let identity = self::read_link_network_binary_integer(input, cursor_2, 8f64, binary);
                                            if identity < 1f64 {
                                                self::MlTranscodeLinkNetworkBinaryJoin14::MlTranscodeLinkNetworkBinaryJoin14Return(((self::ml_string::Text::from_units(vec![45, 50, 49, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_2)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                                                                            self::MlTranscodeLinkNetworkBinaryJoin14::MlTranscodeLinkNetworkBinaryJoin14Next(output_3, output_length_3, cursor_3)
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    let index = 0f64;
                                                                    {
                                                                        let ml_s26 = self::ml_transcode_link_network_binary_loop15(binary, output_2, output_length_2, identity, index);
                                                                        match ml_s26 {
                                                                            self::MlTranscodeLinkNetworkBinaryLoop15Result::MlTranscodeLinkNetworkBinaryLoop15Done(output_4, output_length_4) => {
                                                                                self::MlTranscodeLinkNetworkBinaryJoin14::MlTranscodeLinkNetworkBinaryJoin14Next(output_4, output_length_4, cursor_3)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlTranscodeLinkNetworkBinaryJoin14::MlTranscodeLinkNetworkBinaryJoin14Return(((self::ml_string::Text::from_units(vec![45, 50, 49, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_2)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTranscodeLinkNetworkBinaryJoin14::MlTranscodeLinkNetworkBinaryJoin14Next(output_2, output_length_2, cursor_2)
                                    };
                                    match ml_s25 {
                                        self::MlTranscodeLinkNetworkBinaryJoin14::MlTranscodeLinkNetworkBinaryJoin14Next(output_5, output_length_5, cursor_4) => {
                                            if output_length_5 > (byte_limit * if binary {
                                                20f64
                                            } else {
                                                2f64
                                            }) {
                                                self::MlTranscodeLinkNetworkBinaryLoop12Result::MlTranscodeLinkNetworkBinaryLoop12Return(((self::ml_string::Text::from_units(vec![45, 50, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_4)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                                        self::MlTranscodeLinkNetworkBinaryJoin14::MlTranscodeLinkNetworkBinaryJoin14Return(ml_result14) => {
                                            self::MlTranscodeLinkNetworkBinaryLoop12Result::MlTranscodeLinkNetworkBinaryLoop12Return(ml_result14)
                                        }
                                    }
                                }
                            }
                            self::MlTranscodeLinkNetworkBinaryLoop13Result::MlTranscodeLinkNetworkBinaryLoop13Return(ml_result15) => {
                                self::MlTranscodeLinkNetworkBinaryLoop12Result::MlTranscodeLinkNetworkBinaryLoop12Return(ml_result15)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTranscodeLinkNetworkBinaryLoop12Result::MlTranscodeLinkNetworkBinaryLoop12Done(
                output,
                output_length,
                cursor,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin11 {
    MlTranscodeLinkNetworkBinaryJoin11Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop19Result {
    MlTranscodeLinkNetworkBinaryLoop19Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop19(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut entries: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop19Result {
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
            self::MlTranscodeLinkNetworkBinaryLoop19Result::MlTranscodeLinkNetworkBinaryLoop19Done(
                output,
                output_length,
            )
        };
    }
}
