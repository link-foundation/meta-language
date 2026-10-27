// Generated from complete js/src/translation/rust-primitive-try-from-rules.js. Do not edit by hand.
use super::ml_array;
use super::read_rust_primitive_try_from_consumer_arity;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_guards(
    source_bits: f64,
    source_signed: bool,
    target_bits: f64,
    target_signed: bool,
) -> f64 {
    if !(self::ml_array::includes(
        &vec![8f64, 16f64, 32f64, 64f64, 128f64],
        &source_bits,
        0f64,
        false,
        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
    )) || !(self::ml_array::includes(
        &vec![8f64, 16f64, 32f64, 64f64, 128f64],
        &target_bits,
        0f64,
        false,
        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
    )) {
        -1f64
    } else {
        {
            let lower = source_signed && (!target_signed || (source_bits > target_bits));
            {
                let upper = if target_signed {
                    if source_signed {
                        source_bits > target_bits
                    } else {
                        source_bits >= target_bits
                    }
                } else {
                    source_bits > target_bits
                };
                (if lower { 1f64 } else { 0f64 } + if upper { 2f64 } else { 0f64 })
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_primitive_try_from_consumer(method: &str, guards: f64) -> Vec<String> {
    if ((self::read_rust_primitive_try_from_consumer_arity(method)) < 0f64)
        || !(self::ml_array::includes(
            &vec![0f64, 1f64, 2f64, 3f64],
            &guards,
            0f64,
            false,
            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
        ))
    {
        Vec::<String>::new()
    } else {
        {
            let lower = vec![
                String::from("input"),
                String::from("minimum"),
                String::from("ge"),
            ];
            {
                let upper = vec![
                    String::from("input"),
                    String::from("maximum"),
                    String::from("le"),
                ];
                {
                    let condition = if guards == 0f64 {
                        vec![String::from("true")]
                    } else if guards == 1f64 {
                        lower
                    } else if guards == 2f64 {
                        upper
                    } else {
                        {
                            let source_ml_concat_value17 = lower;
                            {
                                let source_ml_concat_value18 = upper;
                                {
                                    let source_ml_concat_value19 = vec![String::from("and")];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<String>::new(),
                                                source_ml_concat_value17,
                                            ),
                                            source_ml_concat_value18,
                                        ),
                                        source_ml_concat_value19,
                                    )
                                }
                            }
                        }
                    };
                    if method == "is_ok" {
                        condition
                    } else if method == "is_err" {
                        {
                            let source_ml_concat_value20 = condition;
                            {
                                let source_ml_concat_value21 = vec![String::from("not")];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_concat_value20,
                                    ),
                                    source_ml_concat_value21,
                                )
                            }
                        }
                    } else {
                        {
                            let source_ml_concat_value22 = condition;
                            {
                                let source_ml_concat_value23 = vec![
                                    String::from("success"),
                                    String::from("fallback"),
                                    String::from("if"),
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_concat_value22,
                                    ),
                                    source_ml_concat_value23,
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_result_owner(
    method: &str,
    target: &str,
    mapper: &str,
) -> String {
    if (method == "is_ok") || (method == "is_err") {
        String::from("bool")
    } else if method == "unwrap_or" {
        target.to_owned()
    } else if method == "map_or" {
        mapper.to_owned()
    } else {
        String::new()
    }
}
