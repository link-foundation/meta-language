// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlTemporaryTranscodeLinkNetworkBinaryJoin22;
use super::MlTemporaryTranscodeLinkNetworkBinaryJoin23;
use super::MlTemporaryTranscodeLinkNetworkBinaryJoin25;
use super::MlTemporaryTranscodeLinkNetworkBinaryJoin28;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop24Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop26Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop27Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop30Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop35Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop37Result;
use super::MlTemporaryTranscodeLinkNetworkBinaryLoop38Result;
use super::has_ordered_link_network_identity;
use super::link_network_binary_integer_byte;
use super::link_network_binary_unit;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_link_network_binary_integer;
use super::render_link_network_binary_field;
use super::source_ml_temporary_transcode_link_network_binary_loop24;
use super::source_ml_temporary_transcode_link_network_binary_loop26;
use super::source_ml_temporary_transcode_link_network_binary_loop27;
use super::source_ml_temporary_transcode_link_network_binary_loop29;
use super::source_ml_temporary_transcode_link_network_binary_loop30;
use super::source_ml_temporary_transcode_link_network_binary_loop35;
use super::source_ml_temporary_transcode_link_network_binary_loop37;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop38(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut identity: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop38Result {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop38Result::MlTemporaryTranscodeLinkNetworkBinaryLoop38Done(output, output_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin21 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin21Next(f64, bool),
    MlTemporaryTranscodeLinkNetworkBinaryJoin21Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_transcode_link_network_binary_loop9(
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
    mut ordered_identities: bool,
    mut record: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result {
    loop {
        return if record < count {
            {
                let identity = self::read_link_network_binary_integer(input, cursor, 8f64, binary);
                if ((identity < 1f64) || (identity >= 9_007_199_254_740_991f64))
                    || ((version == 2f64) && (identity >= next_identity))
                {
                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result::MlTemporaryTranscodeLinkNetworkBinaryLoop9Return(((self::ml_string::Text::from_units(vec![45, 55, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                } else {
                    {
                        let source_ml_source1_s38 = if identity > greatest_identity {
                            {
                                let greatest_identity_2 = identity;
                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin21::MlTemporaryTranscodeLinkNetworkBinaryJoin21Next(greatest_identity_2, ordered_identities)
                            }
                        } else {
                            {
                                let duplicate = if ordered_identities {
                                    self::has_ordered_link_network_identity(&(identities), identity)
                                } else {
                                    self::ml_array::includes(
                                        &identities,
                                        &identity,
                                        0f64,
                                        false,
                                        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                                    )
                                };
                                if duplicate {
                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin21::MlTemporaryTranscodeLinkNetworkBinaryJoin21Return(((self::ml_string::Text::from_units(vec![45, 56, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                } else {
                                    {
                                        let ordered_identities_2 = false;
                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin21::MlTemporaryTranscodeLinkNetworkBinaryJoin21Next(greatest_identity, ordered_identities_2)
                                    }
                                }
                            }
                        };
                        match source_ml_source1_s38 {
                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin21::MlTemporaryTranscodeLinkNetworkBinaryJoin21Next(greatest_identity_3, ordered_identities_3) => {
                                {
                                    let identities_2 = { let (ml_append_right, ml_append_left) = (vec![identity], identities); self::ml_array::append(ml_append_left, ml_append_right) };
                                    {
                                        let cursor_2 = cursor + if binary {
                                            8f64
                                        } else {
                                            1f64
                                        };
                                        {
                                            let source_ml_source1_s40 = if binary {
                                                {
                                                    let encoded = self::render_link_network_binary_field(identity, binary);
                                                    {
                                                        let output_2 = { let (ml_append_right, ml_append_left) = (vec![encoded.clone()], output); self::ml_array::append(ml_append_left, ml_append_right) };
                                                        {
                                                            let output_length_2 = output_length + encoded.length();
                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin22::MlTemporaryTranscodeLinkNetworkBinaryJoin22Next(output_2, output_length_2)
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let index = 0f64;
                                                    {
                                                        let source_ml_source1_s41 = self::source_ml_temporary_transcode_link_network_binary_loop38(binary, output, output_length, identity, index);
                                                        match source_ml_source1_s41 {
                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop38Result::MlTemporaryTranscodeLinkNetworkBinaryLoop38Done(output_3, output_length_3) => {
                                                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin22::MlTemporaryTranscodeLinkNetworkBinaryJoin22Next(output_3, output_length_3)
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            match source_ml_source1_s40 {
                                                self::MlTemporaryTranscodeLinkNetworkBinaryJoin22::MlTemporaryTranscodeLinkNetworkBinaryJoin22Next(output_4, output_length_4) => {
                                                    {
                                                        let references = self::read_link_network_binary_integer(input, cursor_2, 8f64, binary);
                                                        if (references < 0f64) || (references > (input.len() as f64)) {
                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result::MlTemporaryTranscodeLinkNetworkBinaryLoop9Return(((self::ml_string::Text::from_units(vec![45, 57, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_2)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                                        } else {
                                                            {
                                                                let cursor_3 = cursor_2 + if binary {
                                                                    8f64
                                                                } else {
                                                                    1f64
                                                                };
                                                                {
                                                                    let source_ml_source1_s43 = if binary {
                                                                        {
                                                                            let encoded_2 = self::render_link_network_binary_field(references, binary);
                                                                            {
                                                                                let output_5 = { let (ml_append_right, ml_append_left) = (vec![encoded_2.clone()], output_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                {
                                                                                    let output_length_5 = output_length_4 + encoded_2.length();
                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin23::MlTemporaryTranscodeLinkNetworkBinaryJoin23Next(output_5, output_length_5)
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let index_2 = 0f64;
                                                                            {
                                                                                let source_ml_source1_s44 = self::source_ml_temporary_transcode_link_network_binary_loop37(binary, output_4, output_length_4, references, index_2);
                                                                                match source_ml_source1_s44 {
                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop37Result::MlTemporaryTranscodeLinkNetworkBinaryLoop37Done(output_6, output_length_6) => {
                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin23::MlTemporaryTranscodeLinkNetworkBinaryJoin23Next(output_6, output_length_6)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    match source_ml_source1_s43 {
                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryJoin23::MlTemporaryTranscodeLinkNetworkBinaryJoin23Next(output_7, output_length_7) => {
                                                                            {
                                                                                let reference = 0f64;
                                                                                {
                                                                                    let source_ml_source1_s45 = self::source_ml_temporary_transcode_link_network_binary_loop24(input, binary, output_7, output_length_7, cursor_3, references, reference);
                                                                                    match source_ml_source1_s45 {
                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop24Result::MlTemporaryTranscodeLinkNetworkBinaryLoop24Done(output_8, output_length_8, cursor_4) => {
                                                                                            {
                                                                                                let metadata_end = 0f64;
                                                                                                {
                                                                                                    let metadata = Vec::<self::ml_string::Text>::new();
                                                                                                    {
                                                                                                        let metadata_length = 0f64;
                                                                                                        {
                                                                                                            let length_slot = 0f64;
                                                                                                            {
                                                                                                                let source_ml_source1_s47 = if binary {
                                                                                                                    {
                                                                                                                        let length = self::read_link_network_binary_integer(input, cursor_4, 8f64, true);
                                                                                                                        if (length < 6f64) || (length > (((input.len() as f64) - cursor_4) - 8f64)) {
                                                                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin25::MlTemporaryTranscodeLinkNetworkBinaryJoin25Return(((self::ml_string::Text::from_units(vec![45, 49, 49, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_4)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let cursor_5 = cursor_4 + 8f64;
                                                                                                                                {
                                                                                                                                    let metadata_end_2 = cursor_5 + length;
                                                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin25::MlTemporaryTranscodeLinkNetworkBinaryJoin25Next(cursor_5, metadata_end_2, metadata, metadata_length)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    {
                                                                                                                        let index_3 = 0f64;
                                                                                                                        {
                                                                                                                            let source_ml_source1_s48 = self::source_ml_temporary_transcode_link_network_binary_loop35(binary, metadata, metadata_length, index_3);
                                                                                                                            match source_ml_source1_s48 {
                                                                                                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop35Result::MlTemporaryTranscodeLinkNetworkBinaryLoop35Done(metadata_2, metadata_length_2) => {
                                                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin25::MlTemporaryTranscodeLinkNetworkBinaryJoin25Next(cursor_4, metadata_end, metadata_2, metadata_length_2)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                };
                                                                                                                match source_ml_source1_s47 {
                                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin25::MlTemporaryTranscodeLinkNetworkBinaryJoin25Next(cursor_6, metadata_end_3, metadata_3, metadata_length_3) => {
                                                                                                                        {
                                                                                                                            let metadata_start = metadata_length_3;
                                                                                                                            {
                                                                                                                                let field = 0f64;
                                                                                                                                {
                                                                                                                                    let source_ml_source1_s49 = self::source_ml_temporary_transcode_link_network_binary_loop26(input, binary, cursor_6, metadata_3, metadata_length_3, field);
                                                                                                                                    match source_ml_source1_s49 {
                                                                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop26Result::MlTemporaryTranscodeLinkNetworkBinaryLoop26Done(cursor_7, metadata_4, metadata_length_4) => {
                                                                                                                                            {
                                                                                                                                                let text = 0f64;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_s50 = self::source_ml_temporary_transcode_link_network_binary_loop27(input, binary, cursor_7, metadata_4, metadata_length_4, text);
                                                                                                                                                    match source_ml_source1_s50 {
                                                                                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop27Result::MlTemporaryTranscodeLinkNetworkBinaryLoop27Done(cursor_8, metadata_5, metadata_length_5) => {
                                                                                                                                                            {
                                                                                                                                                                let span = self::read_link_network_binary_integer(input, cursor_8, 1f64, binary);
                                                                                                                                                                if (span != 0f64) && (span != 1f64) {
                                                                                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result::MlTemporaryTranscodeLinkNetworkBinaryLoop9Return(((self::ml_string::Text::from_units(vec![45, 49, 55, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_8)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                                                                                                                                                                                        let source_ml_source1_s52 = if span == 1f64 {
                                                                                                                                                                                            {
                                                                                                                                                                                                let field_2 = 0f64;
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source1_s53 = self::source_ml_temporary_transcode_link_network_binary_loop30(input, binary, cursor_9, metadata_6, metadata_length_6, field_2);
                                                                                                                                                                                                    match source_ml_source1_s53 {
                                                                                                                                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop30Result::MlTemporaryTranscodeLinkNetworkBinaryLoop30Done(cursor_10, metadata_7, metadata_length_7) => {
                                                                                                                                                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin28::MlTemporaryTranscodeLinkNetworkBinaryJoin28Next(cursor_10, metadata_7, metadata_length_7)
                                                                                                                                                                                                        }
                                                                                                                                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop30Result::MlTemporaryTranscodeLinkNetworkBinaryLoop30Return(source_ml_temporary_result30) => {
                                                                                                                                                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin28::MlTemporaryTranscodeLinkNetworkBinaryJoin28Return(source_ml_temporary_result30.clone())
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        } else {
                                                                                                                                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin28::MlTemporaryTranscodeLinkNetworkBinaryJoin28Next(cursor_9, metadata_6, metadata_length_6)
                                                                                                                                                                                        };
                                                                                                                                                                                        match source_ml_source1_s52 {
                                                                                                                                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin28::MlTemporaryTranscodeLinkNetworkBinaryJoin28Next(cursor_11, metadata_8, metadata_length_8) => {
                                                                                                                                                                                                if binary && (cursor_11 != metadata_end_3) {
                                                                                                                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result::MlTemporaryTranscodeLinkNetworkBinaryLoop9Return(((self::ml_string::Text::from_units(vec![45, 49, 57, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_11)))).concat(&self::ml_string::Text::from_units(vec![44])))
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
                                                                                                                                                                                                                                    let encoded_length_2 = self::source_ml_temporary_transcode_link_network_binary_loop29(length_2, encoded_length, index_4);
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        (({
                                                                                                                                                                                                                                            let source_ml_source1_string_range_operand54 = record_output.clone();
                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                let source_ml_source1_string_range_operand55 = 0f64;
                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                    let source_ml_source1_string_range_operand56 = length_slot;
                                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                                        let source_ml_source1_string_range_position57 = source_ml_source1_string_range_operand55;
                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                            let source_ml_source1_string_range_position58 = source_ml_source1_string_range_operand56;
                                                                                                                                                                                                                                                            source_ml_source1_string_range_operand54.slice(source_ml_source1_string_range_position57, source_ml_source1_string_range_position58)
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                        }).concat(&encoded_length_2)).concat(&{
                                                                                                                                                                                                                                            let source_ml_source1_string_range_operand59 = record_output.clone();
                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                let source_ml_source1_string_range_operand60 = length_slot + 16f64;
                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                    let source_ml_source1_string_range_position61 = source_ml_source1_string_range_operand60;
                                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                                        let source_ml_source1_string_range_position62 = f64::INFINITY;
                                                                                                                                                                                                                                                        source_ml_source1_string_range_operand59.slice(source_ml_source1_string_range_position61, source_ml_source1_string_range_position62)
                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                        })
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
                                                                                                                                                                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result::MlTemporaryTranscodeLinkNetworkBinaryLoop9Return(((self::ml_string::Text::from_units(vec![45, 50, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(cursor_11)))).concat(&self::ml_string::Text::from_units(vec![44])))
                                                                                                                                                                                                                        } else {
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let record_2 = record + 1f64;
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    (output, output_length, cursor, identities, greatest_identity, ordered_identities, record) = (output_9, output_length_9, cursor_11, identities_2, greatest_identity_3, ordered_identities_3, record_2);
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
                                                                                                                                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin28::MlTemporaryTranscodeLinkNetworkBinaryJoin28Return(source_ml_temporary_result29) => {
                                                                                                                                                                                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result::MlTemporaryTranscodeLinkNetworkBinaryLoop9Return(source_ml_temporary_result29.clone())
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
                                                                                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop27Result::MlTemporaryTranscodeLinkNetworkBinaryLoop27Return(source_ml_temporary_result31) => {
                                                                                                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result::MlTemporaryTranscodeLinkNetworkBinaryLoop9Return(source_ml_temporary_result31.clone())
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop26Result::MlTemporaryTranscodeLinkNetworkBinaryLoop26Return(source_ml_temporary_result34) => {
                                                                                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result::MlTemporaryTranscodeLinkNetworkBinaryLoop9Return(source_ml_temporary_result34.clone())
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlTemporaryTranscodeLinkNetworkBinaryJoin25::MlTemporaryTranscodeLinkNetworkBinaryJoin25Return(source_ml_temporary_result34_2) => {
                                                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result::MlTemporaryTranscodeLinkNetworkBinaryLoop9Return(source_ml_temporary_result34_2.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::MlTemporaryTranscodeLinkNetworkBinaryLoop24Result::MlTemporaryTranscodeLinkNetworkBinaryLoop24Return(source_ml_temporary_result35) => {
                                                                                            self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result::MlTemporaryTranscodeLinkNetworkBinaryLoop9Return(source_ml_temporary_result35.clone())
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
                            self::MlTemporaryTranscodeLinkNetworkBinaryJoin21::MlTemporaryTranscodeLinkNetworkBinaryJoin21Return(source_ml_temporary_result38) => {
                                self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result::MlTemporaryTranscodeLinkNetworkBinaryLoop9Return(source_ml_temporary_result38.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop9Result::MlTemporaryTranscodeLinkNetworkBinaryLoop9Done(output, output_length, cursor, identities, greatest_identity, ordered_identities)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin8 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin8Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop39Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop39Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop39(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut count: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop39Result {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop39Result::MlTemporaryTranscodeLinkNetworkBinaryLoop39Done(output, output_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin7 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin7Next(Vec<self::ml_string::Text>, f64, f64, f64),
    MlTemporaryTranscodeLinkNetworkBinaryJoin7Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop40Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop40Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop40(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    mut next_identity: f64,
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop40Result {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop40Result::MlTemporaryTranscodeLinkNetworkBinaryLoop40Done(output, output_length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin6 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin6Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryJoin5 {
    MlTemporaryTranscodeLinkNetworkBinaryJoin5Next(Vec<self::ml_string::Text>, f64, f64, f64),
    MlTemporaryTranscodeLinkNetworkBinaryJoin5Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop41Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop41Done,
    MlTemporaryTranscodeLinkNetworkBinaryLoop41Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop41(
    input: &[f64],
    header: &[f64],
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop41Result {
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
                self::MlTemporaryTranscodeLinkNetworkBinaryLoop41Result::MlTemporaryTranscodeLinkNetworkBinaryLoop41Return(((self::ml_string::Text::from_units(vec![45, 51, 44])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(index)))).concat(&self::ml_string::Text::from_units(vec![44])))
            }
        } else {
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop41Result::MlTemporaryTranscodeLinkNetworkBinaryLoop41Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryTranscodeLinkNetworkBinaryLoop42Result {
    MlTemporaryTranscodeLinkNetworkBinaryLoop42Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_transcode_link_network_binary_loop42(
    mut binary: bool,
    mut output: Vec<self::ml_string::Text>,
    mut output_length: f64,
    header: &[f64],
    mut index: f64,
) -> self::MlTemporaryTranscodeLinkNetworkBinaryLoop42Result {
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
            self::MlTemporaryTranscodeLinkNetworkBinaryLoop42Result::MlTemporaryTranscodeLinkNetworkBinaryLoop42Done(output, output_length)
        };
    }
}
