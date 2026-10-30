// Generated from complete js/src/translation/javascript-string-range-rules.js. Do not edit by hand.
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
pub fn plan_string_range_integer(value: &str) -> Vec<String> {
    vec![
        value.to_owned(),
        value.to_owned(),
        String::from("eq"),
        value.to_owned(),
        String::from("trunc"),
        String::from("0"),
        String::from("if"),
    ]
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_string_range_nonnegative(value: &str) -> Vec<String> {
    {
        let source_ml_concat_value1 = self::plan_string_range_integer(value);
        {
            let source_ml_concat_value2 = vec![String::from("0"), String::from("max")];
            self::ml_array::append(
                self::ml_array::append(Vec::<String>::new(), source_ml_concat_value1),
                source_ml_concat_value2,
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_string_range_relative_start() -> Vec<String> {
    {
        let source_ml_concat_value3 = self::plan_string_range_integer("start");
        {
            let source_ml_concat_value4 = vec![
                String::from("0"),
                String::from("lt"),
                String::from("length"),
            ];
            {
                let source_ml_concat_value5 = self::plan_string_range_integer("start");
                {
                    let source_ml_concat_value6 =
                        vec![String::from("add"), String::from("0"), String::from("max")];
                    {
                        let source_ml_concat_value7 = self::plan_string_range_integer("start");
                        {
                            let source_ml_concat_value8 = vec![
                                String::from("length"),
                                String::from("min"),
                                String::from("if"),
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<String>::new(),
                                                    source_ml_concat_value3,
                                                ),
                                                source_ml_concat_value4,
                                            ),
                                            source_ml_concat_value5,
                                        ),
                                        source_ml_concat_value6,
                                    ),
                                    source_ml_concat_value7,
                                ),
                                source_ml_concat_value8,
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_string_range_window(operation: &str) -> Vec<String> {
    if operation == "slice" {
        vec![String::from("start"), String::from("end")]
    } else if operation == "substring" {
        {
            let source_ml_concat_value9 = self::plan_string_range_nonnegative("start");
            {
                let source_ml_concat_value10 = self::plan_string_range_nonnegative("end");
                {
                    let source_ml_concat_value11 = vec![String::from("min")];
                    {
                        let source_ml_concat_value12 = self::plan_string_range_nonnegative("start");
                        {
                            let source_ml_concat_value13 =
                                self::plan_string_range_nonnegative("end");
                            {
                                let source_ml_concat_value14 = vec![String::from("max")];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<String>::new(),
                                                        source_ml_concat_value9,
                                                    ),
                                                    source_ml_concat_value10,
                                                ),
                                                source_ml_concat_value11,
                                            ),
                                            source_ml_concat_value12,
                                        ),
                                        source_ml_concat_value13,
                                    ),
                                    source_ml_concat_value14,
                                )
                            }
                        }
                    }
                }
            }
        }
    } else if operation == "substr" {
        {
            let source_ml_concat_value15 = self::plan_string_range_relative_start();
            {
                let source_ml_concat_value16 = self::plan_string_range_relative_start();
                {
                    let source_ml_concat_value17 = self::plan_string_range_nonnegative("end");
                    {
                        let source_ml_concat_value18 = vec![String::from("add")];
                        self::ml_array::append(
                            self::ml_array::append(
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_concat_value15,
                                    ),
                                    source_ml_concat_value16,
                                ),
                                source_ml_concat_value17,
                            ),
                            source_ml_concat_value18,
                        )
                    }
                }
            }
        }
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_string_range_operation(operation: &str) -> bool {
    ((operation == "slice") || (operation == "substring")) || (operation == "substr")
}
