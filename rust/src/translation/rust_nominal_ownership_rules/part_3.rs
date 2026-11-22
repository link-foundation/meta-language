// Generated from complete js/src/translation/rust-nominal-ownership-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result;
use super::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result;
use super::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Result;
use super::MlTemporaryPlanRustNominalPatternTransfersLoop18Result;
use super::MlTemporaryPlanRustNominalVectorElementOwnersLoop22Result;
use super::accept_rust_source_array_element;
use super::ml_array;
use super::ml_math;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::read_rust_nominal_ownership_graph_error;
use super::source_ml_temporary_plan_rust_nominal_ownership_owners_loop1;
use super::source_ml_temporary_plan_rust_nominal_ownership_owners_loop2;
use super::source_ml_temporary_plan_rust_nominal_ownership_owners_loop3;
use super::source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop28;
use super::source_ml_temporary_plan_rust_nominal_pattern_transfers_loop18;
use super::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop22;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_rust_nominal_ownership_owners(
    owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    {
        let count = {
            let ml_reference_array46 = kinds.clone();
            ml_reference_array46.borrow().len() as f64
        };
        {
            let size = {
                let ml_reference_array47 = owners.clone();
                ml_reference_array47.borrow().len() as f64
            };
            if ((((((size > 4096f64)
                || (({
                    let ml_reference_array48 = copies.clone();
                    ml_reference_array48.borrow().len() as f64
                }) != size))
                || (({
                    let ml_reference_array49 = clones.clone();
                    ml_reference_array49.borrow().len() as f64
                }) != size))
                || (({
                    let ml_reference_array50 = roots.clone();
                    ml_reference_array50.borrow().len() as f64
                }) != size))
                || (({
                    let ml_reference_array51 = ends.clone();
                    ml_reference_array51.borrow().len() as f64
                }) != count))
                || (({
                    let ml_reference_array52 = targets.clone();
                    ml_reference_array52.borrow().len() as f64
                }) != count))
                || (!(self::read_rust_nominal_ownership_graph_error(
                    count,
                    {
                        let ml_reference_array53 = edges.clone();
                        ml_reference_array53.borrow().len() as f64
                    },
                    0f64,
                ))
                .is_empty())
            {
                std::rc::Rc::new(std::cell::RefCell::new(vec![
                    String::from("error"),
                    String::from("incomplete actual nominal ownership graph"),
                ]))
            } else {
                {
                    let previous = 0f64;
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s51 =
                                self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop1(
                                    ends.clone(),
                                    edges.clone(),
                                    targets.clone(),
                                    count,
                                    size,
                                    previous,
                                    at,
                                );
                            match source_ml_source1_s51 {
                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Done(previous_2) => {
                                    if previous_2 != ({ let ml_reference_array54 = edges.clone(); ml_reference_array54.borrow().len() as f64 }) {
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual nominal ownership edges")]))
                                    } else {
                                        {
                                            let owner = 0f64;
                                            {
                                                let source_ml_source1_s52 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop2(owners.clone(), copies.clone(), clones.clone(), kinds.clone(), roots.clone(), count, size, owner);
                                                match source_ml_source1_s52 {
                                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Done => {
                                                        {
                                                            let known = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values55 = { owners.borrow().clone() }; let ml_callback55 = { let copies = copies.clone(); std::rc::Rc::new(move |x_: String, index: f64| -> String { if self::read_required_boolean_fact(copies.clone(), index) {
                                                                String::from("copy")
                                                            } else {
                                                                String::new()
                                                            } }) }; ml_values55.iter().cloned().enumerate().map(|(ml_index55, ml_value55)| ml_callback55(ml_value55, (ml_index55 as f64))).collect::<Vec<_>>() }));
                                                            {
                                                                let implemented = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values57 = { owners.borrow().clone() }; let ml_callback57 = { let copies = copies.clone(); std::rc::Rc::new(move |x_: String, index: f64| -> bool { self::read_required_boolean_fact(copies.clone(), index) }) }; ml_values57.iter().cloned().enumerate().map(|(ml_index57, ml_value57)| ml_callback57(ml_value57, (ml_index57 as f64))).collect::<Vec<_>>() }));
                                                                {
                                                                    let changed = true;
                                                                    {
                                                                        let work = 0f64;
                                                                        {
                                                                            let source_ml_source1_s53 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop3(copies.clone(), clones.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), count, size, known.clone(), implemented.clone(), changed, work);
                                                                            match source_ml_source1_s53 {
                                                                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Done => {
                                                                                    {
                                                                                        let result = std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles")]));
                                                                                        {
                                                                                            let owner_2 = 0f64;
                                                                                            {
                                                                                                let source_ml_source1_s54 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop4(size, known.clone(), implemented.clone(), result.clone(), owner_2);
                                                                                                match source_ml_source1_s54 {
                                                                                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop4Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop4Done => {
                                                                                                        result.clone()
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Return(source_ml_temporary_result6) => {
                                                                                    source_ml_temporary_result6.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Return(source_ml_temporary_result16) => {
                                                        source_ml_temporary_result16.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(source_ml_temporary_result16_2) => {
                                    source_ml_temporary_result16_2.clone()
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_nominal_clone_error(
    ordinary: bool,
    role: &str,
    actual_clone: bool,
    wrappers: f64,
    borrowed: bool,
    tracking: bool,
    available: bool,
    produced: bool,
) -> String {
    if !ordinary || !actual_clone {
        String::from("nominal Clone needs its actual complete derived implementation")
    } else if (wrappers != 0f64) || borrowed {
        String::from("nominal Clone needs an actual direct source value")
    } else if (role == "copy") || (((role == "owned") && tracking) && (available || produced)) {
        String::new()
    } else {
        String::from(
            "nominal Clone needs an available value or actual producer independently of its type",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_nominal_record_update_error(role: &str) -> String {
    if role == "owned" {
        String::from(
            "owned nominal record updates need their actual partial-field move interpretation",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_nominal_clone_implementation_error(implemented: bool) -> String {
    if implemented {
        String::new()
    } else {
        String::from(
            "Clone needs each actual nominal Clone implementation independently of ownership",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_nominal_pattern_transfers(
    identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    owned: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    produced: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    deferred: bool,
) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    {
        let size = {
            let ml_reference_array59 = identities.clone();
            ml_reference_array59.borrow().len() as f64
        };
        if (((size > 65_536f64)
            || (({
                let ml_reference_array60 = fields.clone();
                ml_reference_array60.borrow().len() as f64
            }) != size))
            || (({
                let ml_reference_array61 = owned.clone();
                ml_reference_array61.borrow().len() as f64
            }) != size))
            || (({
                let ml_reference_array62 = produced.clone();
                ml_reference_array62.borrow().len() as f64
            }) != size)
        {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                String::from("error"),
                String::from("incomplete nominal pattern transfer facts"),
            ]))
        } else {
            {
                let plan =
                    std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("transfers")]));
                {
                    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
                    {
                        let index = 0f64;
                        {
                            let source_ml_source1_s55 = self::source_ml_temporary_plan_rust_nominal_pattern_transfers_loop18(identities.clone(), fields.clone(), owned.clone(), produced.clone(), deferred, size, plan.clone(), seen, index);
                            match source_ml_source1_s55 {
                                self::MlTemporaryPlanRustNominalPatternTransfersLoop18Result::MlTemporaryPlanRustNominalPatternTransfersLoop18Done => {
                                    plan.clone()
                                }
                                self::MlTemporaryPlanRustNominalPatternTransfersLoop18Result::MlTemporaryPlanRustNominalPatternTransfersLoop18Return(source_ml_temporary_result18) => {
                                    source_ml_temporary_result18.clone()
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
pub fn read_rust_nominal_pattern_alternative_error(same: bool) -> String {
    if same {
        String::new()
    } else {
        String::from(
            "every alternative needs the same actual nominal field moves, owners and parent lineage",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_nominal_vector_element_leaf(
    kind: &str,
    wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
) -> bool {
    (({
        let ml_reference_array63 = wrappers.clone();
        ml_reference_array63.borrow().len() as f64
    }) == 0f64)
        && self::accept_rust_source_array_element(kind, false)
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_rust_nominal_vector_element_owners(
    owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<bool>>> {
    {
        let checked = self::plan_rust_nominal_ownership_owners(
            owners.clone(),
            copies.clone(),
            clones.clone(),
            kinds.clone(),
            ends.clone(),
            edges.clone(),
            targets.clone(),
            roots.clone(),
        );
        if ((self::read_required_text_fact(checked, 0f64)) != "profiles")
            || (({
                let ml_reference_array64 = leaves.clone();
                ml_reference_array64.borrow().len() as f64
            }) != ({
                let ml_reference_array65 = kinds.clone();
                ml_reference_array65.borrow().len() as f64
            }))
        {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()))
        } else {
            {
                let known = std::rc::Rc::new(std::cell::RefCell::new({
                    let ml_values66 = { owners.borrow().clone() };
                    let ml_callback66 = std::rc::Rc::new(move || -> bool { false });
                    ml_values66
                        .iter()
                        .cloned()
                        .enumerate()
                        .map(|(ml_index66, ml_value66)| ml_callback66())
                        .collect::<Vec<_>>()
                }));
                {
                    let changed = true;
                    {
                        let work = 0f64;
                        {
                            let source_ml_source1_s56 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop22(owners.clone(), copies.clone(), clones.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), leaves.clone(), known.clone(), changed, work);
                            match source_ml_source1_s56 {
                                self::MlTemporaryPlanRustNominalVectorElementOwnersLoop22Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop22Done => {
                                    known.clone()
                                }
                                self::MlTemporaryPlanRustNominalVectorElementOwnersLoop22Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop22Return(source_ml_temporary_result22) => {
                                    source_ml_temporary_result22.clone()
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
pub fn accept_rust_nominal_vector_element(profile: &str, clone: bool, fields: bool) -> bool {
    ((profile == "copy") && clone) && fields
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_nominal_vector_element_admission(
    legacy: bool,
    nominal: bool,
    header: bool,
    collecting: bool,
    complete: bool,
    kind: &str,
    wrappers: f64,
) -> String {
    if legacy || nominal {
        String::from("admit")
    } else if (((header && collecting) && !complete) && (kind == "named")) && (wrappers == 0f64) {
        String::from("defer")
    } else {
        String::from("refuse")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_nominal_constructor_error(actual: bool, fields: f64, arguments_: f64) -> String {
    if ((((actual && self::ml_math::is_integer(fields)) && self::ml_math::is_integer(arguments_))
        && (fields >= 0f64))
        && (fields <= 65_536f64))
        && ((fields).partial_cmp(&(arguments_)) == Some(std::cmp::Ordering::Equal))
    {
        String::new()
    } else {
        String::from(
            "nominal construction needs its actual constructor owner and exact finite field arity",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_nominal_ownership_vector_template_node(
    kind: &str,
    wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    proven: bool,
    owner: &str,
    path: &str,
    arguments_: f64,
    model: &str,
    element: &str,
    scope: bool,
) -> String {
    if (((((((scope && (kind == "array"))
        && (({
            let ml_reference_array68 = wrappers.clone();
            ml_reference_array68.borrow().len() as f64
        }) == 0f64))
        && proven)
        && (owner == "library"))
        && self::ml_array::includes(
            &{
                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    String::from("std::vec::Vec"),
                    String::from("alloc::vec::Vec"),
                ])))
                .borrow()
                .clone()
            },
            &path.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ))
        && (arguments_ == 1f64))
        && (model == "owned vector"))
        && (element == "named")
    {
        String::from("pending vector")
    } else {
        String::from("unknown")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_nominal_ownership_vector_nodes(
    kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    elements: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    if ((({
        let ml_reference_array70 = kinds.clone();
        ml_reference_array70.borrow().len() as f64
    }) != ({
        let ml_reference_array71 = ends.clone();
        ml_reference_array71.borrow().len() as f64
    })) || (({
        let ml_reference_array72 = kinds.clone();
        ml_reference_array72.borrow().len() as f64
    }) != ({
        let ml_reference_array73 = targets.clone();
        ml_reference_array73.borrow().len() as f64
    }))) || (!(self::read_rust_nominal_ownership_graph_error(
        {
            let ml_reference_array74 = kinds.clone();
            ml_reference_array74.borrow().len() as f64
        },
        {
            let ml_reference_array75 = edges.clone();
            ml_reference_array75.borrow().len() as f64
        },
        0f64,
    ))
    .is_empty())
    {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
    } else {
        {
            let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
            {
                let previous = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s57 = self::source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop28(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), elements.clone(), result.clone(), previous, at);
                        match source_ml_source1_s57 {
                            self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Done(previous_2) => {
                                if previous_2 == ({ let ml_reference_array76 = edges.clone(); ml_reference_array76.borrow().len() as f64 }) {
                                    result.clone()
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
                                }
                            }
                            self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Return(source_ml_temporary_result28) => {
                                source_ml_temporary_result28.clone()
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_nominal_verified_vector_clone(
    kind: &str,
    wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    proven: bool,
    owner: &str,
    path: &str,
    arguments_: f64,
    model: &str,
    element: bool,
) -> bool {
    (((((((kind == "array")
        && (({
            let ml_reference_array77 = wrappers.clone();
            ml_reference_array77.borrow().len() as f64
        }) == 0f64))
        && proven)
        && (owner == "library"))
        && self::ml_array::includes(
            &{
                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    String::from("std::vec::Vec"),
                    String::from("alloc::vec::Vec"),
                ])))
                .borrow()
                .clone()
            },
            &path.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ))
        && (arguments_ == 1f64))
        && (model == "owned vector"))
        && element
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop4Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop4Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop4(
    mut size: f64,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut owner: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop4Result {
    loop {
        if owner < size {
            {
                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored5 = {
                    let ml_method_owner79 = result.clone();
                    let ml_method_argument79_0 =
                        self::read_required_text_fact(known.clone(), owner);
                    let ml_inserted = vec![ml_method_argument79_0];
                    let mut ml_method_values79 = ml_method_owner79.borrow_mut();
                    let ml_length = ml_method_values79
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values79.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored6 = {
                        let ml_method_owner80 = result.clone();
                        let ml_method_argument80_0 =
                            if self::read_required_boolean_fact(implemented.clone(), owner) {
                                String::from("Clone")
                            } else {
                                String::new()
                            };
                        let ml_inserted = vec![ml_method_argument80_0];
                        let mut ml_method_values80 = ml_method_owner80.borrow_mut();
                        let ml_length = ml_method_values80
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values80.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let owner_2 = owner + 1f64;
                        {
                            owner = owner_2;
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop4Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop3Done,
    MlTemporaryPlanRustNominalOwnershipOwnersLoop3Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop9Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop9Done(bool, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop9(
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut end: f64,
    mut ready: bool,
    mut clone: bool,
    mut edge: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop9Result {
    loop {
        if edge < end {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                {
                    let ready_3 = if (self::read_required_text_fact(roles.clone(), child)) == "" {
                        { false }
                    } else {
                        ready
                    };
                    if !(self::read_required_boolean_fact(clone_fields.clone(), child)) {
                        {
                            let clone_2 = false;
                            {
                                let edge_2 = edge + 1f64;
                                {
                                    (ready, clone, edge) = (ready_3, clone_2, edge_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let edge_3 = edge + 1f64;
                            {
                                (ready, edge) = (ready_3, edge_3);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop9Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop9Done(ready, clone)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop8(
    mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut size: f64,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut changed: bool,
    mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut owner: f64,
) -> bool {
    loop {
        return if owner < size {
            if self::read_required_boolean_fact(copies.clone(), owner) {
                {
                    let owner_2 = owner + 1f64;
                    {
                        owner = owner_2;
                        continue;
                    }
                }
            }
            {
                {
                    let root = self::read_required_number_fact(roots.clone(), owner);
                    {
                        let start = if root == 0f64 {
                            0f64
                        } else {
                            self::read_required_number_fact(ends.clone(), (root - 1f64))
                        };
                        {
                            let end = self::read_required_number_fact(ends.clone(), root);
                            {
                                let ready = end > start;
                                {
                                    let clone = true;
                                    {
                                        let edge = start;
                                        {
                                            let source_ml_source1_s58 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop9(edges.clone(), roles.clone(), clone_fields.clone(), end, ready, clone, edge);
                                            match source_ml_source1_s58 {
                                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop9Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop9Done(ready_2, clone_2) => {
                                                    {
                                                        let changed_3 = if ready_2 && ((self::read_required_text_fact(known.clone(), owner)).is_empty()) {
                                                            {
                                                                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored10 = { let ml_array_owner81 = known.clone(); let ml_array_argument81_0: f64 = owner; let ml_array_argument81_1: f64 = 1f64; let ml_array_argument81_2: String = String::from("owned"); let ml_array_length = ml_array_owner81.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument81_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument81_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument81_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner81.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                                                {
                                                                    true
                                                                }
                                                            }
                                                        } else {
                                                            changed
                                                        };
                                                        if ((ready_2 && clone_2) && self::read_required_boolean_fact(clones.clone(), owner)) && !(self::read_required_boolean_fact(implemented.clone(), owner)) {
                                                            {
                                                                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored11 = { let ml_array_owner82 = implemented.clone(); let ml_array_argument82_0: f64 = owner; let ml_array_argument82_1: f64 = 1f64; let ml_array_argument82_2: bool = true; let ml_array_length = ml_array_owner82.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument82_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument82_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument82_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner82.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                                                {
                                                                    let changed_4 = true;
                                                                    {
                                                                        let owner_3 = owner + 1f64;
                                                                        {
                                                                            (changed, owner) = (changed_4, owner_3);
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
{
                                                            {
                                                                let owner_4 = owner + 1f64;
                                                                {
                                                                    (changed, owner) = (changed_3, owner_4);
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
                            }
                        }
                    }
                }
            }
        } else {
            changed
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop7Done(f64),
    MlTemporaryPlanRustNominalOwnershipOwnersLoop7Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop12Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop12Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop12(
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
    mut child_roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut child_clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut edge: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop12Result {
    loop {
        if edge < (self::read_required_number_fact(ends.clone(), at)) {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                {
                    let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored15 = {
                        let ml_method_owner83 = child_roles.clone();
                        let ml_method_argument83_0 =
                            self::read_required_text_fact(roles.clone(), child);
                        let ml_inserted = vec![ml_method_argument83_0];
                        let mut ml_method_values83 = ml_method_owner83.borrow_mut();
                        let ml_length = ml_method_values83
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values83.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored16 = {
                            let ml_method_owner84 = child_clones.clone();
                            let ml_method_argument84_0 =
                                self::read_required_boolean_fact(clone_fields.clone(), child);
                            let ml_inserted = vec![ml_method_argument84_0];
                            let mut ml_method_values84 = ml_method_owner84.borrow_mut();
                            let ml_length = ml_method_values84
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4_294_967_295usize)
                                .expect("Invalid array length");
                            ml_method_values84.extend(ml_inserted);
                            ml_length as f64
                        };
                        {
                            let edge_2 = edge + 1f64;
                            {
                                edge = edge_2;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop12Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop12Done
        };
    }
}
