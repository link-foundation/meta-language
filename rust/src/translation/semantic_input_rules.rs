// Generated from complete semantic-input-rules.js. Do not edit by hand.
// JavaScript Number equality and separate arithmetic rounding are preserved.
// Array index casts retain the existing checked runtime guards.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons,
    clippy::float_cmp,
    clippy::suboptimal_flops,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    #[must_use]
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    const fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> f64 {
        if values.is_empty() {
            return -1.0;
        }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) {
                return -1.0;
            }
            let start = if from < 0.0 {
                (values.len() as f64 + from) as usize
            } else {
                from.min((values.len() - 1) as f64) as usize
            };
            (0..=start)
                .rev()
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 {
                (values.len() as f64 + from).max(0.0) as usize
            } else {
                from.min(values.len() as f64) as usize
            };
            (start..values.len())
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        }
    }
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) {
                out[index].1 = value;
            } else {
                out.push((key, value));
            }
        }
        out
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_semantic_source_role(comment: bool, text: &str) -> f64 {
    if comment {
        if text.starts_with("// meta-language:self-translation:v1 ") {
            2f64
        } else if text.starts_with("// meta-language:carried ") {
            3f64
        } else if text.starts_with("// meta-language:translated ") {
            4f64
        } else if (text == "// meta-language:prelude begin")
            || (text == "// meta-language:prelude end")
        {
            5f64
        } else if text.starts_with("// |") {
            6f64
        } else {
            1f64
        }
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_semantic_source_groups(roles: &[f64], line_breaks: &[bool]) -> Vec<f64> {
    {
        let plan = Vec::<f64>::new();
        {
            let index = 0f64;
            { self::ml_plan_semantic_source_groups_loop1(roles, line_breaks, plan, index) }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_semantic_source_groups_loop2(
    roles: &[f64],
    line_breaks: &[bool],
    mut end: f64,
) -> f64 {
    loop {
        return if match {
            let ml_logical1 = ((end + 1f64) < (roles.len() as f64))
                && match (
                    self::ml_array::optional_at(roles, self::ml_array::number_index(end + 1f64)),
                    6f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                };
            if ml_logical1 {
                self::ml_array::optional_at(line_breaks, self::ml_array::number_index(end))
            } else {
                Some(ml_logical1)
            }
        } {
            Some(ml_present) => ml_present,
            None => false,
        } {
            {
                let end_2 = end + 1f64;
                {
                    end = end_2;
                    continue;
                }
            }
        } else {
            end
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanSemanticSourceGroupsJoin3 {
    MlPlanSemanticSourceGroupsJoin3Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_semantic_source_groups_loop4(
    roles: &[f64],
    line_breaks: &[bool],
    mut end: f64,
) -> f64 {
    loop {
        return if match {
            let ml_logical3 = {
                let ml_logical2 = !(match (
                    self::ml_array::optional_at(roles, self::ml_array::number_index(end)),
                    0f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }) && ((end + 1f64) < (roles.len() as f64));
                if ml_logical2 {
                    self::ml_array::optional_at(line_breaks, self::ml_array::number_index(end))
                } else {
                    Some(ml_logical2)
                }
            };
            if (ml_logical3.clone()).unwrap_or_default() {
                Some(
                    !((((self::ml_array::optional_at(
                        roles,
                        self::ml_array::number_index(end + 1f64),
                    ))
                    .unwrap_or(f64::NAN))
                        >= 2f64)
                        && (((self::ml_array::optional_at(
                            roles,
                            self::ml_array::number_index(end + 1f64),
                        ))
                        .unwrap_or(f64::NAN))
                            <= 5f64)),
                )
            } else {
                ml_logical3.clone()
            }
        } {
            Some(ml_present) => ml_present,
            None => false,
        } {
            {
                let end_2 = end + 1f64;
                {
                    end = end_2;
                    continue;
                }
            }
        } else {
            end
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_semantic_source_groups_loop1(
    roles: &[f64],
    line_breaks: &[bool],
    mut plan: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if index < (roles.len() as f64) {
            {
                let role = self::ml_array::optional_at(roles, self::ml_array::number_index(index));
                {
                    let end = index;
                    {
                        let disposition = 0f64;
                        {
                            let end_3 = if match (role, 3f64) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            } || match (role, 4f64) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            } {
                                {
                                    self::ml_plan_semantic_source_groups_loop2(
                                        roles,
                                        line_breaks,
                                        end,
                                    )
                                }
                            } else {
                                end
                            };
                            {
                                let ml_s4 = if match (role, 3f64) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                } {
                                    {
                                        let disposition_2 = 1f64;
                                        self::MlPlanSemanticSourceGroupsJoin3::MlPlanSemanticSourceGroupsJoin3Next(end_3, disposition_2)
                                    }
                                } else if ((((role).unwrap_or(f64::NAN)) >= 2f64)
                                    && (((role).unwrap_or(f64::NAN)) <= 5f64))
                                {
                                    self::MlPlanSemanticSourceGroupsJoin3::MlPlanSemanticSourceGroupsJoin3Next(end_3, disposition)
                                } else {
                                    {
                                        let end_4 = self::ml_plan_semantic_source_groups_loop4(
                                            roles,
                                            line_breaks,
                                            end_3,
                                        );
                                        {
                                            let disposition_3 = if match (
                                                self::ml_array::optional_at(
                                                    roles,
                                                    self::ml_array::number_index(end_4),
                                                ),
                                                0f64,
                                            ) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            } {
                                                3f64
                                            } else {
                                                2f64
                                            };
                                            self::MlPlanSemanticSourceGroupsJoin3::MlPlanSemanticSourceGroupsJoin3Next(end_4, disposition_3)
                                        }
                                    }
                                };
                                match ml_s4 {
                                    self::MlPlanSemanticSourceGroupsJoin3::MlPlanSemanticSourceGroupsJoin3Next(end_5, disposition_4) => {
                                        {
                                            let plan_2 = { let (ml_append_right, ml_append_left) = (vec![index], plan); self::ml_array::append(ml_append_left, ml_append_right) };
                                            {
                                                let plan_3 = { let (ml_append_right, ml_append_left) = (vec![end_5], plan_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                {
                                                    let plan_4 = { let (ml_append_right, ml_append_left) = (vec![disposition_4], plan_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                    {
                                                        let index_2 = end_5;
                                                        {
                                                            let index_3 = index_2 + 1f64;
                                                            {
                                                                (plan, index) = (plan_4, index_3);
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
                }
            }
        } else {
            plan
        };
    }
}
