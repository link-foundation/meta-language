// Generated from complete js/src/translation/rust-owned-recursive-datatype-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Result;
use super::accept_rust_owned_box_type_node;
use super::accept_rust_primitive_from_source;
use super::ml_array;
use super::ml_math;
use super::read_required_text_fact;
use super::source_ml_temporary_accept_rust_owned_box_layer_clone_loop71;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_container_node_role(
    kind: &str,
    children: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    nominal: &str,
) -> String {
    if (kind == "copy") || (kind == "reference") {
        String::from("copy")
    } else if kind == "string" {
        String::from("owned")
    } else if kind == "named" {
        if (nominal == "copy") || (nominal == "owned") {
            nominal.to_owned()
        } else {
            String::new()
        }
    } else if {
        let ml_values103 = { children.borrow().clone() };
        let ml_callback103 = std::rc::Rc::new(move |value: String| -> bool {
            (value != "copy") && (value != "owned")
        });
        ml_values103
            .iter()
            .cloned()
            .enumerate()
            .any(|(ml_index103, ml_value103)| ml_callback103(ml_value103))
    } {
        String::new()
    } else if (kind == "box") || (kind == "vector") {
        {
            let ml_product_condition_1 = {
                let ml_reference_array105 = children.clone();
                ml_reference_array105.borrow().len() as f64
            };
            if ml_product_condition_1 {
                String::from("owned")
            } else {
                String::new()
            }
        }
    } else if kind == "tuple" {
        if {
            let ml_values106 = { children.borrow().clone() };
            let ml_callback106 = std::rc::Rc::new(move |value: String| -> bool { value == "copy" });
            ml_values106
                .iter()
                .cloned()
                .enumerate()
                .all(|(ml_index106, ml_value106)| ml_callback106(ml_value106))
        } {
            String::from("copy")
        } else {
            String::new()
        }
    } else if ((kind == "option")
        && (({
            let ml_reference_array108 = children.clone();
            ml_reference_array108.borrow().len() as f64
        }) == 1f64))
        || ((kind == "result")
            && (({
                let ml_reference_array109 = children.clone();
                ml_reference_array109.borrow().len() as f64
            }) == 2f64))
    {
        if self::ml_array::includes(
            &{ children.borrow().clone() },
            &String::from("owned"),
            0f64,
            false,
            |a, b| a == b,
        ) {
            String::from("owned")
        } else {
            String::from("copy")
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_container_clone(
    kind: &str,
    children: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    nominal: bool,
) -> bool {
    ((((kind == "copy") || (kind == "reference")) || (kind == "string"))
        || ((kind == "named") && nominal))
        || (self::ml_array::includes(
            &{
                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    String::from("box"),
                    String::from("vector"),
                    String::from("tuple"),
                    String::from("option"),
                    String::from("result"),
                ])))
                .borrow()
                .clone()
            },
            &kind.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ) && {
            let ml_values112 = { children.borrow().clone() };
            let ml_callback112 = std::rc::Rc::new(move |value: bool| -> bool { value });
            ml_values112
                .iter()
                .cloned()
                .enumerate()
                .all(|(ml_index112, ml_value112)| ml_callback112(ml_value112))
        })
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_box_canonical_owner(owner: &str) -> String {
    if (owner == "Box") || (owner == "std::boxed::Box") {
        String::from("std::boxed::Box")
    } else if owner == "alloc::boxed::Box" {
        String::from("alloc::boxed::Box")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_box_constructor_error(
    owner: &str,
    payload: &str,
    arguments_: f64,
) -> String {
    if (self::ml_array::includes(
        &{
            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                String::from("std::boxed::Box"),
                String::from("alloc::boxed::Box"),
            ])))
            .borrow()
            .clone()
        },
        &owner.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) && ((payload == "copy") || (payload == "owned")))
        && (arguments_ == 1f64)
    {
        String::new()
    } else {
        String::from(
            "Box construction needs its authentic standard owner and one actual admitted payload",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_box_move_error(tracking: bool, available: bool, produced: bool) -> String {
    if tracking && (available || produced) {
        String::new()
    } else {
        String::from(
            "moving a Box needs its actual available binding or completed allocation producer",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_nominal_vector_element(role: &str, closed: bool) -> bool {
    (role == "owned") && closed
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_container_source(ordinary: bool, minor: f64) -> bool {
    ((ordinary && self::ml_math::is_integer(minor)) && (minor >= 90f64)) && (minor < 100f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_owned_container_leaf_role(
    kind: &str,
    bits: &str,
    signed: bool,
    proven: bool,
    owner: &str,
    path: &str,
    arguments_: f64,
    model: &str,
) -> String {
    if proven {
        if self::accept_rust_primitive_from_source(
            kind, bits, signed, owner, path, arguments_, proven, 0f64,
        ) || ((((kind == "unit") && (owner == "unit")) && ((path).is_empty()))
            && (arguments_ == 0f64))
        {
            String::from("copy")
        } else if ((((kind == "string") && (owner == "library"))
            && self::ml_array::includes(
                &{
                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        String::from("std::string::String"),
                        String::from("alloc::string::String"),
                    ])))
                    .borrow()
                    .clone()
                },
                &path.to_owned(),
                0f64,
                false,
                |a, b| a == b,
            ))
            && (arguments_ == 0f64))
            && (model == "owned string")
        {
            String::from("owned")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_box_intrinsic_copy_value(kind: &str, actual_copy: bool) -> bool {
    self::ml_array::includes(
        &{
            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                String::from("num"),
                String::from("bool"),
                String::from("unit"),
                String::from("str"),
            ])))
            .borrow()
            .clone()
        },
        &kind.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) || ((kind == "tuple") && actual_copy)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_box_value_layers(
    wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
) -> bool {
    (((((({
        let ml_reference_array117 = wrappers.clone();
        ml_reference_array117.borrow().len() as f64
    }) > 0f64)
        && ((self::read_required_text_fact(wrappers.clone(), 0f64)) == "box"))
        && (({
            let ml_reference_array118 = owners.clone();
            ml_reference_array118.borrow().len() as f64
        }) > 0f64))
        && (({
            let ml_reference_array119 = owners.clone();
            ml_reference_array119.borrow().len() as f64
        }) <= 256f64))
        && (({
            let ml_reference_array120 = owners.clone();
            ml_reference_array120.borrow().len() as f64
        }) <= ({
            let ml_reference_array121 = wrappers.clone();
            ml_reference_array121.borrow().len() as f64
        })))
        && {
            let ml_values122 = { owners.borrow().clone() };
            let ml_callback122 = std::rc::Rc::new(move |owner: String| -> bool {
                self::ml_array::includes(
                    &{
                        (std::rc::Rc::new(std::cell::RefCell::new(vec![
                            String::from("std::boxed::Box"),
                            String::from("alloc::boxed::Box"),
                        ])))
                        .borrow()
                        .clone()
                    },
                    &owner.clone(),
                    0f64,
                    false,
                    |a, b| a == b,
                )
            });
            ml_values122
                .iter()
                .cloned()
                .enumerate()
                .all(|(ml_index122, ml_value122)| ml_callback122(ml_value122))
        }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_container_child_model(
    kind: &str,
    owner: &str,
    path: &str,
    arguments_: f64,
) -> String {
    if (((owner == "library") && (kind == "string"))
        && self::ml_array::includes(
            &{
                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    String::from("std::string::String"),
                    String::from("alloc::string::String"),
                ])))
                .borrow()
                .clone()
            },
            &path.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ))
        && (arguments_ == 0f64)
    {
        String::from("owned string")
    } else if (((owner == "library") && (kind == "array"))
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
        && (arguments_ == 1f64)
    {
        String::from("owned vector")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_container_runtime_metadata_cost(
    role: &str,
    units: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if ((!(self::ml_math::is_integer(units)) || (units < 0f64)) || (units > 8_388_608f64))
        || !(self::ml_array::includes(
            &{
                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    String::from("origin"),
                    String::from("list"),
                    String::from("binding"),
                    String::from("layer"),
                    String::from("owner"),
                    String::from("intrinsic copy"),
                    String::from("payload loans"),
                    String::from("payload field loans"),
                    String::from("value loans"),
                    String::from("value field loans"),
                    String::from("place loans"),
                    String::from("loans"),
                    String::from("temporary loans"),
                ])))
                .borrow()
                .clone()
            },
            &role.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ))
    {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
    } else if role == "origin" {
        std::rc::Rc::new(std::cell::RefCell::new(vec![16f64, (256f64 + units)]))
    } else {
        std::rc::Rc::new(std::cell::RefCell::new(vec![
            1f64,
            ((role.encode_utf16().count() as f64) + units),
        ]))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_box_clone_error(
    actual: bool,
    clone: bool,
    readonly: bool,
    closed: bool,
    available: bool,
    produced: bool,
) -> String {
    if !actual || !clone {
        String::from("Box Clone needs its actual admitted payload Clone implementation")
    } else if readonly && !closed {
        String::from("a borrowed Box Clone needs its closed payload lifetime projection")
    } else if (readonly || available) || produced {
        String::new()
    } else {
        String::from("Box Clone needs its actual available source value or actual producer")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_box_layer_clone(
    wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    copied: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
) -> bool {
    if !(self::accept_rust_owned_box_value_layers(wrappers.clone(), owners.clone()))
        || (({
            let ml_reference_array128 = owners.clone();
            ml_reference_array128.borrow().len() as f64
        }) != ({
            let ml_reference_array129 = copied.clone();
            ml_reference_array129.borrow().len() as f64
        }))
    {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source1_s64 =
                    self::source_ml_temporary_accept_rust_owned_box_layer_clone_loop71(
                        wrappers.clone(),
                        owners.clone(),
                        copied.clone(),
                        at,
                    );
                match source_ml_source1_s64 {
                    self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Done => {
                        false
                    }
                    self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Return(source_ml_temporary_result71) => {
                        source_ml_temporary_result71
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_standard_sum_box_payload(
    wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    lifetimes: f64,
    owner: &str,
    path: &str,
    arguments_: f64,
    boxes: f64,
) -> bool {
    (((self::accept_rust_owned_box_type_node(wrappers.clone(), owner, path, arguments_) && {
        let ml_values130 = { wrappers.borrow().clone() };
        let ml_callback130 = std::rc::Rc::new(move |layer: String| -> bool { layer == "box" });
        ml_values130
            .iter()
            .cloned()
            .enumerate()
            .all(|(ml_index130, ml_value130)| ml_callback130(ml_value130))
    }) && (lifetimes == 0f64))
        && self::ml_math::is_integer(boxes))
        && (boxes
            == ({
                let ml_reference_array132 = wrappers.clone();
                ml_reference_array132.borrow().len() as f64
            }))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_container_value_loan_role(role: &str) -> bool {
    self::ml_array::includes(
        &{
            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                String::from("value loans"),
                String::from("value field loans"),
                String::from("payload loans"),
                String::from("payload field loans"),
            ])))
            .borrow()
            .clone()
        },
        &role.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_box_payload_error(
    closed: bool,
    intrinsic: bool,
    borrowed: bool,
    produced: bool,
) -> String {
    if closed || ((intrinsic || produced) && !borrowed) {
        String::new()
    } else {
        String::from(
            "stored Box payload references or callable captures need their complete lifetime/drop profile",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_container_depth_error(wrappers: f64, layers: f64) -> String {
    if ((((!(self::ml_math::is_integer(wrappers)) || !(self::ml_math::is_integer(layers)))
        || (wrappers < 0f64))
        || (layers < 0f64))
        || (wrappers > 256f64))
        || (layers > 256f64)
    {
        String::from(
            "owned container source syntax and value lineage exceed their finite depth profile",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_container_closed(
    kind: &str,
    children: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    nominal: bool,
) -> bool {
    (((kind == "copy") || (kind == "string")) || ((kind == "named") && nominal))
        || (self::ml_array::includes(
            &{
                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    String::from("box"),
                    String::from("vector"),
                    String::from("tuple"),
                    String::from("option"),
                    String::from("result"),
                ])))
                .borrow()
                .clone()
            },
            &kind.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ) && {
            let ml_values135 = { children.borrow().clone() };
            let ml_callback135 = std::rc::Rc::new(move |value: bool| -> bool { value });
            ml_values135
                .iter()
                .cloned()
                .enumerate()
                .all(|(ml_index135, ml_value135)| ml_callback135(ml_value135))
        })
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_box_type_error(
    selected: bool,
    header: bool,
    complete: bool,
    role: &str,
    closed: bool,
) -> String {
    if ((!selected || header) || !complete) || ((role == "owned") && closed) {
        String::new()
    } else {
        String::from("an owned Box source type needs its complete closed payload ownership graph")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_owned_box_call_result(
    actual: bool,
    function_: bool,
    owner: bool,
    type_: bool,
    recorded: bool,
) -> bool {
    (((actual && function_) && owner) && type_) && recorded
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_rust_owned_nominal_runtime_facts(
    selected: bool,
    old_role: &str,
    old_clone: bool,
    role: &str,
    clone: bool,
    closed: bool,
    rank: f64,
    constructor: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    if (old_role == "copy") || (old_role == "owned") {
        std::rc::Rc::new(std::cell::RefCell::new(vec![
            old_role.to_owned(),
            if old_clone {
                String::from("Clone")
            } else {
                String::new()
            },
        ]))
    } else if (((((((!selected || (role != "owned")) || !closed)
        || !(self::ml_math::is_integer(rank)))
        || (rank < 1f64))
        || (rank > 65_536f64))
        || !(self::ml_math::is_integer(constructor)))
        || (constructor < 0f64))
        || (constructor > 65_536f64)
    {
        std::rc::Rc::new(std::cell::RefCell::new(vec![String::new(), String::new()]))
    } else {
        std::rc::Rc::new(std::cell::RefCell::new(vec![
            String::from("owned"),
            if clone {
                String::from("Clone")
            } else {
                String::new()
            },
        ]))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_owned_vector_signature_phase(
    selected: bool,
    header: bool,
    complete: bool,
    kind: &str,
    wrappers: f64,
    owner: &str,
    path: &str,
    arguments_: f64,
    element: &str,
    profile: &str,
    closed: bool,
) -> String {
    if (((((!selected || (kind != "array")) || (wrappers != 0f64)) || (owner != "library"))
        || !(self::ml_array::includes(
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
        )))
        || (arguments_ != 1f64))
        || (element != "named")
    {
        String::new()
    } else if header && !complete {
        String::from("defer")
    } else if (complete && (profile == "owned")) && closed {
        String::from("admit")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_vector_element_production_error(
    role: &str,
    same: bool,
    transferred: bool,
    available: bool,
    produced: bool,
    collecting: bool,
) -> String {
    if !(self::ml_array::includes(
        &{
            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                String::from("copy"),
                String::from("owned"),
            ])))
            .borrow()
            .clone()
        },
        &role.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )) || !same
    {
        String::from("owned vector construction needs each actual complete matching element type")
    } else if (((collecting || (role == "copy")) || transferred) || available) || produced {
        String::new()
    } else {
        String::from(
            "an owned vector element needs its actual available binding or completed producer",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_owned_vector_call_result(
    actual: bool,
    function_: bool,
    owner: bool,
    type_: bool,
    recorded: bool,
) -> bool {
    (((actual && function_) && owner) && type_) && recorded
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_vector_named_element(role: &str, closed: bool, wrappers: f64) -> bool {
    ((wrappers == 0f64) && closed) && ((role == "copy") || (role == "owned"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_vector_element_count_error(count: f64) -> String {
    if (!(self::ml_math::is_safe_integer(count)) || (count < 0f64)) || (count > 65_536f64) {
        String::from("owned vector construction exceeds its finite actual element profile")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_vector_named_admission(previous: &str, actual: bool) -> String {
    if (previous == "refuse") && actual {
        String::from("admit")
    } else {
        previous.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_vector_produced_value(
    node: &str,
    role: &str,
    closed: bool,
    produced: bool,
) -> bool {
    (((node == "vector") && (role == "owned")) && closed) && produced
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_vector_function_result(
    phase: &str,
    actual: bool,
    owner: bool,
    type_: bool,
    value: bool,
    borrowed: bool,
    collecting: bool,
) -> bool {
    (((((phase == "admit") && actual) && owner) && type_) && (collecting || value)) && !borrowed
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_vector_empty_constructor_disposition(
    phase: &str,
    owner: &str,
    method: &str,
    arguments_: f64,
) -> String {
    if ((phase != "admit")
        || !(self::ml_array::includes(
            &{
                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    String::from("Vec"),
                    String::from("std::vec::Vec"),
                    String::from("alloc::vec::Vec"),
                ])))
                .borrow()
                .clone()
            },
            &owner.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )))
        || (method != "new")
    {
        String::new()
    } else {
        let ml_library_inequality_1 = arguments_ != 0f64;
        if ml_library_inequality_1 {
            String::from("arity")
        } else {
            String::from("construct")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_box_clone_source_error(
    collecting: bool,
    actual: bool,
    clone: bool,
    readonly: bool,
    closed: bool,
    available: bool,
    produced: bool,
) -> String {
    {
        let error = self::read_rust_owned_box_clone_error(
            actual, clone, readonly, closed, available, produced,
        );
        if ((collecting && actual) && clone) && (!readonly || closed) {
            String::new()
        } else {
            error
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_vector_clone_error(
    collecting: bool,
    actual: bool,
    clone: bool,
    closed: bool,
    readonly: bool,
    available: bool,
    produced: bool,
) -> String {
    if (!actual || !clone) || !closed {
        String::from("vector Clone needs its actual closed element Clone implementation")
    } else if ((collecting || readonly) || available) || produced {
        String::new()
    } else {
        String::from("vector Clone needs its actual available source value or actual producer")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_argument_read_check(wrappers: f64, transferred: bool) -> bool {
    (wrappers > 0f64) && !transferred
}
