// Generated from complete js/src/translation/rust-borrowed-vector-rules.js. Do not edit by hand.
use super::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result;
use super::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result;
use super::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result;
use super::accept_rust_retained_function_owner;
use super::accept_rust_source_array_owner;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_rust_standard_sum_source_materialization_error;
use super::source_ml_source1_temporary_accept_rust_borrowed_vector_inherent_profiles_loop7;
use super::source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop13;
use super::source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop14;
use super::source_ml_source1_temporary_plan_rust_borrowed_vector_result_parameters_loop8;
use super::source_ml_source1_temporary_read_rust_borrowed_expression_position_reservation_loop10;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_vector_completion_profiles(
    signature: &str,
    profiles: &[String],
) -> bool {
    if (!(self::accept_rust_borrowed_vector_signature_model(signature))
        || ((profiles.len() as f64) != 11f64))
        || {
            let source_ml_source2_comparison_operand22 =
                self::ml_array::optional_at(profiles, self::ml_array::number_index(1f64));
            {
                let source_ml_source2_comparison_operand23 = signature.to_owned();
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
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s24 = self::source_ml_source1_temporary_accept_rust_borrowed_vector_completion_profiles_loop6(profiles, index);
                match source_ml_source2_s24 {
                    self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Done => {
                        true
                    }
                    self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Return(source_ml_source1_temporary_result6) => {
                        source_ml_source1_temporary_result6
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_vector_inherent_profiles(profiles: &[String]) -> bool {
    if ((profiles.len() as f64) != 11f64)
        || !(self::accept_rust_borrowed_vector_signature_model(
            &((self::ml_array::optional_at(profiles, self::ml_array::number_index(2f64)))
                .unwrap_or_else(|| String::new())),
        ))
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s25 = self::source_ml_source1_temporary_accept_rust_borrowed_vector_inherent_profiles_loop7(profiles, index);
                match source_ml_source2_s25 {
                    self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Done => {
                        true
                    }
                    self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Return(source_ml_source1_temporary_result7) => {
                        source_ml_source1_temporary_result7
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_vector_source(model: &str, ordinary: bool, minor: f64) -> bool {
    (((self::accept_rust_borrowed_vector_signature_model(model) && ordinary)
        && self::ml_math::is_integer(minor))
        && (minor >= 90f64))
        && (minor < 100f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_vector_signature_model(model: &str) -> bool {
    model == "Rust source function signature 3"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn retain_rust_borrowed_vector_element_position(wrappers: &[String], child: bool) -> bool {
    self::ml_array::includes(wrappers, &String::from("reference"), 0f64, false, |a, b| {
        a == b
    }) || child
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_borrowed_vector_owned_interpretation(child: bool) -> bool {
    !child
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_vector_position(
    wrappers: &[String],
    lifetimes: f64,
    nested: bool,
) -> bool {
    ((((wrappers.len() as f64) == 1f64) && {
        let source_ml_source2_comparison_operand26 =
            self::ml_array::optional_at(wrappers, self::ml_array::number_index(0f64));
        {
            let source_ml_source2_comparison_operand27 = String::from("reference");
            match (
                source_ml_source2_comparison_operand26,
                source_ml_source2_comparison_operand27,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }) && (lifetimes == 1f64))
        && !nested
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_position_error(model: &str, child: bool) -> String {
    if child && !(self::accept_rust_borrowed_vector_signature_model(model)) {
        String::from(
            "a retained array element position requires the borrowed vector source signature profile",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_element_position_error(
    kind: &str,
    owner: &str,
    path: &str,
    arguments_count: f64,
    type_equal: bool,
    owner_equal: bool,
) -> String {
    if (((((kind != "array") || (owner != "library")) || (path != "std::vec::Vec"))
        || (arguments_count != 1f64))
        || !type_equal)
        || !owner_equal
    {
        String::from(
            "a retained array element position must match its complete canonical vector element type and owner",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_vector_element(
    selected: bool,
    minor: f64,
    wrappers: &[String],
    lifetimes: &[String],
    known: bool,
    mutable: bool,
) -> bool {
    (((((((selected && self::ml_math::is_integer(minor)) && (minor >= 90f64))
        && (minor <= 99f64))
        && known)
        && !mutable)
        && ((wrappers.len() as f64) == 1f64))
        && {
            let source_ml_source2_comparison_operand28 =
                self::ml_array::optional_at(wrappers, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand29 = String::from("reference");
                match (
                    source_ml_source2_comparison_operand28,
                    source_ml_source2_comparison_operand29,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        && ((lifetimes.len() as f64) == 1f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_position_budget_error(visits: f64, depth: f64) -> String {
    if ((((!(self::ml_math::is_safe_integer(visits)) || (visits < 0f64)) || (visits > 100_000f64))
        || !(self::ml_math::is_safe_integer(depth)))
        || (depth < 0f64))
        || (depth > 128f64)
    {
        String::from(
            "retained borrowed vector element positions exceed their finite source profile",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_vector_function_owner(
    model: &str,
    owner: &str,
    type_: &str,
    paths: f64,
    children: f64,
    parameters: f64,
    reference: bool,
) -> bool {
    self::accept_rust_borrowed_vector_signature_model(model)
        && self::accept_rust_retained_function_owner(
            "Rust source function signature 2",
            owner,
            type_,
            paths,
            children,
            parameters,
            reference,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_element_phase(
    selected: bool,
    reference: bool,
    header: bool,
    complete: bool,
    closed: bool,
) -> String {
    if !selected || !reference {
        String::new()
    } else if header && !complete {
        String::from("defer")
    } else if complete && closed {
        String::from("admit")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_vector_shape(
    selected: bool,
    kind: &str,
    owner: &str,
    path: &str,
    arguments_count: f64,
    wrappers: f64,
    child: bool,
) -> bool {
    (((((selected && (kind == "array")) && (owner == "library")) && (path == "std::vec::Vec"))
        && (arguments_count == 1f64))
        && (wrappers == 0f64))
        && child
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_borrowed_vector_result_parameters(
    lifetime: &str,
    references: &[bool],
    lifetimes: &[String],
) -> Vec<f64> {
    if ((references.len() as f64) != (lifetimes.len() as f64))
        || ((references.len() as f64) > 65_536f64)
    {
        vec![-1f64]
    } else {
        {
            let result = Vec::<f64>::new();
            {
                let index = 0f64;
                {
                    let result_2 = self::source_ml_source1_temporary_plan_rust_borrowed_vector_result_parameters_loop8(lifetime, references, lifetimes, result, index);
                    if lifetime == "static" {
                        result_2
                    } else if ((lifetime).is_empty()) || (lifetime == "_") {
                        if (result_2.len() as f64) == 1f64 {
                            result_2
                        } else {
                            vec![-1f64]
                        }
                    } else {
                        {
                            let ml_library_condition_1 = {
                                let ml_value = result_2.len() as f64;
                                ml_value != 0f64 && !ml_value.is_nan()
                            };
                            if ml_library_condition_1 {
                                result_2
                            } else {
                                vec![-1f64]
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
pub fn read_rust_borrowed_vector_production_error(
    same: bool,
    reference: bool,
    available: bool,
    collecting: bool,
) -> String {
    if !same || !reference {
        String::from(
            "a borrowed vector element needs its actual matching immutable reference position",
        )
    } else if ((collecting) || (available)) {
        String::new()
    } else {
        String::from("a borrowed vector element needs its actual live source region")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_result_error(
    same: bool,
    produced: bool,
    allowed: bool,
    collecting: bool,
) -> String {
    if !same || (!collecting && (!produced || !allowed)) {
        String::from(
            "a borrowed vector result needs an actual producer and parameter-derived or static element regions",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_element_count_error(count: f64) -> String {
    if (!(self::ml_math::is_safe_integer(count)) || (count < 0f64)) || (count > 65_536f64) {
        String::from("borrowed vector construction exceeds its finite actual element profile")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_vector_literal_context(
    selected: bool,
    expected: bool,
    reference: bool,
    count: f64,
) -> bool {
    ((((selected && expected) && self::ml_math::is_safe_integer(count)) && (count >= 0f64))
        && (count <= 65_536f64))
        && ((count == 0f64) || reference)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_empty_constructor(
    selected: bool,
    owner: &str,
    method: &str,
    count: f64,
) -> String {
    if (selected && self::accept_rust_source_array_owner(owner, false)) && (method == "new") {
        if count == 0f64 {
            String::from("construct")
        } else {
            String::from("arity")
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_argument_error(
    actual: bool,
    expected: bool,
    same: bool,
    owner: bool,
    lifetime: &str,
    required: &str,
) -> String {
    if (actual != expected)
        || (actual && ((!same || !owner) || ((required == "static") && (lifetime != "static"))))
    {
        String::from(
            "borrowed vector arguments must retain their actual reference element owner and region contract",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_observation_error(borrowed: bool, operation: &str) -> String {
    if borrowed
        && !(self::ml_array::includes(
            &vec![
                String::from("len"),
                String::from("is_empty"),
                String::from("transfer"),
            ],
            &operation.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ))
    {
        String::from(
            "borrowed vector observations need their independent reference-preserving source operation profile",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_borrowed_vector_produced_value(
    selected: bool,
    shape: bool,
    proven: bool,
    producer: bool,
) -> bool {
    ((selected && shape) && proven) && producer
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_call_region_error(
    reference: bool,
    regions: f64,
    collecting: bool,
) -> String {
    if (!(self::ml_math::is_safe_integer(regions)) || (regions < 0f64)) || (regions > 65_536f64) {
        String::from("borrowed vector calls exceed their finite actual argument region profile")
    } else if (!collecting && reference) && (regions == 0f64) {
        String::from(
            "a borrowed vector call needs actual argument region provenance; anonymous temporary references need their separate promotion or extension proof",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_expression_position_model_error(model: &str, language: &str) -> String {
    if (model != "Rust borrowed expression source position 1") || (language != "Rust") {
        String::from(
            "a borrowed expression source position needs its exact Rust source interpretation",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_expression_lifetime_scope_error(
    lifetime: &str,
    bindings: &[String],
) -> String {
    if (((!(lifetime).is_empty()) && (lifetime != "_")) && (lifetime != "static"))
        && !(self::ml_array::includes(bindings, &lifetime.to_owned(), 0f64, false, |a, b| a == b))
    {
        String::from(
            "a borrowed expression source lifetime needs its actual enclosing function binder",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_call_lifetime(result: &str, arguments_: &[String]) -> String {
    if result == "static" {
        String::from("static")
    } else if (arguments_.len() as f64) == 0f64 {
        String::new()
    } else {
        {
            let first =
                (self::ml_array::optional_at(arguments_, self::ml_array::number_index(0f64)))
                    .unwrap_or_else(|| String::new());
            if (first != "_") && {
                let ml_values1 = arguments_.to_vec();
                let ml_callback1 = {
                    let first = first.clone();
                    std::rc::Rc::new(move |value: String| -> bool { value == first })
                };
                ml_values1
                    .iter()
                    .cloned()
                    .enumerate()
                    .all(|(ml_index1, ml_value1)| ml_callback1(ml_value1))
            } {
                first.clone()
            } else {
                String::new()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_static_reference_error(
    selected: bool,
    wrappers: &[String],
    lifetimes: &[String],
) -> String {
    if (selected && {
        let source_ml_source2_comparison_operand30 =
            self::ml_array::optional_at(wrappers, self::ml_array::number_index(0f64));
        {
            let source_ml_source2_comparison_operand31 = String::from("reference");
            match (
                source_ml_source2_comparison_operand30,
                source_ml_source2_comparison_operand31,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }) && {
        let source_ml_source2_comparison_operand32 =
            self::ml_array::optional_at(lifetimes, self::ml_array::number_index(0f64));
        {
            let source_ml_source2_comparison_operand33 = String::from("static");
            match (
                source_ml_source2_comparison_operand32,
                source_ml_source2_comparison_operand33,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    } {
        String::from(
            "static source reference demands need their independent global or parameter region proof",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_vector_mutation_error(selected: bool) -> String {
    if selected {
        String::from(
            "the borrowed vector source profile requires immutable bindings and explicit place-write provenance before source mutation",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_expression_position_reservation(
    nodes: f64,
    units: f64,
    added_nodes: f64,
    added_units: f64,
) -> Vec<f64> {
    {
        let source_ml_source1_temporary_values9 = vec![nodes, units, added_nodes, added_units];
        {
            let source_ml_source1_temporary_index9 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s34 = self::source_ml_source1_temporary_read_rust_borrowed_expression_position_reservation_loop10(&(source_ml_source1_temporary_values9), source_ml_source1_temporary_index9);
                match source_ml_source2_s34 {
                    self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Done => {
                        {
                            let next_nodes = nodes + ((added_nodes + 2f64) * 4f64);
                            {
                                let next_units = units + (((added_units + 13f64) + ((String::from("Rust borrowed expression source position 1")).encode_utf16().count() as f64)) * 4f64);
                                if (self::read_rust_standard_sum_source_materialization_error(0f64, 0f64, next_nodes, next_units)).is_empty() {
                                    vec![next_nodes, next_units]
                                } else {
                                    Vec::<f64>::new()
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Return(source_ml_source1_temporary_result10) => {
                        source_ml_source1_temporary_result10
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_borrowed_inherent_position(actual: &[String], target: &[String]) -> Vec<f64> {
    {
        let actual_boxes = 0f64;
        {
            let target_boxes = 0f64;
            {
                let actual_references = 0f64;
                {
                    let target_references = 0f64;
                    {
                        let source_ml_source1_temporary_values11 = actual.to_vec();
                        {
                            let source_ml_source1_temporary_index11 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s35 = self::source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop13(actual_boxes, actual_references, &(source_ml_source1_temporary_values11), source_ml_source1_temporary_index11);
                                match source_ml_source2_s35 {
                                    self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Done(actual_boxes_2, actual_references_2) => {
                                        {
                                            let source_ml_source1_temporary_values12 = target.to_vec();
                                            {
                                                let source_ml_source1_temporary_index12 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s36 = self::source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop14(target_boxes, target_references, &(source_ml_source1_temporary_values12), source_ml_source1_temporary_index12);
                                                    match source_ml_source2_s36 {
                                                        self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Done(target_boxes_2, target_references_2) => {
                                                            if (((actual.len() as f64) > 256f64) || ((target.len() as f64) > 256f64)) || (target_boxes_2 > actual_boxes_2) {
                                                                Vec::<f64>::new()
                                                            } else {
                                                                vec![(actual_boxes_2 - target_boxes_2), self::ml_math::max((actual_references_2 - target_references_2), 0f64), self::ml_math::max((target_references_2 - actual_references_2), 0f64)]
                                                            }
                                                        }
                                                        self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Return(source_ml_source1_temporary_result14) => {
                                                            source_ml_source1_temporary_result14
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Return(source_ml_source1_temporary_result14_2) => {
                                        source_ml_source1_temporary_result14_2
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
                    let source_ml_source2_comparison_operand37 = self::ml_array::optional_at(
                        direct_copies,
                        self::ml_array::number_index(at),
                    );
                    {
                        let source_ml_source2_comparison_operand38 = true;
                        !(match (
                            source_ml_source2_comparison_operand37,
                            source_ml_source2_comparison_operand38,
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
pub enum MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result {
    MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Done,
    MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_rust_borrowed_vector_completion_profiles_loop6(
    profiles: &[String],
    mut index: f64,
) -> self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result {
    loop {
        return if index < (profiles.len() as f64) {
            if (index != 1f64) && {
                let source_ml_source2_comparison_operand39 =
                    self::ml_array::optional_at(profiles, self::ml_array::number_index(index));
                {
                    let source_ml_source2_comparison_operand40 = String::new();
                    !(match (
                        source_ml_source2_comparison_operand39,
                        source_ml_source2_comparison_operand40,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Return(false)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result {
    MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Done,
    MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Return(bool),
}
