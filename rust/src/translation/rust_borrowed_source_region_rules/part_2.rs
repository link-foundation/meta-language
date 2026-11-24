// Generated from complete js/src/translation/rust-borrowed-source-region-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result;
use super::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result;
use super::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result;
use super::MlTemporaryReadRustBorrowedSourceReadErrorJoin9;
use super::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result;
use super::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result;
use super::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result;
use super::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result;
use super::accept_rust_source_signature_primitive;
use super::consume_rust_owned_binding_state;
use super::expire_rust_owned_binding_state;
use super::merge_rust_owned_binding_states;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_rust_owned_binding_read_error;
use super::source_ml_temporary_accept_rust_borrowed_source_box_reference_loop19;
use super::source_ml_temporary_accept_rust_borrowed_source_box_reference_loop20;
use super::source_ml_temporary_read_rust_borrowed_source_lifetime_error_loop14;
use super::source_ml_temporary_read_rust_borrowed_source_read_error_loop10;
use super::source_ml_temporary_read_rust_borrowed_source_region_budget_error_loop16;
use super::source_ml_temporary_read_rust_borrowed_source_return_error_loop12;
use super::source_ml_temporary_reserve_rust_borrowed_source_position_loop18;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_borrowed_source_value_role(
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    arguments_count: f64,
    wrappers: &[String],
    borrowed: bool,
) -> String {
    if {
        let source_ml_source1_comparison_operand3 =
            self::ml_array::optional_at(wrappers, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand4 = String::from("reference");
            match (
                source_ml_source1_comparison_operand3,
                source_ml_source1_comparison_operand4,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    } {
        String::from("reference")
    } else if {
        let source_ml_source1_comparison_operand5 =
            self::ml_array::optional_at(wrappers, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand6 = String::from("box");
            match (
                source_ml_source1_comparison_operand5,
                source_ml_source1_comparison_operand6,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    } {
        String::from("box")
    } else if (((kind == "array") && (owner == "library")) && (path == "std::vec::Vec"))
        && (arguments_count == 1f64)
    {
        if borrowed {
            String::from("borrowed vector")
        } else {
            String::from("owned")
        }
    } else if (kind == "data") && ((owner == "declaration") || (owner == "library")) {
        String::from("owned")
    } else if (((kind == "string") && (owner == "library"))
        && self::ml_array::includes(
            &vec![
                String::from("std::string::String"),
                String::from("alloc::string::String"),
            ],
            &path.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ))
        && (arguments_count == 0f64)
    {
        String::from("string")
    } else if ((kind == "string") && (owner == "primitive")) && (path == "str") {
        String::from("text")
    } else if ((((kind == "unit") && (owner == "unit")) && ((path).is_empty()))
        && (arguments_count == 0f64))
        || (((owner == "primitive") && (arguments_count == 0f64))
            && self::accept_rust_source_signature_primitive(kind, path, bits, signed))
    {
        String::from("copy")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_use_plan(role: &str, mode: &str) -> String {
    if (self::ml_array::includes(
        &vec![
            String::from("copy"),
            String::from("inferred numeric"),
            String::from("reference"),
            String::from("owned"),
            String::from("string"),
            String::from("box"),
            String::from("borrowed vector"),
        ],
        &role.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )) {
        if (mode == "move")
            && self::ml_array::includes(
                &vec![
                    String::from("owned"),
                    String::from("string"),
                    String::from("box"),
                    String::from("borrowed vector"),
                ],
                &role.to_owned(),
                0f64,
                false,
                |a, b| a == b,
            )
        {
            String::from("consume")
        } else {
            String::from("read")
        }
    } else {
        String::from("unsupported")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_read_error(state: &str, loans: &[String]) -> String {
    {
        let source_ml_source1_s7 = if !(state).is_empty() {
            {
                let error = self::read_rust_owned_binding_read_error(state);
                if error.is_empty() {
                    self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Next
                } else {
                    self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Return(error)
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Next
        };
        match source_ml_source1_s7 {
            self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Next => {
                {
                    let source_ml_temporary_values8 = loans.to_vec();
                    {
                        let source_ml_temporary_index8 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s8 = self::source_ml_temporary_read_rust_borrowed_source_read_error_loop10(&(source_ml_temporary_values8), source_ml_temporary_index8);
                            match source_ml_source1_s8 {
                                self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Done => {
                                    String::new()
                                }
                                self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Return(source_ml_temporary_result10) => {
                                    source_ml_temporary_result10
                                }
                            }
                        }
                    }
                }
            }
            self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Return(source_ml_temporary_result10_2) => {
                source_ml_temporary_result10_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn consume_rust_borrowed_source_region(state: &str) -> String {
    self::consume_rust_owned_binding_state(state)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn expire_rust_borrowed_source_region(state: &str) -> String {
    self::expire_rust_owned_binding_state(state)
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_borrowed_source_adjustment(
    source: &str,
    target: &str,
    same: bool,
    inner: bool,
    source_inner: bool,
    place: bool,
    literal: bool,
    text: bool,
    source_references: f64,
    target_references: f64,
) -> String {
    if (same && (source == target))
        && ((source_references).partial_cmp(&(target_references))
            == Some(std::cmp::Ordering::Equal))
    {
        String::from("preserve")
    } else if (source == "inferred numeric") && (target == "copy") {
        String::from("constrain primitive")
    } else if (target == "reference") && (same || inner) {
        if (source == "reference") && (target_references <= source_references) {
            String::from("project reference")
        } else if place {
            String::from("borrow")
        } else if literal {
            String::from("static literal")
        } else {
            String::from("temporary borrow")
        }
    } else if ((source == "reference") && (target == "copy")) && same {
        String::from("copy referent")
    } else if (target == "box") && inner {
        String::from("construct box")
    } else if (source == "box") && source_inner {
        if target == "copy" {
            String::from("copy box payload")
        } else {
            String::from("move box payload")
        }
    } else if ((source == "reference") && (target == "string")) && text {
        String::from("materialize text")
    } else {
        String::from("unsupported")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_position_flow_error(
    role: &str,
    same: bool,
    static_demand: bool,
    static_proof: bool,
) -> String {
    if same {
        if static_demand && !static_proof {
            String::from("a static source lifetime cannot be supplied by a local binder region")
        } else if (self::ml_array::includes(
            &vec![
                String::from("copy"),
                String::from("reference"),
                String::from("owned"),
                String::from("string"),
                String::from("box"),
                String::from("borrowed vector"),
            ],
            &role.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )) {
            String::new()
        } else {
            String::from("the source expression needs a separate value-domain proof")
        }
    } else {
        String::from("source expression qualifiers or owners disagree with their actual value flow")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_return_error(
    regions: &[f64],
    allowed: &[f64],
    static_regions: &[f64],
    interface_known: bool,
) -> String {
    if interface_known {
        {
            let source_ml_temporary_values11 = regions.to_vec();
            {
                let source_ml_temporary_index11 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s9 =
                        self::source_ml_temporary_read_rust_borrowed_source_return_error_loop12(
                            allowed,
                            static_regions,
                            &(source_ml_temporary_values11),
                            source_ml_temporary_index11,
                        );
                    match source_ml_source1_s9 {
                        self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Done => {
                            String::new()
                        }
                        self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Return(source_ml_temporary_result12) => {
                            source_ml_temporary_result12
                        }
                    }
                }
            }
        }
    } else {
        String::from(
            "a borrowed source result needs its actual source parameter lifetime interface",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_lifetime_error(demand: &str, sources: &[String]) -> String {
    if ((demand).is_empty()) || (demand == "_") {
        String::new()
    } else {
        {
            let source_ml_temporary_values13 = sources.to_vec();
            {
                let source_ml_temporary_index13 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s10 =
                        self::source_ml_temporary_read_rust_borrowed_source_lifetime_error_loop14(
                            demand,
                            &(source_ml_temporary_values13),
                            source_ml_temporary_index13,
                        );
                    match source_ml_source1_s10 {
                        self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Done => {
                            String::new()
                        }
                        self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Return(source_ml_temporary_result14) => {
                            source_ml_temporary_result14
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_target_field_error(
    kind: &str,
    wrappers: &[String],
    child: bool,
) -> String {
    if (self::ml_array::includes(wrappers, &String::from("reference"), 0f64, false, |a, b| {
        a == b
    }) || child)
        || self::ml_array::includes(
            &vec![
                String::from("function"),
                String::from("tuple"),
                String::from("record"),
                String::from("map"),
                String::from("set"),
                String::from("cell"),
            ],
            &kind.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )
    {
        String::from(
            "a borrowed referent needs a closed target-copy field graph with represented source qualifiers",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_operation(kind: &str) -> String {
    if kind == "toString" {
        String::from("show")
    } else if self::ml_array::includes(
        &vec![
            String::from("lit"),
            String::from("unit"),
            String::from("var"),
            String::from("let"),
            String::from("if"),
            String::from("array"),
            String::from("call"),
            String::from("ctor"),
            String::from("index"),
            String::from("length"),
            String::from("print"),
            String::from("cast"),
            String::from("unary"),
            String::from("binary"),
            String::from("math"),
            String::from("show"),
            String::from("concat"),
        ],
        &kind.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        kind.to_owned()
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_region_budget_error(
    visits: f64,
    depth: f64,
    roots: f64,
    loans: f64,
) -> String {
    {
        let source_ml_temporary_values15 = vec![visits, depth, roots, loans];
        {
            let source_ml_temporary_index15 = self::ml::Big::from_i128(0);
            {
                let source_ml_source1_s11 =
                    self::source_ml_temporary_read_rust_borrowed_source_region_budget_error_loop16(
                        &(source_ml_temporary_values15),
                        source_ml_temporary_index15,
                    );
                match source_ml_source1_s11 {
                    self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Done => {
                        if (((visits > 100_000f64) || (depth > 256f64)) || (roots > 100_000f64)) || (loans > 1_048_576f64) {
                            String::from("the immutable source binder and region proof exceeds its finite resource profile")
                        } else {
                            String::new()
                        }
                    }
                    self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Return(source_ml_temporary_result16) => {
                        source_ml_temporary_result16
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn reserve_rust_borrowed_source_position(
    nodes: f64,
    units: f64,
    add_nodes: f64,
    add_units: f64,
) -> Vec<f64> {
    {
        let source_ml_temporary_values17 = vec![nodes, units, add_nodes, add_units];
        {
            let source_ml_temporary_index17 = self::ml::Big::from_i128(0);
            {
                let source_ml_source1_s12 =
                    self::source_ml_temporary_reserve_rust_borrowed_source_position_loop18(
                        &(source_ml_temporary_values17),
                        source_ml_temporary_index17,
                    );
                match source_ml_source1_s12 {
                    self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result::MlTemporaryReserveRustBorrowedSourcePositionLoop18Done => {
                        {
                            let next_nodes = nodes + (4f64 * add_nodes);
                            {
                                let next_units = units + (4f64 * add_units);
                                if (next_nodes > 1_048_576f64) || (next_units > 8_388_608f64) {
                                    Vec::<f64>::new()
                                } else {
                                    vec![next_nodes, next_units]
                                }
                            }
                        }
                    }
                    self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result::MlTemporaryReserveRustBorrowedSourcePositionLoop18Return(source_ml_temporary_result18) => {
                        source_ml_temporary_result18
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_runtime_error(selected: bool, runtime_shell: bool) -> String {
    if selected && runtime_shell {
        String::from(
            "borrowed source body positions require their independent runtime-shell binder adapter",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_observation_error(role: &str, operation: &str) -> String {
    if self::ml_array::includes(
        &vec![
            String::from("source state packet"),
            String::from("source loop packet"),
            String::from("source loop control"),
        ],
        &role.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        String::from("source state packets are interpreted through their actual result/state slots")
    } else if self::ml_array::includes(
        &vec![
            String::from("reference"),
            String::from("box"),
            String::from("owned"),
            String::from("borrowed vector"),
        ],
        &role.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) && (operation != "len")
    {
        String::from(
            "this source observation needs an independently represented value or source trait proof",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_observation_role(
    role: &str,
    kind: &str,
    owner: &str,
    path: &str,
    argument_count: f64,
    wrappers: &[String],
) -> String {
    if (((((role == "reference") && (kind == "string"))
        && (((owner == "primitive") && (path == "str"))
            || ((owner == "library")
                && self::ml_array::includes(
                    &vec![
                        String::from("std::string::String"),
                        String::from("alloc::string::String"),
                    ],
                    &path.to_owned(),
                    0f64,
                    false,
                    |a, b| a == b,
                ))))
        && (argument_count == 0f64))
        && ((wrappers.len() as f64) == 1f64))
        && {
            let source_ml_source1_comparison_operand13 =
                self::ml_array::optional_at(wrappers, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand14 = String::from("reference");
                match (
                    source_ml_source1_comparison_operand13,
                    source_ml_source1_comparison_operand14,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }
    {
        String::from("text")
    } else {
        role.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_match_error(
    role: &str,
    kind: &str,
    pattern: &str,
    fields: f64,
    bindings: f64,
) -> String {
    if (role != "reference") || (kind != "data") {
        String::from("source matching requires its actual borrowed nominal place adapter")
    } else if (self::ml_array::includes(
        &vec![
            String::from("wild"),
            String::from("bind"),
            String::from("ctor"),
        ],
        &pattern.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )) {
        if (pattern == "ctor")
            && ((!(self::ml_math::is_safe_integer(fields)) || (fields < 0f64))
                || (fields != bindings))
        {
            String::from("source constructor patterns need every actual declared field position")
        } else {
            String::new()
        }
    } else {
        String::from("this source pattern needs its independent binding projection")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_owned_match_error(
    composed: bool,
    kind: &str,
    pattern: &str,
    fields: f64,
    bindings: f64,
) -> String {
    if composed {
        self::read_rust_borrowed_source_match_error("reference", kind, pattern, fields, bindings)
    } else {
        String::from("owned matches require the explicit closed-call region composition")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn reserve_rust_borrowed_source_branches(cells: f64, addition: f64) -> f64 {
    if (((((!(self::ml_math::is_safe_integer(cells))
        || !(self::ml_math::is_safe_integer(addition)))
        || (cells < 0f64))
        || (addition < 0f64))
        || {
            let source_ml_source1_same_value_operand15 = cells;
            {
                let source_ml_source1_same_value_operand16 = -0f64;
                ((((source_ml_source1_same_value_operand15)
                    .partial_cmp(&(source_ml_source1_same_value_operand16))
                    == Some(std::cmp::Ordering::Equal))
                    && ((source_ml_source1_same_value_operand15 != 0f64)
                        || ((1f64 / source_ml_source1_same_value_operand15)
                            == (1f64 / source_ml_source1_same_value_operand16))))
                    || ((source_ml_source1_same_value_operand15
                        != source_ml_source1_same_value_operand15)
                        && (source_ml_source1_same_value_operand16
                            != source_ml_source1_same_value_operand16)))
            }
        })
        || {
            let source_ml_source1_same_value_operand17 = addition;
            {
                let source_ml_source1_same_value_operand18 = -0f64;
                ((((source_ml_source1_same_value_operand17)
                    .partial_cmp(&(source_ml_source1_same_value_operand18))
                    == Some(std::cmp::Ordering::Equal))
                    && ((source_ml_source1_same_value_operand17 != 0f64)
                        || ((1f64 / source_ml_source1_same_value_operand17)
                            == (1f64 / source_ml_source1_same_value_operand18))))
                    || ((source_ml_source1_same_value_operand17
                        != source_ml_source1_same_value_operand17)
                        && (source_ml_source1_same_value_operand18
                            != source_ml_source1_same_value_operand18)))
            }
        })
        || (addition > (1_048_576f64 - cells))
    {
        -1f64
    } else {
        cells + addition
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn merge_rust_borrowed_source_region(left: &str, right: &str) -> String {
    self::merge_rust_owned_binding_states(left, right, true, true)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_match_operation(kind: &str) -> String {
    if kind == "match" {
        String::from("match")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_source_box_reference(
    source: &[String],
    target: &[String],
    canonical: bool,
    same_leaf: bool,
) -> bool {
    if (((!canonical || !same_leaf) || {
        let source_ml_source1_comparison_operand19 =
            self::ml_array::optional_at(source, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand20 = String::from("reference");
            !(match (
                source_ml_source1_comparison_operand19,
                source_ml_source1_comparison_operand20,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })
        }
    }) || {
        let source_ml_source1_comparison_operand21 =
            self::ml_array::optional_at(target, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand22 = String::from("reference");
            !(match (
                source_ml_source1_comparison_operand21,
                source_ml_source1_comparison_operand22,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })
        }
    }) || ((source.len() as f64) <= (target.len() as f64))
    {
        false
    } else {
        {
            let index = 1f64;
            {
                let source_ml_source1_s23 =
                    self::source_ml_temporary_accept_rust_borrowed_source_box_reference_loop19(
                        source, index,
                    );
                match source_ml_source1_s23 {
                    self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Done => {
                        {
                            let index_2 = 1f64;
                            {
                                let source_ml_source1_s24 = self::source_ml_temporary_accept_rust_borrowed_source_box_reference_loop20(target, index_2);
                                match source_ml_source1_s24 {
                                    self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Done => {
                                        true
                                    }
                                    self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Return(source_ml_temporary_result20) => {
                                        source_ml_temporary_result20
                                    }
                                }
                            }
                        }
                    }
                    self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Return(source_ml_temporary_result20_2) => {
                        source_ml_temporary_result20_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_iterator_operation(kind: &str, operation: &str) -> String {
    if kind == "append" {
        String::from("iterator concat")
    } else if (kind == "method")
        && self::ml_array::includes(
            &vec![String::from("map"), String::from("slice")],
            &operation.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )
    {
        format!("iterator {}", operation.to_owned())
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_source_storage_write_error(state: &str) -> String {
    if self::ml_array::includes(
        &vec![
            String::from("available"),
            String::from("moved"),
            String::from("possibly moved"),
        ],
        &state.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        String::new()
    } else {
        String::from("a source whole-root write requires its initialized lexical storage scope")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOwnedBindingGuardLoop1Result {
    MlTemporaryAcceptRustOwnedBindingGuardLoop1Done,
    MlTemporaryAcceptRustOwnedBindingGuardLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_rust_owned_binding_guard_loop1(
    before: &[String],
    after: &[String],
    mut index: f64,
) -> self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result {
    loop {
        return if index < (before.len() as f64) {
            if {
                let source_ml_source1_comparison_operand25 =
                    self::ml_array::optional_at(before, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand26 =
                        self::ml_array::optional_at(after, self::ml_array::number_index(index));
                    !(match (
                        source_ml_source1_comparison_operand25,
                        source_ml_source1_comparison_operand26,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result::MlTemporaryAcceptRustOwnedBindingGuardLoop1Return(false)
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
            self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result::MlTemporaryAcceptRustOwnedBindingGuardLoop1Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_rust_owned_loan_bindings_loop4(
    mut result: Vec<String>,
    source_ml_temporary_values2: &[String],
    mut source_ml_temporary_index2: self::ml::Big,
) -> Vec<String> {
    loop {
        if source_ml_temporary_index2
            < (self::ml::Big::from_u128(source_ml_temporary_values2.len() as u128))
        {
            {
                let identity = self::ml_array::at(
                    source_ml_temporary_values2,
                    source_ml_temporary_index2.to_index(),
                );
                if !(self::ml_array::includes(&result, &identity.clone(), 0f64, false, |a, b| {
                    a == b
                })) {
                    {
                        let result_2 = {
                            let (ml_append_right, ml_append_left) =
                                (vec![identity.clone()], result);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let source_ml_temporary_index2_2 =
                                source_ml_temporary_index2.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_temporary_index2) =
                                    (result_2, source_ml_temporary_index2_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_temporary_index2_3 =
                            source_ml_temporary_index2.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index2 = source_ml_temporary_index2_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}
