// Generated from complete callable-source-rules.js through the checked translator.
// Do not edit by hand.
use super::MlCallableAppendNameLoop42Result;
use super::MlCallableCellObservationsLoop55Result;
use super::MlPlanCallableClosureScopeLoop45Result;
use super::MlPlanCallableClosureScopeLoop46Result;
use super::MlPlanCallableClosureScopeLoop47Result;
use super::MlPlanCallableClosureScopeLoop48Result;
use super::MlPlanCallableClosureScopeLoop49Result;
use super::MlPlanCallableMutableCellsLoop52Result;
use super::MlPlanCallableStatementCapturesLoop43Result;
use super::MlReadCallableClosureParameterErrorLoop39Result;
use super::MlReadCallableClosureParameterErrorLoop40Result;
use super::callable_field;
use super::callable_tag;
use super::callable_truthy;
use super::ml_array;
use super::ml_callable_append_name_loop42;
use super::ml_callable_cell_observations_loop55;
use super::ml_callable_cell_observations_loop56;
use super::ml_callable_cell_observations_loop57;
use super::ml_plan_callable_closure_scope_loop45;
use super::ml_plan_callable_closure_scope_loop46;
use super::ml_plan_callable_closure_scope_loop47;
use super::ml_plan_callable_closure_scope_loop48;
use super::ml_plan_callable_closure_scope_loop49;
use super::ml_plan_callable_mutable_cells_loop52;
use super::ml_plan_callable_mutable_cells_loop53;
use super::ml_plan_callable_parameter_ownership_loop37;
use super::ml_plan_callable_statement_captures_loop43;
use super::ml_plan_callable_surface_parameters_loop50;
use super::ml_read_callable_closure_parameter_error_loop39;
use super::ml_read_callable_closure_parameter_error_loop40;
use super::ml_read_callable_function_children_loop38;
use super::ml_render_callable_parameters_loop51;
use super::plan_callable_parameter_types;
use super::read_callable_statement_uses;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the complete checked source algorithm.
#[must_use]
pub fn require_callable_structural_inference(kind: &str, type_kind: &str, operation: &str) -> bool {
    if ((kind == "lambda") || (kind == "functionValue")) || (type_kind == "function") {
        true
    } else if (kind == "stringTest") && (operation == "includes") {
        true
    } else if ((((((((((((((((((kind == "none") || (kind == "coalesce"))
        || (kind == "optionalField"))
        || (kind == "record"))
        || (kind == "method"))
        || (kind == "map"))
        || (kind == "set"))
        || (kind == "arrayFrom"))
        || (kind == "arrayIsArray"))
        || (kind == "copySlice"))
        || (kind == "forOfIterable"))
        || (kind == "stringPoints"))
        || (kind == "cellAllocate"))
        || (kind == "cellRead"))
        || (kind == "cellWrite"))
        || (kind == "referenceWrite"))
        || (kind == "index"))
        || (kind == "tuple"))
        || (kind == "tupleIndex")
    {
        true
    } else {
        ((((type_kind == "option") || (type_kind == "tuple")) || (type_kind == "record"))
            || (type_kind == "map"))
            || (type_kind == "set")
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn require_callable_named_inference(
    head_kind: &str,
    path_length: f64,
    local_present: bool,
) -> bool {
    (head_kind == "name") && ((path_length != 1f64) || !local_present)
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn accept_callable_application_inference_result(
    kind: &str,
    parameters: f64,
    arguments_count: f64,
) -> bool {
    (kind == "function")
        && ((parameters).partial_cmp(&(arguments_count)) == Some(std::cmp::Ordering::Equal))
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn plan_callable_parameter_ownership(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    owner: f64,
) -> Vec<f64> {
    {
        let parameters = self::callable_field(starts, counts, labels, targets, owner, "params");
        {
            let plan = Vec::<f64>::new();
            {
                let at = 0f64;
                {
                    self::ml_plan_callable_parameter_ownership_loop37(
                        starts, counts, targets, owner, parameters, plan, at,
                    )
                }
            }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn read_callable_function_children(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    type_: f64,
) -> Vec<f64> {
    {
        let params = self::callable_field(starts, counts, labels, targets, type_, "params");
        {
            let children = Vec::<f64>::new();
            {
                let at = 0f64;
                {
                    let children_2 = self::ml_read_callable_function_children_loop38(
                        starts, counts, targets, params, children, at,
                    );
                    {
                        let (ml_append_right, ml_append_left) = (
                            vec![self::callable_field(
                                starts, counts, labels, targets, type_, "ret",
                            )],
                            children_2,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    }
                }
            }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_callable_lambda_inference(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    lambda: f64,
    expected: f64,
) -> Vec<f64> {
    {
        let explicit = self::callable_field(starts, counts, labels, targets, lambda, "ret");
        {
            let present = self::callable_truthy(forms, texts, explicit);
            {
                let contextual =
                    (self::callable_tag(texts, starts, counts, labels, targets, expected, "kind"))
                        == "function";
                {
                    let ret = if present {
                        explicit
                    } else if contextual {
                        self::callable_field(starts, counts, labels, targets, expected, "ret")
                    } else {
                        -1f64
                    };
                    {
                        let params = self::plan_callable_parameter_types(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_field(starts, counts, labels, targets, lambda, "params"),
                            expected,
                        );
                        self::ml_array::append(vec![ret, if present { 1f64 } else { 0f64 }], params)
                    }
                }
            }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn read_callable_closure_prefix_error(
    expression: bool,
    token_kind: &str,
    token_value: &str,
) -> String {
    if expression && (token_kind == "identifier") {
        String::from("named function expression")
    } else if token_value == "*" {
        String::from("generator function")
    } else {
        String::new()
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn read_callable_closure_parameter_error(names: &[String], defaults: &[bool]) -> String {
    {
        let at = 0f64;
        {
            let ml_s10 = self::ml_read_callable_closure_parameter_error_loop39(names, at);
            match ml_s10 {
                self::MlReadCallableClosureParameterErrorLoop39Result::MlReadCallableClosureParameterErrorLoop39Done(at_2) => {
                    {
                        let at_3 = 0f64;
                        {
                            let ml_s11 = self::ml_read_callable_closure_parameter_error_loop40(defaults, at_3);
                            match ml_s11 {
                                self::MlReadCallableClosureParameterErrorLoop40Result::MlReadCallableClosureParameterErrorLoop40Done => {
                                    String::new()
                                }
                                self::MlReadCallableClosureParameterErrorLoop40Result::MlReadCallableClosureParameterErrorLoop40Return(ml_result40) => {
                                    ml_result40
                                }
                            }
                        }
                    }
                }
                self::MlReadCallableClosureParameterErrorLoop39Result::MlReadCallableClosureParameterErrorLoop39Return(ml_result40_2) => {
                    ml_result40_2
                }
            }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn callable_append_name(texts: &[String], identities: &[f64], identity: f64) -> Vec<f64> {
    {
        let at = 0f64;
        {
            let ml_s12 = self::ml_callable_append_name_loop42(texts, identities, identity, at);
            match ml_s12 {
                self::MlCallableAppendNameLoop42Result::MlCallableAppendNameLoop42Done => {
                    let (ml_append_right, ml_append_left) = (vec![identity], identities.to_vec());
                    self::ml_array::append(ml_append_left, ml_append_right)
                }
                self::MlCallableAppendNameLoop42Result::MlCallableAppendNameLoop42Return(
                    ml_result42,
                ) => ml_result42,
            }
        }
    }
}

/// Execute the complete checked source algorithm.
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
                        let ml_s13 = self::ml_plan_callable_statement_captures_loop43(
                            texts,
                            outer,
                            &(events),
                            assigned,
                            free,
                            at,
                        );
                        match ml_s13 {
                            self::MlPlanCallableStatementCapturesLoop43Result::MlPlanCallableStatementCapturesLoop43Done(assigned_2, free_2) => {
                                self::ml_array::append(self::ml_array::append(vec![(assigned_2.len() as f64)], assigned_2), free_2)
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the complete checked source algorithm.
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
                                        let ml_s14 = self::ml_plan_callable_closure_scope_loop45(
                                            texts, starts, counts, labels, targets, tokens, names,
                                            at,
                                        );
                                        match ml_s14 {
                                            self::MlPlanCallableClosureScopeLoop45Result::MlPlanCallableClosureScopeLoop45Done(names_2, at_2) => {
                                                {
                                                    let at_3 = 0f64;
                                                    {
                                                        let ml_s15 = self::ml_plan_callable_closure_scope_loop46(texts, starts, counts, targets, old_locals, locals, at_3);
                                                        match ml_s15 {
                                                            self::MlPlanCallableClosureScopeLoop46Result::MlPlanCallableClosureScopeLoop46Done(locals_2, at_4) => {
                                                                {
                                                                    let at_5 = 0f64;
                                                                    {
                                                                        let ml_s16 = self::ml_plan_callable_closure_scope_loop47(texts, starts, counts, targets, old_temporal, temporal, &(names_2), at_5);
                                                                        match ml_s16 {
                                                                            self::MlPlanCallableClosureScopeLoop47Result::MlPlanCallableClosureScopeLoop47Done(temporal_2, at_6) => {
                                                                                {
                                                                                    let at_7 = 0f64;
                                                                                    {
                                                                                        let ml_s17 = self::ml_plan_callable_closure_scope_loop48(texts, starts, counts, targets, old_mutable, mutable, at_7);
                                                                                        match ml_s17 {
                                                                                            self::MlPlanCallableClosureScopeLoop48Result::MlPlanCallableClosureScopeLoop48Done(mutable_2, at_8) => {
                                                                                                {
                                                                                                    let at_9 = 0f64;
                                                                                                    {
                                                                                                        let ml_s18 = self::ml_plan_callable_closure_scope_loop49(texts, starts, counts, labels, targets, tokens, locals_2, mutable_2, at_9);
                                                                                                        match ml_s18 {
                                                                                                            self::MlPlanCallableClosureScopeLoop49Result::MlPlanCallableClosureScopeLoop49Done(locals_3, mutable_3) => {
                                                                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(vec![(locals_3.len() as f64), (temporal_2.len() as f64), (mutable_3.len() as f64)], locals_3), temporal_2), mutable_3)
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

/// Execute the complete checked source algorithm.
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
                self::ml_plan_callable_surface_parameters_loop50(
                    starts, counts, labels, targets, tokens, plan, at,
                )
            }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn require_callable_assigned_capture_field(count: f64) -> bool {
    count > 0f64
}

/// Execute the complete checked source algorithm.
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

/// Execute the complete checked source algorithm.
#[must_use]
pub fn render_callable_function_reference(format: &str, fn_: &str) -> String {
    if format == "Rust" {
        format!("std::rc::Rc::new({})", fn_.to_owned())
    } else {
        fn_.to_owned()
    }
}

/// Execute the complete checked source algorithm.
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

/// Execute the complete checked source algorithm.
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

/// Execute the complete checked source algorithm.
#[must_use]
pub fn render_callable_parameters(format: &str, names: &[String], types: &[String]) -> Vec<String> {
    {
        let parameters = Vec::<String>::new();
        {
            let at = 0f64;
            { self::ml_render_callable_parameters_loop51(format, names, types, parameters, at) }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn render_callable_function_type(format: &str, parameters: &[String], ret: &str) -> String {
    format!(
        "({})",
        ({
            let (ml_append_right, ml_append_left) = (vec![ret.to_owned()], parameters.to_vec());
            self::ml_array::append(ml_append_left, ml_append_right)
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

/// Execute the complete checked source algorithm.
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
                        let ml_s19 = self::ml_plan_callable_mutable_cells_loop52(
                            texts,
                            &(events),
                            captures,
                            assigned,
                            at,
                        );
                        match ml_s19 {
                            self::MlPlanCallableMutableCellsLoop52Result::MlPlanCallableMutableCellsLoop52Done(captures_2, assigned_2, at_2) => {
                                {
                                    let cells = Vec::<f64>::new();
                                    {
                                        let at_3 = 0f64;
                                        {
                                            self::ml_plan_callable_mutable_cells_loop53(texts, &(captures_2), &(assigned_2), at_3, cells)
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

/// Execute the complete checked source algorithm.
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
                    let events_6 = if (self::callable_tag(
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
                                        let ml_s20 = self::ml_callable_cell_observations_loop55(
                                            starts, counts, targets, events_3, captured, at,
                                        );
                                        match ml_s20 {
                                            self::MlCallableCellObservationsLoop55Result::MlCallableCellObservationsLoop55Done(events_4, at_2) => {
                                                {
                                                    let at_3 = 0f64;
                                                    {
                                                        self::ml_callable_cell_observations_loop56(starts, counts, targets, events_4, assigned, at_3)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        events_3
                    };
                    {
                        let at_4 = self::read_required_number_fact(starts, node);
                        {
                            self::ml_callable_cell_observations_loop57(
                                forms, texts, starts, counts, labels, targets, node, events_6, at_4,
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the complete checked source algorithm.
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

/// Execute the complete checked source algorithm.
#[must_use]
pub fn render_callable_cell_parameter_name(source: &str, reference: bool) -> String {
    if reference {
        format!("ml_cell_{}", source.to_owned())
    } else {
        source.to_owned()
    }
}

/// Execute the complete checked source algorithm.
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

/// Execute the complete checked source algorithm.
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

/// Execute the complete checked source algorithm.
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

/// Execute the complete checked source algorithm.
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

/// Execute the complete checked source algorithm.
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

/// Execute the complete checked source algorithm.
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

/// Execute the complete checked source algorithm.
#[must_use]
pub fn read_callable_reference_construct(name: &str, temporal: bool) -> String {
    if temporal {
        format!("use of {} before its declaration", name.to_owned())
    } else {
        format!("function value {}", name.to_owned())
    }
}

/// Execute the complete checked source algorithm.
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

/// Execute the complete checked source algorithm.
#[must_use]
pub fn read_callable_renaming_constant(constant: f64) -> String {
    if constant == 0f64 {
        String::from("cellRead")
    } else {
        String::new()
    }
}
