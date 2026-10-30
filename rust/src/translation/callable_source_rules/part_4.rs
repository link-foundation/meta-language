// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::MlSource1TemporaryCallableAppendNameLoop49Result;
use super::MlSource1TemporaryCallableCellObservationsLoop62Result;
use super::MlSource1TemporaryCallableRenamingBindingLoop66Result;
use super::MlSource1TemporaryPlanCallableClosureScopeLoop52Result;
use super::MlSource1TemporaryPlanCallableClosureScopeLoop53Result;
use super::MlSource1TemporaryPlanCallableClosureScopeLoop54Result;
use super::MlSource1TemporaryPlanCallableClosureScopeLoop55Result;
use super::MlSource1TemporaryPlanCallableClosureScopeLoop56Result;
use super::MlSource1TemporaryPlanCallableMutableCellsLoop59Result;
use super::MlSource1TemporaryPlanCallableStatementCapturesLoop50Result;
use super::callable_element;
use super::callable_field;
use super::callable_renamed_node;
use super::callable_tag;
use super::callable_truthy;
use super::ml_array;
use super::read_callable_statement_uses;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_callable_append_name_loop49;
use super::source_ml_source1_temporary_callable_cell_observations_loop62;
use super::source_ml_source1_temporary_callable_cell_observations_loop63;
use super::source_ml_source1_temporary_callable_cell_observations_loop64;
use super::source_ml_source1_temporary_callable_renaming_binding_loop66;
use super::source_ml_source1_temporary_plan_callable_closure_scope_loop52;
use super::source_ml_source1_temporary_plan_callable_closure_scope_loop53;
use super::source_ml_source1_temporary_plan_callable_closure_scope_loop54;
use super::source_ml_source1_temporary_plan_callable_closure_scope_loop55;
use super::source_ml_source1_temporary_plan_callable_closure_scope_loop56;
use super::source_ml_source1_temporary_plan_callable_mutable_cells_loop59;
use super::source_ml_source1_temporary_plan_callable_mutable_cells_loop60;
use super::source_ml_source1_temporary_plan_callable_source_renaming_loop65;
use super::source_ml_source1_temporary_plan_callable_statement_captures_loop50;
use super::source_ml_source1_temporary_plan_callable_surface_parameters_loop57;
use super::source_ml_source1_temporary_render_callable_parameters_loop58;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_append_name(texts: &[String], identities: &[f64], identity: f64) -> Vec<f64> {
    {
        let at = 0f64;
        {
            let source_ml_source2_s60 =
                self::source_ml_source1_temporary_callable_append_name_loop49(
                    texts, identities, identity, at,
                );
            match source_ml_source2_s60 {
                self::MlSource1TemporaryCallableAppendNameLoop49Result::MlSource1TemporaryCallableAppendNameLoop49Done => {
                    {
                        let source_ml_source2_concat_value61 = identities.to_vec();
                        {
                            let source_ml_source2_concat_value62 = vec![identity];
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value61), source_ml_source2_concat_value62)
                        }
                    }
                }
                self::MlSource1TemporaryCallableAppendNameLoop49Result::MlSource1TemporaryCallableAppendNameLoop49Return(source_ml_source1_temporary_result49) => {
                    source_ml_source1_temporary_result49
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
pub fn plan_callable_statement_captures(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    statements: f64,
    bound: &[String],
    outer: &[String],
) -> Vec<f64> {
    {
        let events = self::read_callable_statement_uses(
            forms, texts, starts, counts, labels, targets, statements, bound,
        );
        {
            let assigned = Vec::<f64>::new();
            {
                let free = Vec::<f64>::new();
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s63 = self::source_ml_source1_temporary_plan_callable_statement_captures_loop50(texts, outer, &(events), assigned, free, at);
                        match source_ml_source2_s63 {
                            self::MlSource1TemporaryPlanCallableStatementCapturesLoop50Result::MlSource1TemporaryPlanCallableStatementCapturesLoop50Done(assigned_2, free_2) => {
                                {
                                    let source_ml_source2_concat_value64 = vec![(assigned_2.len() as f64)];
                                    {
                                        let source_ml_source2_concat_value65 = assigned_2;
                                        {
                                            let source_ml_source2_concat_value66 = free_2;
                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value64), source_ml_source2_concat_value65), source_ml_source2_concat_value66)
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
pub fn plan_callable_closure_scope(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    outer: f64,
    tokens: f64,
) -> Vec<f64> {
    {
        let old_locals = self::callable_field(starts, counts, labels, targets, outer, "locals");
        {
            let old_temporal = self::callable_field(starts, counts, labels, targets, outer, "tdz");
            {
                let old_mutable =
                    self::callable_field(starts, counts, labels, targets, outer, "mutable");
                {
                    let locals = Vec::<f64>::new();
                    {
                        let temporal = Vec::<f64>::new();
                        {
                            let mutable = Vec::<f64>::new();
                            {
                                let names = Vec::<String>::new();
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_source2_s67 = self::source_ml_source1_temporary_plan_callable_closure_scope_loop52(texts, starts, counts, labels, targets, tokens, names, at);
                                        match source_ml_source2_s67 {
                                            self::MlSource1TemporaryPlanCallableClosureScopeLoop52Result::MlSource1TemporaryPlanCallableClosureScopeLoop52Done(names_2, at_2) => {
                                                {
                                                    let at_3 = 0f64;
                                                    {
                                                        let source_ml_source2_s68 = self::source_ml_source1_temporary_plan_callable_closure_scope_loop53(texts, starts, counts, targets, old_locals, locals, at_3);
                                                        match source_ml_source2_s68 {
                                                            self::MlSource1TemporaryPlanCallableClosureScopeLoop53Result::MlSource1TemporaryPlanCallableClosureScopeLoop53Done(locals_2, at_4) => {
                                                                {
                                                                    let at_5 = 0f64;
                                                                    {
                                                                        let source_ml_source2_s69 = self::source_ml_source1_temporary_plan_callable_closure_scope_loop54(texts, starts, counts, targets, old_temporal, temporal, &(names_2), at_5);
                                                                        match source_ml_source2_s69 {
                                                                            self::MlSource1TemporaryPlanCallableClosureScopeLoop54Result::MlSource1TemporaryPlanCallableClosureScopeLoop54Done(temporal_2, at_6) => {
                                                                                {
                                                                                    let at_7 = 0f64;
                                                                                    {
                                                                                        let source_ml_source2_s70 = self::source_ml_source1_temporary_plan_callable_closure_scope_loop55(texts, starts, counts, targets, old_mutable, mutable, at_7);
                                                                                        match source_ml_source2_s70 {
                                                                                            self::MlSource1TemporaryPlanCallableClosureScopeLoop55Result::MlSource1TemporaryPlanCallableClosureScopeLoop55Done(mutable_2, at_8) => {
                                                                                                {
                                                                                                    let at_9 = 0f64;
                                                                                                    {
                                                                                                        let source_ml_source2_s71 = self::source_ml_source1_temporary_plan_callable_closure_scope_loop56(texts, starts, counts, labels, targets, tokens, locals_2, mutable_2, at_9);
                                                                                                        match source_ml_source2_s71 {
                                                                                                            self::MlSource1TemporaryPlanCallableClosureScopeLoop56Result::MlSource1TemporaryPlanCallableClosureScopeLoop56Done(locals_3, mutable_3) => {
                                                                                                                {
                                                                                                                    let source_ml_source2_concat_value72 = vec![(locals_3.len() as f64), (temporal_2.len() as f64), (mutable_3.len() as f64)];
                                                                                                                    {
                                                                                                                        let source_ml_source2_concat_value73 = locals_3;
                                                                                                                        {
                                                                                                                            let source_ml_source2_concat_value74 = temporal_2;
                                                                                                                            {
                                                                                                                                let source_ml_source2_concat_value75 = mutable_3;
                                                                                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value72), source_ml_source2_concat_value73), source_ml_source2_concat_value74), source_ml_source2_concat_value75)
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
pub fn plan_callable_surface_parameters(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    tokens: f64,
) -> Vec<f64> {
    {
        let plan = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                self::source_ml_source1_temporary_plan_callable_surface_parameters_loop57(
                    starts, counts, labels, targets, tokens, plan, at,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_callable_assigned_capture_field(count: f64) -> bool {
    count > 0f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_surface_constructor(form: &str, known_function: bool) -> String {
    if form == "closure" {
        String::from("lambda")
    } else if form == "computed-call" {
        String::from("app")
    } else if (form == "name") && known_function {
        String::from("functionValue")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_function_reference(format: &str, fn_: &str) -> String {
    if format == "Rust" {
        format!("std::rc::Rc::new({})", fn_.to_owned())
    } else {
        fn_.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_application(format: &str, fn_: &str, args: &[String]) -> String {
    if (format == "Rust") || (format == "JavaScript") {
        format!(
            "({})({})",
            fn_.to_owned(),
            args.join((String::from(", ")).as_str())
        )
    } else {
        format!(
            "({} {})",
            fn_.to_owned(),
            args.join((String::from(" ")).as_str())
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_lambda(
    format: &str,
    names: &[String],
    types: &[String],
    ret: &str,
    body: &str,
    copies: &[String],
) -> String {
    if format == "JavaScript" {
        format!(
            "(({}) => {})",
            names.join((String::from(", ")).as_str()),
            body.to_owned()
        )
    } else {
        {
            let parameters = self::render_callable_parameters(format, names, types);
            if format == "Rust" {
                {
                    let closure = format!(
                        "std::rc::Rc::new(move |{}| -> {} {{ {} }})",
                        parameters.join((String::from(", ")).as_str()),
                        ret.to_owned(),
                        body.to_owned()
                    );
                    {
                        let ml_library_condition_1 = {
                            let ml_value = copies.len() as f64;
                            ml_value != 0f64 && !ml_value.is_nan()
                        };
                        if ml_library_condition_1 {
                            format!(
                                "{{ {} {closure} }}",
                                copies.join((String::from(" ")).as_str())
                            )
                        } else {
                            closure
                        }
                    }
                }
            } else {
                format!(
                    "(fun {} => {})",
                    parameters.join((String::from(" ")).as_str()),
                    body.to_owned()
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_parameters(format: &str, names: &[String], types: &[String]) -> Vec<String> {
    {
        let parameters = Vec::<String>::new();
        {
            let at = 0f64;
            {
                self::source_ml_source1_temporary_render_callable_parameters_loop58(
                    format, names, types, parameters, at,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_function_type(format: &str, parameters: &[String], ret: &str) -> String {
    format!(
        "({})",
        ({
            let source_ml_source2_concat_value76 = parameters.to_vec();
            {
                let source_ml_source2_concat_value77 = vec![ret.to_owned()];
                self::ml_array::append(
                    self::ml_array::append(Vec::<String>::new(), source_ml_source2_concat_value76),
                    source_ml_source2_concat_value77,
                )
            }
        })
        .join(
            (if format == "Lean" {
                String::from(" → ")
            } else {
                String::from(" -> ")
            })
            .as_str()
        )
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_callable_mutable_cells(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
) -> Vec<f64> {
    {
        let events =
            self::callable_cell_observations(forms, texts, starts, counts, labels, targets, node);
        {
            let captures = Vec::<f64>::new();
            {
                let assigned = Vec::<String>::new();
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s78 =
                            self::source_ml_source1_temporary_plan_callable_mutable_cells_loop59(
                                texts,
                                &(events),
                                captures,
                                assigned,
                                at,
                            );
                        match source_ml_source2_s78 {
                            self::MlSource1TemporaryPlanCallableMutableCellsLoop59Result::MlSource1TemporaryPlanCallableMutableCellsLoop59Done(captures_2, assigned_2, at_2) => {
                                {
                                    let cells = Vec::<f64>::new();
                                    {
                                        let at_3 = 0f64;
                                        {
                                            self::source_ml_source1_temporary_plan_callable_mutable_cells_loop60(texts, &(captures_2), &(assigned_2), at_3, cells)
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
pub fn callable_cell_observations(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
) -> Vec<f64> {
    if (node < 0f64)
        || (((self::read_required_text_fact(forms, node)) != "object")
            && ((self::read_required_text_fact(forms, node)) != "array"))
    {
        Vec::<f64>::new()
    } else {
        {
            let events = Vec::<f64>::new();
            {
                let events_3 =
                    if (self::callable_tag(texts, starts, counts, labels, targets, node, "s"))
                        == "assign"
                    {
                        {
                            vec![
                                1f64,
                                self::callable_field(starts, counts, labels, targets, node, "name"),
                            ]
                        }
                    } else {
                        events
                    };
                {
                    let events_5 =
                        if (self::callable_tag(texts, starts, counts, labels, targets, node, "k"))
                            == "cellWrite"
                        {
                            {
                                let cell = self::callable_field(
                                    starts, counts, labels, targets, node, "cell",
                                );
                                {
                                    let path = self::callable_field(
                                        starts, counts, labels, targets, cell, "path",
                                    );
                                    if (((self::callable_tag(
                                        texts, starts, counts, labels, targets, cell, "k",
                                    )) == "name")
                                        && (path >= 0f64))
                                        && ((self::read_required_number_fact(counts, path)) == 1f64)
                                    {
                                        {
                                            let name = self::callable_element(
                                                starts, counts, targets, path, 0f64,
                                            );
                                            {
                                                {
                                                    let source_ml_source2_concat_value79 = events_3;
                                                    {
                                                        let source_ml_source2_concat_value80 =
                                                            vec![0f64, name, 1f64, name];
                                                        self::ml_array::append(
                                                            self::ml_array::append(
                                                                Vec::<f64>::new(),
                                                                source_ml_source2_concat_value79,
                                                            ),
                                                            source_ml_source2_concat_value80,
                                                        )
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        events_3
                                    }
                                }
                            }
                        } else {
                            events_3
                        };
                    {
                        let events_8 = if (self::callable_tag(
                            texts, starts, counts, labels, targets, node, "k",
                        )) == "lambda"
                        {
                            {
                                let captured = self::callable_field(
                                    starts, counts, labels, targets, node, "captures",
                                );
                                {
                                    let assigned = self::callable_field(
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        node,
                                        "assignedCaptures",
                                    );
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source2_s81 = self::source_ml_source1_temporary_callable_cell_observations_loop62(starts, counts, targets, events_5, captured, at);
                                            match source_ml_source2_s81 {
                                                self::MlSource1TemporaryCallableCellObservationsLoop62Result::MlSource1TemporaryCallableCellObservationsLoop62Done(events_6, at_2) => {
                                                    {
                                                        let at_3 = 0f64;
                                                        {
                                                            self::source_ml_source1_temporary_callable_cell_observations_loop63(starts, counts, targets, events_6, assigned, at_3)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            events_5
                        };
                        {
                            let at_4 = self::read_required_number_fact(starts, node);
                            {
                                self::source_ml_source1_temporary_callable_cell_observations_loop64(
                                    forms, texts, starts, counts, labels, targets, node, events_8,
                                    at_4,
                                )
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
pub fn read_callable_cell_lowering(operation: &str, reference: bool) -> String {
    if reference {
        if operation == "read" {
            String::from("cellRead")
        } else if operation == "assign" {
            String::from("cellWrite")
        } else if operation == "declare" {
            String::from("cellAllocate")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_cell_parameter_name(source: &str, reference: bool) -> String {
    if reference {
        format!("ml_cell_{}", source.to_owned())
    } else {
        source.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_cell_error(kind: &str, operation: &str) -> String {
    if kind == "cell" {
        String::new()
    } else if operation == "cellRead" {
        String::from("reading a value requires a reference cell")
    } else {
        String::from("writing a value requires a reference cell")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_callable_cell_element(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    expected: f64,
) -> Vec<f64> {
    {
        let type_ = self::callable_field(starts, counts, labels, targets, node, "type");
        {
            let element = self::callable_field(starts, counts, labels, targets, type_, "element");
            if self::callable_truthy(forms, texts, element) {
                vec![element, 1f64]
            } else {
                vec![
                    self::callable_field(starts, counts, labels, targets, expected, "element"),
                    0f64,
                ]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_callable_cell_read_inference(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    cell: f64,
    resolved: f64,
) -> Vec<f64> {
    if (self::callable_tag(texts, starts, counts, labels, targets, resolved, "kind")) == "cell" {
        vec![self::callable_field(
            starts, counts, labels, targets, resolved, "element",
        )]
    } else {
        vec![-1f64, cell]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_callable_cell_option_binding(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    resolved: f64,
    value: f64,
) -> Vec<f64> {
    {
        let element = self::callable_field(starts, counts, labels, targets, resolved, "element");
        if (((self::callable_tag(texts, starts, counts, labels, targets, resolved, "kind"))
            != "cell")
            || ((self::callable_tag(texts, starts, counts, labels, targets, element, "kind"))
                != "var"))
            || ((self::callable_tag(texts, starts, counts, labels, targets, value, "kind"))
                != "option")
        {
            Vec::<f64>::new()
        } else {
            vec![
                self::callable_field(starts, counts, labels, targets, element, "id"),
                value,
            ]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_cell_operation(
    format: &str,
    operation: &str,
    cell: &str,
    value: &str,
    body: &str,
    cell_name: &str,
    value_name: &str,
) -> String {
    if format == "JavaScript" {
        if operation == "cellAllocate" {
            format!("({{ value: {} }})", value.to_owned())
        } else if operation == "cellRead" {
            format!("({}).value", cell.to_owned())
        } else {
            format!(
                "(({}).value = {}, {})",
                cell.to_owned(),
                value.to_owned(),
                body.to_owned()
            )
        }
    } else if operation == "cellAllocate" {
        format!(
            "std::rc::Rc::new(std::cell::RefCell::new({}))",
            value.to_owned()
        )
    } else if operation == "cellRead" {
        format!(
            "{{ let ml_cell_value = {}.borrow().clone(); ml_cell_value }}",
            cell.to_owned()
        )
    } else {
        format!(
            "{{ let {} = {}; let {} = {}; *{}.borrow_mut() = {}; {} }}",
            cell_name.to_owned(),
            cell.to_owned(),
            value_name.to_owned(),
            value.to_owned(),
            cell_name.to_owned(),
            value_name.to_owned(),
            body.to_owned()
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_reference_obligation(temporal: bool, local: bool) -> String {
    if temporal {
        String::from(
            "the binding is in its temporal dead zone, where reading it throws a ReferenceError",
        )
    } else if local {
        String::new()
    } else {
        String::from("functions are only portable when called")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_reference_construct(name: &str, temporal: bool) -> String {
    if temporal {
        format!("use of {} before its declaration", name.to_owned())
    } else {
        format!("function value {}", name.to_owned())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_renaming_field(field: f64) -> String {
    if field == 0f64 {
        String::from("path")
    } else if field == 1f64 {
        String::from("params")
    } else if field == 2f64 {
        String::from("body")
    } else if field == 3f64 {
        String::from("cell")
    } else if field == 4f64 {
        String::from("value")
    } else if field == 5f64 {
        String::from("default")
    } else if field == 6f64 {
        String::from("k")
    } else if field == 7f64 {
        String::from("span")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_renaming_constant(constant: f64) -> String {
    if constant == 0f64 {
        String::from("cellRead")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_callable_source_renaming(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    scope: f64,
) -> Vec<f64> {
    {
        let bindings = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let bindings_2 =
                    self::source_ml_source1_temporary_plan_callable_source_renaming_loop65(
                        forms, texts, starts, counts, labels, targets, scope, bindings, at,
                    );
                self::callable_renamed_node(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(bindings_2),
                    false,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_renaming_binding(texts: &[String], bindings: &[f64], name: f64) -> Vec<f64> {
    {
        let at = bindings.len() as f64;
        {
            let source_ml_source2_s82 =
                self::source_ml_source1_temporary_callable_renaming_binding_loop66(
                    texts, bindings, name, at,
                );
            match source_ml_source2_s82 {
                self::MlSource1TemporaryCallableRenamingBindingLoop66Result::MlSource1TemporaryCallableRenamingBindingLoop66Done => {
                    vec![name, 0f64]
                }
                self::MlSource1TemporaryCallableRenamingBindingLoop66Result::MlSource1TemporaryCallableRenamingBindingLoop66Return(source_ml_source1_temporary_result66) => {
                    source_ml_source1_temporary_result66
                }
            }
        }
    }
}
