// Generated from complete js/src/translation/rust-generic-type-alias-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18;
use super::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19;
use super::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21;
use super::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Result;
use super::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Result;
use super::accept_parameter_name;
use super::group_after;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::parameter_directory;
use super::read_required_boolean_fact;
use super::read_required_text_fact;
use super::read_rust_type_closing_token_parts;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_parameter_directory_loop22(
    names: &[self::ml_string::Text],
    starts: &[self::ml_string::Text],
    ends: &[self::ml_string::Text],
    offsets: &[self::ml_string::Text],
    mut at: f64,
    mut first: f64,
    mut angles: f64,
    closing: &[self::ml_string::Text],
    mut offset: f64,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Result {
    loop {
        return if (offset < (closing.len() as f64))
            && ((self::read_required_text_fact(closing, offset))
                == (self::ml_string::Text::from_units(vec![62])))
        {
            if angles > 0f64 {
                {
                    let angles_2 = angles - 1f64;
                    {
                        let offset_2 = offset + 1f64;
                        {
                            (angles, offset) = (angles_2, offset_2);
                            continue;
                        }
                    }
                }
            }
            {
                if ((first).partial_cmp(&(at)) == Some(std::cmp::Ordering::Equal))
                    && (offset == 0f64)
                {
                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![101, 109, 112, 116, 121, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 100, 101, 102, 97, 117, 108, 116, 32, 116, 121, 112, 101])])
                } else {
                    {
                        let starts_2 = {
                            let (ml_append_right, ml_append_left) = (
                                vec![{
                                    let source_ml_source2_string_argument88 = first;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        source_ml_source2_string_argument88,
                                    ))
                                }],
                                starts.to_vec(),
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let ends_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![{
                                        let source_ml_source2_string_argument89 = at;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source2_string_argument89,
                                            ),
                                        )
                                    }],
                                    ends.to_vec(),
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let offsets_2 = {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![{
                                            let source_ml_source2_string_argument90 = offset;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source2_string_argument90,
                                                ),
                                            )
                                        }],
                                        offsets.to_vec(),
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Return(self::parameter_directory(names, &(starts_2), &(ends_2), &(offsets_2), at, &(self::ml_string::join(&self::ml_array::slice(closing, (offset + 1f64), f64::INFINITY), &self::ml_string::Text::from_units(vec![])))))
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Done(starts.to_vec(), ends.to_vec(), offsets.to_vec(), angles)
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
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_parameter_directory_loop20(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    mut starts: Vec<self::ml_string::Text>,
    mut ends: Vec<self::ml_string::Text>,
    mut offsets: Vec<self::ml_string::Text>,
    mut at: f64,
    mut first: f64,
    mut angles: f64,
    mut found: bool,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Result {
    loop {
        return if at < (values.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, at);
                {
                    let value = self::read_required_text_fact(values, at);
                    if (kind == (self::ml_string::Text::from_units(vec![101, 111, 102])))
                        || ((kind
                            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                            && ((value == (self::ml_string::Text::from_units(vec![59])))
                                || (value == (self::ml_string::Text::from_units(vec![125])))))
                    {
                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 100, 101, 102, 97, 117, 108, 116, 32, 116, 121, 112, 101])])
                    } else if (kind
                        == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && (((value == (self::ml_string::Text::from_units(vec![40])))
                            || (value == (self::ml_string::Text::from_units(vec![91]))))
                            || (value == (self::ml_string::Text::from_units(vec![123]))))
                    {
                        {
                            let after = self::group_after(kinds, values, at);
                            if after < 0f64 {
                                self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 100, 101, 102, 97, 117, 108, 116, 32, 103, 114, 111, 117, 112])])
                            } else {
                                {
                                    let at_2 = after;
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let angles_3 = if (kind
                                == (self::ml_string::Text::from_units(vec![
                                    112, 117, 110, 99, 116,
                                ])))
                                && (value == (self::ml_string::Text::from_units(vec![60])))
                            {
                                { angles + 1f64 }
                            } else {
                                angles
                            };
                            {
                                let closing =
                                    if value == (self::ml_string::Text::from_units(vec![62])) {
                                        vec![self::ml_string::Text::from_units(vec![62])]
                                    } else {
                                        self::read_rust_type_closing_token_parts(&(value))
                                    };
                                {
                                    let source_ml_source2_s92 = if (kind
                                        == (self::ml_string::Text::from_units(vec![
                                            112, 117, 110, 99, 116,
                                        ])))
                                        && ((closing.len() as f64) > 0f64)
                                    {
                                        {
                                            let offset = 0f64;
                                            {
                                                let source_ml_source2_s93 = self::source_ml_source1_temporary_read_rust_generic_type_alias_parameter_directory_loop22(names, &(starts), &(ends), &(offsets), at, first, angles_3, &(closing), offset);
                                                match source_ml_source2_s93 {
                                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Done(starts_2, ends_2, offsets_2, angles_4) => {
                                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21Next(starts_2, ends_2, offsets_2, angles_4)
                                                        }
                                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Return(source_ml_source1_temporary_result22) => {
                                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21Return(source_ml_source1_temporary_result22)
                                                        }
                                                    }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21Next(starts, ends, offsets, angles_3)
                                    };
                                    match source_ml_source2_s92 {
                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21Next(starts_3, ends_3, offsets_3, angles_5) => {
                                                if ((kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (value == (self::ml_string::Text::from_units(vec![44])))) && (angles_5 == 0f64) {
                                                    if first == at {
                                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![101, 109, 112, 116, 121, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 100, 101, 102, 97, 117, 108, 116, 32, 116, 121, 112, 101])])
                                                    } else {
                                                        {
                                                            let starts_4 = { let (ml_append_right, ml_append_left) = (vec![{
                                                                let source_ml_source2_string_argument94 = first;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument94))
                                                            }], starts_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                            {
                                                                let ends_4 = { let (ml_append_right, ml_append_left) = (vec![{
                                                                    let source_ml_source2_string_argument95 = at;
                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument95))
                                                                }], ends_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let offsets_4 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![48])], offsets_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        let found_2 = true;
                                                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Done(starts_4, ends_4, offsets_4, at, found_2)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let at_3 = at + 1f64;
                                                        {
                                                            (starts, ends, offsets, at, angles) = (starts_3, ends_3, offsets_3, at_3, angles_5);
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21Return(source_ml_source1_temporary_result21) => {
                                                self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Return(source_ml_source1_temporary_result21)
                                            }
                                        }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Done(starts, ends, offsets, at, found)
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
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_parameter_directory_loop17(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    raws: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    mut starts: Vec<self::ml_string::Text>,
    mut ends: Vec<self::ml_string::Text>,
    mut offsets: Vec<self::ml_string::Text>,
    mut at: f64,
    mut default_seen: bool,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result {
    loop {
        return if at < (values.len() as f64) {
            {
                let name = self::read_required_text_fact(values, at);
                if ((self::read_required_text_fact(kinds, at))
                    != (self::ml_string::Text::from_units(vec![
                        105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                    ])))
                    || !(self::accept_parameter_name(
                        &(name),
                        &(self::read_required_text_fact(raws, at)),
                    ))
                {
                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 110, 101, 101, 100, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 117, 110, 99, 111, 110, 115, 116, 114, 97, 105, 110, 101, 100, 32, 116, 121, 112, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115])])
                } else if self::ml_array::includes(&names, &name.clone(), 0f64, false, |a, b| {
                    a == b
                }) {
                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 116, 121, 112, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114])])
                } else if (names.len() as f64) >= 64f64 {
                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])])
                } else {
                    {
                        let names_2 = {
                            let (ml_append_right, ml_append_left) = (vec![name.clone()], names);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let at_2 = at + 1f64;
                            if at_2 >= (values.len() as f64) {
                                self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 108, 105, 115, 116])])
                            } else {
                                {
                                    let punctuation = self::read_required_text_fact(values, at_2);
                                    {
                                        let ml_library_inequality_1 =
                                            (self::read_required_text_fact(kinds, at_2))
                                                != (self::ml_string::Text::from_units(vec![
                                                    112, 117, 110, 99, 116,
                                                ]));
                                        if ml_library_inequality_1 {
                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 98, 111, 117, 110, 100, 115, 32, 97, 110, 100, 32, 100, 101, 102, 97, 117, 108, 116, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110])])
                                        } else {
                                            {
                                                let source_ml_source2_s97 = if punctuation
                                                    == (self::ml_string::Text::from_units(vec![61]))
                                                {
                                                    {
                                                        let default_seen_2 = true;
                                                        {
                                                            let at_3 = at_2 + 1f64;
                                                            {
                                                                let first = at_3;
                                                                {
                                                                    let angles = 0f64;
                                                                    {
                                                                        let found = false;
                                                                        {
                                                                            let source_ml_source2_s98 = self::source_ml_source1_temporary_read_rust_generic_type_alias_parameter_directory_loop20(kinds, values, &(names_2), starts, ends, offsets, at_3, first, angles, found);
                                                                            match source_ml_source2_s98 {
                                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Done(starts_2, ends_2, offsets_2, at_4, found_2) => {
                                                                                        if found_2 {
                                                                                            {
                                                                                                let punctuation_2 = self::read_required_text_fact(values, at_4);
                                                                                                self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18Next(starts_2, ends_2, offsets_2, at_4, default_seen_2, punctuation_2)
                                                                                            }
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 108, 105, 115, 116])])
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Return(source_ml_source1_temporary_result20) => {
                                                                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18Return(source_ml_source1_temporary_result20)
                                                                                    }
                                                                                }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else if default_seen {
                                                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 97, 102, 116, 101, 114, 32, 97, 32, 100, 101, 102, 97, 117, 108, 116, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 111, 119, 110, 32, 100, 101, 102, 97, 117, 108, 116, 115])])
                                                } else {
                                                    {
                                                        let starts_3 = {
                                                            let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![45, 49])], starts);
                                                            self::ml_array::append(
                                                                ml_append_left,
                                                                ml_append_right,
                                                            )
                                                        };
                                                        {
                                                            let ends_3 = {
                                                                let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![45, 49])], ends);
                                                                self::ml_array::append(
                                                                    ml_append_left,
                                                                    ml_append_right,
                                                                )
                                                            };
                                                            {
                                                                let offsets_3 = {
                                                                    let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![48])], offsets);
                                                                    self::ml_array::append(
                                                                        ml_append_left,
                                                                        ml_append_right,
                                                                    )
                                                                };
                                                                self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18Next(starts_3, ends_3, offsets_3, at_2, default_seen, punctuation.clone())
                                                            }
                                                        }
                                                    }
                                                };
                                                match source_ml_source2_s97 {
                                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18Next(starts_4, ends_4, offsets_4, at_5, default_seen_3, punctuation_3) => {
                                                            {
                                                                let source_ml_source2_s99 = if punctuation_3 == (self::ml_string::Text::from_units(vec![44])) {
                                                                    {
                                                                        let at_6 = at_5 + 1f64;
                                                                        if at_6 >= (values.len() as f64) {
                                                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 108, 105, 115, 116])])
                                                                        } else {
                                                                            {
                                                                                let punctuation_4 = self::read_required_text_fact(values, at_6);
                                                                                if (punctuation_4 != (self::ml_string::Text::from_units(vec![62]))) && (punctuation_4 != (self::ml_string::Text::from_units(vec![62, 61]))) {
                                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19Continue(at_6, punctuation_4.clone())
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19Next(at_6, punctuation_4.clone())
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19Next(at_5, punctuation_3.clone())
                                                                };
                                                                match source_ml_source2_s99 {
                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19Next(at_7, punctuation_5) => {
                                                                        if (punctuation_5 == (self::ml_string::Text::from_units(vec![62]))) || (punctuation_5 == (self::ml_string::Text::from_units(vec![62, 61]))) {
                                                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Return(self::parameter_directory(&(names_2), &(starts_4), &(ends_4), &(offsets_4), at_7, &(if punctuation_5 == (self::ml_string::Text::from_units(vec![62, 61])) {
                                                                                self::ml_string::Text::from_units(vec![61])
                                                                            } else {
                                                                                self::ml_string::Text::from_units(vec![])
                                                                            })))
                                                                        } else {
                                                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 98, 111, 117, 110, 100, 115, 32, 97, 110, 100, 32, 100, 101, 102, 97, 117, 108, 116, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110])])
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19Return(source_ml_source1_temporary_result19) => {
                                                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Return(source_ml_source1_temporary_result19)
                                                                    }
                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19Continue(at_8, punctuation_6) => {
                                                                        {
                                                                            (names, starts, ends, offsets, at, default_seen) = (names_2, starts_4, ends_4, offsets_4, at_8, default_seen_3);
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18Return(source_ml_source1_temporary_result19_2) => {
                                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Return(source_ml_source1_temporary_result19_2)
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
            self::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result::MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Result {
    MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Done,
    MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Return(
        Vec<self::ml_string::Text>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_default_substitution_plan_loop23(
    parameters: &[self::ml_string::Text],
    mut position: f64,
    occurrences: &[self::ml_string::Text],
    mut at: f64,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Result {
    loop {
        return if at < (occurrences.len() as f64) {
            {
                let identity = self::read_required_text_fact(occurrences, at);
                {
                    let index =
                        self::ml_array::index_of(parameters, &identity, 0f64, false, |a, b| a == b);
                    if (index < 0f64) || (index >= position) {
                        self::MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Result::MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 100, 101, 102, 97, 117, 108, 116, 115, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 116, 104, 101, 105, 114, 32, 111, 119, 110, 32, 111, 114, 32, 108, 97, 116, 101, 114, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 98, 105, 110, 100, 105, 110, 103, 115])])
                    } else {
                        {
                            let at_2 = at + 1f64;
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Result::MlSource1TemporaryReadRustGenericTypeAliasDefaultSubstitutionPlanLoop23Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Result {
    MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Done(
        Vec<self::ml_string::Text>,
    ),
    MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Return(
        Vec<self::ml_string::Text>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_default_argument_plan_loop24(
    defaults: &[bool],
    mut explicit: bool,
    mut result: Vec<self::ml_string::Text>,
    mut at: f64,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Result {
    loop {
        return if at < (defaults.len() as f64) {
            if (self::read_required_boolean_fact(defaults, at)) {
                {
                    let result_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![{
                                let source_ml_source2_string_argument100 = at;
                                self::ml_string::Text::from_text(&self::ml_number::js_number(
                                    source_ml_source2_string_argument100,
                                ))
                            }],
                            result,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (result, at) = (result_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                self::MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Result::MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), if explicit {
                    self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 97, 114, 105, 116, 121, 32, 100, 105, 102, 102, 101, 114, 115, 32, 102, 114, 111, 109, 32, 105, 116, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110])
                } else {
                    self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116])
                }])
            }
        } else {
            self::MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Result::MlSource1TemporaryReadRustGenericTypeAliasDefaultArgumentPlanLoop24Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Result {
    MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Done,
    MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Return(
        Vec<self::ml_string::Text>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_substitution_plan_loop25(
    arguments_proven: &[bool],
    mut at: f64,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Result {
    loop {
        return if at < (arguments_proven.len() as f64) {
            if (self::read_required_boolean_fact(arguments_proven, at)) {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                self::MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Result::MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 110, 101, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 99, 104, 101, 99, 107, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 114, 105, 103, 105, 110, 115])])
            }
        } else {
            self::MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Result::MlSource1TemporaryReadRustGenericTypeAliasSubstitutionPlanLoop25Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Result {
    MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Done,
    MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Return(
        Vec<self::ml_string::Text>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_header_substitution_plan_loop26(
    arguments_typed: &[bool],
    arguments_proven: &[bool],
    arguments_provisional: &[bool],
    mut at: f64,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Result {
    loop {
        return if at < (arguments_typed.len() as f64) {
            if !(self::read_required_boolean_fact(arguments_typed, at))
                || (!(self::read_required_boolean_fact(arguments_proven, at))
                    && !(self::read_required_boolean_fact(arguments_provisional, at)))
            {
                self::MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Result::MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 110, 101, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 99, 104, 101, 99, 107, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 114, 105, 103, 105, 110, 115])])
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Result::MlSource1TemporaryReadRustGenericTypeAliasHeaderSubstitutionPlanLoop26Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Result {
    MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Done(Vec<self::ml_string::Text>),
    MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_binding_plan_loop28(
    parameters: &[self::ml_string::Text],
    occurrences: &[self::ml_string::Text],
    mut plan: Vec<self::ml_string::Text>,
    mut at: f64,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Result {
    loop {
        return if at < (occurrences.len() as f64) {
            {
                let identity = self::read_required_text_fact(occurrences, at);
                if identity == (self::ml_string::Text::from_units(vec![])) {
                    self::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Result::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 111, 99, 99, 117, 114, 114, 101, 110, 99, 101, 32, 110, 101, 101, 100, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103])])
                } else {
                    {
                        let plan_2 = {
                            let (ml_append_right, ml_append_left) = (
                                vec![{
                                    let source_ml_source2_string_argument101 =
                                        self::ml_array::index_of(
                                            parameters,
                                            &identity.clone(),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        );
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        source_ml_source2_string_argument101,
                                    ))
                                }],
                                plan,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                (plan, at) = (plan_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Result::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop28Done(plan)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Result {
    MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Done,
    MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_binding_plan_loop27(
    parameters: &[self::ml_string::Text],
    mut at: f64,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Result {
    loop {
        return if at < (parameters.len() as f64) {
            {
                let identity = self::read_required_text_fact(parameters, at);
                if (identity == (self::ml_string::Text::from_units(vec![])))
                    || self::ml_array::includes(
                        &self::ml_array::slice(parameters, 0f64, at),
                        &identity.clone(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )
                {
                    self::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Result::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 116, 117, 97, 108, 32, 97, 110, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116])])
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Result::MlSource1TemporaryReadRustGenericTypeAliasBindingPlanLoop27Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Result {
    MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Done,
    MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_unused_parameter_error_loop29(
    parameters: &[self::ml_string::Text],
    used: &[self::ml_string::Text],
    mut at: f64,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Result {
    loop {
        return if at < (parameters.len() as f64) {
            if (self::ml_array::includes(
                used,
                &self::read_required_text_fact(parameters, at),
                0f64,
                false,
                |a, b| a == b,
            )) {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                self::MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Result::MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Return(self::ml_string::Text::from_units(vec![117, 110, 117, 115, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 116, 121, 112, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114]))
            }
        } else {
            self::MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Result::MlSource1TemporaryReadRustGenericTypeAliasUnusedParameterErrorLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryGroupAfterLoop30Result {
    MlSource1TemporaryGroupAfterLoop30Done,
    MlSource1TemporaryGroupAfterLoop30Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_group_after_loop30(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut stack: Vec<self::ml_string::Text>,
    mut at: f64,
) -> self::MlSource1TemporaryGroupAfterLoop30Result {
    loop {
        return if at < (values.len() as f64) {
            if (self::read_required_text_fact(kinds, at))
                != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))
            {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                {
                    let value = self::read_required_text_fact(values, at);
                    if ((value == (self::ml_string::Text::from_units(vec![123])))
                        || (value == (self::ml_string::Text::from_units(vec![91]))))
                        || (value == (self::ml_string::Text::from_units(vec![40])))
                    {
                        {
                            let stack_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![
                                        if value == (self::ml_string::Text::from_units(vec![123])) {
                                            self::ml_string::Text::from_units(vec![125])
                                        } else if value
                                            == (self::ml_string::Text::from_units(vec![91]))
                                        {
                                            self::ml_string::Text::from_units(vec![93])
                                        } else {
                                            self::ml_string::Text::from_units(vec![41])
                                        },
                                    ],
                                    stack,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let at_3 = at + 1f64;
                                {
                                    (stack, at) = (stack_2, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if ((value == (self::ml_string::Text::from_units(vec![125])))
                            || (value == (self::ml_string::Text::from_units(vec![93]))))
                            || (value == (self::ml_string::Text::from_units(vec![41])))
                        {
                            if value
                                == (self::read_required_text_fact(
                                    &(stack),
                                    ((stack.len() as f64) - 1f64),
                                ))
                            {
                                {
                                    let stack_3 = self::ml_array::slice(&stack, 0f64, -1f64);
                                    if (stack_3.len() as f64) == 0f64 {
                                        self::MlSource1TemporaryGroupAfterLoop30Result::MlSource1TemporaryGroupAfterLoop30Return(at + 1f64)
                                    } else {
                                        {
                                            let at_4 = at + 1f64;
                                            {
                                                (stack, at) = (stack_3, at_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryGroupAfterLoop30Result::MlSource1TemporaryGroupAfterLoop30Return(-1f64)
                            }
                        } else {
                            {
                                let at_5 = at + 1f64;
                                {
                                    at = at_5;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryGroupAfterLoop30Result::MlSource1TemporaryGroupAfterLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result {
    MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Done(Vec<self::ml_string::Text>),
    MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Return(
        Vec<self::ml_string::Text>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_declaration_plan_loop35(
    mut result: Vec<self::ml_string::Text>,
    source_ml_source1_temporary_values31: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index31: self::ml::Big,
) -> Vec<self::ml_string::Text> {
    loop {
        if source_ml_source1_temporary_index31
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values31.len() as u128))
        {
            {
                let parameter = self::ml_array::at(
                    source_ml_source1_temporary_values31,
                    source_ml_source1_temporary_index31.to_index(),
                );
                {
                    let result_2 = {
                        let (ml_append_right, ml_append_left) = (vec![parameter], result);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let source_ml_source1_temporary_index31_2 =
                            source_ml_source1_temporary_index31.add(&self::ml::Big::from_i128(1));
                        {
                            (result, source_ml_source1_temporary_index31) =
                                (result_2, source_ml_source1_temporary_index31_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}
