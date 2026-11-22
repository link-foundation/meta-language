// Generated from complete js/src/translation/rust-borrowed-iterator-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_primitive_method_default_value;
use super::source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop3;
use super::source_ml_source1_temporary_read_rust_borrowed_iterator_budget_error_loop6;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_primitive_method_argument_obligation(
    receiver: &str,
    operation: &str,
    element: &str,
) -> String {
    if (receiver == "array") && (operation == "join") {
        {
            let form = self::read_primitive_array_join_element_form(element);
            if (form == "refuse") || (form == "optional") {
                String::from(
                    "joining structured values needs their JavaScript string conversion and cycle semantics",
                )
            } else {
                String::new()
            }
        }
    } else if (receiver != "array")
        || (((operation != "includes") && (operation != "indexOf")) && (operation != "lastIndexOf"))
    {
        String::new()
    } else if (((((element == "array") || (element == "record")) || (element == "map"))
        || (element == "set"))
        || (element == "data"))
        || (element == "function")
    {
        String::from("object identity must be represented before searching arrays of objects")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_primitive_array_search_comparator(
    operation: &str,
    element: &str,
    absence: &str,
) -> String {
    {
        let present = if (operation == "includes") && (element == "float") {
            String::from("a == b || (a.is_nan() && b.is_nan())")
        } else {
            String::from("a == b")
        };
        if absence == "both" {
            format!(
                "|a, b| match (a, b) {{ (crate::MlOptional::Value(a), crate::MlOptional::Value(b)) => {present}, (crate::MlOptional::Null, crate::MlOptional::Null) | (crate::MlOptional::Undefined, crate::MlOptional::Undefined) => true, _ => false }}"
            )
        } else if (absence == "null") || (absence == "undefined") {
            format!(
                "|a, b| match (a, b) {{ (Some(a), Some(b)) => {present}, (None, None) => true, _ => false }}"
            )
        } else if (operation == "includes") && (element == "float") {
            format!("|a: &f64, b: &f64| {present}")
        } else {
            format!("|a, b| {present}")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_primitive_array_search_expression(
    operation: &str,
    object: &str,
    search: &str,
    from: &str,
    element: &str,
    absence: &str,
) -> String {
    {
        let comparator =
            self::render_primitive_array_search_comparator(operation, element, absence);
        format!(
            "crate::ml_array::{}({}, &{}, {}, {}, {comparator})",
            if operation == "includes" {
                String::from("includes")
            } else {
                String::from("index_of")
            },
            object.to_owned(),
            search.to_owned(),
            from.to_owned(),
            if operation == "lastIndexOf" {
                String::from("true")
            } else {
                String::from("false")
            }
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_primitive_method_rendering_refusal(
    target: &str,
    receiver: &str,
    operation: &str,
) -> String {
    if (target != "Lean") && (target != "Rocq") {
        String::new()
    } else if receiver == "array" {
        if (((operation == "slice") || (operation == "includes")) || (operation == "indexOf"))
            || (operation == "lastIndexOf")
        {
            format!(
                "this array operation needs an operational encoding in {}",
                target.to_owned()
            )
        } else {
            String::new()
        }
    } else {
        String::from("this target needs a UTF-16 operational string encoding for this operation")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_primitive_method_argument(
    target: &str,
    receiver: &str,
    operation: &str,
    index: f64,
    absent: bool,
    text: &str,
) -> String {
    if (target == "JavaScript") && absent {
        String::from("undefined")
    } else if (((target == "Rust") && (receiver == "string")) && (operation == "split"))
        && (index == 0f64)
    {
        if absent {
            String::from("None")
        } else {
            format!("Some({})", text.to_owned())
        }
    } else {
        text.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_primitive_method_expression(
    target: &str,
    receiver: &str,
    operation: &str,
    object: &str,
    arguments_text: &str,
) -> String {
    if target == "JavaScript" {
        format!(
            "({}).{}({})",
            object.to_owned(),
            operation.to_owned(),
            arguments_text.to_owned()
        )
    } else if ((target == "Rust") && (receiver == "array")) && (operation == "slice") {
        format!(
            "crate::ml_array::slice({}, {})",
            object.to_owned(),
            arguments_text.to_owned()
        )
    } else if (target != "Rust") || (receiver != "string") {
        String::new()
    } else if operation == "indexOf" {
        format!(
            "{}.index_of({})",
            object.to_owned(),
            arguments_text.to_owned()
        )
    } else if operation == "lastIndexOf" {
        format!(
            "{}.last_index_of({})",
            object.to_owned(),
            arguments_text.to_owned()
        )
    } else if operation == "codePointAt" {
        format!(
            "{}.code_point_at({})",
            object.to_owned(),
            arguments_text.to_owned()
        )
    } else if (operation == "replace") || (operation == "replaceAll") {
        format!(
            "{}.replace({}, {})",
            object.to_owned(),
            arguments_text.to_owned(),
            if operation == "replaceAll" {
                String::from("true")
            } else {
                String::from("false")
            }
        )
    } else if (operation == "padStart") || (operation == "padEnd") {
        format!(
            "{}.pad({}, {})",
            object.to_owned(),
            arguments_text.to_owned(),
            if operation == "padStart" {
                String::from("true")
            } else {
                String::from("false")
            }
        )
    } else {
        format!(
            "{}.{}({})",
            object.to_owned(),
            operation.to_owned(),
            arguments_text.to_owned()
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_primitive_method_utf16(expression: &str, receiver: &str) -> bool {
    (expression == "method") && (receiver == "string")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_primitive_method_lowering_form(receiver: &str, operation: &str) -> String {
    if (receiver == "array") && (operation == "at") {
        String::from("relative-element-template")
    } else if (receiver == "string") && (operation == "at") {
        String::from("relative-character-template")
    } else if (receiver == "string") && (operation == "charAt") {
        String::from("character-template")
    } else if (receiver == "string") && (operation == "charCodeAt") {
        String::from("code-unit-template")
    } else {
        String::from("method")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_primitive_array_join_element_form(kind: &str) -> String {
    if kind == "string" {
        String::from("string")
    } else if kind == "unit" {
        String::from("empty")
    } else if kind == "option" {
        String::from("optional")
    } else if ((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "int"))
        || (kind == "nat")
    {
        String::from("text")
    } else {
        String::from("refuse")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_primitive_method_argument_obligation_feature(operation: &str) -> String {
    if operation == "join" {
        String::from("array join")
    } else {
        String::from("array identity search")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_primitive_array_join_operand_form(
    units: bool,
    string_element: bool,
    operand: &str,
) -> String {
    if units && ((operand == "separator") || string_element) {
        String::from("borrow")
    } else {
        String::from("receiver")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_primitive_array_join_element(
    kind: &str,
    absence: &str,
    name: &str,
    present: &str,
    empty: &str,
) -> String {
    {
        let form = self::read_primitive_array_join_element_form(kind);
        if form == "empty" {
            empty.to_owned()
        } else if form == "optional" {
            if absence == "both" {
                format!(
                    "match {} {{ crate::MlOptional::Value(ml_present) => {}, _ => {} }}",
                    name.to_owned(),
                    present.to_owned(),
                    empty.to_owned()
                )
            } else {
                format!(
                    "match {} {{ Some(ml_present) => {}, None => {} }}",
                    name.to_owned(),
                    present.to_owned(),
                    empty.to_owned()
                )
            }
        } else {
            present.to_owned()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_primitive_array_join_expression(
    units: bool,
    string_element: bool,
    object: &str,
    separator: &str,
    element_text: &str,
) -> String {
    if string_element {
        if units {
            format!(
                "crate::ml_string::join({}, {})",
                object.to_owned(),
                separator.to_owned()
            )
        } else {
            format!(
                "{}.join({}.as_str())",
                object.to_owned(),
                separator.to_owned()
            )
        }
    } else {
        {
            let values = format!(
                "{}.iter().cloned().map(|ml_value| {}).collect::<Vec<_>>()",
                object.to_owned(),
                element_text.to_owned()
            );
            if units {
                format!(
                    "crate::ml_string::join(&{values}, {})",
                    separator.to_owned()
                )
            } else {
                format!("({values}).join({}.as_str())", separator.to_owned())
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
pub fn accept_rust_borrowed_iterator_receiver(
    selected: bool,
    method: &str,
    count: f64,
    kind: &str,
    owner: &str,
    path: &str,
    arguments_count: f64,
    closed: bool,
) -> bool {
    ((((((selected && (method == "iter")) && (count == 0f64)) && (kind == "array"))
        && (owner == "library"))
        && (path == "std::vec::Vec"))
        && (arguments_count == 1f64))
        && closed
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_iterator_continuation(operation: &str) -> String {
    if operation == "map" {
        String::from("map")
    } else if operation == "chain" {
        String::from("chain")
    } else if operation == "collect" {
        String::from("collect")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_iterator_projection_syntax(kind: &str) -> String {
    if kind == "name" {
        String::from("name")
    } else if kind == "match" {
        String::from("match")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_iterator_callback_error(
    selected: bool,
    parameters: f64,
    immutable: bool,
    reference: bool,
    closed: bool,
    captures: f64,
    body_projection: bool,
) -> String {
    if (((((selected && (parameters == 1f64)) && immutable) && reference) && closed)
        && (captures == 0f64))
        && body_projection
    {
        String::new()
    } else {
        String::from(
            "a readonly iterator callback needs one actual immutable reference parameter, a closed reference result and a capture-free parameter projection",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn project_rust_borrowed_iterator_callback_regions(
    loans: &[String],
    existing: &[String],
    local: &[String],
    receiver: &[String],
) -> Vec<String> {
    {
        let result = Vec::<String>::new();
        {
            let source_ml_source1_temporary_values1 = loans.to_vec();
            {
                let source_ml_source1_temporary_index1 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s1 = self::source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop3(existing, local, receiver, result, &(source_ml_source1_temporary_values1), source_ml_source1_temporary_index1);
                    match source_ml_source2_s1 {
                        self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Done(result_2) => {
                            result_2
                        }
                        self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Return(source_ml_source1_temporary_result3) => {
                            source_ml_source1_temporary_result3
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_iterator_region_error(
    actual: f64,
    projected: f64,
    collecting: bool,
) -> String {
    if !collecting && ((actual == 0f64) || (projected == 0f64)) {
        String::from("a readonly iterator value needs its actual live receiver-derived regions")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_iterator_once(
    path: &str,
    shadowed: bool,
    arguments_count: f64,
) -> bool {
    ((path == "std::iter::once") && !shadowed) && (arguments_count == 1f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_iterator_box_reference(
    selected: bool,
    canonical: bool,
    arguments_count: f64,
    reference: bool,
    live: bool,
    collecting: bool,
) -> bool {
    (((selected && canonical) && (arguments_count == 0f64)) && reference) && (live || collecting)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_iterator_budget_error(stages: f64, nodes: f64, regions: f64) -> String {
    {
        let source_ml_source1_temporary_values5 = vec![stages, nodes, regions];
        {
            let source_ml_source1_temporary_index5 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s2 = self::source_ml_source1_temporary_read_rust_borrowed_iterator_budget_error_loop6(&(source_ml_source1_temporary_values5), source_ml_source1_temporary_index5);
                match source_ml_source2_s2 {
                    self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Done => {
                        if ((stages > 64f64) || (nodes > 100_000f64)) || (regions > 65_536f64) {
                            String::from("the readonly iterator source pipeline exceeds its finite profile")
                        } else {
                            String::new()
                        }
                    }
                    self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Return(source_ml_source1_temporary_result6) => {
                        source_ml_source1_temporary_result6
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_iterator_collection_error(
    type_: bool,
    owner: bool,
    reference: bool,
    context: bool,
) -> String {
    if ((type_ && owner) && reference) && context {
        String::new()
    } else {
        String::from(
            "readonly collection needs its actual matching borrowed vector result position",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_iterator_production(
    operation: &str,
    source: bool,
    result: bool,
    projection: bool,
) -> bool {
    ((self::ml_array::includes(
        &vec![
            String::from("slice"),
            String::from("map"),
            String::from("concat"),
        ],
        &operation.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) && source)
        && result)
        && projection
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_borrowed_iterator_whole_slice(
    kinds: &[String],
    types: &[String],
    values: &[String],
) -> bool {
    (((((((((kinds.len() as f64) == 2f64) && ((types.len() as f64) == 2f64))
        && ((values.len() as f64) == 2f64))
        && {
            let source_ml_source2_comparison_operand3 =
                self::ml_array::optional_at(kinds, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand4 = String::from("lit");
                match (
                    source_ml_source2_comparison_operand3,
                    source_ml_source2_comparison_operand4,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        && {
            let source_ml_source2_comparison_operand5 =
                self::ml_array::optional_at(kinds, self::ml_array::number_index(1f64));
            {
                let source_ml_source2_comparison_operand6 = String::from("lit");
                match (
                    source_ml_source2_comparison_operand5,
                    source_ml_source2_comparison_operand6,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        && {
            let source_ml_source2_comparison_operand7 =
                self::ml_array::optional_at(types, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand8 = String::from("float");
                match (
                    source_ml_source2_comparison_operand7,
                    source_ml_source2_comparison_operand8,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        && {
            let source_ml_source2_comparison_operand9 =
                self::ml_array::optional_at(types, self::ml_array::number_index(1f64));
            {
                let source_ml_source2_comparison_operand10 = String::from("float");
                match (
                    source_ml_source2_comparison_operand9,
                    source_ml_source2_comparison_operand10,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        && {
            let source_ml_source2_comparison_operand11 =
                self::ml_array::optional_at(values, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand12 =
                    self::read_primitive_method_default_value("array", "slice", 0f64);
                match (
                    source_ml_source2_comparison_operand11,
                    source_ml_source2_comparison_operand12,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        && {
            let source_ml_source2_comparison_operand13 =
                self::ml_array::optional_at(values, self::ml_array::number_index(1f64));
            {
                let source_ml_source2_comparison_operand14 =
                    self::read_primitive_method_default_value("array", "slice", 1f64);
                match (
                    source_ml_source2_comparison_operand13,
                    source_ml_source2_comparison_operand14,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_iterator_snapshot_error(count: f64, text: f64) -> String {
    if ((((((!(self::ml_math::is_safe_integer(count))
        || !(self::ml_math::is_safe_integer(text)))
        || (count < 0f64))
        || (text < 0f64))
        || {
            let source_ml_source2_same_value_operand15 = count;
            {
                let source_ml_source2_same_value_operand16 = -0f64;
                ((((source_ml_source2_same_value_operand15)
                    .partial_cmp(&(source_ml_source2_same_value_operand16))
                    == Some(std::cmp::Ordering::Equal))
                    && ((source_ml_source2_same_value_operand15 != 0f64)
                        || ((1f64 / source_ml_source2_same_value_operand15)
                            == (1f64 / source_ml_source2_same_value_operand16))))
                    || ((source_ml_source2_same_value_operand15
                        != source_ml_source2_same_value_operand15)
                        && (source_ml_source2_same_value_operand16
                            != source_ml_source2_same_value_operand16)))
            }
        })
        || {
            let source_ml_source2_same_value_operand17 = text;
            {
                let source_ml_source2_same_value_operand18 = -0f64;
                ((((source_ml_source2_same_value_operand17)
                    .partial_cmp(&(source_ml_source2_same_value_operand18))
                    == Some(std::cmp::Ordering::Equal))
                    && ((source_ml_source2_same_value_operand17 != 0f64)
                        || ((1f64 / source_ml_source2_same_value_operand17)
                            == (1f64 / source_ml_source2_same_value_operand18))))
                    || ((source_ml_source2_same_value_operand17
                        != source_ml_source2_same_value_operand17)
                        && (source_ml_source2_same_value_operand18
                            != source_ml_source2_same_value_operand18)))
            }
        })
        || (count > 100_000f64))
        || (text > 16_777_216f64)
    {
        String::from("readonly callback ownership snapshots exceed their finite source profile")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_iterator_parameter_error(name: &str, raw_: &str) -> String {
    if raw_.starts_with("r#") {
        String::new()
    } else if self::ml_array::includes(
        &vec![
            String::from("_"),
            String::from("as"),
            String::from("break"),
            String::from("const"),
            String::from("continue"),
            String::from("crate"),
            String::from("else"),
            String::from("enum"),
            String::from("extern"),
            String::from("false"),
            String::from("fn"),
            String::from("for"),
            String::from("if"),
            String::from("impl"),
            String::from("in"),
            String::from("let"),
            String::from("loop"),
            String::from("match"),
            String::from("mod"),
            String::from("move"),
            String::from("mut"),
            String::from("pub"),
            String::from("ref"),
            String::from("return"),
            String::from("self"),
            String::from("Self"),
            String::from("static"),
            String::from("struct"),
            String::from("super"),
            String::from("trait"),
            String::from("true"),
            String::from("type"),
            String::from("unsafe"),
            String::from("use"),
            String::from("where"),
            String::from("while"),
            String::from("async"),
            String::from("await"),
            String::from("dyn"),
            String::from("abstract"),
            String::from("become"),
            String::from("box"),
            String::from("do"),
            String::from("final"),
            String::from("gen"),
            String::from("macro"),
            String::from("override"),
            String::from("priv"),
            String::from("try"),
            String::from("typeof"),
            String::from("unsized"),
            String::from("virtual"),
            String::from("yield"),
        ],
        &name.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        String::from("the readonly callback needs a fresh direct immutable binding identifier")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_iterator_binding_owner_error(
    value_kind: &str,
    declared_type: bool,
) -> String {
    if ((value_kind == "const") || (value_kind == "ctor")) || declared_type {
        String::from(
            "readonly callback parameters require a fresh binder rather than a constructor or constant pattern",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_iterator_region_work_error(count: f64) -> String {
    if ((!(self::ml_math::is_safe_integer(count)) || (count < 0f64)) || {
        let source_ml_source2_same_value_operand19 = count;
        {
            let source_ml_source2_same_value_operand20 = -0f64;
            ((((source_ml_source2_same_value_operand19)
                .partial_cmp(&(source_ml_source2_same_value_operand20))
                == Some(std::cmp::Ordering::Equal))
                && ((source_ml_source2_same_value_operand19 != 0f64)
                    || ((1f64 / source_ml_source2_same_value_operand19)
                        == (1f64 / source_ml_source2_same_value_operand20))))
                || ((source_ml_source2_same_value_operand19
                    != source_ml_source2_same_value_operand19)
                    && (source_ml_source2_same_value_operand20
                        != source_ml_source2_same_value_operand20)))
        }
    }) || (count > 256f64)
    {
        String::from("readonly iterator region union exceeds its bounded source work profile")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_borrowed_iterator_box_reference_error(
    canonical: bool,
    count: f64,
    admitted: bool,
    projection: bool,
) -> String {
    if canonical {
        if count == 0f64 {
            if !admitted || !projection {
                String::from(
                    "canonical Box::as_ref requires its actual live readonly receiver projection",
                )
            } else {
                String::new()
            }
        } else {
            String::from("canonical Box::as_ref expects zero arguments")
        }
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result {
    MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Done(Vec<String>),
    MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Return(Vec<String>),
}
