// Generated from complete js/src/translation/rust-standard-sum-combinator-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_rust_standard_sum_combinator_arity;
use super::read_rust_standard_sum_source_constructors;
use super::read_rust_standard_sum_source_field_argument;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_standard_sum_combinator_right_arguments(
    owner: &str,
    method: &str,
    variant: &str,
) -> Vec<String> {
    if owner == "std::result::Result" {
        if (method == "and") && (variant == "Ok") {
            vec![String::from("argument"), String::from("left error")]
        } else if (method == "or") && (variant == "Err") {
            vec![String::from("left value"), String::from("argument")]
        } else {
            Vec::<String>::new()
        }
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_combinator_branch(
    owner: &str,
    left: &str,
    method: &str,
    right: &str,
) -> String {
    if (self::read_rust_standard_sum_combinator_arity(owner, method)) < 0f64 {
        String::new()
    } else {
        {
            let field = self::read_rust_standard_sum_source_field_argument(owner, left);
            if field == -2f64 {
                String::new()
            } else if method == "and" {
                if field == 0f64 {
                    String::from("right")
                } else {
                    String::from("construct failure")
                }
            } else if (method == "or") || (method == "or_else") {
                if field == 0f64 {
                    if owner == "std::option::Option" {
                        String::from("left")
                    } else {
                        String::from("construct success")
                    }
                } else if method == "or_else" {
                    String::from("callback")
                } else {
                    String::from("right")
                }
            } else {
                {
                    let other = self::read_rust_standard_sum_source_field_argument(owner, right);
                    if other == -2f64 {
                        String::new()
                    } else if (field == 0f64) && (other != 0f64) {
                        String::from("left")
                    } else if (field != 0f64) && (other == 0f64) {
                        String::from("right")
                    } else {
                        String::from("construct failure")
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_combinator_constructor(owner: &str, success: bool) -> String {
    {
        let variants = self::read_rust_standard_sum_source_constructors(owner);
        {
            let source_ml_source1_temporary_values6 = variants;
            {
                let source_ml_source1_temporary_index6 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s22 = self::source_ml_source1_temporary_read_rust_standard_sum_combinator_constructor_loop7(owner, success, &(source_ml_source1_temporary_values6), source_ml_source1_temporary_index6);
                    match source_ml_source2_s22 {
                        self::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Result::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Done => {
                            String::new()
                        }
                        self::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Result::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Return(source_ml_source1_temporary_result7) => {
                            source_ml_source1_temporary_result7
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_combinator_version_error(
    owner: &str,
    method: &str,
    minor: f64,
) -> String {
    if ((self::read_rust_standard_sum_combinator_arity(owner, method)) < 0f64)
        || ((self::ml_math::is_integer(minor) && (minor >= 90f64)) && (minor < 100f64))
    {
        String::new()
    } else {
        String::from(
            "standard sum combinator source semantics are uncovered outside Rust >=1.90.0 <1.100.0",
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(f64),
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2(
    mut depth: f64,
    source_ml_source1_temporary_values1: &[f64],
    mut source_ml_source1_temporary_index1: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    loop {
        return if source_ml_source1_temporary_index1
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values1.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_source1_temporary_values1,
                    source_ml_source1_temporary_index1.to_index(),
                );
                if (!(self::ml_math::is_integer(child)) || (child < 0f64)) || (child > 16f64) {
                    self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(-1f64)
                } else {
                    if child > depth {
                        {
                            let depth_2 = child;
                            {
                                let source_ml_source1_temporary_index1_2 =
                                    source_ml_source1_temporary_index1
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (depth, source_ml_source1_temporary_index1) =
                                        (depth_2, source_ml_source1_temporary_index1_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_source1_temporary_index1_3 =
                                source_ml_source1_temporary_index1
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1 =
                                    source_ml_source1_temporary_index1_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(depth)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result {
    MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done,
    MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4(
    source_ml_source1_temporary_values3: &[f64],
    mut source_ml_source1_temporary_index3: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result {
    loop {
        return if source_ml_source1_temporary_index3
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values3.len() as u128))
        {
            {
                let value = self::ml_array::at(
                    source_ml_source1_temporary_values3,
                    source_ml_source1_temporary_index3.to_index(),
                );
                if !(self::ml_math::is_integer(value)) || (value < 0f64) {
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(String::from("invalid standard sum source materialization count"))
                } else {
                    {
                        let source_ml_source1_temporary_index3_2 =
                            source_ml_source1_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index3 =
                                source_ml_source1_temporary_index3_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result {
    MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done,
    MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5(
    identities: &[String],
    direct_copies: &[bool],
    mut count: f64,
    mut at: f64,
) -> self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result {
    loop {
        return if at < count {
            if (((self::ml_array::optional_at(identities, self::ml_array::number_index(at)))
                .unwrap_or_else(|| String::new()))
                == "")
                || {
                    let source_ml_source2_comparison_operand23 = self::ml_array::optional_at(
                        direct_copies,
                        self::ml_array::number_index(at),
                    );
                    {
                        let source_ml_source2_comparison_operand24 = true;
                        !(match (
                            source_ml_source2_comparison_operand23,
                            source_ml_source2_comparison_operand24,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                }
            {
                self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(false)
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Result {
    MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Done,
    MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_sum_combinator_constructor_loop7(
    owner: &str,
    mut success: bool,
    source_ml_source1_temporary_values6: &[String],
    mut source_ml_source1_temporary_index6: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Result {
    loop {
        return if source_ml_source1_temporary_index6
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values6.len() as u128))
        {
            {
                let variant = self::ml_array::at(
                    source_ml_source1_temporary_values6,
                    source_ml_source1_temporary_index6.to_index(),
                );
                if ((self::read_rust_standard_sum_source_field_argument(owner, &(variant))) == 0f64)
                    == success
                {
                    self::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Result::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Return(variant.clone())
                } else {
                    {
                        let source_ml_source1_temporary_index6_2 =
                            source_ml_source1_temporary_index6.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index6 =
                                source_ml_source1_temporary_index6_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Result::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Done
        };
    }
}
