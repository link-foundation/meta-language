// Generated automatically from the linked source-text byte JavaScript kernel. Do not edit.
pub use crate::translation::frontend_rules::ml_array;

#[must_use]
fn source_text_integer(x: f64) -> bool {
    x.is_finite() && matches!(x.trunc().partial_cmp(&x), Some(std::cmp::Ordering::Equal))
}

/// Evaluate the shared Unicode source-text byte model.
#[must_use]
pub fn read_source_text_scalar(units: &[f64], index: f64) -> f64 {
    if ((!(source_text_integer(index)) || (index < 0f64))
        || (index
            >= serde_json::Number::from(units.len())
                .as_f64()
                .expect("a source unit vector length has a finite binary64 representation")))
        || (serde_json::Number::from(units.len())
            .as_f64()
            .expect("a source unit vector length has a finite binary64 representation")
            > 4_294_967_295f64)
    {
        -1f64
    } else {
        {
            let first =
                (ml_array::optional_at(units, ml_array::number_index(index))).unwrap_or(-1f64);
            if (!(source_text_integer(first)) || (first < 0f64)) || (first > 65_535f64) {
                -1f64
            } else if (55_296f64..=56_319f64).contains(&first) {
                if (index + 1f64)
                    >= serde_json::Number::from(units.len())
                        .as_f64()
                        .expect("a source unit vector length has a finite binary64 representation")
                {
                    -1f64
                } else {
                    {
                        let second =
                            (ml_array::optional_at(units, ml_array::number_index(index + 1f64)))
                                .unwrap_or(-1f64);
                        if (!(source_text_integer(second)) || (second < 56_320f64))
                            || (second > 57_343f64)
                        {
                            -1f64
                        } else {
                            {
                                let scaled = (first - 55_296f64) * 1024f64;
                                ((65_536f64 + scaled) + second) - 56_320f64
                            }
                        }
                    }
                }
            } else if (56_320f64..=57_343f64).contains(&first) {
                -1f64
            } else {
                first
            }
        }
    }
}

/// Evaluate the shared Unicode source-text byte model.
#[must_use]
pub fn read_source_text_scalar_byte_width(scalar: f64) -> f64 {
    if ((!(source_text_integer(scalar)) || (scalar < 0f64)) || (scalar > 1_114_111f64))
        || ((55_296f64..=57_343f64).contains(&scalar))
    {
        -1f64
    } else if scalar < 128f64 {
        1f64
    } else if scalar < 2048f64 {
        2f64
    } else if scalar < 65_536f64 {
        3f64
    } else {
        4f64
    }
}

/// Evaluate the shared Unicode source-text byte model.
#[must_use]
pub fn read_source_text_byte_length(units: &[f64]) -> f64 {
    if serde_json::Number::from(units.len())
        .as_f64()
        .expect("a source unit vector length has a finite binary64 representation")
        > 4_294_967_295f64
    {
        -1f64
    } else {
        {
            let index = 0f64;
            {
                let bytes = 0f64;
                {
                    let ml_s1 =
                        crate::translation::source_text_byte_rules::ml_read_source_text_byte_length_loop1(units, index, bytes);
                    match ml_s1 { crate::translation::source_text_byte_rules::MlReadSourceTextByteLengthLoop1Result::MlReadSourceTextByteLengthLoop1Done(value) | crate::translation::source_text_byte_rules::MlReadSourceTextByteLengthLoop1Result::MlReadSourceTextByteLengthLoop1Return(value) => value }
                }
            }
        }
    }
}

/// Evaluate the shared Unicode source-text byte model.
#[must_use]
pub fn accept_source_text_scalar_units(units: &[f64]) -> bool {
    (crate::translation::source_text_byte_rules::read_source_text_byte_length(units)) >= 0f64
}

