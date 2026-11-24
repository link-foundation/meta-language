// Generated from complete js/src/translation/rust-owned-recursive-datatype-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result;
use super::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result;
use super::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result;
use super::MlTemporaryPlanRustNominalOwnershipOwnersLoop4Result;
use super::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Result;
use super::MlTemporaryPlanRustNominalPatternTransfersLoop18Result;
use super::MlTemporaryPlanRustNominalVectorElementOwnersLoop22Result;
use super::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop31Result;
use super::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result;
use super::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Result;
use super::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Result;
use super::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Result;
use super::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop36Result;
use super::accept_rust_source_array_element;
use super::ml_array;
use super::ml_math;
use super::read_required_boolean_fact;
use super::read_required_text_fact;
use super::read_rust_nominal_ownership_graph_error;
use super::source_ml_temporary_plan_rust_nominal_ownership_owners_loop1;
use super::source_ml_temporary_plan_rust_nominal_ownership_owners_loop2;
use super::source_ml_temporary_plan_rust_nominal_ownership_owners_loop3;
use super::source_ml_temporary_plan_rust_nominal_ownership_owners_loop4;
use super::source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop28;
use super::source_ml_temporary_plan_rust_nominal_pattern_transfers_loop18;
use super::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop22;
use super::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop31;
use super::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop32;
use super::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop33;
use super::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop34;
use super::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop35;
use super::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop36;

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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_rust_owned_recursive_nominal_owners(
    owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
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
        if (((self::read_required_text_fact(checked, 0f64)) != "profiles")
            || (({
                let ml_reference_array79 = leaves.clone();
                ml_reference_array79.borrow().len() as f64
            }) != ({
                let ml_reference_array80 = kinds.clone();
                ml_reference_array80.borrow().len() as f64
            })))
            || (!(self::read_rust_nominal_ownership_graph_error(
                {
                    let ml_reference_array81 = kinds.clone();
                    ml_reference_array81.borrow().len() as f64
                },
                {
                    let ml_reference_array82 = edges.clone();
                    ml_reference_array82.borrow().len() as f64
                },
                0f64,
            ))
            .is_empty())
        {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                String::from("error"),
                String::from("incomplete actual recursive datatype graph"),
            ]))
        } else {
            {
                let count = {
                    let ml_reference_array83 = kinds.clone();
                    ml_reference_array83.borrow().len() as f64
                };
                {
                    let size = {
                        let ml_reference_array84 = owners.clone();
                        ml_reference_array84.borrow().len() as f64
                    };
                    {
                        let eligible = std::rc::Rc::new(std::cell::RefCell::new({
                            let ml_values85 = { owners.borrow().clone() };
                            let ml_callback85 = std::rc::Rc::new(move || -> bool { true });
                            ml_values85
                                .iter()
                                .cloned()
                                .enumerate()
                                .map(|(ml_index85, ml_value85)| ml_callback85())
                                .collect::<Vec<_>>()
                        }));
                        {
                            let copy_nodes =
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                            {
                                let work = 0f64;
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_source1_s58 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop31(copies.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), count, copy_nodes.clone(), at);
                                        match source_ml_source1_s58 {
                                            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop31Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop31Done => {
                                                {
                                                    let owner = 0f64;
                                                    {
                                                        let source_ml_source1_s59 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop32(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), count, size, work, owner);
                                                        match source_ml_source1_s59 {
                                                            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Done(work_2) => {
                                                                {
                                                                    let changed = true;
                                                                    {
                                                                        let source_ml_source1_s60 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop33(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), leaves.clone(), count, size, eligible.clone(), copy_nodes.clone(), work_2, changed);
                                                                        match source_ml_source1_s60 {
                                                                            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Done(work_3, changed_2) => {
                                                                                {
                                                                                    let inhabited = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values87 = { owners.borrow().clone() }; let ml_callback87 = std::rc::Rc::new(move || -> bool { false }); ml_values87.iter().cloned().enumerate().map(|(ml_index87, ml_value87)| ml_callback87()).collect::<Vec<_>>() }));
                                                                                    {
                                                                                        let ranks = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values89 = { owners.borrow().clone() }; let ml_callback89 = std::rc::Rc::new(move || -> f64 { -1f64 }); ml_values89.iter().cloned().enumerate().map(|(ml_index89, ml_value89)| ml_callback89()).collect::<Vec<_>>() }));
                                                                                        {
                                                                                            let witnesses = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values91 = { owners.borrow().clone() }; let ml_callback91 = std::rc::Rc::new(move || -> f64 { -1f64 }); ml_values91.iter().cloned().enumerate().map(|(ml_index91, ml_value91)| ml_callback91()).collect::<Vec<_>>() }));
                                                                                            {
                                                                                                let rank = 0f64;
                                                                                                {
                                                                                                    let changed_3 = true;
                                                                                                    {
                                                                                                        let source_ml_source1_s61 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop34(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), leaves.clone(), count, size, eligible.clone(), work_3, changed_3, inhabited.clone(), ranks.clone(), witnesses.clone(), rank);
                                                                                                        match source_ml_source1_s61 {
                                                                                                            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Done(work_4, changed_4) => {
                                                                                                                {
                                                                                                                    let implemented = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values93 = { owners.borrow().clone() }; let ml_callback93 = { let eligible = eligible.clone(); let inhabited = inhabited.clone(); let copies = copies.clone(); let clones = clones.clone(); std::rc::Rc::new(move |x_: String, owner_2: f64| -> bool { (self::read_required_boolean_fact(eligible.clone(), owner_2) && self::read_required_boolean_fact(inhabited.clone(), owner_2)) && (self::read_required_boolean_fact(copies.clone(), owner_2) || self::read_required_boolean_fact(clones.clone(), owner_2)) }) }; ml_values93.iter().cloned().enumerate().map(|(ml_index93, ml_value93)| ml_callback93(ml_value93, (ml_index93 as f64))).collect::<Vec<_>>() }));
                                                                                                                    {
                                                                                                                        let changed_5 = true;
                                                                                                                        {
                                                                                                                            let source_ml_source1_s62 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop35(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), leaves.clone(), count, size, work_4, changed_5, implemented.clone());
                                                                                                                            match source_ml_source1_s62 {
                                                                                                                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Done => {
                                                                                                                                    {
                                                                                                                                        let result = std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles")]));
                                                                                                                                        {
                                                                                                                                            let owner_2 = 0f64;
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_s63 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop36(copies.clone(), size, eligible.clone(), inhabited.clone(), ranks.clone(), witnesses.clone(), implemented.clone(), result.clone(), owner_2);
                                                                                                                                                match source_ml_source1_s63 {
                                                                                                                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop36Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop36Done => {
                                                                                                                                                        result.clone()
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Return(source_ml_temporary_result41) => {
                                                                                                                                    source_ml_temporary_result41.clone()
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Return(source_ml_temporary_result46) => {
                                                                                                                source_ml_temporary_result46.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Return(source_ml_temporary_result55) => {
                                                                                source_ml_temporary_result55.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Return(source_ml_temporary_result60) => {
                                                                source_ml_temporary_result60.clone()
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
pub fn accept_rust_owned_box_type_node(
    wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    owner: &str,
    path: &str,
    arguments_: f64,
) -> bool {
    ((((({
        let ml_reference_array95 = wrappers.clone();
        ml_reference_array95.borrow().len() as f64
    }) > 0f64)
        && ((self::read_required_text_fact(wrappers.clone(), 0f64)) == "box"))
        && (owner == "library"))
        && self::ml_array::includes(
            &{
                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    String::from("std::boxed::Box"),
                    String::from("alloc::boxed::Box"),
                ])))
                .borrow()
                .clone()
            },
            &path.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ))
        && (arguments_ == 1f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_nominal_header_type(data: bool, complete: bool, role: &str) -> bool {
    (data && complete) && ((role == "copy") || (role == "owned"))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_owned_container_type_node(
    kind: &str,
    wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    proven: bool,
    owner: &str,
    path: &str,
    arguments_: f64,
    model: &str,
    leaf: &str,
    copy: bool,
    nominal: &str,
    scope: bool,
) -> String {
    if !scope || !proven {
        String::from("unknown")
    } else if self::accept_rust_owned_box_type_node(wrappers.clone(), owner, path, arguments_) {
        String::from("box")
    } else {
        {
            let ml_library_condition_2 = {
                let ml_value = {
                    let ml_reference_array98 = wrappers.clone();
                    ml_reference_array98.borrow().len() as f64
                };
                ml_value != 0f64 && !ml_value.is_nan()
            };
            if ml_library_condition_2 {
                if {
                    let ml_values99 = { wrappers.borrow().clone() };
                    let ml_callback99 =
                        std::rc::Rc::new(move |value: String| -> bool { value == "reference" });
                    ml_values99
                        .iter()
                        .cloned()
                        .enumerate()
                        .all(|(ml_index99, ml_value99)| ml_callback99(ml_value99))
                } {
                    String::from("reference")
                } else {
                    String::from("unknown")
                }
            } else if (kind == "named") && ((nominal == "copy") || (nominal == "owned")) {
                String::from("named")
            } else if ((((kind == "array") && (owner == "library"))
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
                && (model == "owned vector")
            {
                String::from("vector")
            } else if ((leaf == "copy") && copy)
                && self::ml_array::includes(
                    &{
                        (std::rc::Rc::new(std::cell::RefCell::new(vec![
                            String::from("bool"),
                            String::from("fixed"),
                            String::from("float"),
                            String::from("unit"),
                        ])))
                        .borrow()
                        .clone()
                    },
                    &kind.to_owned(),
                    0f64,
                    false,
                    |a, b| a == b,
                )
            {
                String::from("copy")
            } else if leaf == "string" {
                String::from("string")
            } else if ((kind == "tuple") && (leaf == "tuple")) && copy {
                String::from("tuple")
            } else {
                String::from("unknown")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_container_sum_node(
    owner: &str,
    arguments_: f64,
    proven: bool,
    provisional: bool,
    wrappers: f64,
) -> String {
    if (proven && !provisional) && (wrappers == 0f64) {
        if (owner == "std::option::Option") && (arguments_ == 1f64) {
            String::from("option")
        } else if (owner == "std::result::Result") && (arguments_ == 2f64) {
            String::from("result")
        } else {
            String::from("unknown")
        }
    } else {
        String::from("unknown")
    }
}
