// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlTranscodeLinkNetworkBinaryJoin11;
use super::MlTranscodeLinkNetworkBinaryLoop10Result;
use super::MlTranscodeLinkNetworkBinaryLoop12Result;
use super::MlTranscodeLinkNetworkBinaryLoop19Result;
use super::link_network_binary_integer_byte;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::ml_transcode_link_network_binary_loop12;
use super::ml_transcode_link_network_binary_loop19;
use super::read_link_network_binary_integer;
use super::render_link_network_binary_field;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_transcode_link_network_binary_loop10(
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
) -> self::MlTranscodeLinkNetworkBinaryLoop10Result {
    loop {
        return if section < 2f64 {
            {
                let entries = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if ((entries < 0f64) || (entries > record_limit))
                    || (entries > (input.len() as f64))
                {
                    self::MlTranscodeLinkNetworkBinaryLoop10Result::MlTranscodeLinkNetworkBinaryLoop10Return(((self::ml_string::Text::from_units(vec![45, 50, 48, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = cursor + if binary { 8f64 } else { 1f64 };
                        {
                            let ml_s28 = if binary {
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
                                            self::MlTranscodeLinkNetworkBinaryJoin11::MlTranscodeLinkNetworkBinaryJoin11Next(output_2, output_length_2)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index = 0f64;
                                    {
                                        let ml_s29 = self::ml_transcode_link_network_binary_loop19(
                                            binary,
                                            output,
                                            output_length,
                                            entries,
                                            index,
                                        );
                                        match ml_s29 {
                                            self::MlTranscodeLinkNetworkBinaryLoop19Result::MlTranscodeLinkNetworkBinaryLoop19Done(output_3, output_length_3) => {
                                                self::MlTranscodeLinkNetworkBinaryJoin11::MlTranscodeLinkNetworkBinaryJoin11Next(output_3, output_length_3)
                                            }
                                        }
                                    }
                                }
                            };
                            match ml_s28 {
                                self::MlTranscodeLinkNetworkBinaryJoin11::MlTranscodeLinkNetworkBinaryJoin11Next(output_4, output_length_4) => {
                                    {
                                        let entry = 0f64;
                                        {
                                            let ml_s30 = self::ml_transcode_link_network_binary_loop12(input, binary, byte_limit, output_4, output_length_4, cursor_2, identities, greatest_identity, section, entries, entry);
                                            match ml_s30 {
                                                self::MlTranscodeLinkNetworkBinaryLoop12Result::MlTranscodeLinkNetworkBinaryLoop12Done(output_5, output_length_5, cursor_3) => {
                                                    {
                                                        let section_2 = section + 1f64;
                                                        {
                                                            (output, output_length, cursor, section) = (output_5, output_length_5, cursor_3, section_2);
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlTranscodeLinkNetworkBinaryLoop12Result::MlTranscodeLinkNetworkBinaryLoop12Return(ml_result12) => {
                                                    self::MlTranscodeLinkNetworkBinaryLoop10Result::MlTranscodeLinkNetworkBinaryLoop10Return(ml_result12)
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
            self::MlTranscodeLinkNetworkBinaryLoop10Result::MlTranscodeLinkNetworkBinaryLoop10Done(
                output,
                output_length,
                cursor,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop8Result {
    MlTranscodeLinkNetworkBinaryLoop8Done(Vec<self::ml_string::Text>, f64, f64, Vec<f64>, f64),
    MlTranscodeLinkNetworkBinaryLoop8Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop28(
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
pub enum MlTranscodeLinkNetworkBinaryJoin27 {
    MlTranscodeLinkNetworkBinaryJoin27Next(f64, Vec<self::ml_string::Text>, f64),
    MlTranscodeLinkNetworkBinaryJoin27Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop29Result {
    MlTranscodeLinkNetworkBinaryLoop29Done(f64, Vec<self::ml_string::Text>, f64),
    MlTranscodeLinkNetworkBinaryLoop29Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop30Result {
    MlTranscodeLinkNetworkBinaryLoop30Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop30(
    mut binary: bool,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut value: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop30Result {
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
            self::MlTranscodeLinkNetworkBinaryLoop30Result::MlTranscodeLinkNetworkBinaryLoop30Done(
                metadata,
                metadata_length,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop29(
    input: &[f64],
    mut binary: bool,
    mut cursor: f64,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut field: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop29Result {
    loop {
        return if field < 6f64 {
            {
                let value = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if value < 0f64 {
                    self::MlTranscodeLinkNetworkBinaryLoop29Result::MlTranscodeLinkNetworkBinaryLoop29Return(((self::ml_string::Text::from_units(vec![45, 49, 56, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                                    let ml_s31 = self::ml_transcode_link_network_binary_loop30(
                                        binary,
                                        metadata,
                                        metadata_length,
                                        value,
                                        index,
                                    );
                                    match ml_s31 {
                                        self::MlTranscodeLinkNetworkBinaryLoop30Result::MlTranscodeLinkNetworkBinaryLoop30Done(metadata_3, metadata_length_3) => {
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
            self::MlTranscodeLinkNetworkBinaryLoop29Result::MlTranscodeLinkNetworkBinaryLoop29Done(
                cursor,
                metadata,
                metadata_length,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop26Result {
    MlTranscodeLinkNetworkBinaryLoop26Done(f64, Vec<self::ml_string::Text>, f64),
    MlTranscodeLinkNetworkBinaryLoop26Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop32Result {
    MlTranscodeLinkNetworkBinaryLoop32Done(f64, Vec<self::ml_string::Text>, f64),
    MlTranscodeLinkNetworkBinaryLoop32Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop32(
    input: &[f64],
    mut binary: bool,
    mut cursor: f64,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut length: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop32Result {
    loop {
        return if index < length {
            {
                let value = self::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if (value < 0f64) || (value > 255f64) {
                    self::MlTranscodeLinkNetworkBinaryLoop32Result::MlTranscodeLinkNetworkBinaryLoop32Return(((self::ml_string::Text::from_units(vec![45, 49, 54, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
            self::MlTranscodeLinkNetworkBinaryLoop32Result::MlTranscodeLinkNetworkBinaryLoop32Done(
                cursor,
                metadata,
                metadata_length,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin31 {
    MlTranscodeLinkNetworkBinaryJoin31Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop33Result {
    MlTranscodeLinkNetworkBinaryLoop33Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop33(
    mut binary: bool,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut length: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop33Result {
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
            self::MlTranscodeLinkNetworkBinaryLoop33Result::MlTranscodeLinkNetworkBinaryLoop33Done(
                metadata,
                metadata_length,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop26(
    input: &[f64],
    mut binary: bool,
    mut cursor: f64,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut text: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop26Result {
    loop {
        return if text < 3f64 {
            {
                let present = self::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if (present != 0f64) && (present != 1f64) {
                    self::MlTranscodeLinkNetworkBinaryLoop26Result::MlTranscodeLinkNetworkBinaryLoop26Return(((self::ml_string::Text::from_units(vec![45, 49, 52, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                                                self::MlTranscodeLinkNetworkBinaryLoop26Result::MlTranscodeLinkNetworkBinaryLoop26Return(((self::ml_string::Text::from_units(vec![45, 49, 53, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_2)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                            } else {
                                                {
                                                    let cursor_3 =
                                                        cursor_2 + if binary { 8f64 } else { 1f64 };
                                                    {
                                                        let ml_s33 = if binary {
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
                                                                        self::MlTranscodeLinkNetworkBinaryJoin31::MlTranscodeLinkNetworkBinaryJoin31Next(metadata_3, metadata_length_3)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let index = 0f64;
                                                                {
                                                                    let ml_s34 = self::ml_transcode_link_network_binary_loop33(binary, metadata_2, metadata_length_2, length, index);
                                                                    match ml_s34 {
                                                                        self::MlTranscodeLinkNetworkBinaryLoop33Result::MlTranscodeLinkNetworkBinaryLoop33Done(metadata_4, metadata_length_4) => {
                                                                            self::MlTranscodeLinkNetworkBinaryJoin31::MlTranscodeLinkNetworkBinaryJoin31Next(metadata_4, metadata_length_4)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        match ml_s33 {
                                                            self::MlTranscodeLinkNetworkBinaryJoin31::MlTranscodeLinkNetworkBinaryJoin31Next(metadata_5, metadata_length_5) => {
                                                                {
                                                                    let index_2 = 0f64;
                                                                    {
                                                                        let ml_s35 = self::ml_transcode_link_network_binary_loop32(input, binary, cursor_3, metadata_5, metadata_length_5, length, index_2);
                                                                        match ml_s35 {
                                                                            self::MlTranscodeLinkNetworkBinaryLoop32Result::MlTranscodeLinkNetworkBinaryLoop32Done(cursor_4, metadata_6, metadata_length_6) => {
                                                                                {
                                                                                    let text_2 = text + 1f64;
                                                                                    {
                                                                                        (cursor, metadata, metadata_length, text) = (cursor_4, metadata_6, metadata_length_6, text_2);
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlTranscodeLinkNetworkBinaryLoop32Result::MlTranscodeLinkNetworkBinaryLoop32Return(ml_result32) => {
                                                                                self::MlTranscodeLinkNetworkBinaryLoop26Result::MlTranscodeLinkNetworkBinaryLoop26Return(ml_result32)
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
            self::MlTranscodeLinkNetworkBinaryLoop26Result::MlTranscodeLinkNetworkBinaryLoop26Done(
                cursor,
                metadata,
                metadata_length,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop25Result {
    MlTranscodeLinkNetworkBinaryLoop25Done(f64, Vec<self::ml_string::Text>, f64),
    MlTranscodeLinkNetworkBinaryLoop25Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop25(
    input: &[f64],
    mut binary: bool,
    mut cursor: f64,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut field: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop25Result {
    loop {
        return if field < 6f64 {
            {
                let value = self::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if (value < 0f64) || (value > 255f64) {
                    self::MlTranscodeLinkNetworkBinaryLoop25Result::MlTranscodeLinkNetworkBinaryLoop25Return(((self::ml_string::Text::from_units(vec![45, 49, 50, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                    self::MlTranscodeLinkNetworkBinaryLoop25Result::MlTranscodeLinkNetworkBinaryLoop25Return(((self::ml_string::Text::from_units(vec![45, 49, 51, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
            self::MlTranscodeLinkNetworkBinaryLoop25Result::MlTranscodeLinkNetworkBinaryLoop25Done(
                cursor,
                metadata,
                metadata_length,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin24 {
    MlTranscodeLinkNetworkBinaryJoin24Next(f64, f64, Vec<self::ml_string::Text>, f64),
    MlTranscodeLinkNetworkBinaryJoin24Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop34Result {
    MlTranscodeLinkNetworkBinaryLoop34Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop34(
    mut binary: bool,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop34Result {
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
            self::MlTranscodeLinkNetworkBinaryLoop34Result::MlTranscodeLinkNetworkBinaryLoop34Done(
                metadata,
                metadata_length,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop23Result {
    MlTranscodeLinkNetworkBinaryLoop23Done(Vec<self::ml_string::Text>, f64, f64),
    MlTranscodeLinkNetworkBinaryLoop23Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop35Result {
    MlTranscodeLinkNetworkBinaryLoop35Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop35(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut target: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop35Result {
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
            self::MlTranscodeLinkNetworkBinaryLoop35Result::MlTranscodeLinkNetworkBinaryLoop35Done(
                output,
                output_length,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop23(
    input: &[f64],
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut cursor: f64,
    mut references: f64,
    mut reference: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop23Result {
    loop {
        return if reference < references {
            {
                let target = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if target < 1f64 {
                    self::MlTranscodeLinkNetworkBinaryLoop23Result::MlTranscodeLinkNetworkBinaryLoop23Return(((self::ml_string::Text::from_units(vec![45, 49, 48, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                                    let ml_s36 = self::ml_transcode_link_network_binary_loop35(
                                        binary,
                                        output,
                                        output_length,
                                        target,
                                        index,
                                    );
                                    match ml_s36 {
                                        self::MlTranscodeLinkNetworkBinaryLoop35Result::MlTranscodeLinkNetworkBinaryLoop35Done(output_3, output_length_3) => {
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
            self::MlTranscodeLinkNetworkBinaryLoop23Result::MlTranscodeLinkNetworkBinaryLoop23Done(
                output,
                output_length,
                cursor,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin22 {
    MlTranscodeLinkNetworkBinaryJoin22Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop36Result {
    MlTranscodeLinkNetworkBinaryLoop36Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_transcode_link_network_binary_loop36(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut references: f64,
    mut index: f64,
) -> self::MlTranscodeLinkNetworkBinaryLoop36Result {
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
            self::MlTranscodeLinkNetworkBinaryLoop36Result::MlTranscodeLinkNetworkBinaryLoop36Done(
                output,
                output_length,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryJoin21 {
    MlTranscodeLinkNetworkBinaryJoin21Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTranscodeLinkNetworkBinaryLoop37Result {
    MlTranscodeLinkNetworkBinaryLoop37Done(Vec<self::ml_string::Text>, f64),
}
