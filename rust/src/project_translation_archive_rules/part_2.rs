// Generated from complete js/src/project-translation-archive-rules.js. Do not edit by hand.
use super::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Result;
use super::ml;
use super::ml_array;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_project_translation_archive_binding_error_loop1(
    issues: &[String],
    actual_issues: &[String],
    gap_issues: &[String],
    mut index: f64,
) -> self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Result {
    loop {
        return if index < (issues.len() as f64) {
            if {
                let source_ml_source1_comparison_operand9 =
                    self::ml_array::optional_at(issues, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand10 = self::ml_array::optional_at(
                        actual_issues,
                        self::ml_array::number_index(index),
                    );
                    !(match (
                        source_ml_source1_comparison_operand9,
                        source_ml_source1_comparison_operand10,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } || {
                let source_ml_source1_comparison_operand11 =
                    self::ml_array::optional_at(issues, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand12 = self::ml_array::optional_at(
                        gap_issues,
                        self::ml_array::number_index(index),
                    );
                    !(match (
                        source_ml_source1_comparison_operand11,
                        source_ml_source1_comparison_operand12,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Result::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Return(String::from("project source issues or dependency directory changed"))
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
            self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Result::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result {
    MlTemporaryPlanProjectTranslationArchiveCommandLoop3Done(f64, f64, bool, bool),
    MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_project_translation_archive_command_loop3(
    args: &[String],
    mut file: f64,
    mut target: f64,
    mut abstract_: bool,
    mut binary: bool,
    mut ended: bool,
    mut at: f64,
) -> self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result {
    loop {
        return if at < (args.len() as f64) {
            {
                let arg = (self::ml_array::optional_at(args, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| String::new());
                if !ended && (((arg == "--help") || (arg == "-h")) || (arg == "help")) {
                    self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(vec![-1f64])
                } else {
                    if !ended && (arg == "--") {
                        {
                            let ended_2 = true;
                            {
                                let at_2 = at + 1f64;
                                {
                                    (ended, at) = (ended_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if !ended && (arg == "--abstract") {
                            if abstract_ {
                                self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(vec![-2f64])
                            } else {
                                {
                                    let abstract_2 = true;
                                    {
                                        let at_3 = at + 1f64;
                                        {
                                            (abstract_, at) = (abstract_2, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if !ended && (arg == "--binary") {
                            if binary {
                                self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(vec![-2f64])
                            } else {
                                {
                                    let binary_2 = true;
                                    {
                                        let at_4 = at + 1f64;
                                        {
                                            (binary, at) = (binary_2, at_4);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if !ended && (arg == "--to") {
                            if (target >= 0f64) || ((at + 1f64) >= (args.len() as f64)) {
                                self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(vec![-2f64])
                            } else {
                                {
                                    let at_5 = at + 1f64;
                                    {
                                        let target_2 = at_5;
                                        {
                                            let at_6 = at_5 + 1f64;
                                            {
                                                (target, at) = (target_2, at_6);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        } else if (file >= 0f64) || (!ended && arg.starts_with("-")) {
                            self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(vec![-2f64])
                        } else {
                            {
                                let file_2 = at;
                                {
                                    let at_7 = at + 1f64;
                                    {
                                        (file, at) = (file_2, at_7);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Done(file, target, abstract_, binary)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadProjectTranslationArchiveImportErrorLoop5Result {
    MlTemporaryReadProjectTranslationArchiveImportErrorLoop5Done,
    MlTemporaryReadProjectTranslationArchiveImportErrorLoop5Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_project_translation_archive_import_error_loop5(
    source_ml_temporary_values4: &[bool],
    mut source_ml_temporary_index4: self::ml::Big,
) -> self::MlTemporaryReadProjectTranslationArchiveImportErrorLoop5Result {
    loop {
        return if source_ml_temporary_index4
            < (self::ml::Big::from_u128(source_ml_temporary_values4.len() as u128))
        {
            {
                let known = self::ml_array::at(
                    source_ml_temporary_values4,
                    source_ml_temporary_index4.to_index(),
                );
                if known {
                    {
                        let source_ml_temporary_index4_2 =
                            source_ml_temporary_index4.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index4 = source_ml_temporary_index4_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlTemporaryReadProjectTranslationArchiveImportErrorLoop5Result::MlTemporaryReadProjectTranslationArchiveImportErrorLoop5Return(String::from("duplicate or foreign project source dependency"))
                }
            }
        } else {
            self::MlTemporaryReadProjectTranslationArchiveImportErrorLoop5Result::MlTemporaryReadProjectTranslationArchiveImportErrorLoop5Done
        };
    }
}
