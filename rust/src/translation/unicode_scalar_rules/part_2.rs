// Generated from complete js/src/translation/unicode-scalar-rules.js. Do not edit by hand.
use super::MlNormalizeUnicodeScalarsJoin8;
use super::MlNormalizeUnicodeScalarsJoin17;
use super::MlNormalizeUnicodeScalarsLoop4Result;
use super::MlNormalizeUnicodeScalarsLoop12Result;
use super::MlNormalizeUnicodeScalarsLoop15Result;
use super::accept_unicode_scalar;
use super::find_unicode_table_row;
use super::ml_array;
use super::ml_normalize_unicode_scalars_loop12;
use super::read_unicode_combining_class;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_normalize_unicode_scalars_loop15(
    mut classes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut limit: f64,
    mut ordered: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut steps: f64,
    mut combining: f64,
    mut position: f64,
) -> self::MlNormalizeUnicodeScalarsLoop15Result {
    loop {
        return if (combining > 0f64) && (position > 0f64) {
            {
                let steps_2 = steps + 1f64;
                {
                    assert!(
                        !(steps_2 > limit),
                        "{}",
                        "Unicode normalization resource limit exceeded"
                    );

                    {
                        let previous = (self::ml_array::optional_at(
                            &{ ordered.borrow().clone() },
                            self::ml_array::number_index(position - 1f64),
                        ))
                        .unwrap_or_else(|| -1f64);
                        if (self::read_unicode_combining_class(classes.clone(), previous))
                            <= combining
                        {
                            self::MlNormalizeUnicodeScalarsLoop15Result::MlNormalizeUnicodeScalarsLoop15Done(steps_2, position)
                        } else {
                            {
                                let ml_s6 = if position
                                    < (({ ordered.borrow().clone() }).len() as f64)
                                {
                                    {
                                        let ml_normalize_unicode_scalars_ignored18 = {
                                            let ml_array_owner21 = ordered.clone();
                                            let ml_array_argument21_0: f64 = position;
                                            let ml_array_argument21_1: f64 = 1f64;
                                            let ml_array_argument21_2: f64 = previous;
                                            let ml_array_length = ml_array_owner21.borrow().len();
                                            let ml_array_start = {
                                                let ml_relative_value: f64 = ml_array_argument21_0;
                                                let ml_relative = if ml_relative_value.is_nan() {
                                                    0f64
                                                } else {
                                                    ml_relative_value.trunc()
                                                };
                                                let ml_bound = ml_array_length as f64;
                                                (if ml_relative < 0f64 {
                                                    (ml_bound + ml_relative).clamp(0f64, ml_bound)
                                                } else {
                                                    ml_relative.min(ml_bound)
                                                })
                                                    as usize
                                            };
                                            let ml_array_count = {
                                                let ml_delete_value: f64 = ml_array_argument21_1;
                                                ml_delete_value
                                                    .trunc()
                                                    .max(0f64)
                                                    .min((ml_array_length - ml_array_start) as f64)
                                                    as usize
                                            };
                                            let ml_array_inserted = vec![ml_array_argument21_2];
                                            let _ = (ml_array_length - ml_array_count)
                                                .checked_add(ml_array_inserted.len())
                                                .filter(|length| *length <= 4_294_967_295usize)
                                                .expect("Invalid array length");
                                            let ml_removed: Vec<_> = ml_array_owner21
                                                .borrow_mut()
                                                .splice(
                                                    ml_array_start..ml_array_start + ml_array_count,
                                                    ml_array_inserted,
                                                )
                                                .collect();
                                            std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                                        };
                                        self::MlNormalizeUnicodeScalarsJoin17::MlNormalizeUnicodeScalarsJoin17Next
                                    }
                                } else {
                                    self::MlNormalizeUnicodeScalarsJoin17::MlNormalizeUnicodeScalarsJoin17Next
                                };
                                match ml_s6 {
                                    self::MlNormalizeUnicodeScalarsJoin17::MlNormalizeUnicodeScalarsJoin17Next => {
                                        {
                                            let position_2 = position - 1f64;
                                            {
                                                (steps, position) = (steps_2, position_2);
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
            self::MlNormalizeUnicodeScalarsLoop15Result::MlNormalizeUnicodeScalarsLoop15Done(
                steps, position,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_normalize_unicode_scalars_loop4(
    mut classes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut decompositions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut limit: f64,
    mut stack: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut ordered: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut steps: f64,
) -> self::MlNormalizeUnicodeScalarsLoop4Result {
    loop {
        return if (({ stack.borrow().clone() }).len() as f64) > 0f64 {
            {
                let steps_2 = steps + 1f64;
                {
                    assert!(
                        !(steps_2 > limit),
                        "{}",
                        "Unicode normalization resource limit exceeded"
                    );

                    {
                        let scalar = ({
                            let ml_method_owner23 = stack.clone();
                            ml_method_owner23.borrow_mut().pop()
                        })
                        .clone()
                        .unwrap_or_else(|| -1f64);
                        if (self::accept_unicode_scalar(scalar)) {
                            if (44_032f64..55_204f64).contains(&scalar) {
                                {
                                    let syllable = scalar - 44_032f64;
                                    {
                                        let tail = syllable % 28f64;
                                        {
                                            let ml_s7 = {
                                                let ml_library_inequality_1 = tail != 0f64;
                                                if ml_library_inequality_1 {
                                                    {
                                                        let ml_normalize_unicode_scalars_ignored11 = {
                                                            let ml_method_owner24 = stack.clone();
                                                            let ml_method_argument24_0 =
                                                                4519f64 + tail;
                                                            let ml_inserted =
                                                                vec![ml_method_argument24_0];
                                                            let mut ml_method_values24 =
                                                                ml_method_owner24.borrow_mut();
                                                            let ml_length = ml_method_values24
                                                                .len()
                                                                .checked_add(ml_inserted.len())
                                                                .filter(|length| {
                                                                    *length <= 4_294_967_295usize
                                                                })
                                                                .expect("Invalid array length");
                                                            ml_method_values24.extend(ml_inserted);
                                                            ml_length as f64
                                                        };
                                                        self::MlNormalizeUnicodeScalarsJoin8::MlNormalizeUnicodeScalarsJoin8Next
                                                    }
                                                } else {
                                                    self::MlNormalizeUnicodeScalarsJoin8::MlNormalizeUnicodeScalarsJoin8Next
                                                }
                                            };
                                            match ml_s7 {
                                                self::MlNormalizeUnicodeScalarsJoin8::MlNormalizeUnicodeScalarsJoin8Next => {
                                                    {
                                                        let ml_normalize_unicode_scalars_ignored9 = { let ml_method_owner25 = stack.clone(); let ml_method_argument25_0 = 4449f64 + ((syllable / 28f64).floor() % 21f64); let ml_inserted = vec![ml_method_argument25_0]; let mut ml_method_values25 = ml_method_owner25.borrow_mut(); let ml_length = ml_method_values25.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values25.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let ml_normalize_unicode_scalars_ignored10 = { let ml_method_owner26 = stack.clone(); let ml_method_argument26_0 = 4352f64 + (syllable / 588f64).floor(); let ml_inserted = vec![ml_method_argument26_0]; let mut ml_method_values26 = ml_method_owner26.borrow_mut(); let ml_length = ml_method_values26.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values26.extend(ml_inserted); ml_length as f64 };
                                                            {
                                                                steps = steps_2;
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
                                {
                                    let row = self::find_unicode_table_row(
                                        decompositions.clone(),
                                        3f64,
                                        scalar,
                                    );
                                    if row >= 0f64 {
                                        {
                                            let offset = (self::ml_array::optional_at(
                                                &{ decompositions.borrow().clone() },
                                                self::ml_array::number_index(row + 1f64),
                                            ))
                                            .unwrap_or_else(|| -1f64);
                                            {
                                                let count = (self::ml_array::optional_at(
                                                    &{ decompositions.borrow().clone() },
                                                    self::ml_array::number_index(row + 2f64),
                                                ))
                                                .unwrap_or_else(|| -1f64);
                                                {
                                                    assert!(
                                                        !((((offset < 0f64) || (count < 1f64))
                                                            || ((offset + count)
                                                                > (({ values.borrow().clone() })
                                                                    .len()
                                                                    as f64)))
                                                            || (((({ stack.borrow().clone() }).len()
                                                                as f64)
                                                                + count)
                                                                > limit)),
                                                        "{}",
                                                        "invalid Unicode canonical decomposition table"
                                                    );

                                                    {
                                                        let index = count;
                                                        {
                                                            let ml_s8 = self::ml_normalize_unicode_scalars_loop12(values.clone(), stack.clone(), offset, index);
                                                            match ml_s8 {
                                                                self::MlNormalizeUnicodeScalarsLoop12Result::MlNormalizeUnicodeScalarsLoop12Done => {
                                                                    {
                                                                        steps = steps_2;
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
                                        {
                                            let combining = self::read_unicode_combining_class(
                                                classes.clone(),
                                                scalar,
                                            );
                                            {
                                                let position =
                                                    ({ ordered.borrow().clone() }).len() as f64;
                                                {
                                                    let ml_normalize_unicode_scalars_ignored14 = {
                                                        let ml_method_owner32 = ordered.clone();
                                                        let ml_method_argument32_0 = scalar;
                                                        let ml_inserted =
                                                            vec![ml_method_argument32_0];
                                                        let mut ml_method_values32 =
                                                            ml_method_owner32.borrow_mut();
                                                        let ml_length = ml_method_values32
                                                            .len()
                                                            .checked_add(ml_inserted.len())
                                                            .filter(|length| {
                                                                *length <= 4_294_967_295usize
                                                            })
                                                            .expect("Invalid array length");
                                                        ml_method_values32.extend(ml_inserted);
                                                        ml_length as f64
                                                    };
                                                    {
                                                        let ml_s9 = self::ml_normalize_unicode_scalars_loop15(classes.clone(), limit, ordered.clone(), steps_2, combining, position);
                                                        match ml_s9 {
                                                            self::MlNormalizeUnicodeScalarsLoop15Result::MlNormalizeUnicodeScalarsLoop15Done(steps_3, position_2) => {
                                                                if position_2 < (({ ordered.borrow().clone() }).len() as f64) {
                                                                    {
                                                                        let ml_normalize_unicode_scalars_ignored16 = { let ml_array_owner34 = ordered.clone(); let ml_array_argument34_0: f64 = position_2; let ml_array_argument34_1: f64 = 1f64; let ml_array_argument34_2: f64 = scalar; let ml_array_length = ml_array_owner34.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument34_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument34_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument34_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner34.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                                                        {
                                                                            steps = steps_3;
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
{
                                                                    {
                                                                        steps = steps_3;
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
                            }
                        } else {
                            panic!("{}", "Unicode normalization requires scalar values")
                        }
                    }
                }
            }
        } else {
            self::MlNormalizeUnicodeScalarsLoop4Result::MlNormalizeUnicodeScalarsLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlNormalizeUnicodeScalarsLoop3Result {
    MlNormalizeUnicodeScalarsLoop3Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_normalize_unicode_scalars_loop3(
    mut input: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut stack: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlNormalizeUnicodeScalarsLoop3Result {
    loop {
        if index > 0f64 {
            {
                let ml_normalize_unicode_scalars_ignored19 = {
                    let ml_method_owner35 = stack.clone();
                    let ml_method_argument35_0 = (self::ml_array::optional_at(
                        &{ input.borrow().clone() },
                        self::ml_array::number_index(index - 1f64),
                    ))
                    .unwrap_or_else(|| -1f64);
                    let ml_inserted = vec![ml_method_argument35_0];
                    let mut ml_method_values35 = ml_method_owner35.borrow_mut();
                    let ml_length = ml_method_values35
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values35.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let index_2 = index - 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        }
        return { self::MlNormalizeUnicodeScalarsLoop3Result::MlNormalizeUnicodeScalarsLoop3Done };
    }
}
