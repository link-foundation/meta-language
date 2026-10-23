// Generated from complete semantic-input-rules.js. Do not edit by hand.
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
pub fn read_rust_public_signature_name_position(kinds: &[String], values: &[String]) -> f64 {
    {
        let position = 0f64;
        {
            let ml_s1 =
                self::ml_read_rust_public_signature_name_position_loop5(kinds, values, position);
            match ml_s1 {
                self::MlReadRustPublicSignatureNamePositionLoop5Result::MlReadRustPublicSignatureNamePositionLoop5Done(position_2) => {
                    if ((position_2 >= (values.len() as f64)) || !(match (self::ml_array::optional_at(kinds, self::ml_array::number_index(position_2)), String::from("identifier")) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })) || !(match (self::ml_array::optional_at(values, self::ml_array::number_index(position_2)), String::from("pub")) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) {
                        -1f64
                    } else {
                        {
                            let position_3 = position_2 + 1f64;
                            if ((position_3 < (values.len() as f64)) && match (self::ml_array::optional_at(kinds, self::ml_array::number_index(position_3)), String::from("punct")) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) && match (self::ml_array::optional_at(values, self::ml_array::number_index(position_3)), String::from("(")) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } {
                                -1f64
                            } else {
                                {
                                    let ml_s2 = self::ml_read_rust_public_signature_name_position_loop6(kinds, values, position_3);
                                    match ml_s2 {
                                        self::MlReadRustPublicSignatureNamePositionLoop6Result::MlReadRustPublicSignatureNamePositionLoop6Done => {
                                            -1f64
                                        }
                                        self::MlReadRustPublicSignatureNamePositionLoop6Result::MlReadRustPublicSignatureNamePositionLoop6Return(ml_result6) => {
                                            ml_result6
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlReadRustPublicSignatureNamePositionLoop5Result::MlReadRustPublicSignatureNamePositionLoop5Return(ml_result6_2) => {
                    ml_result6_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_semantic_module_item(term: &str, comment: bool, role: f64) -> bool {
    if role == 3f64 {
        false
    } else {
        ((comment || (term == "function_item")) || (term == "attribute_item"))
            || (term == "inner_attribute_item")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_semantic_module_export(source: &str, target: &str) -> String {
    format!(
        "export {{ {} as {} }};",
        target.to_owned(),
        source.to_owned()
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn select_self_translation_module_references(
    explicit: bool,
    javascript: bool,
    lexical: bool,
) -> bool {
    explicit || (javascript && lexical)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn refine_self_translation_module_references(
    selected: bool,
    explicit: bool,
    javascript: bool,
    complete: bool,
    reference: bool,
) -> bool {
    if ((!explicit && selected) && javascript) && complete {
        reference
    } else {
        selected
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_semantic_branch_context_reordering(
    expected_known: bool,
    first_needs_context: bool,
    second_needs_context: bool,
) -> bool {
    (!expected_known && first_needs_context) && !second_needs_context
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
            let ml_logical3 = ((end + 1f64) < (roles.len() as f64))
                && match (
                    self::ml_array::optional_at(roles, self::ml_array::number_index(end + 1f64)),
                    6f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                };
            if ml_logical3 {
                self::ml_array::optional_at(line_breaks, self::ml_array::number_index(end))
            } else {
                Some(ml_logical3)
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
            let ml_logical5 = {
                let ml_logical4 = !(match (
                    self::ml_array::optional_at(roles, self::ml_array::number_index(end)),
                    0f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }) && ((end + 1f64) < (roles.len() as f64));
                if ml_logical4 {
                    self::ml_array::optional_at(line_breaks, self::ml_array::number_index(end))
                } else {
                    Some(ml_logical4)
                }
            };
            if (ml_logical5.clone()).unwrap_or_default() {
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
                ml_logical5.clone()
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
                                let ml_s6 = if match (role, 3f64) {
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
                                match ml_s6 {
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRustPublicSignatureNamePositionLoop6Result {
    MlReadRustPublicSignatureNamePositionLoop6Done,
    MlReadRustPublicSignatureNamePositionLoop6Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_rust_public_signature_name_position_loop6(
    kinds: &[String],
    values: &[String],
    mut position: f64,
) -> self::MlReadRustPublicSignatureNamePositionLoop6Result {
    loop {
        return if position < (values.len() as f64) {
            if match (
                self::ml_array::optional_at(kinds, self::ml_array::number_index(position)),
                String::from("identifier"),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } && match (
                self::ml_array::optional_at(values, self::ml_array::number_index(position)),
                String::from("fn"),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } {
                if ((position + 1f64) < (values.len() as f64))
                    && match (
                        self::ml_array::optional_at(
                            kinds,
                            self::ml_array::number_index(position + 1f64),
                        ),
                        String::from("identifier"),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                {
                    self::MlReadRustPublicSignatureNamePositionLoop6Result::MlReadRustPublicSignatureNamePositionLoop6Return(position + 1f64)
                } else {
                    self::MlReadRustPublicSignatureNamePositionLoop6Result::MlReadRustPublicSignatureNamePositionLoop6Return(-1f64)
                }
            } else if match (
                self::ml_array::optional_at(kinds, self::ml_array::number_index(position)),
                String::from("punct"),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } && (match (
                self::ml_array::optional_at(values, self::ml_array::number_index(position)),
                String::from("{"),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } || match (
                self::ml_array::optional_at(values, self::ml_array::number_index(position)),
                String::from(";"),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }) {
                self::MlReadRustPublicSignatureNamePositionLoop6Result::MlReadRustPublicSignatureNamePositionLoop6Return(-1f64)
            } else {
                {
                    let position_2 = position + 1f64;
                    {
                        position = position_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlReadRustPublicSignatureNamePositionLoop6Result::MlReadRustPublicSignatureNamePositionLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRustPublicSignatureNamePositionLoop5Result {
    MlReadRustPublicSignatureNamePositionLoop5Done(f64),
    MlReadRustPublicSignatureNamePositionLoop5Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRustPublicSignatureNamePositionLoop7Result {
    MlReadRustPublicSignatureNamePositionLoop7Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_rust_public_signature_name_position_loop7(
    kinds: &[String],
    values: &[String],
    mut position: f64,
    mut depth: f64,
) -> self::MlReadRustPublicSignatureNamePositionLoop7Result {
    loop {
        if (position < (values.len() as f64)) && (depth > 0f64) {
            {
                let depth_3 = if match (
                    self::ml_array::optional_at(kinds, self::ml_array::number_index(position)),
                    String::from("punct"),
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                } && match (
                    self::ml_array::optional_at(values, self::ml_array::number_index(position)),
                    String::from("["),
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                } {
                    { depth + 1f64 }
                } else {
                    depth
                };
                {
                    let depth_5 = if match (
                        self::ml_array::optional_at(kinds, self::ml_array::number_index(position)),
                        String::from("punct"),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    } && match (
                        self::ml_array::optional_at(values, self::ml_array::number_index(position)),
                        String::from("]"),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    } {
                        { depth_3 - 1f64 }
                    } else {
                        depth_3
                    };
                    {
                        let position_2 = position + 1f64;
                        {
                            (position, depth) = (position_2, depth_5);
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlReadRustPublicSignatureNamePositionLoop7Result::MlReadRustPublicSignatureNamePositionLoop7Done(position, depth)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_rust_public_signature_name_position_loop5(
    kinds: &[String],
    values: &[String],
    mut position: f64,
) -> self::MlReadRustPublicSignatureNamePositionLoop5Result {
    loop {
        return if ((position < (values.len() as f64))
            && match (
                self::ml_array::optional_at(kinds, self::ml_array::number_index(position)),
                String::from("punct"),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })
            && match (
                self::ml_array::optional_at(values, self::ml_array::number_index(position)),
                String::from("#"),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } {
            {
                let position_2 = position + 1f64;
                {
                    let position_4 = if ((position_2 < (values.len() as f64))
                        && match (
                            self::ml_array::optional_at(
                                kinds,
                                self::ml_array::number_index(position_2),
                            ),
                            String::from("punct"),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                        && match (
                            self::ml_array::optional_at(
                                values,
                                self::ml_array::number_index(position_2),
                            ),
                            String::from("!"),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } {
                        { position_2 + 1f64 }
                    } else {
                        position_2
                    };
                    if ((position_4 >= (values.len() as f64))
                        || !(match (
                            self::ml_array::optional_at(
                                kinds,
                                self::ml_array::number_index(position_4),
                            ),
                            String::from("punct"),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }))
                        || !(match (
                            self::ml_array::optional_at(
                                values,
                                self::ml_array::number_index(position_4),
                            ),
                            String::from("["),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    {
                        self::MlReadRustPublicSignatureNamePositionLoop5Result::MlReadRustPublicSignatureNamePositionLoop5Return(-1f64)
                    } else {
                        {
                            let depth = 1f64;
                            {
                                let position_5 = position_4 + 1f64;
                                {
                                    let ml_s7 =
                                        self::ml_read_rust_public_signature_name_position_loop7(
                                            kinds, values, position_5, depth,
                                        );
                                    match ml_s7 {
                                        self::MlReadRustPublicSignatureNamePositionLoop7Result::MlReadRustPublicSignatureNamePositionLoop7Done(position_6, depth_2) => {
                                            {
let ml_library_inequality_1 = depth_2 != 0f64 ;
if ml_library_inequality_1 {
                                                self::MlReadRustPublicSignatureNamePositionLoop5Result::MlReadRustPublicSignatureNamePositionLoop5Return(-1f64)
                                            } else {
                                                {
                                                    position = position_6;
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
            self::MlReadRustPublicSignatureNamePositionLoop5Result::MlReadRustPublicSignatureNamePositionLoop5Done(position)
        };
    }
}
