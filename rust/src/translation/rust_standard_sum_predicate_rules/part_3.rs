// Generated from complete js/src/translation/rust-standard-sum-predicate-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_rust_standard_sum_predicate_arity;
use super::read_rust_standard_sum_predicate_callback_argument;
use super::read_rust_standard_sum_source_field_argument;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_predicate_branch(owner: &str, variant: &str, method: &str) -> String {
    if (self::read_rust_standard_sum_predicate_arity(owner, method)) < 0f64 {
        String::new()
    } else {
        {
            let field = self::read_rust_standard_sum_source_field_argument(owner, variant);
            if field == -2f64 {
                String::new()
            } else if (method == "map_or") || (method == "map_or_else") {
                if field == 0f64 {
                    String::from("mapper")
                } else if method == "map_or" {
                    String::from("default")
                } else {
                    String::from("fallback")
                }
            } else if field
                == (self::read_rust_standard_sum_predicate_callback_argument(
                    owner,
                    method,
                    "predicate",
                ))
            {
                String::from("predicate")
            } else if method == "is_none_or" {
                String::from("true")
            } else {
                String::from("false")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_predicate_version_error(
    owner: &str,
    method: &str,
    minor: f64,
) -> String {
    if ((self::read_rust_standard_sum_predicate_arity(owner, method)) < 0f64)
        || ((self::ml_math::is_integer(minor) && (minor >= 90f64)) && (minor < 100f64))
    {
        String::new()
    } else {
        String::from(
            "standard sum predicate and mapping source semantics are uncovered outside Rust >=1.90.0 <1.100.0",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_predicate_borrow_error(owner: &str, method: &str) -> String {
    if (owner == "std::option::Option") && (method == "filter") {
        String::from(
            "filter needs its actual borrowed predicate parameter and complete reference lifetime model",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_predicate_deferred_error(
    kind: &str,
    pending: bool,
    parameters: f64,
    expected: f64,
    layers: f64,
    identity: bool,
) -> String {
    if ((((kind == "lambda") && pending)
        && ((parameters).partial_cmp(&(expected)) == Some(std::cmp::Ordering::Equal)))
        && (layers == 0f64))
        && identity
    {
        String::new()
    } else {
        String::from(
            "an unfinished callback needs its actual parameter origins, closure source identity and independently established sibling result",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_predicate_result_source(first: bool, mapper: bool) -> String {
    if first {
        String::from("first result")
    } else if mapper {
        String::from("mapper result")
    } else {
        String::new()
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
                    let source_ml_source2_comparison_operand22 = self::ml_array::optional_at(
                        direct_copies,
                        self::ml_array::number_index(at),
                    );
                    {
                        let source_ml_source2_comparison_operand23 = true;
                        !(match (
                            source_ml_source2_comparison_operand22,
                            source_ml_source2_comparison_operand23,
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
