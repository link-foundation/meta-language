// Generated from resource-description-ntriples-rules.js and its dependencies by the checked translator.
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
pub fn ml_read_resource_description_n_triples_token_loop6(
    units: &[f64],
    mut index: f64,
    mut beginning: f64,
) -> super::MlReadResourceDescriptionNTriplesTokenLoop6Result {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let scalar = super::read_resource_description_n_triples_scalar(units, index, 0f64);
                if (super::resource_description_unit(&scalar, 2f64)) == (0f64) {
                    if (super::resource_description_n_triples_name(
                        super::resource_description_unit(&scalar, 1f64),
                        index == beginning,
                    )) {
                        {
                            let index_2 = (index + super::resource_description_unit(&scalar, 0f64));
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                    {
                        super::MlReadResourceDescriptionNTriplesTokenLoop6Result::MlReadResourceDescriptionNTriplesTokenLoop6Done(index)
                    }
                } else {
                    super::MlReadResourceDescriptionNTriplesTokenLoop6Result::MlReadResourceDescriptionNTriplesTokenLoop6Return(vec![0f64, index, 1f64])
                }
            }
        } else {
            super::MlReadResourceDescriptionNTriplesTokenLoop6Result::MlReadResourceDescriptionNTriplesTokenLoop6Done(index)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadResourceDescriptionNTriplesTokenLoop8Result {
    MlReadResourceDescriptionNTriplesTokenLoop8Done(f64),
    MlReadResourceDescriptionNTriplesTokenLoop8Return(Vec<f64>),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn ml_read_resource_description_n_triples_token_loop8(
    units: &[f64],
    mut index: f64,
    mut kind: f64,
    mut close: f64,
) -> super::MlReadResourceDescriptionNTriplesTokenLoop8Result {
    loop {
        return if (index < (units.len() as f64)) {
            if ((super::resource_description_unit(units, index)) == close) {
                super::MlReadResourceDescriptionNTriplesTokenLoop8Result::MlReadResourceDescriptionNTriplesTokenLoop8Return(vec![kind, (index + 1f64), 0f64])
            } else {
                {
                    let scalar =
                        super::read_resource_description_n_triples_scalar(units, index, kind);
                    {
                        if (super::resource_description_unit(&scalar, 2f64)) == (0f64) {
                            {
                                let index_2 =
                                    (index + super::resource_description_unit(&scalar, 0f64));
                                {
                                    index = index_2;
                                    continue;
                                }
                            }
                        }
                        {
                            super::MlReadResourceDescriptionNTriplesTokenLoop8Result::MlReadResourceDescriptionNTriplesTokenLoop8Return(vec![0f64, index, super::resource_description_unit(&scalar, 2f64)])
                        }
                    }
                }
            }
        } else {
            super::MlReadResourceDescriptionNTriplesTokenLoop8Result::MlReadResourceDescriptionNTriplesTokenLoop8Done(index)
        };
    }
}
