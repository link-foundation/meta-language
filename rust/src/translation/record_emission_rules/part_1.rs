// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::*;

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
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
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
    fn integer(value: f64) -> f64 {
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
pub fn refuse_required_number_fact() -> f64 {
    panic!("{}", "missing required Number fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    panic!("{}", "missing required text fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    panic!("{}", "missing required Boolean fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required Number fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required text fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required Boolean fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn join_record_items(items: &[String], separator: &str) -> String {
    {
        let text = String::from("");
        {
            let index = 0f64;
            {
                let text_2 = self::ml_join_record_items_loop1(items, separator, text, index);
                text_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_record_value(target: &str, labels: &[String], items: &[String]) -> String {
    if target == "JavaScript" {
        {
            let entries = Vec::<String>::new();
            {
                let index = 0f64;
                {
                    let entries_2 =
                        self::ml_render_record_value_loop2(labels, items, entries, index);
                    format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from("({")),
                            (self::join_record_items(&(entries_2), ", "))
                        )),
                        (String::from("})"))
                    )
                }
            }
        }
    } else if target == "Rust" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(")),
                    (self::join_record_items(items, ", "))
                )),
                (if (items.len() as f64) == 1f64 {
                    String::from(",")
                } else {
                    String::from("")
                })
            )),
            (String::from(")"))
        )
    } else if target == "Lean" {
        if (items.len() as f64) > 0f64 {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(")),
                    (self::join_record_items(items, ", "))
                )),
                (String::from(")"))
            )
        } else {
            String::from("()")
        }
    } else if target == "Rocq" {
        if (items.len() as f64) > 0f64 {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(")),
                    (self::join_record_items(items, ", "))
                )),
                (String::from(")"))
            )
        } else {
            String::from("tt")
        }
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_record_field(target: &str, object: &str, field: &str, index: &str) -> String {
    if target == "JavaScript" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(")), object)),
                    (String::from(")["))
                )),
                field
            )),
            (String::from("]"))
        )
    } else if target == "Rust" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!("{}{}", (String::from("(")), object)),
                (String::from(")."))
            )),
            index
        )
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_record_product_field(target: &str, object: &str, index: f64, count: f64) -> String {
    if target == "Lean" {
        if count == 1f64 {
            object.to_owned()
        } else {
            {
                let suffix = String::from("");
                {
                    let position = 0f64;
                    {
                        let suffix_2 =
                            self::ml_render_record_product_field_loop3(index, suffix, position);
                        format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (format!("{}{}", (String::from("(")), object)),
                                    (String::from(")"))
                                )),
                                suffix_2
                            )),
                            (if index < (count - 1f64) {
                                String::from(".1")
                            } else {
                                String::from("")
                            })
                        )
                    }
                }
            }
        }
    } else if target == "Rocq" {
        {
            let value = object.to_owned();
            {
                let position_2 = count - 1f64;
                {
                    let value_2 =
                        self::ml_render_record_product_field_loop4(index, value, position_2);
                    if index == 0f64 {
                        value_2.clone()
                    } else {
                        format!(
                            "{}{}",
                            (format!("{}{}", (String::from("(snd ")), value_2)),
                            (String::from(")"))
                        )
                    }
                }
            }
        }
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_record_type(target: &str, fields: &[String]) -> String {
    if target == "Lean" {
        if (fields.len() as f64) > 0f64 {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(")),
                    (self::join_record_items(fields, " × "))
                )),
                (String::from(")"))
            )
        } else {
            String::from("Unit")
        }
    } else if target == "Rocq" {
        if (fields.len() as f64) > 0f64 {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(")),
                    (self::join_record_items(fields, " * "))
                )),
                (String::from(")%type"))
            )
        } else {
            String::from("unit")
        }
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_join_record_items_loop1(
    items: &[String],
    separator: &str,
    mut text: String,
    mut index: f64,
) -> String {
    loop {
        if index < (items.len() as f64) {
            {
                let text_3 = if index > 0f64 {
                    {
                        let text_2 = format!("{}{}", text, separator);
                        text_2
                    }
                } else {
                    text.clone()
                };
                {
                    let text_4 = format!(
                        "{}{}",
                        text_3,
                        (self::read_required_text_fact(items, index))
                    );
                    {
                        let index_2 = index + 1f64;
                        {
                            (text, index) = (text_4, index_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { text.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_record_value_loop2(
    labels: &[String],
    items: &[String],
    mut entries: Vec<String>,
    mut index: f64,
) -> Vec<String> {
    loop {
        if index < (items.len() as f64) {
            {
                let entries_2 = self::ml_array::append(
                    entries.clone(),
                    vec![format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (String::from("[")),
                                (self::read_required_text_fact(labels, index))
                            )),
                            (String::from("]: "))
                        )),
                        (self::read_required_text_fact(items, index))
                    )],
                );
                {
                    let index_2 = index + 1f64;
                    {
                        (entries, index) = (entries_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { entries.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_record_product_field_loop3(
    mut index: f64,
    mut suffix: String,
    mut position: f64,
) -> String {
    loop {
        if position < index {
            {
                let suffix_2 = format!("{}{}", suffix, (String::from(".2")));
                {
                    let position_2 = position + 1f64;
                    {
                        (suffix, position) = (suffix_2, position_2);
                        continue;
                    }
                }
            }
        }
        return { suffix.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_record_product_field_loop4(
    mut index: f64,
    mut value: String,
    mut position: f64,
) -> String {
    loop {
        if position > index {
            {
                let value_2 = format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(fst ")), value)),
                    (String::from(")"))
                );
                {
                    let position_2 = position - 1f64;
                    {
                        (value, position) = (value_2, position_2);
                        continue;
                    }
                }
            }
        }
        return { value.clone() };
    }
}
