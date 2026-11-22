// Generated from complete js/src/translation/rust-owned-standard-sum-method-rules.js. Do not edit by hand.
use super::ml_array;
use super::ml_math;
use super::read_rust_owned_standard_sum_method_arity;
use super::read_rust_primitive_default_term;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_standard_sum_method_call_production(
    role: &str,
    function_: bool,
    recorded: bool,
    identity: bool,
    wrappers: f64,
    borrowed: bool,
) -> bool {
    (((((role == "owned") && function_) && recorded) && identity) && (wrappers == 0f64))
        && !borrowed
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_standard_sum_method_version_error(
    owner: &str,
    method: &str,
    minor: f64,
) -> String {
    if (self::read_rust_owned_standard_sum_method_arity(owner, method)) < 0f64 {
        String::from("the consuming owned standard sum method is outside this profile")
    } else if (self::ml_math::is_integer(minor) && (minor >= 90f64)) && (minor < 100f64) {
        String::new()
    } else {
        String::from("the consuming owned standard sum method source version is uncovered")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_standard_sum_method_result(owner: &str, method: &str) -> String {
    if (self::read_rust_owned_standard_sum_method_arity(owner, method)) < 0f64 {
        String::new()
    } else if (method == "unwrap_or") || (method == "unwrap_or_default") {
        String::from("payload")
    } else if method == "ok_or" {
        String::from("std::result::Result")
    } else {
        String::from("std::option::Option")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_owned_standard_sum_method_arguments(owner: &str, method: &str) -> Vec<f64> {
    if (self::read_rust_owned_standard_sum_method_arity(owner, method)) < 0f64 {
        Vec::<f64>::new()
    } else if method == "ok_or" {
        vec![0f64, -1f64]
    } else if method == "ok" {
        vec![0f64]
    } else if method == "err" {
        vec![1f64]
    } else {
        Vec::<f64>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_owned_standard_sum_method_branch(
    owner: &str,
    variant: &str,
    method: &str,
) -> Vec<String> {
    {
        let source_ml_source1_s19 = if owner == "std::option::Option" {
            if method == "unwrap_or" {
                self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Return(if variant == "Some" {
                    vec![String::new(), String::from("payload")]
                } else if variant == "None" {
                        vec![String::new(), String::from("argument")]
                    } else {
                        Vec::<String>::new()
                    })
            } else if method == "unwrap_or_default" {
                self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Return(if variant == "Some" {
                        vec![String::new(), String::from("payload")]
                    } else if variant == "None" {
                            vec![String::new(), String::from("standard default")]
                        } else {
                            Vec::<String>::new()
                        })
            } else if method == "ok_or" {
                self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Return(if variant == "Some" {
                            vec![String::from("Ok"), String::from("payload")]
                        } else if variant == "None" {
                                vec![String::from("Err"), String::from("argument")]
                            } else {
                                Vec::<String>::new()
                            })
            } else {
                self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Next
            }
        } else {
            self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Next
        };
        match source_ml_source1_s19 {
            self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Next => {
                {
                    let source_ml_source1_s20 = if owner == "std::result::Result" {
                        if method == "unwrap_or" {
                            self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Return(if variant == "Ok" {
                                vec![String::new(), String::from("payload")]
                            } else if variant == "Err" {
                                    vec![String::new(), String::from("argument")]
                                } else {
                                    Vec::<String>::new()
                                })
                        } else if method == "unwrap_or_default" {
                                self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Return(if variant == "Ok" {
                                    vec![String::new(), String::from("payload")]
                                } else if variant == "Err" {
                                        vec![String::new(), String::from("standard default")]
                                    } else {
                                        Vec::<String>::new()
                                    })
                            } else if method == "ok" {
                                    self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Return(if variant == "Ok" {
                                        vec![String::from("Some"), String::from("payload")]
                                    } else if variant == "Err" {
                                            vec![String::from("None"), String::from("empty")]
                                        } else {
                                            Vec::<String>::new()
                                        })
                                } else if method == "err" {
                                        self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Return(if variant == "Err" {
                                            vec![String::from("Some"), String::from("payload")]
                                        } else if variant == "Ok" {
                                                vec![String::from("None"), String::from("empty")]
                                            } else {
                                                Vec::<String>::new()
                                            })
                                    } else {
                                        self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Next
                                    }
                    } else {
                        self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Next
                    };
                    match source_ml_source1_s20 {
                        self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Next => {
                            Vec::<String>::new()
                        }
                        self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Return(source_ml_temporary_result2) => {
                            source_ml_temporary_result2
                        }
                    }
                }
            }
            self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Return(source_ml_temporary_result2_2) => {
                source_ml_temporary_result2_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_owned_standard_sum_method_capture_error(
    role: &str,
    wrappers: f64,
    borrowed: bool,
    tracking: bool,
    before: &str,
    after: &str,
    available: bool,
    produced: bool,
) -> String {
    if (((((((role == "owned") && (wrappers == 0f64)) && !borrowed) && tracking)
        && (!(after).is_empty()))
        && (after != before))
        && available)
        && produced
    {
        String::new()
    } else {
        String::from(
            "the consuming value needs its actual move into a fresh tracked owned temporary",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_standard_sum_method_argument(role: &str) -> bool {
    (role == "copy") || (role == "owned")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_standard_sum_method_production(
    role: &str,
    receiver: bool,
    argument: bool,
) -> bool {
    ((role == "owned") && receiver) && argument
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_standard_sum_default_term(
    kind: &str,
    owner_kind: &str,
    owner: &str,
    storage: &str,
    wrappers: f64,
    proven: bool,
) -> String {
    if !proven || (wrappers != 0f64) {
        String::new()
    } else if (((kind == "string") && (owner_kind == "library")) && (storage == "owned string"))
        && self::ml_array::includes(
            &vec![
                String::from("alloc::string::String"),
                String::from("std::string::String"),
            ],
            &owner.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )
    {
        String::from("empty owned string")
    } else if (((kind == "array") && (owner_kind == "library")) && (storage == "owned vector"))
        && self::ml_array::includes(
            &vec![
                String::from("alloc::vec::Vec"),
                String::from("std::vec::Vec"),
            ],
            &owner.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )
    {
        String::from("empty owned vector")
    } else if (kind == "named") && (owner == "std::option::Option") {
        String::from("empty actual Option")
    } else if (kind == "unit") && (owner_kind == "unit") {
        String::from("unit")
    } else {
        {
            let primitive = if owner_kind == "primitive" {
                self::read_rust_primitive_default_term(owner)
            } else {
                String::new()
            };
            if (((kind == "bool") && (primitive == "false"))
                || ((kind == "float") && (primitive == "positive floating zero")))
                || ((kind == "fixed") && (primitive == "integer zero"))
            {
                primitive.clone()
            } else {
                String::new()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_clone_source_error(
    owner: &str,
    role: &str,
    wrappers: f64,
    borrowed: bool,
    tracking: bool,
    available: bool,
    produced: bool,
) -> String {
    if (!(self::ml_array::includes(
        &vec![
            String::from("std::option::Option"),
            String::from("std::result::Result"),
        ],
        &owner.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )) || (wrappers != 0f64))
        || borrowed
    {
        String::from("standard sum clone needs its actual direct ordinary value source")
    } else if (role == "copy") || (((role == "owned") && tracking) && (available || produced)) {
        String::new()
    } else {
        String::from(
            "standard sum clone needs an available actual owner or an actual fresh producer, independently of its type",
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2 {
    MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Next,
    MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Return(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1 {
    MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Next,
    MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Return(Vec<String>),
}
