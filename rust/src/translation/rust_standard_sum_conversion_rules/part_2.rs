// Generated from complete js/src/translation/rust-standard-sum-conversion-rules.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result;
use super::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result;
use super::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result;
use super::accept_rust_primitive_selection_receiver;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::read_rust_primitive_owner_path;
use super::source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5;
use super::source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2;
use super::source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_selection_argument(
    expected: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    actual: &str,
    actual_bits: &str,
    actual_signed: bool,
    actual_owner: &str,
    layers: f64,
    proven: bool,
    known: bool,
    literal: bool,
) -> bool {
    if layers == 0f64 {
        if ((literal && (expected == "fixed")) && ((actual).is_empty()))
            && ((actual_owner).is_empty())
        {
            true
        } else {
            ((((known && proven) && (expected == actual)) && (bits == actual_bits))
                && (signed == actual_signed))
                && (owner == actual_owner)
        }
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_from_method(owner: &str, method: &str) -> bool {
    (method == "from")
        && self::ml_array::includes(
            &vec![
                String::from("bool"),
                String::from("i8"),
                String::from("u8"),
                String::from("i16"),
                String::from("u16"),
                String::from("i32"),
                String::from("u32"),
                String::from("i64"),
                String::from("u64"),
                String::from("i128"),
                String::from("u128"),
                String::from("isize"),
                String::from("usize"),
                String::from("f64"),
            ],
            &owner.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_from_owner(
    segments: &[String],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> String {
    if !ordinary || shadowed {
        String::new()
    } else if ((((segments.len() as f64) == 2f64) && !absolute) && {
        let source_ml_source2_comparison_operand7 =
            self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
        {
            let source_ml_source2_comparison_operand8 = String::from("f64");
            match (
                source_ml_source2_comparison_operand7,
                source_ml_source2_comparison_operand8,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }) || (((((segments.len() as f64) == 4f64)
        && ({
            let source_ml_source2_comparison_operand9 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand10 = String::from("std");
                match (
                    source_ml_source2_comparison_operand9,
                    source_ml_source2_comparison_operand10,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } || {
            let source_ml_source2_comparison_operand11 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand12 = String::from("core");
                match (
                    source_ml_source2_comparison_operand11,
                    source_ml_source2_comparison_operand12,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }))
        && {
            let source_ml_source2_comparison_operand13 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(1f64));
            {
                let source_ml_source2_comparison_operand14 = String::from("primitive");
                match (
                    source_ml_source2_comparison_operand13,
                    source_ml_source2_comparison_operand14,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        && {
            let source_ml_source2_comparison_operand15 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(2f64));
            {
                let source_ml_source2_comparison_operand16 = String::from("f64");
                match (
                    source_ml_source2_comparison_operand15,
                    source_ml_source2_comparison_operand16,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
    {
        String::from("f64")
    } else {
        self::read_rust_primitive_owner_path(segments, ordinary, shadowed, absolute)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_from_literal_owner(form: &str, typed: bool, kind: &str) -> String {
    if (form != "numeral") && (form != "negative numeral") {
        String::new()
    } else if kind == "float" {
        String::from("f64")
    } else if typed {
        String::new()
    } else {
        String::from("i32")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_from_source(
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((((((((kind == "float") && ((bits).is_empty())) && !signed) && (owner == "primitive"))
        && (path == "f64"))
        && (children == 0f64))
        && proven)
        && (layers == 0f64))
        || self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_primitive_from(target: &str, source: &str) -> String {
    if !(self::accept_rust_primitive_from_method(target, "from"))
        || !(self::accept_rust_primitive_from_method(source, "from"))
    {
        String::new()
    } else if target == source {
        String::from("identity")
    } else if ((source == "bool") && (target != "bool")) && (target != "f64") {
        String::from("boolean integer")
    } else if (target == "f64")
        && self::ml_array::includes(
            &vec![
                String::from("i8"),
                String::from("i16"),
                String::from("i32"),
                String::from("u8"),
                String::from("u16"),
                String::from("u32"),
            ],
            &source.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )
    {
        String::from("floating exact")
    } else {
        {
            let admitted = ((((((((source == "u8")
                && self::ml_array::includes(
                    &vec![
                        String::from("u16"),
                        String::from("u32"),
                        String::from("u64"),
                        String::from("u128"),
                        String::from("usize"),
                        String::from("i16"),
                        String::from("i32"),
                        String::from("i64"),
                        String::from("i128"),
                        String::from("isize"),
                    ],
                    &target.to_owned(),
                    0f64,
                    false,
                    |a, b| a == b,
                ))
                || ((source == "u16")
                    && self::ml_array::includes(
                        &vec![
                            String::from("u32"),
                            String::from("u64"),
                            String::from("u128"),
                            String::from("usize"),
                            String::from("i32"),
                            String::from("i64"),
                            String::from("i128"),
                        ],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "u32")
                    && self::ml_array::includes(
                        &vec![
                            String::from("u64"),
                            String::from("u128"),
                            String::from("i64"),
                            String::from("i128"),
                        ],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "u64")
                    && self::ml_array::includes(
                        &vec![String::from("u128"), String::from("i128")],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i8")
                    && self::ml_array::includes(
                        &vec![
                            String::from("i16"),
                            String::from("i32"),
                            String::from("i64"),
                            String::from("i128"),
                            String::from("isize"),
                        ],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i16")
                    && self::ml_array::includes(
                        &vec![
                            String::from("i32"),
                            String::from("i64"),
                            String::from("i128"),
                            String::from("isize"),
                        ],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i32")
                    && self::ml_array::includes(
                        &vec![String::from("i64"), String::from("i128")],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i64") && (target == "i128"));
            if admitted {
                String::from("integer exact")
            } else {
                String::new()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_owner(
    path: &str,
    standard_definition: bool,
    layers: f64,
) -> String {
    if !standard_definition || (layers != 0f64) {
        String::new()
    } else if (path == "std::option::Option") || (path == "core::option::Option") {
        String::from("std::option::Option")
    } else if (path == "std::result::Result") || (path == "core::result::Result") {
        String::from("std::result::Result")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_path_owner(path: &str) -> String {
    if self::ml_array::includes(
        &vec![
            String::from("Option"),
            String::from("std::option::Option"),
            String::from("core::option::Option"),
            String::from("None"),
            String::from("Some"),
            String::from("Option::None"),
            String::from("Option::Some"),
            String::from("std::option::Option::None"),
            String::from("std::option::Option::Some"),
            String::from("core::option::Option::None"),
            String::from("core::option::Option::Some"),
        ],
        &path.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        String::from("std::option::Option")
    } else if self::ml_array::includes(
        &vec![
            String::from("Result"),
            String::from("std::result::Result"),
            String::from("core::result::Result"),
            String::from("Ok"),
            String::from("Err"),
            String::from("Result::Ok"),
            String::from("Result::Err"),
            String::from("std::result::Result::Ok"),
            String::from("std::result::Result::Err"),
            String::from("core::result::Result::Ok"),
            String::from("core::result::Result::Err"),
        ],
        &path.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        String::from("std::result::Result")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_path_constructor(path: &str) -> String {
    if self::ml_array::includes(
        &vec![
            String::from("None"),
            String::from("Option::None"),
            String::from("std::option::Option::None"),
            String::from("core::option::Option::None"),
        ],
        &path.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        String::from("None")
    } else if self::ml_array::includes(
        &vec![
            String::from("Some"),
            String::from("Option::Some"),
            String::from("std::option::Option::Some"),
            String::from("core::option::Option::Some"),
        ],
        &path.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        String::from("Some")
    } else if self::ml_array::includes(
        &vec![
            String::from("Ok"),
            String::from("Result::Ok"),
            String::from("std::result::Result::Ok"),
            String::from("core::result::Result::Ok"),
        ],
        &path.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        String::from("Ok")
    } else if self::ml_array::includes(
        &vec![
            String::from("Err"),
            String::from("Result::Err"),
            String::from("std::result::Result::Err"),
            String::from("core::result::Result::Err"),
        ],
        &path.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        String::from("Err")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_instantiation_limit() -> f64 {
    256f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_depth_limit() -> f64 {
    16f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_specialization_name(ordinal: f64, collision: f64) -> String {
    if ((((!(self::ml_math::is_integer(ordinal)) || (ordinal < 0f64)) || (ordinal >= 256f64))
        || !(self::ml_math::is_integer(collision)))
        || (collision < 0f64))
        || (collision >= 4096f64)
    {
        String::new()
    } else {
        format!(
            "ml_standard_sum_value{}_{}",
            {
                let source_ml_source2_string_argument17 = ordinal;
                self::ml_number::js_number(source_ml_source2_string_argument17)
            },
            {
                let source_ml_source2_string_argument18 = collision;
                self::ml_number::js_number(source_ml_source2_string_argument18)
            }
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_phase(header: bool, complete: bool, copies: bool) -> String {
    if header {
        String::from("provisional")
    } else if complete && copies {
        String::from("admitted")
    } else {
        String::from("refuse")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_depth(children: &[f64], sum: bool) -> f64 {
    {
        let depth = 0f64;
        {
            let source_ml_source1_temporary_values1 = children.to_vec();
            {
                let source_ml_source1_temporary_index1 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s19 =
                        self::source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2(
                            depth,
                            &(source_ml_source1_temporary_values1),
                            source_ml_source1_temporary_index1,
                        );
                    match source_ml_source2_s19 {
                        self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(depth_2) => {
                            {
                                let result = depth_2 + if sum {
                                    1f64
                                } else {
                                    0f64
                                };
                                if result <= 16f64 {
                                    result
                                } else {
                                    -1f64
                                }
                            }
                        }
                        self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(source_ml_source1_temporary_result2) => {
                            source_ml_source1_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_materialization_error(
    nodes: f64,
    units: f64,
    additional_nodes: f64,
    additional_units: f64,
) -> String {
    {
        let source_ml_source1_temporary_values3 =
            vec![nodes, units, additional_nodes, additional_units];
        {
            let source_ml_source1_temporary_index3 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s20 = self::source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4(&(source_ml_source1_temporary_values3), source_ml_source1_temporary_index3);
                match source_ml_source2_s20 {
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done => {
                        if (((nodes > 1_048_576f64) || (units > 8_388_608f64)) || (additional_nodes > (1_048_576f64 - nodes))) || (additional_units > (8_388_608f64 - units)) {
                            String::from("the complete standard sum source metadata exceeds its materialization budget")
                        } else {
                            String::new()
                        }
                    }
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(source_ml_source1_temporary_result4) => {
                        source_ml_source1_temporary_result4
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_argument_count(owner: &str) -> f64 {
    if owner == "std::option::Option" {
        1f64
    } else if owner == "std::result::Result" {
        2f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_source_arguments(
    owner: &str,
    identities: &[String],
    direct_copies: &[bool],
) -> bool {
    {
        let count = self::read_rust_standard_sum_source_argument_count(owner);
        if ((count < 0f64) || ((identities.len() as f64) != count))
            || ((direct_copies.len() as f64) != count)
        {
            false
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s21 = self::source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5(identities, direct_copies, count, at);
                    match source_ml_source2_s21 {
                        self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(source_ml_source1_temporary_result5) => {
                            source_ml_source1_temporary_result5
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_constructors(owner: &str) -> Vec<String> {
    if owner == "std::option::Option" {
        vec![String::from("None"), String::from("Some")]
    } else if owner == "std::result::Result" {
        vec![String::from("Ok"), String::from("Err")]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_field_argument(owner: &str, constructor: &str) -> f64 {
    if owner == "std::option::Option" {
        if constructor == "None" {
            -1f64
        } else if constructor == "Some" {
            0f64
        } else {
            -2f64
        }
    } else if owner == "std::result::Result" {
        if constructor == "Ok" {
            0f64
        } else if constructor == "Err" {
            1f64
        } else {
            -2f64
        }
    } else {
        -2f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_constructor_arity(owner: &str, constructor: &str) -> f64 {
    {
        let argument = self::read_rust_standard_sum_source_field_argument(owner, constructor);
        if argument == -2f64 {
            -1f64
        } else if argument == -1f64 {
            0f64
        } else {
            1f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_source_inferred_constructor(
    owner: &str,
    constructor: &str,
) -> bool {
    (owner == "std::option::Option") && (constructor == "Some")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_question_context(
    operand_owner: &str,
    return_owner: &str,
    ordinary_callable: bool,
    direct_copy_operand: bool,
    operand_error: &str,
    return_error: &str,
) -> bool {
    (ordinary_callable && direct_copy_operand)
        && self::accept_rust_standard_sum_question_family(
            operand_owner,
            return_owner,
            operand_error,
            return_error,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_question_family(
    operand_owner: &str,
    return_owner: &str,
    operand_error: &str,
    return_error: &str,
) -> bool {
    if operand_owner == return_owner {
        if operand_owner == "std::option::Option" {
            ((operand_error).is_empty()) && ((return_error).is_empty())
        } else {
            ((operand_owner == "std::result::Result") && (!(operand_error).is_empty()))
                && (operand_error == return_error)
        }
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_owned_question_context(
    operand_owner: &str,
    return_owner: &str,
    ordinary: bool,
    role: &str,
    wrappers: f64,
    operand_error: &str,
    return_error: &str,
) -> bool {
    ((ordinary && (role == "owned")) && (wrappers == 0f64))
        && self::accept_rust_standard_sum_question_family(
            operand_owner,
            return_owner,
            operand_error,
            return_error,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_standard_sum_primitive_question_conversion_plan(
    operand_owner: &str,
    return_owner: &str,
    ordinary: bool,
    role: &str,
    wrappers: f64,
    direct_copy: bool,
    source_known: bool,
    target_known: bool,
    source: &str,
    target: &str,
    minor: f64,
) -> Vec<String> {
    if ((((((((!ordinary || (operand_owner != "std::result::Result"))
        || (return_owner != "std::result::Result"))
        || (wrappers != 0f64))
        || !source_known)
        || !target_known)
        || !(self::ml_math::is_integer(minor)))
        || (minor < 90f64))
        || (minor >= 100f64))
        || ((source == target) || ((role != "owned") && ((role != "copy") || !direct_copy)))
    {
        Vec::<String>::new()
    } else {
        {
            let plan = self::plan_rust_primitive_from(target, source);
            if (plan).is_empty() {
                Vec::<String>::new()
            } else {
                vec![role.to_owned(), target.to_owned(), plan]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_primitive_question_completion(
    profile: &str,
    consumed: bool,
) -> bool {
    (profile == "copy") || ((profile == "owned") && consumed)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_question_branch(owner: &str, constructor: &str) -> String {
    if (self::read_rust_standard_sum_source_constructor_arity(owner, constructor)) < 0f64 {
        String::new()
    } else if owner == "std::option::Option" {
        if constructor == "Some" {
            String::from("continue field")
        } else {
            String::from("return absent")
        }
    } else if constructor == "Ok" {
        String::from("continue field")
    } else {
        String::from("return error field")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_conversion_arity(owner: &str, method: &str) -> f64 {
    if owner == "std::option::Option" {
        if (method == "ok_or") || (method == "ok_or_else") {
            1f64
        } else {
            -1f64
        }
    } else if owner == "std::result::Result" {
        if (method == "ok") || (method == "err") {
            0f64
        } else if method == "map_err" {
            1f64
        } else {
            -1f64
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_conversion_source_version_error(
    owner: &str,
    method: &str,
    minor: f64,
) -> String {
    if ((self::read_rust_standard_sum_conversion_arity(owner, method)) < 0f64)
        || ((self::ml_math::is_integer(minor) && (minor >= 90f64)) && (minor < 100f64))
    {
        String::new()
    } else {
        String::from(
            "standard sum conversion source semantics are uncovered outside Rust >=1.90.0 <1.100.0",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_conversion_source_version_range(owner: &str, method: &str) -> String {
    if (self::read_rust_standard_sum_conversion_arity(owner, method)) < 0f64 {
        String::new()
    } else {
        String::from(">=1.90.0 <1.100.0")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_conversion_owner(owner: &str, method: &str) -> String {
    if (self::read_rust_standard_sum_conversion_arity(owner, method)) < 0f64 {
        String::new()
    } else if (method == "ok") || (method == "err") {
        String::from("std::option::Option")
    } else {
        String::from("std::result::Result")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_standard_sum_conversion_arguments(owner: &str, method: &str) -> Vec<f64> {
    if (self::read_rust_standard_sum_conversion_arity(owner, method)) < 0f64 {
        Vec::<f64>::new()
    } else if method == "ok" {
        vec![0f64]
    } else if method == "err" {
        vec![1f64]
    } else {
        vec![0f64, -1f64]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_conversion_callback_argument(owner: &str, method: &str) -> f64 {
    if (self::read_rust_standard_sum_conversion_arity(owner, method)) < 0f64 {
        -2f64
    } else if method == "ok_or_else" {
        -1f64
    } else if method == "map_err" {
        1f64
    } else {
        -2f64
    }
}
