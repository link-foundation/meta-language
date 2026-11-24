// Generated from complete js/src/translation/callable-data-derivation-rules.js. Do not edit by hand.
use super::accept_callable_data_constructive_node;
use super::ml_array;
use super::ml_number;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_temporary_render_callable_data_lean_default_loop31;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_data_lean_default(name: &str, constructor: &str, fields: f64) -> String {
    {
        let value = constructor.to_owned();
        {
            let at = 0f64;
            {
                let value_2 = self::source_ml_temporary_render_callable_data_lean_default_loop31(
                    fields, value, at,
                );
                format!("instance : Inhabited {} := ⟨{value_2}⟩", name.to_owned())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_formal_datatype_lean_block_derivation(
    callable: bool,
    output: bool,
    undecidable: bool,
) -> String {
    if callable || output {
        String::new()
    } else if undecidable {
        String::from("Repr")
    } else {
        String::from("Repr, DecidableEq")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_datatype_rocq_mutual_block(
    definitions: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
) -> String {
    format!(
        "#[warnings=\"-register-all\"]\n{}",
        ({ definitions.borrow().clone() }).join((String::from("\n")).as_str())
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop4Result {
    MlTemporaryReadDatatypeConstructivePlanLoop4Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_datatype_constructive_plan_loop4(
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut at: f64,
) -> self::MlTemporaryReadDatatypeConstructivePlanLoop4Result {
    loop {
        if at
            < ({
                let ml_reference_array39 = roots.clone();
                ml_reference_array39.borrow().len() as f64
            })
        {
            {
                let source_ml_temporary_read_datatype_constructive_plan_ignored5 = {
                    let ml_method_owner40 = result.clone();
                    let ml_method_argument40_0 = {
                        let source_ml_source1_string_argument12 =
                            self::read_required_number_fact(witnesses.clone(), at);
                        self::ml_number::js_number(source_ml_source1_string_argument12)
                    };
                    let ml_inserted = vec![ml_method_argument40_0];
                    let mut ml_method_values40 = ml_method_owner40.borrow_mut();
                    let ml_length = ml_method_values40
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values40.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let source_ml_temporary_read_datatype_constructive_plan_ignored6 = {
                        let ml_method_owner41 = result.clone();
                        let ml_method_argument41_0 = {
                            let source_ml_source1_string_argument13 =
                                self::read_required_number_fact(ranks.clone(), at);
                            self::ml_number::js_number(source_ml_source1_string_argument13)
                        };
                        let ml_inserted = vec![ml_method_argument41_0];
                        let mut ml_method_values41 = ml_method_owner41.borrow_mut();
                        let ml_length = ml_method_values41
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values41.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryReadDatatypeConstructivePlanLoop4Result::MlTemporaryReadDatatypeConstructivePlanLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop3Result {
    MlTemporaryReadDatatypeConstructivePlanLoop3Done,
    MlTemporaryReadDatatypeConstructivePlanLoop3Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_datatype_constructive_plan_loop9(
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut changed: bool,
    mut rank: f64,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut declaration: f64,
    mut at: f64,
    mut start: f64,
    mut edge: f64,
) -> bool {
    loop {
        return if edge < (self::read_required_number_fact(ends.clone(), at)) {
            if self::read_required_boolean_fact(
                values.clone(),
                self::read_required_number_fact(edges.clone(), edge),
            ) {
                {
                    let source_ml_temporary_read_datatype_constructive_plan_ignored10 = {
                        let ml_array_owner42 = known.clone();
                        let ml_array_argument42_0: f64 = declaration;
                        let ml_array_argument42_1: f64 = 1f64;
                        let ml_array_argument42_2: bool = true;
                        let ml_array_length = ml_array_owner42.borrow().len();
                        let ml_array_start = {
                            let ml_relative_value: f64 = ml_array_argument42_0;
                            let ml_relative = if ml_relative_value.is_nan() {
                                0f64
                            } else {
                                ml_relative_value.trunc()
                            };
                            let ml_bound = ml_array_length as f64;
                            (if ml_relative < 0f64 {
                                (ml_bound + ml_relative).clamp(0f64, ml_bound)
                            } else {
                                ml_relative.min(ml_bound)
                            }) as usize
                        };
                        let ml_array_count = {
                            let ml_delete_value: f64 = ml_array_argument42_1;
                            ml_delete_value
                                .trunc()
                                .max(0f64)
                                .min((ml_array_length - ml_array_start) as f64)
                                as usize
                        };
                        let ml_array_inserted = vec![ml_array_argument42_2];
                        let _ = (ml_array_length - ml_array_count)
                            .checked_add(ml_array_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        let ml_removed: Vec<_> = ml_array_owner42
                            .borrow_mut()
                            .splice(
                                ml_array_start..ml_array_start + ml_array_count,
                                ml_array_inserted,
                            )
                            .collect();
                        std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                    };
                    {
                        let source_ml_temporary_read_datatype_constructive_plan_ignored11 = {
                            let ml_array_owner43 = witnesses.clone();
                            let ml_array_argument43_0: f64 = declaration;
                            let ml_array_argument43_1: f64 = 1f64;
                            let ml_array_argument43_2: f64 = edge - start;
                            let ml_array_length = ml_array_owner43.borrow().len();
                            let ml_array_start = {
                                let ml_relative_value: f64 = ml_array_argument43_0;
                                let ml_relative = if ml_relative_value.is_nan() {
                                    0f64
                                } else {
                                    ml_relative_value.trunc()
                                };
                                let ml_bound = ml_array_length as f64;
                                (if ml_relative < 0f64 {
                                    (ml_bound + ml_relative).clamp(0f64, ml_bound)
                                } else {
                                    ml_relative.min(ml_bound)
                                }) as usize
                            };
                            let ml_array_count = {
                                let ml_delete_value: f64 = ml_array_argument43_1;
                                ml_delete_value
                                    .trunc()
                                    .max(0f64)
                                    .min((ml_array_length - ml_array_start) as f64)
                                    as usize
                            };
                            let ml_array_inserted = vec![ml_array_argument43_2];
                            let _ = (ml_array_length - ml_array_count)
                                .checked_add(ml_array_inserted.len())
                                .filter(|length| *length <= 4_294_967_295usize)
                                .expect("Invalid array length");
                            let ml_removed: Vec<_> = ml_array_owner43
                                .borrow_mut()
                                .splice(
                                    ml_array_start..ml_array_start + ml_array_count,
                                    ml_array_inserted,
                                )
                                .collect();
                            std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                        };
                        {
                            let source_ml_temporary_read_datatype_constructive_plan_ignored12 = {
                                let ml_array_owner44 = ranks.clone();
                                let ml_array_argument44_0: f64 = declaration;
                                let ml_array_argument44_1: f64 = 1f64;
                                let ml_array_argument44_2: f64 = rank;
                                let ml_array_length = ml_array_owner44.borrow().len();
                                let ml_array_start = {
                                    let ml_relative_value: f64 = ml_array_argument44_0;
                                    let ml_relative = if ml_relative_value.is_nan() {
                                        0f64
                                    } else {
                                        ml_relative_value.trunc()
                                    };
                                    let ml_bound = ml_array_length as f64;
                                    (if ml_relative < 0f64 {
                                        (ml_bound + ml_relative).clamp(0f64, ml_bound)
                                    } else {
                                        ml_relative.min(ml_bound)
                                    }) as usize
                                };
                                let ml_array_count = {
                                    let ml_delete_value: f64 = ml_array_argument44_1;
                                    ml_delete_value
                                        .trunc()
                                        .max(0f64)
                                        .min((ml_array_length - ml_array_start) as f64)
                                        as usize
                                };
                                let ml_array_inserted = vec![ml_array_argument44_2];
                                let _ = (ml_array_length - ml_array_count)
                                    .checked_add(ml_array_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                let ml_removed: Vec<_> = ml_array_owner44
                                    .borrow_mut()
                                    .splice(
                                        ml_array_start..ml_array_start + ml_array_count,
                                        ml_array_inserted,
                                    )
                                    .collect();
                                std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                            };
                            { true }
                        }
                    }
                }
            } else {
                {
                    let edge_2 = edge + 1f64;
                    {
                        edge = edge_2;
                        continue;
                    }
                }
            }
        } else {
            changed
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
pub fn source_ml_temporary_read_datatype_constructive_plan_loop8(
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut changed: bool,
    mut rank: f64,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut declaration: f64,
) -> bool {
    loop {
        if declaration
            < ({
                let ml_reference_array45 = roots.clone();
                ml_reference_array45.borrow().len() as f64
            })
        {
            if !(self::read_required_boolean_fact(known.clone(), declaration)) {
                {
                    let at = self::read_required_number_fact(roots.clone(), declaration);
                    {
                        let start = if at == 0f64 {
                            0f64
                        } else {
                            self::read_required_number_fact(ends.clone(), (at - 1f64))
                        };
                        {
                            let edge = start;
                            {
                                let changed_2 =
                                    self::source_ml_temporary_read_datatype_constructive_plan_loop9(
                                        ends.clone(),
                                        edges.clone(),
                                        known.clone(),
                                        witnesses.clone(),
                                        ranks.clone(),
                                        changed,
                                        rank,
                                        values.clone(),
                                        declaration,
                                        at,
                                        start,
                                        edge,
                                    );
                                {
                                    let declaration_2 = declaration + 1f64;
                                    {
                                        (changed, declaration) = (changed_2, declaration_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            {
                {
                    let declaration_3 = declaration + 1f64;
                    {
                        declaration = declaration_3;
                        continue;
                    }
                }
            }
        }
        return { changed };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop7Result {
    MlTemporaryReadDatatypeConstructivePlanLoop7Done(f64),
    MlTemporaryReadDatatypeConstructivePlanLoop7Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop13Result {
    MlTemporaryReadDatatypeConstructivePlanLoop13Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_datatype_constructive_plan_loop13(
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
    mut children: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut edge: f64,
) -> self::MlTemporaryReadDatatypeConstructivePlanLoop13Result {
    loop {
        if edge < (self::read_required_number_fact(ends.clone(), at)) {
            {
                let source_ml_temporary_read_datatype_constructive_plan_ignored15 = {
                    let ml_method_owner46 = children.clone();
                    let ml_method_argument46_0 = self::read_required_boolean_fact(
                        values.clone(),
                        self::read_required_number_fact(edges.clone(), edge),
                    );
                    let ml_inserted = vec![ml_method_argument46_0];
                    let mut ml_method_values46 = ml_method_owner46.borrow_mut();
                    let ml_length = ml_method_values46
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values46.extend(ml_inserted);
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
        return {
            self::MlTemporaryReadDatatypeConstructivePlanLoop13Result::MlTemporaryReadDatatypeConstructivePlanLoop13Done
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
pub fn source_ml_temporary_read_datatype_constructive_plan_loop7(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut references: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut count: f64,
    mut work: f64,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
) -> self::MlTemporaryReadDatatypeConstructivePlanLoop7Result {
    loop {
        return if at < count {
            {
                let work_2 = work + 1f64;
                if work_2 > 1_048_576f64 {
                    self::MlTemporaryReadDatatypeConstructivePlanLoop7Result::MlTemporaryReadDatatypeConstructivePlanLoop7Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("callable data graph reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let start = if at == 0f64 {
                            0f64
                        } else {
                            self::read_required_number_fact(ends.clone(), (at - 1f64))
                        };
                        {
                            let children =
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                            {
                                let edge = start;
                                {
                                    let source_ml_source1_s14 = self::source_ml_temporary_read_datatype_constructive_plan_loop13(ends.clone(), edges.clone(), values.clone(), at, children.clone(), edge);
                                    match source_ml_source1_s14 {
                                        self::MlTemporaryReadDatatypeConstructivePlanLoop13Result::MlTemporaryReadDatatypeConstructivePlanLoop13Done => {
                                            {
                                                let reference = self::read_required_number_fact(references.clone(), at);
                                                {
                                                    let kind = self::read_required_text_fact(kinds.clone(), at);
                                                    {
                                                        let source_ml_temporary_read_datatype_constructive_plan_ignored14 = { let ml_method_owner47 = values.clone(); let ml_method_argument47_0 = if reference >= 0f64 {
                                                            self::read_required_boolean_fact(known.clone(), reference)
                                                        } else if kind == "data" {
                                                                { let ml_values48 = { children.borrow().clone() }; let ml_callback48 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values48.iter().cloned().enumerate().any(|(ml_index48, ml_value48)| ml_callback48(ml_value48)) }
                                                            } else if kind == "function" {
                                                                    (({ let ml_reference_array50 = children.clone(); ml_reference_array50.borrow().len() as f64 }) > 0f64) && self::read_required_boolean_fact(children.clone(), ({ let ml_reference_array51 = children.clone(); ml_reference_array51.borrow().len() as f64 } - 1f64))
                                                                } else if self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("array"), String::from("option"), String::from("set"), String::from("map")]))).borrow().clone() }, &kind.clone(), 0f64, false, |a, b| a == b) {
                                                                        true
                                                                    } else {
                                                                        self::accept_callable_data_constructive_node(&(kind), children.clone())
                                                                    }; let ml_inserted = vec![ml_method_argument47_0]; let mut ml_method_values47 = ml_method_owner47.borrow_mut(); let ml_length = ml_method_values47.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values47.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let at_2 = at + 1f64;
                                                            {
                                                                (work, at) = (work_2, at_2);
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
        } else {
            self::MlTemporaryReadDatatypeConstructivePlanLoop7Result::MlTemporaryReadDatatypeConstructivePlanLoop7Done(work)
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
pub fn source_ml_temporary_read_datatype_constructive_plan_loop3(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut references: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut count: f64,
    mut work: f64,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut changed: bool,
    mut rank: f64,
) -> self::MlTemporaryReadDatatypeConstructivePlanLoop3Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let rank_2 = rank + 1f64;
                    {
                        let values = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_s15 =
                                    self::source_ml_temporary_read_datatype_constructive_plan_loop7(
                                        kinds.clone(),
                                        ends.clone(),
                                        edges.clone(),
                                        references.clone(),
                                        count,
                                        work,
                                        known.clone(),
                                        values.clone(),
                                        at,
                                    );
                                match source_ml_source1_s15 {
                                    self::MlTemporaryReadDatatypeConstructivePlanLoop7Result::MlTemporaryReadDatatypeConstructivePlanLoop7Done(work_2) => {
                                        {
                                            let declaration = 0f64;
                                            {
                                                let changed_3 = self::source_ml_temporary_read_datatype_constructive_plan_loop8(ends.clone(), edges.clone(), roots.clone(), known.clone(), witnesses.clone(), ranks.clone(), changed_2, rank_2, values.clone(), declaration);
                                                {
                                                    (work, changed, rank) = (work_2, changed_3, rank_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadDatatypeConstructivePlanLoop7Result::MlTemporaryReadDatatypeConstructivePlanLoop7Return(source_ml_temporary_result12) => {
                                        self::MlTemporaryReadDatatypeConstructivePlanLoop3Result::MlTemporaryReadDatatypeConstructivePlanLoop3Return(source_ml_temporary_result12.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadDatatypeConstructivePlanLoop3Result::MlTemporaryReadDatatypeConstructivePlanLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop2Result {
    MlTemporaryReadDatatypeConstructivePlanLoop2Done(f64, bool),
    MlTemporaryReadDatatypeConstructivePlanLoop2Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop17Result {
    MlTemporaryReadDatatypeConstructivePlanLoop17Done(f64, bool),
    MlTemporaryReadDatatypeConstructivePlanLoop17Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop25Result {
    MlTemporaryReadDatatypeConstructivePlanLoop25Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_datatype_constructive_plan_loop25(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut domains: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut crossings: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
    mut domain: bool,
    mut crossed: bool,
    mut end: f64,
    mut child: f64,
) -> self::MlTemporaryReadDatatypeConstructivePlanLoop25Result {
    loop {
        if child < end {
            {
                let source_ml_temporary_read_datatype_constructive_plan_ignored26 = {
                    let ml_method_owner53 = pending.clone();
                    let ml_method_argument53_0 =
                        self::read_required_number_fact(edges.clone(), child);
                    let ml_inserted = vec![ml_method_argument53_0];
                    let mut ml_method_values53 = ml_method_owner53.borrow_mut();
                    let ml_length = ml_method_values53
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values53.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let source_ml_temporary_read_datatype_constructive_plan_ignored27 = {
                        let ml_method_owner54 = domains.clone();
                        let ml_method_argument54_0 = domain
                            || (((self::read_required_text_fact(kinds.clone(), at)) == "function")
                                && (child < (end - 1f64)));
                        let ml_inserted = vec![ml_method_argument54_0];
                        let mut ml_method_values54 = ml_method_owner54.borrow_mut();
                        let ml_length = ml_method_values54
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values54.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let source_ml_temporary_read_datatype_constructive_plan_ignored28 = {
                            let ml_method_owner55 = crossings.clone();
                            let ml_method_argument55_0 = crossed;
                            let ml_inserted = vec![ml_method_argument55_0];
                            let mut ml_method_values55 = ml_method_owner55.borrow_mut();
                            let ml_length = ml_method_values55
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4_294_967_295usize)
                                .expect("Invalid array length");
                            ml_method_values55.extend(ml_inserted);
                            ml_length as f64
                        };
                        {
                            let child_2 = child + 1f64;
                            {
                                child = child_2;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryReadDatatypeConstructivePlanLoop25Result::MlTemporaryReadDatatypeConstructivePlanLoop25Done
        };
    }
}
