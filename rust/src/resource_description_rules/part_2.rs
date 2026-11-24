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
pub fn source_ml_temporary_accept_resource_description_language_loop22(
    units: &[f64],
    mut starts: Vec<f64>,
    mut lengths: Vec<f64>,
    mut part: f64,
    mut earlier: f64,
    mut equal: bool,
    mut offset: f64,
) -> bool {
    loop {
        return {
            if (offset < (super::resource_description_unit(&lengths, part))) {
                {
                    let left = super::resource_description_unit(
                        units,
                        super::resource_description_unit(&starts, earlier) + offset,
                    );
                    {
                        let right = super::resource_description_unit(
                            units,
                            super::resource_description_unit(&starts, part) + offset,
                        );
                        {
                            let left_3 = if ((65f64..=90f64).contains(&left)) {
                                { (left + 32f64) }
                            } else {
                                left
                            };
                            {
                                let right_3 = if ((65f64..=90f64).contains(&right)) {
                                    { (right + 32f64) }
                                } else {
                                    right
                                };
                                {
                                    if left_3 == right_3 {
                                        {
                                            let offset_3 = (offset + 1f64);
                                            {
                                                offset = offset_3;
                                                continue;
                                            }
                                        }
                                    }
                                    {
                                        {
                                            let equal_2 = false;
                                            {
                                                let offset_2 = (offset + 1f64);
                                                {
                                                    (equal, offset) = (equal_2, offset_2);
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
            { equal }
        };
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_language_loop21(
    units: &[f64],
    mut starts: Vec<f64>,
    mut lengths: Vec<f64>,
    mut part: f64,
    mut source_ml_temporary_values9: Vec<f64>,
    mut source_ml_temporary_index9: super::ml::Big,
) -> super::MlTemporaryAcceptResourceDescriptionLanguageLoop21Result {
    loop {
        return if (source_ml_temporary_index9
            < (super::ml::Big::from_u128(source_ml_temporary_values9.len() as u128)))
        {
            {
                let earlier = super::ml_array::at(
                    &source_ml_temporary_values9,
                    source_ml_temporary_index9.to_index(),
                );
                if ((super::resource_description_unit(&lengths, earlier))
                    == (super::resource_description_unit(&lengths, part)))
                {
                    {
                        let equal = true;
                        {
                            let offset = 0f64;
                            {
                                let equal_2 = super::source_ml_temporary_accept_resource_description_language_loop22(units, starts.clone(), lengths.clone(), part, earlier, equal, offset);
                                if equal_2 {
                                    super::MlTemporaryAcceptResourceDescriptionLanguageLoop21Result::MlTemporaryAcceptResourceDescriptionLanguageLoop21Return(false)
                                } else {
                                    {
                                        let source_ml_temporary_index9_2 =
                                            source_ml_temporary_index9
                                                .add(&super::ml::Big::from_i128(1));
                                        {
                                            source_ml_temporary_index9 =
                                                source_ml_temporary_index9_2.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index9_3 =
                            source_ml_temporary_index9.add(&super::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index9 = source_ml_temporary_index9_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            super::MlTemporaryAcceptResourceDescriptionLanguageLoop21Result::MlTemporaryAcceptResourceDescriptionLanguageLoop21Done
        };
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_language_loop17(
    units: &[f64],
    mut starts: Vec<f64>,
    mut lengths: Vec<f64>,
    mut part: f64,
    mut variants: Vec<f64>,
) -> super::MlTemporaryAcceptResourceDescriptionLanguageLoop17Result {
    loop {
        return if ((part < (starts.len() as f64))
            && (((super::resource_description_unit(&lengths, part)) >= (5f64))
                || (((super::resource_description_unit(&lengths, part)) == (4f64))
                    && super::resource_description_digit(super::resource_description_unit(
                        units,
                        super::resource_description_unit(&starts, part),
                    )))))
        {
            {
                let source_ml_temporary_values9 = variants.clone();
                {
                    let source_ml_temporary_index9 = super::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s23 =
                            super::source_ml_temporary_accept_resource_description_language_loop21(
                                units,
                                starts.clone(),
                                lengths.clone(),
                                part,
                                source_ml_temporary_values9.clone(),
                                source_ml_temporary_index9.clone(),
                            );
                        match source_ml_source1_s23 {
                            super::MlTemporaryAcceptResourceDescriptionLanguageLoop21Result::MlTemporaryAcceptResourceDescriptionLanguageLoop21Done => {
                                {
                                    let variants_2 = {
                                        let source_ml_source1_concat_value24 = variants.clone();
                                        {
                                            let source_ml_source1_concat_value25 = vec![part];
                                            super::ml_array::append(super::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value24), source_ml_source1_concat_value25)
                                        }
                                    };
                                    {
                                        let part_2 = (part + 1f64);
                                        {
                                            (part, variants) = (part_2, variants_2.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                            super::MlTemporaryAcceptResourceDescriptionLanguageLoop21Result::MlTemporaryAcceptResourceDescriptionLanguageLoop21Return(source_ml_temporary_result21) => {
                                super::MlTemporaryAcceptResourceDescriptionLanguageLoop17Result::MlTemporaryAcceptResourceDescriptionLanguageLoop17Return(source_ml_temporary_result21)
                            }
                        }
                    }
                }
            }
        } else {
            super::MlTemporaryAcceptResourceDescriptionLanguageLoop17Result::MlTemporaryAcceptResourceDescriptionLanguageLoop17Done(part)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptResourceDescriptionLanguageLoop13Result {
    MlTemporaryAcceptResourceDescriptionLanguageLoop13Done,
    MlTemporaryAcceptResourceDescriptionLanguageLoop13Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_language_loop13(
    units: &[f64],
    mut lengths: Vec<f64>,
    mut index: f64,
) -> super::MlTemporaryAcceptResourceDescriptionLanguageLoop13Result {
    loop {
        return if (index < (super::resource_description_unit(&lengths, 0f64))) {
            {
                if (super::resource_description_letter(super::resource_description_unit(
                    units, index,
                ))) {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
                {
                    super::MlTemporaryAcceptResourceDescriptionLanguageLoop13Result::MlTemporaryAcceptResourceDescriptionLanguageLoop13Return(false)
                }
            }
        } else {
            super::MlTemporaryAcceptResourceDescriptionLanguageLoop13Result::MlTemporaryAcceptResourceDescriptionLanguageLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAcceptResourceDescriptionLanguageLoop11Result {
    MlTemporaryAcceptResourceDescriptionLanguageLoop11Done(Vec<f64>, Vec<f64>),
    MlTemporaryAcceptResourceDescriptionLanguageLoop11Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_accept_resource_description_language_loop11(
    units: &[f64],
    mut starts: Vec<f64>,
    mut lengths: Vec<f64>,
    mut index: f64,
) -> super::MlTemporaryAcceptResourceDescriptionLanguageLoop11Result {
    loop {
        return if (index <= (units.len() as f64)) {
            if ((index == (units.len() as f64))
                || ((super::resource_description_unit(units, index)) == (45f64)))
            {
                {
                    let length = (index
                        - super::resource_description_unit(&starts, (starts.len() as f64) - 1f64));
                    if ((length < (1f64)) || (length > (8f64))) {
                        super::MlTemporaryAcceptResourceDescriptionLanguageLoop11Result::MlTemporaryAcceptResourceDescriptionLanguageLoop11Return(false)
                    } else {
                        {
                            let lengths_2 = {
                                let source_ml_source1_concat_value26 = lengths.clone();
                                {
                                    let source_ml_source1_concat_value27 = vec![length];
                                    super::ml_array::append(
                                        super::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value26,
                                        ),
                                        source_ml_source1_concat_value27,
                                    )
                                }
                            };
                            {
                                if (index < (units.len() as f64)) {
                                    {
                                        let starts_2 = {
                                            let source_ml_source1_concat_value28 = starts.clone();
                                            {
                                                let source_ml_source1_concat_value29 =
                                                    vec![(index + 1f64)];
                                                super::ml_array::append(
                                                    super::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source1_concat_value28,
                                                    ),
                                                    source_ml_source1_concat_value29,
                                                )
                                            }
                                        };
                                        {
                                            let index_2 = (index + 1f64);
                                            {
                                                (starts, lengths, index) =
                                                    (starts_2.clone(), lengths_2.clone(), index_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let index_3 = (index + 1f64);
                                        {
                                            (lengths, index) = (lengths_2.clone(), index_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else if (!(super::resource_description_letter(super::resource_description_unit(
                units, index,
            ))) && !(super::resource_description_digit(
                super::resource_description_unit(units, index),
            ))) {
                super::MlTemporaryAcceptResourceDescriptionLanguageLoop11Result::MlTemporaryAcceptResourceDescriptionLanguageLoop11Return(false)
            } else {
                {
                    let index_4 = (index + 1f64);
                    {
                        index = index_4;
                        continue;
                    }
                }
            }
        } else {
            super::MlTemporaryAcceptResourceDescriptionLanguageLoop11Result::MlTemporaryAcceptResourceDescriptionLanguageLoop11Done(starts, lengths)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporarySameResourceDescriptionTermLoop24Result {
    MlTemporarySameResourceDescriptionTermLoop24Done,
    MlTemporarySameResourceDescriptionTermLoop24Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn source_ml_temporary_same_resource_description_term_loop24(
    left_fields: &[String],
    right_fields: &[String],
    mut count: f64,
    mut index: f64,
) -> super::MlTemporarySameResourceDescriptionTermLoop24Result {
    loop {
        return if (index < count) {
            if {
                let source_ml_source1_comparison_operand30 =
                    super::ml_array::optional_at(left_fields, super::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand31 = super::ml_array::optional_at(
                        right_fields,
                        super::ml_array::number_index(index),
                    );
                    !(match (
                        source_ml_source1_comparison_operand30,
                        source_ml_source1_comparison_operand31,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } {
                super::MlTemporarySameResourceDescriptionTermLoop24Result::MlTemporarySameResourceDescriptionTermLoop24Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            super::MlTemporarySameResourceDescriptionTermLoop24Result::MlTemporarySameResourceDescriptionTermLoop24Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporarySameResourceDescriptionTermJoin23 {
    MlTemporarySameResourceDescriptionTermJoin23Next(f64),
    MlTemporarySameResourceDescriptionTermJoin23Return(bool),
}
