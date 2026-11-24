// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlTemporaryTranscodeLinkNetworkBinaryJoin12;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop11Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop13Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop20Result;
use super::link_network_binary_integer_byte;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_link_network_binary_integer;
use super::render_link_network_binary_field;
use super::source_ml_temporary_transcode_link_network_binary_loop13;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop20(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut entries: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop20Result {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop20Result::MlTemporaryTranscodeLinkNetworkBinaryLoop20Done(output, output_length)
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
pub fn source_ml_temporary_transcode_link_network_binary_loop11(
    input: &[f64],
    mut binary: bool,
    mut record_limit: f64,
    mut byte_limit: f64,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut cursor: f64,
    identities: &[f64],
    mut greatest_identity: f64,
    mut ordered_identities: bool,
    mut section: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop11Result {
    loop {
        return if section < 2f64 {
            {
                let entries = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if ((entries < 0f64) || (entries > record_limit))
                    || (entries > (input.len() as f64))
                {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop11Result::MlTemporaryTranscodeLinkNetworkBinaryLoop11Return(((self::ml_string::Text::from_units(vec![45, 50, 48, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let cursor_2 = cursor + if binary { 8f64 } else { 1f64 };
                        {
                            let source_ml_source1_s29 = if binary {
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
                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin12::MlTemporaryTranscodeLinkNetworkBinaryJoin12Next(output_2, output_length_2)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index = 0f64;
                                    {
                                        let source_ml_source1_s30 = self::source_ml_temporary_transcode_link_network_binary_loop20(binary, output, output_length, entries, index);
                                        match source_ml_source1_s30 {
                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop20Result::MlTemporaryTranscodeLinkNetworkBinaryLoop20Done(output_3, output_length_3) => {
                                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin12::MlTemporaryTranscodeLinkNetworkBinaryJoin12Next(output_3, output_length_3)
                                            }
                                        }
                                    }
                                }
                            };
                            match source_ml_source1_s29 {
                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin12::MlTemporaryTranscodeLinkNetworkBinaryJoin12Next(output_4, output_length_4) => {
                                    {
                                        let entry = 0f64;
                                        {
                                            let source_ml_source1_s31 = self::source_ml_temporary_transcode_link_network_binary_loop13(input, binary, byte_limit, output_4, output_length_4, cursor_2, identities, greatest_identity, ordered_identities, section, entries, entry);
                                            match source_ml_source1_s31 {
                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop13Result::MlTemporaryTranscodeLinkNetworkBinaryLoop13Done(output_5, output_length_5, cursor_3) => {
                                                    {
                                                        let section_2 = section + 1f64;
                                                        {
                                                            (output, output_length, cursor, section) = (output_5, output_length_5, cursor_3, section_2);
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop13Result::MlTemporaryTranscodeLinkNetworkBinaryLoop13Return(source_ml_temporary_result13) => {
                                                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop11Result::MlTemporaryTranscodeLinkNetworkBinaryLoop11Return(source_ml_temporary_result13)
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop11Result::MlTemporaryTranscodeLinkNetworkBinaryLoop11Done(output, output_length, cursor)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop9Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop9Done(
        Vec<self::ml_string::Text>,
        f64,
        f64,
        Vec<f64>,
        f64,
        bool,
    ),
    MlTemporaryTranscodeLinkNetworkBinaryLoop9Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop29(
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
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin28 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin28Next(f64, Vec<self::ml_string::Text>, f64),
    MlTemporaryTranscodeLinkNetworkBinaryJoin28Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop30Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop30Done(f64, Vec<self::ml_string::Text>, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop30Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop31Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop31Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop31(
    mut binary: bool,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut value: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop31Result {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop31Result::MlTemporaryTranscodeLinkNetworkBinaryLoop31Done(metadata, metadata_length)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop30(
    input: &[f64],
    mut binary: bool,
    mut cursor: f64,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut field: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop30Result {
    loop {
        return if field < 6f64 {
            {
                let value = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if value < 0f64 {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop30Result::MlTemporaryTranscodeLinkNetworkBinaryLoop30Return(((self::ml_string::Text::from_units(vec![45, 49, 56, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                                    let source_ml_source1_s32 = self::source_ml_temporary_transcode_link_network_binary_loop31(binary, metadata, metadata_length, value, index);
                                    match source_ml_source1_s32 {
                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop31Result::MlTemporaryTranscodeLinkNetworkBinaryLoop31Done(metadata_3, metadata_length_3) => {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop30Result::MlTemporaryTranscodeLinkNetworkBinaryLoop30Done(cursor, metadata, metadata_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop27Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop27Done(f64, Vec<self::ml_string::Text>, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop27Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop33Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop33Done(f64, Vec<self::ml_string::Text>, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop33Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop33(
    input: &[f64],
    mut binary: bool,
    mut cursor: f64,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut length: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop33Result {
    loop {
        return if index < length {
            {
                let value = self::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if (value < 0f64) || (value > 255f64) {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop33Result::MlTemporaryTranscodeLinkNetworkBinaryLoop33Return(((self::ml_string::Text::from_units(vec![45, 49, 54, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop33Result::MlTemporaryTranscodeLinkNetworkBinaryLoop33Done(cursor, metadata, metadata_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin32 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin32Next(Vec<self::ml_string::Text>, f64),
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
    mut length: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop34Result {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop34Result::MlTemporaryTranscodeLinkNetworkBinaryLoop34Done(metadata, metadata_length)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop27(
    input: &[f64],
    mut binary: bool,
    mut cursor: f64,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut text: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop27Result {
    loop {
        return if text < 3f64 {
            {
                let present = self::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if (present != 0f64) && (present != 1f64) {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop27Result::MlTemporaryTranscodeLinkNetworkBinaryLoop27Return(((self::ml_string::Text::from_units(vec![45, 49, 52, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop27Result::MlTemporaryTranscodeLinkNetworkBinaryLoop27Return(((self::ml_string::Text::from_units(vec![45, 49, 53, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_2)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                            } else {
                                                {
                                                    let cursor_3 =
                                                        cursor_2 + if binary { 8f64 } else { 1f64 };
                                                    {
                                                        let source_ml_source1_s34 = if binary {
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
                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin32::MlTemporaryTranscodeLinkNetworkBinaryJoin32Next(metadata_3, metadata_length_3)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let index = 0f64;
                                                                {
                                                                    let source_ml_source1_s35 = self::source_ml_temporary_transcode_link_network_binary_loop34(binary, metadata_2, metadata_length_2, length, index);
                                                                    match source_ml_source1_s35 {
                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop34Result::MlTemporaryTranscodeLinkNetworkBinaryLoop34Done(metadata_4, metadata_length_4) => {
                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin32::MlTemporaryTranscodeLinkNetworkBinaryJoin32Next(metadata_4, metadata_length_4)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        match source_ml_source1_s34 {
                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin32::MlTemporaryTranscodeLinkNetworkBinaryJoin32Next(metadata_5, metadata_length_5) => {
                                                                {
                                                                    let index_2 = 0f64;
                                                                    {
                                                                        let source_ml_source1_s36 = self::source_ml_temporary_transcode_link_network_binary_loop33(input, binary, cursor_3, metadata_5, metadata_length_5, length, index_2);
                                                                        match source_ml_source1_s36 {
                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop33Result::MlTemporaryTranscodeLinkNetworkBinaryLoop33Done(cursor_4, metadata_6, metadata_length_6) => {
                                                                                {
                                                                                    let text_2 = text + 1f64;
                                                                                    {
                                                                                        (cursor, metadata, metadata_length, text) = (cursor_4, metadata_6, metadata_length_6, text_2);
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop33Result::MlTemporaryTranscodeLinkNetworkBinaryLoop33Return(source_ml_temporary_result33) => {
                                                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop27Result::MlTemporaryTranscodeLinkNetworkBinaryLoop27Return(source_ml_temporary_result33)
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop27Result::MlTemporaryTranscodeLinkNetworkBinaryLoop27Done(cursor, metadata, metadata_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop26Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop26Done(f64, Vec<self::ml_string::Text>, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop26Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop26(
    input: &[f64],
    mut binary: bool,
    mut cursor: f64,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut field: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop26Result {
    loop {
        return if field < 6f64 {
            {
                let value = self::read_link_network_binary_integer(input, cursor, 1f64, binary);
                if (value < 0f64) || (value > 255f64) {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop26Result::MlTemporaryTranscodeLinkNetworkBinaryLoop26Return(((self::ml_string::Text::from_units(vec![45, 49, 50, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop26Result::MlTemporaryTranscodeLinkNetworkBinaryLoop26Return(((self::ml_string::Text::from_units(vec![45, 49, 51, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop26Result::MlTemporaryTranscodeLinkNetworkBinaryLoop26Done(cursor, metadata, metadata_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin25 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin25Next(f64, f64, Vec<self::ml_string::Text>, f64),
    MlTemporaryTranscodeLinkNetworkBinaryJoin25Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop35Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop35Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop35(
    mut binary: bool,
    mut metadata: Vec<self::ml_string::Text>,
    mut metadata_length: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop35Result {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop35Result::MlTemporaryTranscodeLinkNetworkBinaryLoop35Done(metadata, metadata_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop24Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop24Done(Vec<self::ml_string::Text>, f64, f64),
    MlTemporaryTranscodeLinkNetworkBinaryLoop24Return(self::ml_string::Text),
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
    mut target: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop36Result {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop36Result::MlTemporaryTranscodeLinkNetworkBinaryLoop36Done(output, output_length)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop24(
    input: &[f64],
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut cursor: f64,
    mut references: f64,
    mut reference: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop24Result {
    loop {
        return if reference < references {
            {
                let target = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if target < 1f64 {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop24Result::MlTemporaryTranscodeLinkNetworkBinaryLoop24Return(((self::ml_string::Text::from_units(vec![45, 49, 48, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                                    let source_ml_source1_s37 = self::source_ml_temporary_transcode_link_network_binary_loop36(binary, output, output_length, target, index);
                                    match source_ml_source1_s37 {
                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop36Result::MlTemporaryTranscodeLinkNetworkBinaryLoop36Done(output_3, output_length_3) => {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop24Result::MlTemporaryTranscodeLinkNetworkBinaryLoop24Done(output, output_length, cursor)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin23 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin23Next(Vec<self::ml_string::Text>, f64),
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
    mut references: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop37Result {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop37Result::MlTemporaryTranscodeLinkNetworkBinaryLoop37Done(output, output_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin22 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin22Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop38Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop38Done(Vec<self::ml_string::Text>, f64),
}
