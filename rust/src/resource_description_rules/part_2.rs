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
pub fn ml_accept_resource_description_language_loop22(
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
pub fn ml_accept_resource_description_language_loop21(
    units: &[f64],
    mut starts: Vec<f64>,
    mut lengths: Vec<f64>,
    mut part: f64,
    mut ml_values9: Vec<f64>,
    mut ml_index9: super::ml::Big,
) -> super::MlAcceptResourceDescriptionLanguageLoop21Result {
    loop {
        return if (ml_index9 < (super::ml::Big::from_u128(ml_values9.len() as u128))) {
            {
                let earlier = super::ml_array::at(&ml_values9, ml_index9.to_index());
                if ((super::resource_description_unit(&lengths, earlier))
                    == (super::resource_description_unit(&lengths, part)))
                {
                    {
                        let equal = true;
                        {
                            let offset = 0f64;
                            {
                                let equal_2 = super::ml_accept_resource_description_language_loop22(
                                    units,
                                    starts,
                                    lengths.clone(),
                                    part,
                                    earlier,
                                    equal,
                                    offset,
                                );
                                if equal_2 {
                                    super::MlAcceptResourceDescriptionLanguageLoop21Result::MlAcceptResourceDescriptionLanguageLoop21Return(false)
                                } else {
                                    {
                                        let ml_index9_2 =
                                            ml_index9.add(&super::ml::Big::from_i128(1));
                                        {
                                            ml_index9 = ml_index9_2.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let ml_index9_3 = ml_index9.add(&super::ml::Big::from_i128(1));
                        {
                            ml_index9 = ml_index9_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            super::MlAcceptResourceDescriptionLanguageLoop21Result::MlAcceptResourceDescriptionLanguageLoop21Done
        };
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn ml_accept_resource_description_language_loop17(
    units: &[f64],
    mut starts: Vec<f64>,
    mut lengths: Vec<f64>,
    mut part: f64,
    mut variants: Vec<f64>,
) -> super::MlAcceptResourceDescriptionLanguageLoop17Result {
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
                let ml_values9 = variants.clone();
                {
                    let ml_index9 = super::ml::Big::from_i128(0);
                    {
                        let ml_s21 = super::ml_accept_resource_description_language_loop21(
                            units,
                            starts.clone(),
                            lengths.clone(),
                            part,
                            ml_values9.clone(),
                            ml_index9.clone(),
                        );
                        match ml_s21 {
                            super::MlAcceptResourceDescriptionLanguageLoop21Result::MlAcceptResourceDescriptionLanguageLoop21Done => {
                                {
                                    let variants_2 = super::ml_array::append(variants.clone(), vec![part]);
                                    {
                                        let part_2 = (part + 1f64);
                                        {
                                            (part, variants) = (part_2, variants_2.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                            super::MlAcceptResourceDescriptionLanguageLoop21Result::MlAcceptResourceDescriptionLanguageLoop21Return(ml_result21) => {
                                super::MlAcceptResourceDescriptionLanguageLoop17Result::MlAcceptResourceDescriptionLanguageLoop17Return(ml_result21)
                            }
                        }
                    }
                }
            }
        } else {
            super::MlAcceptResourceDescriptionLanguageLoop17Result::MlAcceptResourceDescriptionLanguageLoop17Done(part)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptResourceDescriptionLanguageLoop13Result {
    MlAcceptResourceDescriptionLanguageLoop13Done,
    MlAcceptResourceDescriptionLanguageLoop13Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn ml_accept_resource_description_language_loop13(
    units: &[f64],
    mut lengths: Vec<f64>,
    mut index: f64,
) -> super::MlAcceptResourceDescriptionLanguageLoop13Result {
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
                    super::MlAcceptResourceDescriptionLanguageLoop13Result::MlAcceptResourceDescriptionLanguageLoop13Return(false)
                }
            }
        } else {
            super::MlAcceptResourceDescriptionLanguageLoop13Result::MlAcceptResourceDescriptionLanguageLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptResourceDescriptionLanguageLoop11Result {
    MlAcceptResourceDescriptionLanguageLoop11Done(Vec<f64>, Vec<f64>),
    MlAcceptResourceDescriptionLanguageLoop11Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn ml_accept_resource_description_language_loop11(
    units: &[f64],
    mut starts: Vec<f64>,
    mut lengths: Vec<f64>,
    mut index: f64,
) -> super::MlAcceptResourceDescriptionLanguageLoop11Result {
    loop {
        return if (index <= (units.len() as f64)) {
            if ((index == (units.len() as f64))
                || ((super::resource_description_unit(units, index)) == (45f64)))
            {
                {
                    let length = (index
                        - super::resource_description_unit(&starts, (starts.len() as f64) - 1f64));
                    if ((length < (1f64)) || (length > (8f64))) {
                        super::MlAcceptResourceDescriptionLanguageLoop11Result::MlAcceptResourceDescriptionLanguageLoop11Return(false)
                    } else {
                        {
                            let lengths_2 = super::ml_array::append(lengths.clone(), vec![length]);
                            {
                                if (index < (units.len() as f64)) {
                                    {
                                        let starts_2 = super::ml_array::append(
                                            starts.clone(),
                                            vec![(index + 1f64)],
                                        );
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
                super::MlAcceptResourceDescriptionLanguageLoop11Result::MlAcceptResourceDescriptionLanguageLoop11Return(false)
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
            super::MlAcceptResourceDescriptionLanguageLoop11Result::MlAcceptResourceDescriptionLanguageLoop11Done(starts, lengths)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSameResourceDescriptionTermLoop24Result {
    MlSameResourceDescriptionTermLoop24Done,
    MlSameResourceDescriptionTermLoop24Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn ml_same_resource_description_term_loop24(
    left_fields: &[String],
    right_fields: &[String],
    mut count: f64,
    mut index: f64,
) -> super::MlSameResourceDescriptionTermLoop24Result {
    loop {
        return if (index < count) {
            {
                if (match (
                    super::ml_array::optional_at(left_fields, super::ml_array::number_index(index)),
                    super::ml_array::optional_at(
                        right_fields,
                        super::ml_array::number_index(index),
                    ),
                ) {
                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                    (None, None) => true,
                    _ => false,
                }) {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
                {
                    super::MlSameResourceDescriptionTermLoop24Result::MlSameResourceDescriptionTermLoop24Return(false)
                }
            }
        } else {
            super::MlSameResourceDescriptionTermLoop24Result::MlSameResourceDescriptionTermLoop24Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSameResourceDescriptionTermJoin23 {
    MlSameResourceDescriptionTermJoin23Next(f64),
    MlSameResourceDescriptionTermJoin23Return(bool),
}
