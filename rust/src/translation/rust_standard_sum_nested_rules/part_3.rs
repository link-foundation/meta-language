// Generated from complete js/src/translation/rust-standard-sum-nested-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_rust_standard_sum_nested_arity;
use super::read_rust_standard_sum_source_field_argument;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_standard_sum_nested_branch(
    profile: &str,
    owner: &str,
    constructor: &str,
    inner: bool,
) -> Vec<String> {
    {
        let field = self::read_rust_standard_sum_source_field_argument(owner, constructor);
        if field == -2f64 {
            Vec::<String>::new()
        } else if (profile == "option flatten") && (owner == "std::option::Option") {
            if field == -1f64 {
                vec![String::from("result None")]
            } else if inner {
                vec![String::from("payload"), String::from("result Some")]
            } else {
                vec![String::from("match inner")]
            }
        } else {
            {
                let source_ml_source2_s22 = if profile == "option result transpose" {
                    if !inner && (owner == "std::option::Option") {
                        self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Return(if field == -1f64 {
                                vec![String::from("child None"), String::from("result Ok")]
                            } else {
                                vec![String::from("match inner")]
                            })
                    } else if inner && (owner == "std::result::Result") {
                        self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Return(if field == 0f64 {
                                    vec![String::from("payload"), String::from("child Some"), String::from("result Ok")]
                                } else {
                                    vec![String::from("payload"), String::from("result Err")]
                                })
                    } else {
                        self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Next
                    }
                } else {
                    self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Next
                };
                match source_ml_source2_s22 {
                        self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Next => {
                            {
                                let source_ml_source2_s23 = if profile == "result option transpose" {
                                    if !inner && (owner == "std::result::Result") {
                                        self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Return(if field == 0f64 {
                                            vec![String::from("match inner")]
                                        } else {
                                            vec![String::from("payload"), String::from("child Err"), String::from("result Some")]
                                        })
                                    } else if inner && (owner == "std::option::Option") {
                                            self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Return(if field == -1f64 {
                                                vec![String::from("result None")]
                                            } else {
                                                vec![String::from("payload"), String::from("child Ok"), String::from("result Some")]
                                            })
                                        } else {
                                            self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Next
                                        }
                                } else {
                                    self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Next
                                };
                                match source_ml_source2_s23 {
                                    self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Next => {
                                        Vec::<String>::new()
                                    }
                                    self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Return(source_ml_source1_temporary_result7) => {
                                        source_ml_source1_temporary_result7
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Return(source_ml_source1_temporary_result7_2) => {
                            source_ml_source1_temporary_result7_2
                        }
                    }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_nested_construction_position(term: &str) -> f64 {
    if self::ml_array::includes(
        &vec![
            String::from("result None"),
            String::from("result Some"),
            String::from("result Ok"),
            String::from("result Err"),
        ],
        &term.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        -1f64
    } else if self::ml_array::includes(
        &vec![
            String::from("child None"),
            String::from("child Some"),
            String::from("child Ok"),
            String::from("child Err"),
        ],
        &term.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        0f64
    } else {
        -2f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_nested_construction_variant(term: &str) -> String {
    if (term == "result None") || (term == "child None") {
        String::from("None")
    } else if (term == "result Some") || (term == "child Some") {
        String::from("Some")
    } else if (term == "result Ok") || (term == "child Ok") {
        String::from("Ok")
    } else if (term == "result Err") || (term == "child Err") {
        String::from("Err")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_nested_source_version_error(
    owner: &str,
    method: &str,
    minor: f64,
) -> String {
    if ((self::read_rust_standard_sum_nested_arity(owner, method)) < 0f64)
        || ((self::ml_math::is_integer(minor) && (minor >= 90f64)) && (minor < 100f64))
    {
        String::new()
    } else {
        String::from("nested sum source semantics are uncovered outside Rust >=1.90.0 <1.100.0")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_nested_source_version_range(owner: &str, method: &str) -> String {
    if (self::read_rust_standard_sum_nested_arity(owner, method)) < 0f64 {
        String::new()
    } else {
        String::from(">=1.90.0 <1.100.0")
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
                    let source_ml_source2_comparison_operand24 = self::ml_array::optional_at(
                        direct_copies,
                        self::ml_array::number_index(at),
                    );
                    {
                        let source_ml_source2_comparison_operand25 = true;
                        !(match (
                            source_ml_source2_comparison_operand24,
                            source_ml_source2_comparison_operand25,
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
pub enum MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7 {
    MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Next,
    MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Return(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6 {
    MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Next,
    MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Return(Vec<String>),
}