/// Evaluate the shared Unicode source-text byte model.
#[must_use]
pub fn read_source_text_code_unit_offset(units: &[f64], offset: f64) -> f64 {
    {
        let length =
            crate::translation::source_text_byte_rules::read_source_text_byte_length(units);
        if (((length < 0f64) || !(source_text_integer(offset))) || (offset < 0f64))
            || (offset > length)
        {
            -1f64
        } else {
            {
                let index = 0f64;
                {
                    let bytes = 0f64;
                    {
                        let crate::translation::source_text_byte_rules::MlReadSourceTextCodeUnitOffsetLoop2Result::MlReadSourceTextCodeUnitOffsetLoop2Done(index_2, bytes_2) = crate::translation::source_text_byte_rules::ml_read_source_text_code_unit_offset_loop2(
                            units,
                            offset,
                            index,
                            bytes,
                        );

                        if matches!(
                            bytes_2.partial_cmp(&offset),
                            Some(std::cmp::Ordering::Equal)
                        ) {
                            index_2
                        } else {
                            -1f64
                        }
                    }
                }
            }
        }
    }
}

/// Evaluate the shared Unicode source-text byte model.
#[must_use]
pub fn accept_source_text_byte_slice(units: &[f64], start: f64, end: f64) -> bool {
    ((start <= end)
        && ((crate::translation::source_text_byte_rules::read_source_text_code_unit_offset(
            units, start,
        )) >= 0f64))
        && ((crate::translation::source_text_byte_rules::read_source_text_code_unit_offset(
            units, end,
        )) >= 0f64)
}

