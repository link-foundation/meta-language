// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
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
pub fn read_java_script_tag_owner_position(owners: &[String], receiver: &str) -> f64 {
    if (owners.len() as f64) == 1f64 {
        0f64
    } else if (receiver).is_empty() {
        -1f64
    } else {
        {
            let selected = -1f64;
            {
                let index = 0f64;
                {
                    let ml_s1 = self::ml_read_java_script_tag_owner_position_loop1(
                        owners, receiver, selected, index,
                    );
                    match ml_s1 {
                            self::MlReadJavaScriptTagOwnerPositionLoop1Result::MlReadJavaScriptTagOwnerPositionLoop1Done(selected_2) => {
                                selected_2
                            }
                            self::MlReadJavaScriptTagOwnerPositionLoop1Result::MlReadJavaScriptTagOwnerPositionLoop1Return(ml_result1) => {
                                ml_result1
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_tag_receiver_form(
    kind: &str,
    function_kind: &str,
    path: &[String],
    root: &str,
) -> f64 {
    if (((kind == "app") && (function_kind == "name")) && ((path.len() as f64) == 2f64))
        && match (
            self::ml_array::optional_at(path, self::ml_array::number_index(0f64)),
            root.to_owned(),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        }
    {
        1f64
    } else {
        let ml_library_inequality_1 = kind != "name";
        if ml_library_inequality_1 {
            0f64
        } else if ((path.len() as f64) == 2f64)
            && match (
                self::ml_array::optional_at(path, self::ml_array::number_index(0f64)),
                root.to_owned(),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        {
            1f64
        } else if (path.len() as f64) == 1f64 {
            2f64
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn read_java_script_tag_binding_step(owns_binding: bool, boundary: f64) -> f64 {
    if owns_binding {
        1f64
    } else if boundary == 0f64 {
        0f64
    } else {
        2f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_java_script_tag_header_fact(
    has_type: bool,
    has_header: bool,
    asynchronous: bool,
) -> bool {
    (!has_type && has_header) && !asynchronous
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_tag_declared_owner(
    kind: &str,
    path: &[String],
    name: &str,
    root: &str,
) -> String {
    if kind == "data" {
        name.to_owned()
    } else if ((kind == "named") && ((path.len() as f64) == 2f64))
        && match (
            self::ml_array::optional_at(path, self::ml_array::number_index(0f64)),
            root.to_owned(),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        }
    {
        (self::ml_array::optional_at(path, self::ml_array::number_index(1f64))).unwrap_or_default()
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadJavaScriptTagOwnerPositionLoop1Result {
    MlReadJavaScriptTagOwnerPositionLoop1Done(f64),
    MlReadJavaScriptTagOwnerPositionLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_java_script_tag_owner_position_loop1(
    owners: &[String],
    receiver: &str,
    mut selected: f64,
    mut index: f64,
) -> self::MlReadJavaScriptTagOwnerPositionLoop1Result {
    loop {
        return if index < (owners.len() as f64) {
            if match (
                self::ml_array::optional_at(owners, self::ml_array::number_index(index)),
                receiver.to_owned(),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } {
                if selected >= 0f64 {
                    self::MlReadJavaScriptTagOwnerPositionLoop1Result::MlReadJavaScriptTagOwnerPositionLoop1Return(-1f64)
                } else {
                    {
                        let selected_2 = index;
                        {
                            let index_2 = index + 1f64;
                            {
                                (selected, index) = (selected_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlReadJavaScriptTagOwnerPositionLoop1Result::MlReadJavaScriptTagOwnerPositionLoop1Done(selected)
        };
    }
}
