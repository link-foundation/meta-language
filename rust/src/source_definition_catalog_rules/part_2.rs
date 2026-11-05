// Generated from complete js/src/source-definition-catalog-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Result;
use super::accept_source_definition_catalog_asset;
use super::accept_source_definition_catalog_digest;
use super::accept_source_definition_catalog_root;
use super::ml_array;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_source_definition_catalog_digest_loop2(
    text: &self::ml_string::Text,
    mut index: f64,
) -> self::MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Result {
    loop {
        return if index < (text.length()) {
            {
                let code = {
                    let source_ml_source1_string_unit_operand41 = text.clone();
                    {
                        let source_ml_source1_string_unit_operand42 = index;
                        (std::rc::Rc::new(
                            move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                        f64::NAN
                                    } else {
                                        ({
                                    let source_ml_source1_string_unit_operand48 = {
                                        let source_ml_source1_string_range_operand43 = text_2.clone();
                                        {
                                            let source_ml_source1_string_range_operand44 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand45 = index_2 + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position46 = source_ml_source1_string_range_operand44;
                                                    {
                                                        let source_ml_source1_string_range_position47 = source_ml_source1_string_range_operand45;
                                                        source_ml_source1_string_range_operand43.slice(source_ml_source1_string_range_position46, source_ml_source1_string_range_position47)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand49 = 0f64;
                                        source_ml_source1_string_unit_operand48.code_point_at(source_ml_source1_string_unit_operand49)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand41,
                            source_ml_source1_string_unit_operand42,
                        )
                    }
                };
                if (((48f64..=57f64).contains(&code)) || ((97f64..=102f64).contains(&code))) {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Result::MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Return(false)
                }
            }
        } else {
            self::MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Result::MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Result {
    MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Done,
    MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_source_definition_catalog_root_loop3(
    text: &self::ml_string::Text,
    mut index: f64,
) -> self::MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Result {
    loop {
        return if index < (text.length()) {
            {
                let code = {
                    let source_ml_source1_string_unit_operand50 = text.clone();
                    {
                        let source_ml_source1_string_unit_operand51 = index;
                        (std::rc::Rc::new(
                            move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                        f64::NAN
                                    } else {
                                        ({
                                    let source_ml_source1_string_unit_operand57 = {
                                        let source_ml_source1_string_range_operand52 = text_2.clone();
                                        {
                                            let source_ml_source1_string_range_operand53 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand54 = index_2 + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position55 = source_ml_source1_string_range_operand53;
                                                    {
                                                        let source_ml_source1_string_range_position56 = source_ml_source1_string_range_operand54;
                                                        source_ml_source1_string_range_operand52.slice(source_ml_source1_string_range_position55, source_ml_source1_string_range_position56)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand58 = 0f64;
                                        source_ml_source1_string_unit_operand57.code_point_at(source_ml_source1_string_unit_operand58)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand50,
                            source_ml_source1_string_unit_operand51,
                        )
                    }
                };
                if (code < 48f64) || (code > 57f64) {
                    self::MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Result::MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Return(false)
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Result::MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Result {
    MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Done,
    MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryValidateSourceDefinitionCatalogEntriesLoop5Result {
    MlTemporaryValidateSourceDefinitionCatalogEntriesLoop5Done,
    MlTemporaryValidateSourceDefinitionCatalogEntriesLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_validate_source_definition_catalog_entries_loop5(
    identifiers: &[self::ml_string::Text],
    assets: &[self::ml_string::Text],
    mut index: f64,
    mut previous: f64,
) -> self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop5Result {
    loop {
        return if previous < index {
            if {
                let source_ml_source1_comparison_operand59 = self::ml_array::optional_at(
                    identifiers,
                    self::ml_array::number_index(previous),
                );
                {
                    let source_ml_source1_comparison_operand60 = self::ml_array::optional_at(
                        identifiers,
                        self::ml_array::number_index(index),
                    );
                    match (
                        source_ml_source1_comparison_operand59,
                        source_ml_source1_comparison_operand60,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            } || {
                let source_ml_source1_comparison_operand61 =
                    self::ml_array::optional_at(assets, self::ml_array::number_index(previous));
                {
                    let source_ml_source1_comparison_operand62 =
                        self::ml_array::optional_at(assets, self::ml_array::number_index(index));
                    match (
                        source_ml_source1_comparison_operand61,
                        source_ml_source1_comparison_operand62,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            } {
                self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop5Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop5Return(-7f64)
            } else {
                {
                    let previous_2 = previous + 1f64;
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop5Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop5Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_validate_source_definition_catalog_entries_loop4(
    identifiers: &[self::ml_string::Text],
    assets: &[self::ml_string::Text],
    digests: &[self::ml_string::Text],
    bytes: &[f64],
    roots: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Result {
    loop {
        return if index < (identifiers.len() as f64) {
            {
                let identifier =
                    (self::ml_array::optional_at(identifiers, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let asset =
                        (self::ml_array::optional_at(assets, self::ml_array::number_index(index)))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let digest = (self::ml_array::optional_at(
                            digests,
                            self::ml_array::number_index(index),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let count = (self::ml_array::optional_at(
                                bytes,
                                self::ml_array::number_index(index),
                            ))
                            .unwrap_or_else(|| 0f64);
                            {
                                let root = (self::ml_array::optional_at(
                                    roots,
                                    self::ml_array::number_index(index),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                if (self::accept_source_definition_catalog_asset(&(asset))) {
                                    if identifier != ((self::ml_string::Text::from_units(vec![109, 101, 116, 97, 45, 108, 97, 110, 103, 117, 97, 103, 101, 47, 115, 111, 117, 114, 99, 101, 45, 112, 114, 111, 103, 114, 97, 109, 47])).concat(&{
                                        let source_ml_source1_string_range_operand68 = asset.clone();
                                        {
                                            let source_ml_source1_string_range_operand69 = 0f64;
                                            {
                                                let source_ml_source1_string_range_operand70 = asset.length() - 5f64;
                                                {
                                                    let source_ml_source1_string_range_position71 = source_ml_source1_string_range_operand69;
                                                    {
                                                        let source_ml_source1_string_range_position72 = source_ml_source1_string_range_operand70;
                                                        source_ml_source1_string_range_operand68.slice(source_ml_source1_string_range_position71, source_ml_source1_string_range_position72)
                                                    }
                                                }
                                            }
                                        }
                                    })) {
                                        self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Return(-3f64)
                                    } else if (self::accept_source_definition_catalog_digest(&(digest))) {if ((count < 1f64) || (count > 134_217_728f64)) || ((count % 1f64) != 0f64) {
                                                self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Return(-5f64)
                                            } else if (self::accept_source_definition_catalog_root(&(root))) {
                                                    {
                                                        let previous = 0f64;
                                                        {
                                                            let source_ml_source1_s73 = self::source_ml_temporary_validate_source_definition_catalog_entries_loop5(identifiers, assets, index, previous);
                                                            match source_ml_source1_s73 {
                                                                self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop5Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop5Done => {
                                                                    {
                                                                        let index_2 = index + 1f64;
                                                                        {
                                                                            index = index_2;
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                                self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop5Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop5Return(source_ml_temporary_result5) => {
                                                                    self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Return(source_ml_temporary_result5)
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Return(-6f64)
                                                }} else {
                                            self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Return(-4f64)
                                        }
                                } else {
                                    self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Return(-2f64)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporarySelectSourceDefinitionCatalogEntryLoop6Result {
    MlTemporarySelectSourceDefinitionCatalogEntryLoop6Done,
    MlTemporarySelectSourceDefinitionCatalogEntryLoop6Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_select_source_definition_catalog_entry_loop6(
    identifiers: &[self::ml_string::Text],
    identifier: &self::ml_string::Text,
    mut index: f64,
) -> self::MlTemporarySelectSourceDefinitionCatalogEntryLoop6Result {
    loop {
        return if index < (identifiers.len() as f64) {
            if {
                let source_ml_source1_comparison_operand74 =
                    self::ml_array::optional_at(identifiers, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand75 = identifier.clone();
                    match (
                        source_ml_source1_comparison_operand74,
                        source_ml_source1_comparison_operand75,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } {
                self::MlTemporarySelectSourceDefinitionCatalogEntryLoop6Result::MlTemporarySelectSourceDefinitionCatalogEntryLoop6Return(index)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporarySelectSourceDefinitionCatalogEntryLoop6Result::MlTemporarySelectSourceDefinitionCatalogEntryLoop6Done
        };
    }
}
