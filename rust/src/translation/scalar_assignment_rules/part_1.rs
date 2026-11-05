// Generated from complete js/src/translation/scalar-assignment-rules.js. Do not edit by hand.
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
pub fn normalize_java_script_unsigned_integer(value: f64) -> f64 {
    if !(value.is_finite()) || (value == 0f64) {
        0f64
    } else {
        {
            let integral = if value < 0f64 {
                -((-value).floor())
            } else {
                value.floor()
            };
            {
                let remainder = integral % 4_294_967_296f64;
                if remainder < 0f64 {
                    remainder + 4_294_967_296f64
                } else if remainder == 0f64 {
                    0f64
                } else {
                    remainder
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_signed_integer(value: f64) -> f64 {
    {
        let unsigned = self::normalize_java_script_unsigned_integer(value);
        if unsigned >= 2_147_483_648f64 {
            unsigned - 4_294_967_296f64
        } else {
            unsigned
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_operation_code(operation: &str) -> f64 {
    if operation == "conjoinJavaScriptNumberBits" {
        0f64
    } else if operation == "disjoinJavaScriptNumberBits" {
        1f64
    } else if operation == "excludeEqualJavaScriptNumberBits" {
        2f64
    } else if operation == "shiftJavaScriptNumberBitsLeft" {
        3f64
    } else if operation == "shiftJavaScriptNumberBitsRight" {
        4f64
    } else if operation == "shiftJavaScriptNumberBitsRightUnsigned" {
        5f64
    } else if operation == "complementJavaScriptNumberBits" {
        6f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_operation(symbol: &str) -> String {
    if symbol == "&" {
        String::from("conjoinJavaScriptNumberBits")
    } else if symbol == "|" {
        String::from("disjoinJavaScriptNumberBits")
    } else if symbol == "^" {
        String::from("excludeEqualJavaScriptNumberBits")
    } else if symbol == "<<" {
        String::from("shiftJavaScriptNumberBitsLeft")
    } else if symbol == ">>" {
        String::from("shiftJavaScriptNumberBitsRight")
    } else if symbol == ">>>" {
        String::from("shiftJavaScriptNumberBitsRightUnsigned")
    } else if symbol == "~" {
        String::from("complementJavaScriptNumberBits")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function1(symbol: &str) -> String {
    if symbol == "&=" {
        self::read_java_script_number_bitwise_operation("&")
    } else if symbol == "|=" {
        self::read_java_script_number_bitwise_operation("|")
    } else if symbol == "^=" {
        self::read_java_script_number_bitwise_operation("^")
    } else if symbol == "<<=" {
        self::read_java_script_number_bitwise_operation("<<")
    } else if symbol == ">>=" {
        self::read_java_script_number_bitwise_operation(">>")
    } else if symbol == ">>>=" {
        self::read_java_script_number_bitwise_operation(">>>")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_symbol(operation: &str) -> String {
    if operation == "conjoinJavaScriptNumberBits" {
        String::from("&")
    } else if operation == "disjoinJavaScriptNumberBits" {
        String::from("|")
    } else if operation == "excludeEqualJavaScriptNumberBits" {
        String::from("^")
    } else if operation == "shiftJavaScriptNumberBitsLeft" {
        String::from("<<")
    } else if operation == "shiftJavaScriptNumberBitsRight" {
        String::from(">>")
    } else if operation == "shiftJavaScriptNumberBitsRightUnsigned" {
        String::from(">>>")
    } else if operation == "complementJavaScriptNumberBits" {
        String::from("~")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_arity(operation: &str) -> f64 {
    {
        let code = self::read_java_script_number_bitwise_operation_code(operation);
        if code == 6f64 {
            1f64
        } else if code >= 0f64 {
            2f64
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_number_bitwise_operation(kind: &str, operation: &str) -> bool {
    (kind == "math") && ((self::read_java_script_number_bitwise_arity(operation)) > 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_number_bitwise_operand_count(operation: &str, count: f64) -> bool {
    {
        let arity = self::read_java_script_number_bitwise_arity(operation);
        ((arity > 0f64) && ((count).partial_cmp(&(arity)) == Some(std::cmp::Ordering::Equal)))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_number_bitwise_precedence(symbol: &str, level: f64) -> bool {
    (((level == 0f64) && (symbol == "|")) || ((level == 1f64) && (symbol == "^")))
        || ((level == 2f64) && (symbol == "&"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_number_bitwise_operation(operation: f64, left: f64, right: f64) -> f64 {
    if operation == 6f64 {
        -1f64 - self::normalize_java_script_signed_integer(left)
    } else if ((operation == 3f64) || (operation == 4f64)) || (operation == 5f64) {
        {
            let count = self::normalize_java_script_unsigned_integer(right) % 32f64;
            {
                let factor = 1f64;
                {
                    let position = 0f64;
                    {
                        let factor_2 = self::source_ml_temporary_apply_java_script_number_bitwise_operation_loop1(count, factor, position);
                        if operation == 3f64 {
                            self::normalize_java_script_signed_integer(
                                self::normalize_java_script_unsigned_integer(left) * factor_2,
                            )
                        } else if operation == 4f64 {
                            (self::normalize_java_script_signed_integer(left) / factor_2).floor()
                        } else {
                            (self::normalize_java_script_unsigned_integer(left) / factor_2).floor()
                        }
                    }
                }
            }
        }
    } else if (((operation == 0f64) || (operation == 1f64)) || (operation == 2f64)) {
        {
            let first = self::normalize_java_script_unsigned_integer(left);
            {
                let second = self::normalize_java_script_unsigned_integer(right);
                {
                    let result = 0f64;
                    {
                        let weight = 1f64;
                        {
                            let position_2 = 0f64;
                            {
                                let result_2 = self::source_ml_temporary_apply_java_script_number_bitwise_operation_loop2(operation, first, second, result, weight, position_2);
                                self::normalize_java_script_signed_integer(result_2)
                            }
                        }
                    }
                }
            }
        }
    } else {
        f64::NAN
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn conjoin_java_script_number_bits(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(0f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn disjoin_java_script_number_bits(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(1f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn exclude_equal_java_script_number_bits(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(2f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn shift_java_script_number_bits_left(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(3f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn shift_java_script_number_bits_right(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(4f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn shift_java_script_number_bits_right_unsigned(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(5f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn complement_java_script_number_bits(left: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(6f64, left, 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_scalar_assignment_operation(operator: &str) -> String {
    if operator == "=" {
        String::from("replace")
    } else if operator == "+=" {
        String::from("plus")
    } else if operator == "-=" {
        String::from("sub")
    } else if operator == "*=" {
        String::from("mul")
    } else if operator == "/=" {
        String::from("div")
    } else if operator == "%=" {
        String::from("rem")
    } else if operator == "**=" {
        String::from("exponentiateJavaScriptNumericValue")
    } else if operator == "&&=" {
        String::from("and")
    } else if operator == "||=" {
        String::from("or")
    } else if operator == "??=" {
        String::from("coalesce")
    } else if operator == "++" {
        String::from("add")
    } else if operator == "--" {
        String::from("sub")
    } else {
        self::dependency_function1(operator)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_sequence_expression() -> Vec<f64> {
    vec![
        0f64, 0f64, 0f64, 0f64, 0f64, 1f64, 0f64, 0f64, 7f64, 0f64, 0f64, 1f64,
    ]
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_void_expression() -> Vec<f64> {
    vec![
        0f64, 0f64, 0f64, 0f64, 11f64, 0f64, 0f64, 0f64, 7f64, 0f64, 0f64, 1f64,
    ]
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_scalar_assignment_expression(operator: &str, postfix: bool) -> Vec<f64> {
    {
        let operation = self::read_scalar_assignment_operation(operator);
        if ((operation).is_empty()) || ((postfix && (operator != "++")) && (operator != "--")) {
            Vec::<f64>::new()
        } else {
            {
                let plan = vec![
                    0f64, 0f64, 0f64, 0f64, 1f64, 0f64, 0f64, 0f64, 1f64, 1f64, 0f64, 0f64, 2f64,
                    0f64, 0f64, 0f64,
                ];
                if operation == "replace" {
                    {
                        let source_ml_source1_concat_value1 = plan;
                        {
                            let source_ml_source1_concat_value2 = vec![
                                0f64, 1f64, 0f64, 0f64, 8f64, 0f64, 2f64, 2f64, 7f64, 1f64, 4f64,
                                5f64,
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source1_concat_value1,
                                ),
                                source_ml_source1_concat_value2,
                            )
                        }
                    }
                } else if ((operation == "and") || (operation == "or")) || (operation == "coalesce")
                {
                    {
                        let plan_2 = {
                            let source_ml_source1_concat_value3 = plan;
                            {
                                let source_ml_source1_concat_value4 = vec![
                                    0f64, 1f64, 0f64, 0f64, 8f64, 0f64, 2f64, 2f64, 7f64, 1f64,
                                    4f64, 5f64,
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value3,
                                    ),
                                    source_ml_source1_concat_value4,
                                )
                            }
                        };
                        {
                            let plan_6 = if operation == "and" {
                                {
                                    {
                                        let source_ml_source1_concat_value5 = plan_2;
                                        {
                                            let source_ml_source1_concat_value6 =
                                                vec![9f64, 1f64, 6f64, 1f64];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source1_concat_value5,
                                                ),
                                                source_ml_source1_concat_value6,
                                            )
                                        }
                                    }
                                }
                            } else if operation == "or" {
                                {
                                    {
                                        let source_ml_source1_concat_value7 = plan_2;
                                        {
                                            let source_ml_source1_concat_value8 =
                                                vec![9f64, 1f64, 1f64, 6f64];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source1_concat_value7,
                                                ),
                                                source_ml_source1_concat_value8,
                                            )
                                        }
                                    }
                                }
                            } else {
                                {
                                    {
                                        let source_ml_source1_concat_value9 = plan_2;
                                        {
                                            let source_ml_source1_concat_value10 =
                                                vec![6f64, 1f64, 6f64, 0f64];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source1_concat_value9,
                                                ),
                                                source_ml_source1_concat_value10,
                                            )
                                        }
                                    }
                                }
                            };
                            {
                                let source_ml_source1_concat_value11 = plan_6;
                                {
                                    let source_ml_source1_concat_value12 =
                                        vec![7f64, 0f64, 3f64, 7f64];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value11,
                                        ),
                                        source_ml_source1_concat_value12,
                                    )
                                }
                            }
                        }
                    }
                } else {
                    {
                        let update = (operator == "++") || (operator == "--");
                        {
                            let previous = 3f64;
                            {
                                let source_ml_source1_s15 = if update {
                                    {
                                        let plan_7 = {
                                            let source_ml_source1_concat_value16 = plan;
                                            {
                                                let source_ml_source1_concat_value17 = vec![
                                                    10f64, 3f64, 0f64, 0f64, 10f64, 4f64, 0f64,
                                                    0f64,
                                                ];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source1_concat_value16,
                                                    ),
                                                    source_ml_source1_concat_value17,
                                                )
                                            }
                                        };
                                        {
                                            let previous_2 = 5f64;
                                            self::MlTemporaryPlanScalarAssignmentExpressionJoin3::MlTemporaryPlanScalarAssignmentExpressionJoin3Next(plan_7, previous_2)
                                        }
                                    }
                                } else {
                                    self::MlTemporaryPlanScalarAssignmentExpressionJoin3::MlTemporaryPlanScalarAssignmentExpressionJoin3Next(plan, previous)
                                };
                                match source_ml_source1_s15 {
                                        self::MlTemporaryPlanScalarAssignmentExpressionJoin3::MlTemporaryPlanScalarAssignmentExpressionJoin3Next(plan_8, previous_3) => {
                                            {
                                                let right = (plan_8.len() as f64) / 4f64;
                                                {
                                                    let plan_9 = {
                                                        let source_ml_source1_concat_value18 = plan_8;
                                                        {
                                                            let source_ml_source1_concat_value19 = vec![if update {
                                                                3f64
                                                            } else {
                                                                0f64
                                                            }, 1f64, 0f64, 0f64];
                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value18), source_ml_source1_concat_value19)
                                                        }
                                                    };
                                                    {
                                                        let computed = right + 1f64;
                                                        {
                                                            let written = right + 2f64;
                                                            {
                                                                let bound = right + 3f64;
                                                                {
                                                                    let arithmetic = (((((operation == "plus") || (operation == "add")) || (operation == "sub")) || (operation == "mul")) || (operation == "div")) || (operation == "rem");
                                                                    {
                                                                        let source_ml_source1_concat_value20 = plan_9;
                                                                        {
                                                                            let source_ml_source1_concat_value21 = vec![if arithmetic {
                                                                                4f64
                                                                            } else {
                                                                                5f64
                                                                            }, 0f64, 1f64, right, 8f64, 0f64, 2f64, if postfix {
                                                                                1f64
                                                                            } else {
                                                                                2f64
                                                                            }, 7f64, 1f64, computed, written, 7f64, 0f64, previous_3, bound];
                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value20), source_ml_source1_concat_value21)
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
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_apply_java_script_number_bitwise_operation_loop1(
    mut count: f64,
    mut factor: f64,
    mut position: f64,
) -> f64 {
    loop {
        if position < count {
            {
                let factor_2 = factor * 2f64;
                {
                    let position_2 = position + 1f64;
                    {
                        (factor, position) = (factor_2, position_2);
                        continue;
                    }
                }
            }
        }
        return { factor };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_apply_java_script_number_bitwise_operation_loop2(
    mut operation: f64,
    mut first: f64,
    mut second: f64,
    mut result: f64,
    mut weight: f64,
    mut position: f64,
) -> f64 {
    loop {
        if position < 32f64 {
            {
                let first_bit = first % 2f64;
                {
                    let second_bit = second % 2f64;
                    {
                        let result_3 = if ((((operation == 0f64) && (first_bit == 1f64))
                            && (second_bit == 1f64))
                            || ((operation == 1f64)
                                && ((first_bit == 1f64) || (second_bit == 1f64))))
                            || ((operation == 2f64) && (first_bit != second_bit))
                        {
                            { result + weight }
                        } else {
                            result
                        };
                        {
                            let first_2 = (first - first_bit) / 2f64;
                            {
                                let second_2 = (second - second_bit) / 2f64;
                                {
                                    let weight_2 = weight * 2f64;
                                    {
                                        let position_2 = position + 1f64;
                                        {
                                            (first, second, result, weight, position) =
                                                (first_2, second_2, result_3, weight_2, position_2);
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
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanScalarAssignmentExpressionJoin3 {
    MlTemporaryPlanScalarAssignmentExpressionJoin3Next(Vec<f64>, f64),
}
