// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlTemporaryTranscodeLinkNetworkBinaryJoin11;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop10Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop12Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop19Result;
use super::link_network_binary_integer_byte;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_link_network_binary_integer;
use super::render_link_network_binary_field;
use super::source_ml_temporary_transcode_link_network_binary_loop12;
use super::source_ml_temporary_transcode_link_network_binary_loop19;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_transcode_link_network_binary_loop10(
    input: &[f64],
    mut binary: bool,
    mut record_limit: f64,
    mut byte_limit: f64,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut cursor: f64,
    identities: &[f64],
    mut greatest_identity: f64,
    mut section: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop10Result {
    loop {
        return if section < 2f64 {
            {
                let entries = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if ((entries < 0f64) || (entries > record_limit))
                    || (entries > (input.len() as f64))
                {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop10Result::MlTemporaryTranscodeLinkNetworkBinaryLoop10Return(((self::ml_string::Text::from_units(vec![45, 50, 48, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = cursor + if binary { 8f64 } else { 1f64 };
                        {
                            let source_ml_source1_s28 = if binary {
                                {
                                    let encoded =
                                        self::render_link_network_binary_field(entries, binary);
                                    {
                                        let output_2 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![encoded.clone()], output);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let output_length_2 = output_length + encoded.length();
                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin11::MlTemporaryTranscodeLinkNetworkBinaryJoin11Next(output_2, output_length_2)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index = 0f64;
                                    {
                                        let source_ml_source1_s29 = self::source_ml_temporary_transcode_link_network_binary_loop19(binary, output, output_length, entries, index);
                                        match source_ml_source1_s29 {
                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop19Result::MlTemporaryTranscodeLinkNetworkBinaryLoop19Done(output_3, output_length_3) => {
                                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin11::MlTemporaryTranscodeLinkNetworkBinaryJoin11Next(output_3, output_length_3)
                                            }
                                        }
                                    }
                                }
                            };
                            match source_ml_source1_s28 {
                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin11::MlTemporaryTranscodeLinkNetworkBinaryJoin11Next(output_4, output_length_4) => {
                                    {
                                        let entry = 0f64;
                                        {
                                            let source_ml_source1_s30 = self::source_ml_temporary_transcode_link_network_binary_loop12(input, binary, byte_limit, output_4, output_length_4, cursor_2, identities, greatest_identity, section, entries, entry);
                                            match source_ml_source1_s30 {
                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop12Result::MlTemporaryTranscodeLinkNetworkBinaryLoop12Done(output_5, output_length_5, cursor_3) => {
                                                    {
                                                        let section_2 = section + 1f64;
                                                        {
                                                            (output, output_length, cursor, section) = (output_5, output_length_5, cursor_3, section_2);
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop12Result::MlTemporaryTranscodeLinkNetworkBinaryLoop12Return(source_ml_temporary_result12) => {
                                                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop10Result::MlTemporaryTranscodeLinkNetworkBinaryLoop10Return(source_ml_temporary_result12)
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop10Result::MlTemporaryTranscodeLinkNetworkBinaryLoop10Done(output, output_length, cursor)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop8Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop8Done(
        Vec<self::ml_string::Text>,
        f64,
        f64,
        Vec<f64>,
        f64,
    ),
    MlTemporaryTranscodeLinkNetworkBinaryLoop8Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop28(
    mut length: f64,
    mut encoded_length: self::ml_string::Text,
    mut index: f64,
) -> self::ml_string::Text {
    loop {
        if index < 8f64 {
            {
                let encoded_length_2 =
                    encoded_length.concat(&self::render_link_network_binary_field(
                        self::link_network_binary_integer_byte(length, index),
                        false,
                    ));
                {
                    let index_2 = index + 1f64;
                    {
                        (encoded_length, index) = (encoded_length_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { encoded_length };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin27 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin27Next(f64, Vec<self::ml_string::Text>, f64),
    MlTemporaryTranscodeLinkNetworkBinaryJoin27Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop29Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop29Done(f64, Vec<self::ml_string::Text>, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop29Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop30Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop30Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop30(
    mut binary: bool,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut value: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop30Result {
    loop {
        if index < 8f64 {
            {
                let encoded = self::render_link_network_binary_field(
                    self::link_network_binary_integer_byte(value, index),
                    binary,
                );
                {
                    let metadata_2 = {
                        let (ml_append_right, ml_append_left) = (vec![encoded.clone()], metadata);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let metadata_length_2 = metadata_length + encoded.length();
                        {
                            let index_2 = index + 1f64;
                            {
                                (metadata, metadata_length, index) =
                                    (metadata_2, metadata_length_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop30Result::MlTemporaryTranscodeLinkNetworkBinaryLoop30Done(metadata, metadata_length)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop29(
    input: &[f64],
    mut binary: bool,
    mut cursor: f64,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut field: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop29Result {
    loop {
        return if field < 6f64 {
            {
                let value = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if value < 0f64 {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop29Result::MlTemporaryTranscodeLinkNetworkBinaryLoop29Return(((self::ml_string::Text::from_units(vec![45, 49, 56, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = cursor + if binary { 8f64 } else { 1f64 };
                        if binary {
                            {
                                let encoded = self::render_link_network_binary_field(value, binary);
                                {
                                    let metadata_2 = {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![encoded.clone()], metadata);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let metadata_length_2 = metadata_length + encoded.length();
                                        {
                                            let field_2 = field + 1f64;
                                            {
                                                (cursor, metadata, metadata_length, field) = (
                                                    cursor_2,
                                                    metadata_2,
                                                    metadata_length_2,
                                                    field_2,
                                                );
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        {
                            {
                                let index = 0f64;
                                {
                                    let source_ml_source1_s31 = self::source_ml_temporary_transcode_link_network_binary_loop30(binary, metadata, metadata_length, value, index);
                                    match source_ml_source1_s31 {
                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop30Result::MlTemporaryTranscodeLinkNetworkBinaryLoop30Done(metadata_3, metadata_length_3) => {
                                            {
                                                let field_3 = field + 1f64;
                                                {
                                                    (cursor, metadata, metadata_length, field) = (cursor_2, metadata_3, metadata_length_3, field_3);
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
        } else {
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop29Result::MlTemporaryTranscodeLinkNetworkBinaryLoop29Done(cursor, metadata, metadata_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop26Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop26Done(f64, Vec<self::ml_string::Text>, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop26Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop32Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop32Done(f64, Vec<self::ml_string::Text>, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop32Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop32(
    input: &[f64],
    mut binary: bool,
    mut cursor: f64,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut length: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop32Result {
    loop {
        return if index < length {
            {
                let value = self::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if (value < 0f64) || (value > 255f64) {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop32Result::MlTemporaryTranscodeLinkNetworkBinaryLoop32Return(((self::ml_string::Text::from_units(vec![45, 49, 54, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = cursor + 1f64;
                        {
                            let encoded = self::render_link_network_binary_field(value, binary);
                            {
                                let metadata_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![encoded.clone()], metadata);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let metadata_length_2 = metadata_length + encoded.length();
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (cursor, metadata, metadata_length, index) =
                                                (cursor_2, metadata_2, metadata_length_2, index_2);
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop32Result::MlTemporaryTranscodeLinkNetworkBinaryLoop32Done(cursor, metadata, metadata_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin31 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin31Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop33Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop33Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop33(
    mut binary: bool,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut length: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop33Result {
    loop {
        if index < 8f64 {
            {
                let encoded = self::render_link_network_binary_field(
                    self::link_network_binary_integer_byte(length, index),
                    binary,
                );
                {
                    let metadata_2 = {
                        let (ml_append_right, ml_append_left) = (vec![encoded.clone()], metadata);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let metadata_length_2 = metadata_length + encoded.length();
                        {
                            let index_2 = index + 1f64;
                            {
                                (metadata, metadata_length, index) =
                                    (metadata_2, metadata_length_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop33Result::MlTemporaryTranscodeLinkNetworkBinaryLoop33Done(metadata, metadata_length)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop26(
    input: &[f64],
    mut binary: bool,
    mut cursor: f64,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut text: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop26Result {
    loop {
        return if text < 3f64 {
            {
                let present = self::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if (present != 0f64) && (present != 1f64) {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop26Result::MlTemporaryTranscodeLinkNetworkBinaryLoop26Return(((self::ml_string::Text::from_units(vec![45, 49, 52, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = cursor + 1f64;
                        {
                            let encoded = self::render_link_network_binary_field(present, binary);
                            {
                                let metadata_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![encoded.clone()], metadata);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let metadata_length_2 = metadata_length + encoded.length();
                                    if present == 1f64 {
                                        {
                                            let length = self::read_link_network_binary_integer(
                                                input, cursor_2, 8f64, binary,
                                            );
                                            if (length < 0f64)
                                                || (length
                                                    > (((input.len() as f64) - cursor_2)
                                                        - if binary { 8f64 } else { 1f64 }))
                                            {
                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop26Result::MlTemporaryTranscodeLinkNetworkBinaryLoop26Return(((self::ml_string::Text::from_units(vec![45, 49, 53, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_2)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                            } else {
                                                {
                                                    let cursor_3 =
                                                        cursor_2 + if binary { 8f64 } else { 1f64 };
                                                    {
                                                        let source_ml_source1_s33 = if binary {
                                                            {
                                                                let encoded_2 = self::render_link_network_binary_field(length, binary);
                                                                {
                                                                    let metadata_3 = {
                                                                        let (
                                                                            ml_append_right,
                                                                            ml_append_left,
                                                                        ) = (
                                                                            vec![encoded_2.clone()],
                                                                            metadata_2,
                                                                        );
                                                                        self::ml_array::append(
                                                                            ml_append_left,
                                                                            ml_append_right,
                                                                        )
                                                                    };
                                                                    {
                                                                        let metadata_length_3 =
                                                                            metadata_length_2
                                                                                + encoded_2
                                                                                    .length();
                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin31::MlTemporaryTranscodeLinkNetworkBinaryJoin31Next(metadata_3, metadata_length_3)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let index = 0f64;
                                                                {
                                                                    let source_ml_source1_s34 = self::source_ml_temporary_transcode_link_network_binary_loop33(binary, metadata_2, metadata_length_2, length, index);
                                                                    match source_ml_source1_s34 {
                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop33Result::MlTemporaryTranscodeLinkNetworkBinaryLoop33Done(metadata_4, metadata_length_4) => {
                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin31::MlTemporaryTranscodeLinkNetworkBinaryJoin31Next(metadata_4, metadata_length_4)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        match source_ml_source1_s33 {
                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin31::MlTemporaryTranscodeLinkNetworkBinaryJoin31Next(metadata_5, metadata_length_5) => {
                                                                {
                                                                    let index_2 = 0f64;
                                                                    {
                                                                        let source_ml_source1_s35 = self::source_ml_temporary_transcode_link_network_binary_loop32(input, binary, cursor_3, metadata_5, metadata_length_5, length, index_2);
                                                                        match source_ml_source1_s35 {
                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop32Result::MlTemporaryTranscodeLinkNetworkBinaryLoop32Done(cursor_4, metadata_6, metadata_length_6) => {
                                                                                {
                                                                                    let text_2 = text + 1f64;
                                                                                    {
                                                                                        (cursor, metadata, metadata_length, text) = (cursor_4, metadata_6, metadata_length_6, text_2);
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop32Result::MlTemporaryTranscodeLinkNetworkBinaryLoop32Return(source_ml_temporary_result32) => {
                                                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop26Result::MlTemporaryTranscodeLinkNetworkBinaryLoop26Return(source_ml_temporary_result32)
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
                                        {
                                            let text_3 = text + 1f64;
                                            {
                                                (cursor, metadata, metadata_length, text) = (
                                                    cursor_2,
                                                    metadata_2,
                                                    metadata_length_2,
                                                    text_3,
                                                );
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
        } else {
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop26Result::MlTemporaryTranscodeLinkNetworkBinaryLoop26Done(cursor, metadata, metadata_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop25Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop25Done(f64, Vec<self::ml_string::Text>, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop25Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop25(
    input: &[f64],
    mut binary: bool,
    mut cursor: f64,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut field: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop25Result {
    loop {
        return if field < 6f64 {
            {
                let value = self::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if (value < 0f64) || (value > 255f64) {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop25Result::MlTemporaryTranscodeLinkNetworkBinaryLoop25Return(((self::ml_string::Text::from_units(vec![45, 49, 50, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else if (((((field == 0f64) && (value != 1f64))
                    || ((field == 1f64) && (value > 15f64)))
                    || (((field == 2f64) || (field == 4f64)) && (value > 1f64)))
                    || ((field == 5f64) && (value != 0f64)))
                    || ((field == 3f64)
                        && (((value > 15f64)
                            || (((value % 2f64) == 1f64)
                                && (((value / 2f64).floor() % 2f64) != 1f64)))
                            || ((((value / 4f64).floor() % 2f64) == 1f64)
                                && (((value / 2f64).floor() % 2f64) != 1f64))))
                {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop25Result::MlTemporaryTranscodeLinkNetworkBinaryLoop25Return(((self::ml_string::Text::from_units(vec![45, 49, 51, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = cursor + 1f64;
                        {
                            let encoded = self::render_link_network_binary_field(value, binary);
                            {
                                let metadata_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![encoded.clone()], metadata);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let metadata_length_2 = metadata_length + encoded.length();
                                    {
                                        let field_2 = field + 1f64;
                                        {
                                            (cursor, metadata, metadata_length, field) =
                                                (cursor_2, metadata_2, metadata_length_2, field_2);
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop25Result::MlTemporaryTranscodeLinkNetworkBinaryLoop25Done(cursor, metadata, metadata_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin24 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin24Next(f64, f64, Vec<self::ml_string::Text>, f64),
    MlTemporaryTranscodeLinkNetworkBinaryJoin24Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop34Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop34Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop34(
    mut binary: bool,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop34Result {
    loop {
        if index < 8f64 {
            {
                let encoded = self::render_link_network_binary_field(0f64, binary);
                {
                    let metadata_2 = {
                        let (ml_append_right, ml_append_left) = (vec![encoded.clone()], metadata);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let metadata_length_2 = metadata_length + encoded.length();
                        {
                            let index_2 = index + 1f64;
                            {
                                (metadata, metadata_length, index) =
                                    (metadata_2, metadata_length_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop34Result::MlTemporaryTranscodeLinkNetworkBinaryLoop34Done(metadata, metadata_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop23Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop23Done(Vec<self::ml_string::Text>, f64, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop23Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop35Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop35Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop35(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut target: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop35Result {
    loop {
        if index < 8f64 {
            {
                let encoded = self::render_link_network_binary_field(
                    self::link_network_binary_integer_byte(target, index),
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop35Result::MlTemporaryTranscodeLinkNetworkBinaryLoop35Done(output, output_length)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop23(
    input: &[f64],
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut cursor: f64,
    mut references: f64,
    mut reference: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop23Result {
    loop {
        return if reference < references {
            {
                let target = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if target < 1f64 {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop23Result::MlTemporaryTranscodeLinkNetworkBinaryLoop23Return(((self::ml_string::Text::from_units(vec![45, 49, 48, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = cursor + if binary { 8f64 } else { 1f64 };
                        if binary {
                            {
                                let encoded =
                                    self::render_link_network_binary_field(target, binary);
                                {
                                    let output_2 = {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![encoded.clone()], output);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let output_length_2 = output_length + encoded.length();
                                        {
                                            let reference_2 = reference + 1f64;
                                            {
                                                (output, output_length, cursor, reference) = (
                                                    output_2,
                                                    output_length_2,
                                                    cursor_2,
                                                    reference_2,
                                                );
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        {
                            {
                                let index = 0f64;
                                {
                                    let source_ml_source1_s36 = self::source_ml_temporary_transcode_link_network_binary_loop35(binary, output, output_length, target, index);
                                    match source_ml_source1_s36 {
                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop35Result::MlTemporaryTranscodeLinkNetworkBinaryLoop35Done(output_3, output_length_3) => {
                                            {
                                                let reference_3 = reference + 1f64;
                                                {
                                                    (output, output_length, cursor, reference) = (output_3, output_length_3, cursor_2, reference_3);
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
        } else {
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop23Result::MlTemporaryTranscodeLinkNetworkBinaryLoop23Done(output, output_length, cursor)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin22 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin22Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop36Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop36Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop36(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut references: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop36Result {
    loop {
        if index < 8f64 {
            {
                let encoded = self::render_link_network_binary_field(
                    self::link_network_binary_integer_byte(references, index),
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop36Result::MlTemporaryTranscodeLinkNetworkBinaryLoop36Done(output, output_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin21 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin21Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop37Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop37Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop37(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut identity: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop37Result {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop37Result::MlTemporaryTranscodeLinkNetworkBinaryLoop37Done(output, output_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin20 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin20Next(f64),
    MlTemporaryTranscodeLinkNetworkBinaryJoin20Return(self::ml_string::Text),
}
