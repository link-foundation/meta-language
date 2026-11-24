// Generated from complete js/src/translation/rust-nominal-ownership-rules.js. Do not edit by hand.
use super::accept_rust_primitive_from_method;
use super::accept_rust_primitive_selection_receiver;
use super::ml_array;
use super::ml_math;
use super::read_rust_primitive_owner_path;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_from_owner(
    segments: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> String {
    if !ordinary || shadowed {
        String::new()
    } else if (((({
        let ml_reference_array16 = segments.clone();
        ml_reference_array16.borrow().len() as f64
    }) == 2f64)
        && !absolute)
        && {
            let source_ml_source1_comparison_operand25 = {
                let source_ml_source1_array_read_value23 = segments.clone();
                {
                    let source_ml_source1_array_read_index24 = 0f64;
                    {
                        let ml_reference_array17 = source_ml_source1_array_read_value23;
                        let ml_reference_position17 = source_ml_source1_array_read_index24;
                        self::ml_array::optional_at(
                            &*ml_reference_array17.borrow(),
                            self::ml_array::number_index(ml_reference_position17),
                        )
                    }
                }
            };
            {
                let source_ml_source1_comparison_operand26 = String::from("f64");
                match (
                    source_ml_source1_comparison_operand25,
                    source_ml_source1_comparison_operand26,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        || ((((({
            let ml_reference_array18 = segments.clone();
            ml_reference_array18.borrow().len() as f64
        }) == 4f64)
            && ({
                let source_ml_source1_comparison_operand29 = {
                    let source_ml_source1_array_read_value27 = segments.clone();
                    {
                        let source_ml_source1_array_read_index28 = 0f64;
                        {
                            let ml_reference_array19 = source_ml_source1_array_read_value27;
                            let ml_reference_position19 = source_ml_source1_array_read_index28;
                            self::ml_array::optional_at(
                                &*ml_reference_array19.borrow(),
                                self::ml_array::number_index(ml_reference_position19),
                            )
                        }
                    }
                };
                {
                    let source_ml_source1_comparison_operand30 = String::from("std");
                    match (
                        source_ml_source1_comparison_operand29,
                        source_ml_source1_comparison_operand30,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } || {
                let source_ml_source1_comparison_operand33 = {
                    let source_ml_source1_array_read_value31 = segments.clone();
                    {
                        let source_ml_source1_array_read_index32 = 0f64;
                        {
                            let ml_reference_array20 = source_ml_source1_array_read_value31;
                            let ml_reference_position20 = source_ml_source1_array_read_index32;
                            self::ml_array::optional_at(
                                &*ml_reference_array20.borrow(),
                                self::ml_array::number_index(ml_reference_position20),
                            )
                        }
                    }
                };
                {
                    let source_ml_source1_comparison_operand34 = String::from("core");
                    match (
                        source_ml_source1_comparison_operand33,
                        source_ml_source1_comparison_operand34,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            }))
            && {
                let source_ml_source1_comparison_operand37 = {
                    let source_ml_source1_array_read_value35 = segments.clone();
                    {
                        let source_ml_source1_array_read_index36 = 1f64;
                        {
                            let ml_reference_array21 = source_ml_source1_array_read_value35;
                            let ml_reference_position21 = source_ml_source1_array_read_index36;
                            self::ml_array::optional_at(
                                &*ml_reference_array21.borrow(),
                                self::ml_array::number_index(ml_reference_position21),
                            )
                        }
                    }
                };
                {
                    let source_ml_source1_comparison_operand38 = String::from("primitive");
                    match (
                        source_ml_source1_comparison_operand37,
                        source_ml_source1_comparison_operand38,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            })
            && {
                let source_ml_source1_comparison_operand41 = {
                    let source_ml_source1_array_read_value39 = segments.clone();
                    {
                        let source_ml_source1_array_read_index40 = 2f64;
                        {
                            let ml_reference_array22 = source_ml_source1_array_read_value39;
                            let ml_reference_position22 = source_ml_source1_array_read_index40;
                            self::ml_array::optional_at(
                                &*ml_reference_array22.borrow(),
                                self::ml_array::number_index(ml_reference_position22),
                            )
                        }
                    }
                };
                {
                    let source_ml_source1_comparison_operand42 = String::from("f64");
                    match (
                        source_ml_source1_comparison_operand41,
                        source_ml_source1_comparison_operand42,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            })
    {
        String::from("f64")
    } else {
        self::read_rust_primitive_owner_path(segments.clone(), ordinary, shadowed, absolute)
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
            &{
                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    String::from("i8"),
                    String::from("i16"),
                    String::from("i32"),
                    String::from("u8"),
                    String::from("u16"),
                    String::from("u32"),
                ])))
                .borrow()
                .clone()
            },
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
                    &{
                        (std::rc::Rc::new(std::cell::RefCell::new(vec![
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
                        ])))
                        .borrow()
                        .clone()
                    },
                    &target.to_owned(),
                    0f64,
                    false,
                    |a, b| a == b,
                ))
                || ((source == "u16")
                    && self::ml_array::includes(
                        &{
                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                String::from("u32"),
                                String::from("u64"),
                                String::from("u128"),
                                String::from("usize"),
                                String::from("i32"),
                                String::from("i64"),
                                String::from("i128"),
                            ])))
                            .borrow()
                            .clone()
                        },
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "u32")
                    && self::ml_array::includes(
                        &{
                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                String::from("u64"),
                                String::from("u128"),
                                String::from("i64"),
                                String::from("i128"),
                            ])))
                            .borrow()
                            .clone()
                        },
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "u64")
                    && self::ml_array::includes(
                        &{
                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                String::from("u128"),
                                String::from("i128"),
                            ])))
                            .borrow()
                            .clone()
                        },
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i8")
                    && self::ml_array::includes(
                        &{
                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                String::from("i16"),
                                String::from("i32"),
                                String::from("i64"),
                                String::from("i128"),
                                String::from("isize"),
                            ])))
                            .borrow()
                            .clone()
                        },
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i16")
                    && self::ml_array::includes(
                        &{
                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                String::from("i32"),
                                String::from("i64"),
                                String::from("i128"),
                                String::from("isize"),
                            ])))
                            .borrow()
                            .clone()
                        },
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i32")
                    && self::ml_array::includes(
                        &{
                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                String::from("i64"),
                                String::from("i128"),
                            ])))
                            .borrow()
                            .clone()
                        },
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
pub fn accept_rust_source_array_owner(owner: &str, shadowed: bool) -> bool {
    !shadowed && ((owner == "Vec") || (owner == "std::vec::Vec"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_element(kind: &str, indirected: bool) -> bool {
    !indirected
        && (((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "unit"))
            || (kind == "string"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_coercion(actual: &str, expected: &str, borrowed: bool) -> bool {
    (actual == expected)
        || ((borrowed && (actual == "owned vector")) && (expected == "immutable slice"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_result_model(model: &str) -> bool {
    ((model).is_empty()) || (model == "immutable fresh scalar vector results 1")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_result(model: &str, kind: &str, owned: bool, fresh: bool) -> bool {
    (((model == "immutable fresh scalar vector results 1") && owned) && fresh)
        && ((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "unit"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_fresh_array_result_tail(kind: &str, constructor: bool, children: bool) -> bool {
    ((kind == "array") && constructor) || (((kind == "let") || (kind == "if")) && children)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_retained_source_signature_model(model: &str) -> bool {
    model == "Rust source function signature 2"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_rust_retained_source_signature_model(model: &str, retain: bool) -> String {
    if !(model).is_empty() {
        model.to_owned()
    } else if retain {
        String::from("Rust source function signature 2")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_retained_function_owner(
    model: &str,
    owner: &str,
    type_: &str,
    paths: f64,
    children: f64,
    parameters: f64,
    reference: bool,
) -> bool {
    (((((((self::accept_rust_retained_source_signature_model(model) && (owner == "function"))
        && (type_ == "function"))
        && (paths == 0f64))
        && self::ml_math::is_integer(parameters))
        && (parameters >= 0f64))
        && (parameters <= 100_000f64))
        && (children == (parameters + 1f64)))
        && !reference
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_nominal_clone_inherent_override(
    owner: &str,
    selected_owner: &str,
    mode: f64,
) -> bool {
    ((!(owner).is_empty()) && (owner == selected_owner)) && (mode > 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_nominal_ownership_leaf(
    kind: &str,
    bits: &str,
    signed: bool,
    wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    proven: bool,
    copy: bool,
    owner: &str,
    path: &str,
    owner_paths: f64,
    arguments_: f64,
    model: &str,
    pointer_header: bool,
    parameters: f64,
    scope: bool,
    closed: bool,
    element: &str,
) -> String {
    if !scope || !closed {
        String::from("unknown")
    } else {
        {
            let ml_library_condition_1 = {
                let ml_value = {
                    let ml_reference_array32 = wrappers.clone();
                    ml_reference_array32.borrow().len() as f64
                };
                ml_value != 0f64 && !ml_value.is_nan()
            };
            if ml_library_condition_1 {
                if {
                    let ml_values33 = { wrappers.borrow().clone() };
                    let ml_callback33 =
                        std::rc::Rc::new(move |layer: String| -> bool { layer == "reference" });
                    ml_values33
                        .iter()
                        .cloned()
                        .enumerate()
                        .all(|(ml_index33, ml_value33)| ml_callback33(ml_value33))
                } {
                    String::from("copy")
                } else {
                    String::from("unknown")
                }
            } else if (((proven && copy)
                && self::accept_rust_primitive_from_source(
                    kind, bits, signed, owner, path, arguments_, true, 0f64,
                ))
                || ((((((kind == "unit") && proven) && copy) && (owner == "unit"))
                    && ((path).is_empty()))
                    && (arguments_ == 0f64)))
                || (((kind == "function") && ((proven && copy) || pointer_header))
                    && self::accept_rust_retained_function_owner(
                        "Rust source function signature 2",
                        owner,
                        kind,
                        owner_paths,
                        arguments_,
                        parameters,
                        false,
                    ))
            {
                String::from("copy")
            } else if (((((kind == "string") && proven) && (owner == "library"))
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
                String::from("string")
            } else if ((((((kind == "array") && proven) && (owner == "library"))
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
                && self::accept_rust_source_array_element(element, false)
            {
                String::from("vector")
            } else if (kind == "tuple") && (owner == "tuple") {
                String::from("tuple")
            } else if (kind == "named") && (owner == "declaration") {
                String::from("named")
            } else {
                String::from("unknown")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_nominal_ownership_sum_node(
    owner: &str,
    arguments_: f64,
    provisional: bool,
    proven: bool,
    scope: bool,
) -> String {
    if scope && (proven || provisional) {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_nominal_ownership_declaration_owner(
    kind: &str,
    path: &str,
    arguments_: f64,
    declared: &str,
) -> bool {
    (((kind == "declaration") && (!(declared).is_empty())) && (path == declared))
        && (arguments_ == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_nominal_ownership_graph_error(nodes: f64, edges: f64, depth: f64) -> String {
    if (((((((!(self::ml_math::is_integer(nodes)) || !(self::ml_math::is_integer(edges)))
        || !(self::ml_math::is_integer(depth)))
        || (nodes < 0f64))
        || (edges < 0f64))
        || (depth < 0f64))
        || (nodes > 65_536f64))
        || (edges > 131_072f64))
        || (depth > 256f64)
    {
        String::from("the actual nominal ownership field graph exceeds its finite source profile")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_nominal_ownership_node_role(
    kind: &str,
    roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
) -> String {
    if kind == "copy" {
        String::from("copy")
    } else if (kind == "string") || (kind == "standard collection") {
        String::from("owned")
    } else if kind == "vector" {
        if (({
            let ml_reference_array37 = roles.clone();
            ml_reference_array37.borrow().len() as f64
        }) == 1f64)
            && ({
                let source_ml_source1_comparison_operand45 = {
                    let source_ml_source1_array_read_value43 = roles.clone();
                    {
                        let source_ml_source1_array_read_index44 = 0f64;
                        {
                            let ml_reference_array38 = source_ml_source1_array_read_value43;
                            let ml_reference_position38 = source_ml_source1_array_read_index44;
                            self::ml_array::optional_at(
                                &*ml_reference_array38.borrow(),
                                self::ml_array::number_index(ml_reference_position38),
                            )
                        }
                    }
                };
                {
                    let source_ml_source1_comparison_operand46 = String::from("copy");
                    match (
                        source_ml_source1_comparison_operand45,
                        source_ml_source1_comparison_operand46,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } || {
                let source_ml_source1_comparison_operand49 = {
                    let source_ml_source1_array_read_value47 = roles.clone();
                    {
                        let source_ml_source1_array_read_index48 = 0f64;
                        {
                            let ml_reference_array39 = source_ml_source1_array_read_value47;
                            let ml_reference_position39 = source_ml_source1_array_read_index48;
                            self::ml_array::optional_at(
                                &*ml_reference_array39.borrow(),
                                self::ml_array::number_index(ml_reference_position39),
                            )
                        }
                    }
                };
                {
                    let source_ml_source1_comparison_operand50 = String::from("owned");
                    match (
                        source_ml_source1_comparison_operand49,
                        source_ml_source1_comparison_operand50,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            })
        {
            String::from("owned")
        } else {
            String::new()
        }
    } else if ((kind == "option")
        && (({
            let ml_reference_array40 = roles.clone();
            ml_reference_array40.borrow().len() as f64
        }) != 1f64))
        || ((kind == "result")
            && (({
                let ml_reference_array41 = roles.clone();
                ml_reference_array41.borrow().len() as f64
            }) != 2f64))
    {
        String::new()
    } else {
        if !(self::ml_array::includes(
            &{
                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    String::from("constructor"),
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
        )) || {
            let ml_values43 = { roles.borrow().clone() };
            let ml_callback43 = std::rc::Rc::new(move |role: String| -> bool {
                (role != "copy") && (role != "owned")
            });
            ml_values43
                .iter()
                .cloned()
                .enumerate()
                .any(|(ml_index43, ml_value43)| ml_callback43(ml_value43))
        } {
            String::new()
        } else {
            {
                let owned = self::ml_array::includes(
                    &{ roles.borrow().clone() },
                    &String::from("owned"),
                    0f64,
                    false,
                    |a, b| a == b,
                );
                if (kind == "tuple") && owned {
                    String::new()
                } else if owned {
                    String::from("owned")
                } else {
                    String::from("copy")
                }
            }
        }
    }
}
