// Generated from complete js/src/grammar-attribute-inference-rules.js. Do not edit by hand.
/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_attribute_observation_limit() -> f64 {
    32f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_attribute_observation_byte_limit() -> f64 {
    262_144f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_attribute_candidate_values(field: &str, child_attribute: &str) -> Vec<String> {
    {
        let matched = String::from("(matched)");
        {
            let candidates = vec![
                matched.clone(),
                format!("(uppercase {matched})"),
                format!("(length {matched})"),
                format!("(number {matched})"),
            ];
            if (field).is_empty() {
                candidates
            } else {
                {
                    let captured = format!("(fieldText {})", field.to_owned());
                    {
                        let projections = vec![
                            captured.clone(),
                            format!("(uppercase {captured})"),
                            format!("(length {captured})"),
                            format!("(number {captured})"),
                        ];
                        if (child_attribute).is_empty() {
                            {
                                let source_ml_concat_value1 = candidates;
                                {
                                    let source_ml_concat_value2 = projections;
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_concat_value1,
                                        ),
                                        source_ml_concat_value2,
                                    )
                                }
                            }
                        } else {
                            {
                                let source_ml_concat_value3 = candidates;
                                {
                                    let source_ml_concat_value4 = projections;
                                    {
                                        let source_ml_concat_value5 = vec![
                                            format!(
                                                "(attribute {} {})",
                                                field.to_owned(),
                                                child_attribute.to_owned()
                                            ),
                                            format!(
                                                "(sumOf {} {})",
                                                field.to_owned(),
                                                child_attribute.to_owned()
                                            ),
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<String>::new(),
                                                    source_ml_concat_value3,
                                                ),
                                                source_ml_concat_value4,
                                            ),
                                            source_ml_concat_value5,
                                        )
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