/// Evaluate the shared Unicode source-text byte model.
#[must_use]
pub fn read_source_text_byte(units: &[f64], offset: f64) -> f64 {
    {
        let length =
            crate::translation::source_text_byte_rules::read_source_text_byte_length(units);
        if (((length < 0f64) || !(source_text_integer(offset))) || (offset < 0f64))
            || (offset >= length)
        {
            -1f64
        } else {
            {
                let index = 0f64;
                {
                    let bytes = 0f64;
                    {
                        let ml_s3 = crate::translation::source_text_byte_rules::ml_read_source_text_byte_loop3(
                            units,
                            offset,
                            index,
                            bytes,
                        );
                        match ml_s3 {
                            crate::translation::source_text_byte_rules::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Done => {
                                -1f64
                            }
                            crate::translation::source_text_byte_rules::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(ml_result3) => {
                                ml_result3
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Evaluate the shared Unicode source-text byte model.
#[must_use]
pub fn accept_source_text_unit_count(count: f64) -> bool {
    (source_text_integer(count) && (count >= 0f64)) && (count <= 4_294_967_295f64)
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadSourceTextByteLengthLoop1Result {
    MlReadSourceTextByteLengthLoop1Done(f64),
    MlReadSourceTextByteLengthLoop1Return(f64),
}

/// Evaluate the shared Unicode source-text byte model.
#[must_use]
pub fn ml_read_source_text_byte_length_loop1(
    units: &[f64],
    mut index: f64,
    mut bytes: f64,
) -> crate::translation::source_text_byte_rules::MlReadSourceTextByteLengthLoop1Result {
    loop {
        return if index
            < serde_json::Number::from(units.len())
                .as_f64()
                .expect("a source unit vector length has a finite binary64 representation")
        {
            {
                let scalar = crate::translation::source_text_byte_rules::read_source_text_scalar(
                    units, index,
                );
                {
                    let width = crate::translation::source_text_byte_rules::read_source_text_scalar_byte_width(scalar);
                    if width < 0f64 {
                        crate::translation::source_text_byte_rules::MlReadSourceTextByteLengthLoop1Result::MlReadSourceTextByteLengthLoop1Return(-1f64)
                    } else {
                        {
                            let bytes_2 = bytes + width;
                            {
                                let index_2 = index + if scalar >= 65_536f64 { 2f64 } else { 1f64 };
                                {
                                    (index, bytes) = (index_2, bytes_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::source_text_byte_rules::MlReadSourceTextByteLengthLoop1Result::MlReadSourceTextByteLengthLoop1Done(bytes)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadSourceTextCodeUnitOffsetLoop2Result {
    MlReadSourceTextCodeUnitOffsetLoop2Done(f64, f64),
}

/// Evaluate the shared Unicode source-text byte model.
#[must_use]
pub fn ml_read_source_text_code_unit_offset_loop2(
    units: &[f64],
    offset: f64,
    mut index: f64,
    mut bytes: f64,
) -> crate::translation::source_text_byte_rules::MlReadSourceTextCodeUnitOffsetLoop2Result {
    loop {
        if bytes < offset {
            {
                let scalar = crate::translation::source_text_byte_rules::read_source_text_scalar(
                    units, index,
                );
                {
                    let bytes_2 = bytes + crate::translation::source_text_byte_rules::read_source_text_scalar_byte_width(scalar);
                    {
                        let index_2 = index + if scalar >= 65_536f64 { 2f64 } else { 1f64 };
                        {
                            (index, bytes) = (index_2, bytes_2);
                            continue;
                        }
                    }
                }
            }
        }
        return {
            crate::translation::source_text_byte_rules::MlReadSourceTextCodeUnitOffsetLoop2Result::MlReadSourceTextCodeUnitOffsetLoop2Done(index, bytes)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadSourceTextByteLoop3Result {
    MlReadSourceTextByteLoop3Done,
    MlReadSourceTextByteLoop3Return(f64),
}

/// Evaluate the shared Unicode source-text byte model.
#[must_use]
pub fn ml_read_source_text_byte_loop3(
    units: &[f64],
    offset: f64,
    mut index: f64,
    mut bytes: f64,
) -> crate::translation::source_text_byte_rules::MlReadSourceTextByteLoop3Result {
    loop {
        return if index
            < serde_json::Number::from(units.len())
                .as_f64()
                .expect("a source unit vector length has a finite binary64 representation")
        {
            {
                let scalar = crate::translation::source_text_byte_rules::read_source_text_scalar(
                    units, index,
                );
                {
                    let width = crate::translation::source_text_byte_rules::read_source_text_scalar_byte_width(scalar);
                    {
                        let position = offset - bytes;
                        if position < width {
                            if matches!(width.partial_cmp(&1f64), Some(std::cmp::Ordering::Equal)) {
                                crate::translation::source_text_byte_rules::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(scalar)
                            } else if matches!(
                                position.partial_cmp(&0f64),
                                Some(std::cmp::Ordering::Equal)
                            ) {
                                if matches!(
                                    width.partial_cmp(&2f64),
                                    Some(std::cmp::Ordering::Equal)
                                ) {
                                    crate::translation::source_text_byte_rules::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(192f64 + (scalar / 64f64).floor())
                                } else if matches!(
                                    width.partial_cmp(&3f64),
                                    Some(std::cmp::Ordering::Equal)
                                ) {
                                    crate::translation::source_text_byte_rules::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(224f64 + (scalar / 4096f64).floor())
                                } else {
                                    crate::translation::source_text_byte_rules::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(240f64 + (scalar / 262_144f64).floor())
                                }
                            } else if matches!(
                                position.partial_cmp(&(width - 1f64)),
                                Some(std::cmp::Ordering::Equal)
                            ) {
                                crate::translation::source_text_byte_rules::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(128f64 + (scalar % 64f64))
                            } else if (matches!(
                                width.partial_cmp(&4f64),
                                Some(std::cmp::Ordering::Equal)
                            )) && (matches!(
                                position.partial_cmp(&1f64),
                                Some(std::cmp::Ordering::Equal)
                            )) {
                                crate::translation::source_text_byte_rules::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(128f64 + ((scalar / 4096f64).floor() % 64f64))
                            } else {
                                crate::translation::source_text_byte_rules::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(128f64 + ((scalar / 64f64).floor() % 64f64))
                            }
                        } else {
                            {
                                let bytes_2 = bytes + width;
                                {
                                    let index_2 =
                                        index + if scalar >= 65_536f64 { 2f64 } else { 1f64 };
                                    {
                                        (index, bytes) = (index_2, bytes_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::source_text_byte_rules::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Done
        };
    }
}
