// Generated from complete js/src/translation/rust-standard-sum-method-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_rust_standard_sum_source_argument_count;
use super::read_rust_standard_sum_source_field_argument;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_message_source_error(
    consumer: &str,
    kind: &str,
    storage: &str,
    layers: f64,
    known: bool,
    references: bool,
) -> String {
    if (consumer != "expect")
        || (((((kind == "string")
            && ((storage == "string slice") || (storage == "owned string")))
            && (layers >= 1f64))
            && known)
            && references)
    {
        String::new()
    } else {
        String::from("expect needs its actual immutable standard str message source")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_option_panic_consumer(consumer: &str) -> bool {
    (consumer == "unwrap") || (consumer == "expect")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_option_panic_message(consumer: &str, message: &str) -> String {
    if consumer == "unwrap" {
        String::from("called `Option::unwrap()` on a `None` value")
    } else if consumer == "expect" {
        message.to_owned()
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_method_arity(owner: &str, method: &str) -> f64 {
    {
        let count = self::read_rust_standard_sum_source_argument_count(owner);
        if count < 0f64 {
            -1f64
        } else if (owner == "std::option::Option")
            && ((method == "is_some") || (method == "is_none"))
        {
            0f64
        } else if (owner == "std::option::Option") && self::test_rust_option_panic_consumer(method)
        {
            if method == "unwrap" { 0f64 } else { 1f64 }
        } else if (owner == "std::result::Result") && ((method == "is_ok") || (method == "is_err"))
        {
            0f64
        } else if self::ml_array::includes(
            &vec![
                String::from("unwrap_or"),
                String::from("unwrap_or_else"),
                String::from("map"),
                String::from("and_then"),
            ],
            &method.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ) {
            1f64
        } else {
            -1f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_owned_panic_receiver(
    owner: &str,
    method: &str,
    role: &str,
) -> bool {
    ((owner == "std::option::Option") && self::test_rust_option_panic_consumer(method))
        && (role == "owned")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_method_callback_argument(owner: &str, method: &str) -> f64 {
    if (self::read_rust_standard_sum_method_arity(owner, method)) < 0f64 {
        -2f64
    } else if (method == "map") || (method == "and_then") {
        0f64
    } else if method == "unwrap_or_else" {
        if owner == "std::option::Option" {
            -1f64
        } else {
            1f64
        }
    } else {
        -2f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_method_result(owner: &str, method: &str) -> String {
    if (self::read_rust_standard_sum_method_arity(owner, method)) < 0f64 {
        String::new()
    } else if method == "map" {
        String::from("mapped sum")
    } else if method == "and_then" {
        String::from("bound sum")
    } else if ((method == "unwrap_or") || (method == "unwrap_or_else"))
        || self::test_rust_option_panic_consumer(method)
    {
        String::from("payload")
    } else {
        String::from("boolean")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_method_branch(
    owner: &str,
    constructor: &str,
    method: &str,
) -> String {
    if (self::read_rust_standard_sum_method_arity(owner, method)) < 0f64 {
        String::new()
    } else {
        {
            let argument = self::read_rust_standard_sum_source_field_argument(owner, constructor);
            if argument == -2f64 {
                String::new()
            } else {
                {
                    let success = argument == 0f64;
                    if self::test_rust_option_panic_consumer(method) {
                        if success {
                            String::from("payload")
                        } else {
                            String::from("panic")
                        }
                    } else if (method == "is_some") || (method == "is_ok") {
                        if success {
                            String::from("true")
                        } else {
                            String::from("false")
                        }
                    } else if (method == "is_none") || (method == "is_err") {
                        if success {
                            String::from("false")
                        } else {
                            String::from("true")
                        }
                    } else if method == "unwrap_or" {
                        if success {
                            String::from("payload")
                        } else {
                            String::from("default")
                        }
                    } else if method == "unwrap_or_else" {
                        if success {
                            String::from("payload")
                        } else {
                            String::from("invoke fallback")
                        }
                    } else if method == "map" {
                        if success {
                            String::from("construct mapped")
                        } else {
                            String::from("forward failure")
                        }
                    } else if success {
                        String::from("invoke mapper")
                    } else {
                        String::from("forward failure")
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_method_source_version_error(
    owner: &str,
    method: &str,
    minor: f64,
) -> String {
    if ((owner != "std::option::Option") || !(self::test_rust_option_panic_consumer(method)))
        || ((self::ml_math::is_integer(minor) && (minor >= 90f64)) && (minor < 100f64))
    {
        String::new()
    } else {
        String::from(
            "Option unwrap and expect source semantics are uncovered outside Rust >=1.90.0 <1.100.0",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_method_source_version_range(owner: &str, method: &str) -> String {
    if (owner == "std::option::Option") && self::test_rust_option_panic_consumer(method) {
        String::from(">=1.90.0 <1.100.0")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_method_panic_message_role(owner: &str, method: &str) -> String {
    if (owner == "std::option::Option") && self::test_rust_option_panic_consumer(method) {
        if method == "expect" {
            String::from("supplied message")
        } else {
            String::from("standard message")
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_method_callback_error(
    kind: &str,
    parameters: f64,
    expected: f64,
    layers: f64,
    copy: bool,
    result: bool,
) -> String {
    if ((((kind == "function")
        && ((parameters).partial_cmp(&(expected)) == Some(std::cmp::Ordering::Equal)))
        && (layers == 0f64))
        && copy)
        && result
    {
        String::new()
    } else {
        String::from(
            "a standard sum callback needs its complete direct Copy callable and actual parameter/result source types",
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
