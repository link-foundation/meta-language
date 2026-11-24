// Generated from resource-description-rules.js by the checked translator.
// Do not edit by hand.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn same_resource_description_term(
    left_kind: &str,
    left_fields: &[String],
    right_kind: &str,
    right_fields: &[String],
) -> bool {
    if left_kind == right_kind {
        {
            let count = 0f64;
            {
                let source_ml_source1_s17 = if (left_kind == "identifier") {
                    {
                        let count_2 = 1f64;
                        super::MlTemporarySameResourceDescriptionTermJoin23::MlTemporarySameResourceDescriptionTermJoin23Next(count_2)
                    }
                } else if (left_kind == "blank") {
                    {
                        let count_3 = 2f64;
                        super::MlTemporarySameResourceDescriptionTermJoin23::MlTemporarySameResourceDescriptionTermJoin23Next(count_3)
                    }
                } else if (left_kind == "literal") {
                    {
                        let count_4 = 3f64;
                        super::MlTemporarySameResourceDescriptionTermJoin23::MlTemporarySameResourceDescriptionTermJoin23Next(count_4)
                    }
                } else {
                    super::MlTemporarySameResourceDescriptionTermJoin23::MlTemporarySameResourceDescriptionTermJoin23Return(false)
                };
                match source_ml_source1_s17 {
                    super::MlTemporarySameResourceDescriptionTermJoin23::MlTemporarySameResourceDescriptionTermJoin23Next(count_5) => {
                        if (((left_fields.len() as f64) != count_5) || ((right_fields.len() as f64) != count_5)) {
                            false
                        } else {
                            {
                                let index = 0f64;
                                {
                                    let source_ml_source1_s18 = super::source_ml_temporary_same_resource_description_term_loop24(left_fields, right_fields, count_5, index);
                                    match source_ml_source1_s18 {
                                        super::MlTemporarySameResourceDescriptionTermLoop24Result::MlTemporarySameResourceDescriptionTermLoop24Done => {
                                            true
                                        }
                                        super::MlTemporarySameResourceDescriptionTermLoop24Result::MlTemporarySameResourceDescriptionTermLoop24Return(source_ml_temporary_result24) => {
                                            source_ml_temporary_result24
                                        }
                                    }
                                }
                            }
                        }
                    }
                    super::MlTemporarySameResourceDescriptionTermJoin23::MlTemporarySameResourceDescriptionTermJoin23Return(source_ml_temporary_result24_2) => {
                        source_ml_temporary_result24_2
                    }
                }
            }
        }
    } else {
        false
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result {
    MlTemporaryAcceptResourceDescriptionUnicodeLoop1Done,
    MlTemporaryAcceptResourceDescriptionUnicodeLoop1Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_unicode_loop1(
    units: &[f64],
    mut index: f64,
) -> super::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let unit = super::resource_description_unit(units, index);
                if ((!(super::ml_math::is_safe_integer(unit)) || (unit < (0f64)))
                    || (unit > (65535f64)))
                {
                    super::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Return(false)
                } else if ((55296f64..=56319f64).contains(&unit)) {
                    {
                        let index_2 = (index + 1f64);
                        if (((index_2 >= (units.len() as f64))
                            || ((super::resource_description_unit(units, index_2)) < (56320f64)))
                            || ((super::resource_description_unit(units, index_2)) > (57343f64)))
                        {
                            super::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Return(false)
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
                } else if ((56320f64..=57343f64).contains(&unit)) {
                    super::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Return(false)
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
            super::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptResourceDescriptionIdentifierLoop4Result {
    MlTemporaryAcceptResourceDescriptionIdentifierLoop4Done,
    MlTemporaryAcceptResourceDescriptionIdentifierLoop4Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAcceptResourceDescriptionIdentifierJoin5 {
    MlTemporaryAcceptResourceDescriptionIdentifierJoin5Next(f64, f64),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
// The checked continuation explicitly carries its lexical state.
#[allow(clippy::too_many_lines)]
pub fn source_ml_temporary_accept_resource_description_identifier_loop4(
    units: &[f64],
    mut query: bool,
    mut fragment: bool,
    mut index: f64,
) -> super::MlTemporaryAcceptResourceDescriptionIdentifierLoop4Result {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let code = super::resource_description_unit(units, index);
                if (code == (35f64)) {
                    if fragment {
                        super::MlTemporaryAcceptResourceDescriptionIdentifierLoop4Result::MlTemporaryAcceptResourceDescriptionIdentifierLoop4Return(false)
                    } else {
                        {
                            let fragment_2 = true;
                            {
                                let query_2 = false;
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        (query, fragment, index) = (query_2, fragment_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        if (code == (63f64)) {
                            {
                                let query_4 = if fragment { query } else { { true } };
                                {
                                    let index_3 = (index + 1f64);
                                    {
                                        (query, index) = (query_4, index_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if (code == (37f64)) {
                                if ((((index + 2f64) >= (units.len() as f64))
                                    || !(super::resource_description_hexadecimal(
                                        super::resource_description_unit(units, index + 1f64),
                                    )))
                                    || !(super::resource_description_hexadecimal(
                                        super::resource_description_unit(units, index + 2f64),
                                    )))
                                {
                                    super::MlTemporaryAcceptResourceDescriptionIdentifierLoop4Result::MlTemporaryAcceptResourceDescriptionIdentifierLoop4Return(false)
                                } else {
                                    {
                                        let index_4 = (index + 2f64);
                                        {
                                            let index_5 = (index_4 + 1f64);
                                            {
                                                index = index_5;
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let source_ml_source1_s19 = if ((55296f64..=56319f64)
                                        .contains(&code))
                                    {
                                        {
                                            let code_2 = ((({
                                                let separately_rounded_product =
                                                    (code - 55296f64) * 1024f64;
                                                65536f64 + separately_rounded_product
                                            }) + super::resource_description_unit(
                                                units,
                                                index + 1f64,
                                            )) - 56320f64);
                                            {
                                                let index_6 = (index + 1f64);
                                                super::MlTemporaryAcceptResourceDescriptionIdentifierJoin5::MlTemporaryAcceptResourceDescriptionIdentifierJoin5Next(index_6, code_2)
                                            }
                                        }
                                    } else {
                                        super::MlTemporaryAcceptResourceDescriptionIdentifierJoin5::MlTemporaryAcceptResourceDescriptionIdentifierJoin5Next(index, code)
                                    };
                                    match source_ml_source1_s19 {
                                    super::MlTemporaryAcceptResourceDescriptionIdentifierJoin5::MlTemporaryAcceptResourceDescriptionIdentifierJoin5Next(index_7, code_3) => {
                                        { if (((code_3 == (47f64)) || (code_3 == (58f64))) || (code_3 == (64f64))) {
                                            {
                                                let index_8 = (index_7 + 1f64);
                                                {
                                                    index = index_8;
                                                    continue;
                                                }
                                            }
                                        }
{
                                            { if (super::accept_resource_description_identifier_character(code_3, query)) {
                                                {
                                                    let index_9 = (index_7 + 1f64);
                                                    {
                                                        index = index_9;
                                                        continue;
                                                    }
                                                }
                                            }
{
                                                super::MlTemporaryAcceptResourceDescriptionIdentifierLoop4Result::MlTemporaryAcceptResourceDescriptionIdentifierLoop4Return(false)
                                            } }
                                        } }
                                    }
                                }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            super::MlTemporaryAcceptResourceDescriptionIdentifierLoop4Result::MlTemporaryAcceptResourceDescriptionIdentifierLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptResourceDescriptionIdentifierJoin3 {
    MlTemporaryAcceptResourceDescriptionIdentifierJoin3Next,
    MlTemporaryAcceptResourceDescriptionIdentifierJoin3Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_identifier_loop6(
    units: &[f64],
    mut authority_end: f64,
) -> f64 {
    loop {
        return {
            if ((((authority_end < (units.len() as f64))
                && ((super::resource_description_unit(units, authority_end)) != (47f64)))
                && ((super::resource_description_unit(units, authority_end)) != (63f64)))
                && ((super::resource_description_unit(units, authority_end)) != (35f64)))
            {
                {
                    let authority_end_2 = (authority_end + 1f64);
                    {
                        authority_end = authority_end_2;
                        continue;
                    }
                }
            }
            { authority_end }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptResourceDescriptionIdentifierLoop8Result {
    MlTemporaryAcceptResourceDescriptionIdentifierLoop8Done,
    MlTemporaryAcceptResourceDescriptionIdentifierLoop8Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_identifier_loop8(
    units: &[f64],
    mut authority_end: f64,
    mut found_port: bool,
    mut index: f64,
) -> super::MlTemporaryAcceptResourceDescriptionIdentifierLoop8Result {
    loop {
        return if (index < authority_end) {
            if ((super::resource_description_unit(units, index)) == (58f64)) {
                if found_port {
                    super::MlTemporaryAcceptResourceDescriptionIdentifierLoop8Result::MlTemporaryAcceptResourceDescriptionIdentifierLoop8Return(false)
                } else {
                    {
                        let found_port_2 = true;
                        {
                            let index_2 = (index + 1f64);
                            {
                                (found_port, index) = (found_port_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            } else if (found_port
                && !(super::resource_description_digit(super::resource_description_unit(
                    units, index,
                ))))
            {
                super::MlTemporaryAcceptResourceDescriptionIdentifierLoop8Result::MlTemporaryAcceptResourceDescriptionIdentifierLoop8Return(false)
            } else {
                {
                    let index_3 = (index + 1f64);
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        } else {
            super::MlTemporaryAcceptResourceDescriptionIdentifierLoop8Result::MlTemporaryAcceptResourceDescriptionIdentifierLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAcceptResourceDescriptionIdentifierLoop7Result {
    MlTemporaryAcceptResourceDescriptionIdentifierLoop7Done(f64),
    MlTemporaryAcceptResourceDescriptionIdentifierLoop7Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_identifier_loop7(
    units: &[f64],
    mut authority_end: f64,
    mut host: f64,
    mut found_user: bool,
    mut index: f64,
) -> super::MlTemporaryAcceptResourceDescriptionIdentifierLoop7Result {
    loop {
        return if (index < authority_end) {
            if ((super::resource_description_unit(units, index)) == (64f64)) {
                if found_user {
                    super::MlTemporaryAcceptResourceDescriptionIdentifierLoop7Result::MlTemporaryAcceptResourceDescriptionIdentifierLoop7Return(false)
                } else {
                    {
                        let found_user_2 = true;
                        {
                            let host_2 = (index + 1f64);
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (host, found_user, index) = (host_2, found_user_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            } else {
                {
                    let index_3 = (index + 1f64);
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        } else {
            super::MlTemporaryAcceptResourceDescriptionIdentifierLoop7Result::MlTemporaryAcceptResourceDescriptionIdentifierLoop7Done(host)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAcceptResourceDescriptionIdentifierLoop2Result {
    MlTemporaryAcceptResourceDescriptionIdentifierLoop2Done(f64),
    MlTemporaryAcceptResourceDescriptionIdentifierLoop2Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_identifier_loop2(
    units: &[f64],
    mut start: f64,
) -> super::MlTemporaryAcceptResourceDescriptionIdentifierLoop2Result {
    loop {
        return if ((start < (units.len() as f64))
            && ((super::resource_description_unit(units, start)) != (58f64)))
        {
            {
                let unit = super::resource_description_unit(units, start);
                if ((((!(super::resource_description_letter(unit))
                    && !(super::resource_description_digit(unit)))
                    && (unit != (43f64)))
                    && (unit != (45f64)))
                    && (unit != (46f64)))
                {
                    super::MlTemporaryAcceptResourceDescriptionIdentifierLoop2Result::MlTemporaryAcceptResourceDescriptionIdentifierLoop2Return(false)
                } else {
                    {
                        let start_2 = (start + 1f64);
                        {
                            start = start_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            super::MlTemporaryAcceptResourceDescriptionIdentifierLoop2Result::MlTemporaryAcceptResourceDescriptionIdentifierLoop2Done(start)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAcceptResourceDescriptionLanguageJoin12 {
    MlTemporaryAcceptResourceDescriptionLanguageJoin12Next(f64),
    MlTemporaryAcceptResourceDescriptionLanguageJoin12Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_language_loop15(
    units: &[f64],
    mut starts: Vec<f64>,
    mut part: f64,
    mut letters: bool,
    mut index: f64,
) -> bool {
    loop {
        return {
            if (index < (super::resource_description_unit(&starts, part) + 3f64)) {
                {
                    if (super::resource_description_letter(super::resource_description_unit(
                        units, index,
                    ))) {
                        {
                            let index_3 = (index + 1f64);
                            {
                                index = index_3;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let letters_2 = false;
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (letters, index) = (letters_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            { letters }
        };
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_language_loop14(
    units: &[f64],
    mut starts: Vec<f64>,
    mut lengths: Vec<f64>,
    mut part: f64,
    mut count: f64,
) -> f64 {
    loop {
        return if (((part < (starts.len() as f64))
            && ((super::resource_description_unit(&lengths, part)) == (3f64)))
            && (count < (3f64)))
        {
            {
                let letters = true;
                {
                    let index = super::resource_description_unit(&starts, part);
                    {
                        let letters_2 =
                            super::source_ml_temporary_accept_resource_description_language_loop15(
                                units,
                                starts.clone(),
                                part,
                                letters,
                                index,
                            );
                        {
                            if letters_2 {
                                {
                                    let part_2 = (part + 1f64);
                                    {
                                        let count_2 = (count + 1f64);
                                        {
                                            (part, count) = (part_2, count_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                            { part }
                        }
                    }
                }
            }
        } else {
            part
        };
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_language_loop16(
    units: &[f64],
    mut starts: Vec<f64>,
    mut part: f64,
    mut letters: bool,
    mut index: f64,
) -> bool {
    loop {
        return {
            if (index < (super::resource_description_unit(&starts, part) + 4f64)) {
                {
                    if (super::resource_description_letter(super::resource_description_unit(
                        units, index,
                    ))) {
                        {
                            let index_3 = (index + 1f64);
                            {
                                index = index_3;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let letters_2 = false;
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (letters, index) = (letters_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            { letters }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAcceptResourceDescriptionLanguageLoop18Result {
    MlTemporaryAcceptResourceDescriptionLanguageLoop18Done(f64),
    MlTemporaryAcceptResourceDescriptionLanguageLoop18Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_language_loop20(
    mut starts: Vec<f64>,
    mut lengths: Vec<f64>,
    mut part: f64,
) -> f64 {
    loop {
        return {
            if ((part < (starts.len() as f64))
                && ((super::resource_description_unit(&lengths, part)) >= (2f64)))
            {
                {
                    let part_2 = (part + 1f64);
                    {
                        part = part_2;
                        continue;
                    }
                }
            }
            { part }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptResourceDescriptionLanguageLoop19Result {
    MlTemporaryAcceptResourceDescriptionLanguageLoop19Done,
    MlTemporaryAcceptResourceDescriptionLanguageLoop19Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_language_loop19(
    mut singleton: f64,
    mut source_ml_temporary_values10: Vec<f64>,
    mut source_ml_temporary_index10: super::ml::Big,
) -> super::MlTemporaryAcceptResourceDescriptionLanguageLoop19Result {
    loop {
        return if (source_ml_temporary_index10
            < (super::ml::Big::from_u128(source_ml_temporary_values10.len() as u128)))
        {
            {
                let earlier = super::ml_array::at(
                    &source_ml_temporary_values10,
                    source_ml_temporary_index10.to_index(),
                );
                if (earlier == singleton) {
                    super::MlTemporaryAcceptResourceDescriptionLanguageLoop19Result::MlTemporaryAcceptResourceDescriptionLanguageLoop19Return(false)
                } else {
                    {
                        let source_ml_temporary_index10_2 =
                            source_ml_temporary_index10.add(&super::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index10 = source_ml_temporary_index10_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            super::MlTemporaryAcceptResourceDescriptionLanguageLoop19Result::MlTemporaryAcceptResourceDescriptionLanguageLoop19Done
        };
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_language_loop18(
    units: &[f64],
    mut starts: Vec<f64>,
    mut lengths: Vec<f64>,
    mut part: f64,
    mut singletons: Vec<f64>,
) -> super::MlTemporaryAcceptResourceDescriptionLanguageLoop18Result {
    loop {
        return if ((((part < (starts.len() as f64))
            && ((super::resource_description_unit(&lengths, part)) == (1f64)))
            && ((super::resource_description_unit(
                units,
                super::resource_description_unit(&starts, part),
            )) != (88f64)))
            && ((super::resource_description_unit(
                units,
                super::resource_description_unit(&starts, part),
            )) != (120f64)))
        {
            {
                let singleton = super::resource_description_unit(
                    units,
                    super::resource_description_unit(&starts, part),
                );
                {
                    let singleton_3 = if ((65f64..=90f64).contains(&singleton)) {
                        { (singleton + 32f64) }
                    } else {
                        singleton
                    };
                    {
                        let source_ml_temporary_values10 = singletons.clone();
                        {
                            let source_ml_temporary_index10 = super::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_s20 = super::source_ml_temporary_accept_resource_description_language_loop19(singleton_3, source_ml_temporary_values10.clone(), source_ml_temporary_index10.clone());
                                match source_ml_source1_s20 {
                                    super::MlTemporaryAcceptResourceDescriptionLanguageLoop19Result::MlTemporaryAcceptResourceDescriptionLanguageLoop19Done => {
                                        {
                                            let singletons_2 = {
                                                let source_ml_source1_concat_value21 = singletons.clone();
                                                {
                                                    let source_ml_source1_concat_value22 = vec![singleton_3];
                                                    super::ml_array::append(super::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value21), source_ml_source1_concat_value22)
                                                }
                                            };
                                            {
                                                let part_2 = (part + 1f64);
                                                {
                                                    let beginning = part_2;
                                                    {
                                                        let part_3 = super::source_ml_temporary_accept_resource_description_language_loop20(starts.clone(), lengths.clone(), part_2);
                                                        if (part_3 == beginning) {
                                                            super::MlTemporaryAcceptResourceDescriptionLanguageLoop18Result::MlTemporaryAcceptResourceDescriptionLanguageLoop18Return(false)
                                                        } else {
                                                            {
                                                                (part, singletons) = (part_3, singletons_2.clone());
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    super::MlTemporaryAcceptResourceDescriptionLanguageLoop19Result::MlTemporaryAcceptResourceDescriptionLanguageLoop19Return(source_ml_temporary_result20) => {
                                        super::MlTemporaryAcceptResourceDescriptionLanguageLoop18Result::MlTemporaryAcceptResourceDescriptionLanguageLoop18Return(source_ml_temporary_result20)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            super::MlTemporaryAcceptResourceDescriptionLanguageLoop18Result::MlTemporaryAcceptResourceDescriptionLanguageLoop18Done(part)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAcceptResourceDescriptionLanguageLoop17Result {
    MlTemporaryAcceptResourceDescriptionLanguageLoop17Done(f64),
    MlTemporaryAcceptResourceDescriptionLanguageLoop17Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptResourceDescriptionLanguageLoop21Result {
    MlTemporaryAcceptResourceDescriptionLanguageLoop21Done,
    MlTemporaryAcceptResourceDescriptionLanguageLoop21Return(bool),
}
