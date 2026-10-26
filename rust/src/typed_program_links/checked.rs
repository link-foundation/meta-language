//! Adapt the supported typed IR to the existing checker, preserving policies.
use super::projection::{project as expression, project_type};
use super::{Result, error, typed_program_links_schema};
use crate::translation::check::check_projected_program;
use crate::translation::ir::{Decl, Effect, Item, Program};
use crate::translation::primitive_value_rules::{
    accept_linked_text_conversion_kind, read_number_display_refusal,
};
use crate::translation::surface::{SEffect, SMain, SProgram, ShowStyle};
use crate::translation::typed_program_link_rules::{
    accept_program_division_rounding, accept_program_expression_operation,
    accept_program_machine_integer_width, accept_program_optional_absence,
    read_program_proposition_binding_form, read_program_proposition_operand_fields,
};
use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;
use std::sync::OnceLock;

mod declarations;
mod loop_modules;

fn project_assertion_proposition(mut value: Value, program: &Program) -> Result<Value> {
    let kind = value["p"].as_str().unwrap_or_default();
    let form = read_program_proposition_binding_form(kind);
    if form.is_empty() {
        return Err(error(
            "the linked proposition has no checked surface projection",
        ));
    }
    let fields = read_program_proposition_operand_fields(kind);
    if form == "binders" {
        for binder in value["binders"]
            .as_array_mut()
            .ok_or_else(|| error("a linked proposition has no owned binder list"))?
        {
            let ty = serde_json::from_value(binder["type"].clone())
                .map_err(|failure| error(failure.to_string()))?;
            binder["type"] = serde_json::to_value(project_type(&ty, program)?)
                .map_err(|failure| error(failure.to_string()))?;
        }
    }
    for field in fields.split_whitespace() {
        let child = value
            .get_mut(field)
            .ok_or_else(|| error("a linked proposition has no shared operand"))?;
        *child = if form == "expressions" {
            let checked = serde_json::from_value(child.clone())
                .map_err(|failure| error(failure.to_string()))?;
            serde_json::to_value(expression(&checked, program)?)
                .map_err(|failure| error(failure.to_string()))?
        } else {
            project_assertion_proposition(child.clone(), program)?
        };
    }
    Ok(value)
}

fn valid_float(value: &str) -> bool {
    static SPELLING: OnceLock<Regex> = OnceLock::new();
    SPELLING
        .get_or_init(|| {
            Regex::new(
                r"^(?:-?(?:[0-9]+(?:\.[0-9]+)?)(?:[eE][+-]?[0-9]+)?|NaN|Infinity|-Infinity)$",
            )
            .unwrap()
        })
        .is_match(value)
}

fn validate_type_records(
    value: &Value,
    data_names: &HashSet<&str>,
    source_language: &str,
) -> Result<()> {
    // Preserve the original depth-first order and every per-record check while
    // transporting traversal frames on the heap rather than the native stack.
    let mut pending = vec![value];
    while let Some(value) = pending.pop() {
        validate_type_record(value, data_names, source_language)?;
        if let Some(fields) = value.as_object() {
            pending.extend(fields.values().rev());
        } else if let Some(values) = value.as_array() {
            pending.extend(values.iter().rev());
        }
    }
    Ok(())
}

fn validate_type_record(
    value: &Value,
    data_names: &HashSet<&str>,
    source_language: &str,
) -> Result<()> {
    if let Some(fields) = value.as_object() {
        if value["k"] == "binary"
            && value
                .get("rounding")
                .is_some_and(|rounding| !rounding.is_null())
            && !accept_program_division_rounding(value["rounding"].as_str().unwrap_or_default())
        {
            return Err(error(
                "the linked program contains an unrecognized division rounding policy",
            ));
        }
        let kind = value["k"].as_str().unwrap_or_default();
        if kind == "binary" {
            let left = &value["left"]["type"];
            let right = &value["right"]["type"];
            let domain = value.get("domain").filter(|domain| !domain.is_null());
            let refusal = crate::translation::reference_identity_rules::read_reference_identity_comparison_error(
                value["op"].as_str().unwrap_or_default(), left["kind"].as_str().unwrap_or_default(), right["kind"].as_str().unwrap_or_default(),
                left["reference"] == true, right["reference"] == true, left == right, domain.is_some(),
                domain.is_some_and(|domain| domain["reference"] == true), domain == Some(left),
            );
            if !refusal.is_empty() {
                return Err(error(refusal));
            }
        }
        if kind == "apply" {
            let count = |field: &Value| {
                serde_json::Number::from(field.as_array().map_or(0, Vec::len))
                    .as_f64()
                    .expect("an allocated callable arity has a Number representation")
            };
            let refusal =
                crate::translation::callable_source_rules::read_callable_linked_application_error(
                    value["fn"]["type"]["kind"].as_str().unwrap_or_default(),
                    count(&value["fn"]["type"]["params"]),
                    count(&value["args"]),
                );
            if !refusal.is_empty() {
                return Err(error(refusal));
            }
        }
        if kind == "toString" {
            let argument_kind = value["arg"]["type"]["kind"].as_str().unwrap_or_default();
            if !accept_linked_text_conversion_kind(argument_kind) {
                return Err(error(
                    "a linked text conversion requires a supported scalar or existing record representation",
                ));
            }
            let refusal = read_number_display_refusal(source_language, argument_kind);
            if !refusal.is_empty() {
                return Err(error(format!("Rust Float display: {refusal}")));
            }
        }
        if typed_program_links_schema()["expressionOperations"]
            .get(kind)
            .is_some()
            && !accept_program_expression_operation(kind, value["op"].as_str().unwrap_or_default())
            && !crate::translation::string_array_sort_rules::accept_linked_string_array_copy_sort(
                kind, value["op"].as_str().unwrap_or_default(),
                value["object"]["type"]["kind"].as_str().unwrap_or_default(),
                value["object"]["type"]["element"]["kind"].as_str().unwrap_or_default(),
                value["object"]["type"]["reference"].as_bool()==Some(true),
                value["args"].as_array().map_or(-1.0, |args|serde_json::Number::from(args.len()).as_f64().expect("actual argument count")),
            )
            && !crate::translation::number_bitwise_rules::accept_linked_number_bitwise_operation(
                kind,
                value["op"].as_str().unwrap_or_default(),
            )
            && !crate::translation::reference_method_rules::accept_linked_reference_method_operation(
                kind,
                value["op"].as_str().unwrap_or_default(),
                value["object"]["type"]["kind"].as_str().unwrap_or_default(),
                value["object"]["type"]["reference"].as_bool() == Some(true),
            )
            && !crate::translation::record_write_rules::accept_linked_record_field_write(
                kind,
                value["op"].as_str().unwrap_or_default(),
                value["object"]["type"]["kind"].as_str().unwrap_or_default(),
                value["object"]["type"]["reference"].as_bool() == Some(true),
            )
            && !crate::translation::rust_shift_rules::accept_linked_rust_checked_shift(
                kind,value["op"].as_str().unwrap_or_default(),
            )
            && !crate::translation::rust_string_byte_length_rules::accept_rust_string_byte_length_operation(kind,value["op"].as_str().unwrap_or_default())
            && !crate::translation::array_state_method_rules::accept_linked_array_state_method(
                kind,value["op"].as_str().unwrap_or_default(),
                value["object"]["type"]["kind"].as_str().unwrap_or_default(),
                value["object"]["type"]["reference"].as_bool()==Some(true),
            )
            && !crate::translation::rust_primitive_bitwise_rules::accept_linked_rust_primitive_bitwise_operation(
                kind, value["op"].as_str().unwrap_or_default(),
            )
            && !crate::translation::big_integer_bitwise_rules::accept_linked_big_integer_bitwise_operation(
                kind, value["op"].as_str().unwrap_or_default(),
            )
            && !crate::translation::string_number_rules::accept_linked_java_script_number_conversion(
                kind, value["op"].as_str().unwrap_or_default(),
            )
            && !crate::translation::source_exception_rules::accept_linked_java_script_source_exception(
                kind, value["op"].as_str().unwrap_or_default(),
            )
            && !crate::translation::big_integer_conversion_rules::accept_linked_java_script_big_integer_result(
                kind, value["op"].as_str().unwrap_or_default(),
            )
            && !crate::translation::javascript_string_unit_rules::accept_linked_java_script_string_unit_construction(kind,value["op"].as_str().unwrap_or_default())
            && !crate::translation::string_construction_rules::accept_linked_java_script_string_construction(kind,value["op"].as_str().unwrap_or_default())
            && !(kind=="math"&&crate::translation::javascript_enumeration_rules::read_java_script_record_order_kernel_arity(value["op"].as_str().unwrap_or_default())>0.0)
            && !crate::translation::javascript_enumeration_rules::accept_linked_ordered_record_order_read(kind,value["op"].as_str().unwrap_or_default(),value["object"]["type"]["kind"].as_str().unwrap_or_default(),value["object"]["type"]["ordered"]==true,value["object"]["type"]["reference"]==true)
        {
            return Err(error(
                "the linked program contains an unrecognized expression operation",
            ));
        }
        if kind == "method" && value["op"] == "toSorted" && value["type"] != value["object"]["type"]
        {
            return Err(error(
                "copying sort result type differs from its actual receiver",
            ));
        }
        if value["k"] == "lit"
            && value["type"]["kind"] == "float"
            && !value["value"].as_str().is_some_and(valid_float)
        {
            return Err(error("a linked floating-point literal is invalid"));
        }
        if matches!(
            value["kind"].as_str(),
            Some("named" | "literal" | "hypothesis" | "output")
        ) {
            return Err(error(
                "the Rust typed links adapter does not yet recheck unresolved or threaded value types",
            ));
        }
        if value["kind"] == "data"
            && !value["name"]
                .as_str()
                .is_some_and(|name| data_names.contains(name))
        {
            return Err(error("the linked data type has no data declaration"));
        }
        if value["kind"] == "union" {
            let admitted_fields = fields
                .keys()
                .all(|field| crate::translation::type_syntax_rules::accept_union_type_field(field));
            let admitted_count = value["alternatives"]
                .as_array()
                .is_some_and(|alternatives| {
                    let count = serde_json::Number::from(alternatives.len())
                        .as_f64()
                        .expect("an allocated union alternative count has a Number representation");
                    crate::translation::type_syntax_rules::accept_union_type_count(count)
                });
            if !admitted_fields || !admitted_count {
                return Err(error(
                    "a linked union type has a malformed ordered alternative layout",
                ));
            }
        }
        if value["kind"] == "absence" {
            return Err(error(
                "the linked program contains unsupported type absence",
            ));
        }
        if value["kind"] == "fixed" {
            let width = value["bits"]
                .as_f64()
                .filter(|width| {
                    super::program_numeric_metadata_rules::accept_program_integral_metadata(
                        "fixed",
                        "bits",
                        *width,
                        *width == 0.0 && width.is_sign_negative(),
                    )
                })
                .map_or_else(String::new, |width| width.to_string());
            if !accept_program_machine_integer_width(&width) || !value["signed"].is_boolean() {
                return Err(error(
                    "the linked program contains an invalid machine integer type",
                ));
            }
        } else if value["kind"] == "option"
            && !accept_program_optional_absence(value["absence"].as_str().unwrap_or("both"))
        {
            return Err(error(
                "a linked optional type has an invalid absence policy",
            ));
        }
    }
    Ok(())
}

pub(super) fn validate(program: &Program) -> Result<()> {
    crate::translation::source_const_functions::validate_const_functions(program)
        .map_err(|failure| error(failure.to_string()))?;
    if crate::translation::runtime_shell::validate_program(program)
        .map_err(|failure| error(failure.to_string()))?
        .is_some()
    {
        return Ok(());
    }
    crate::translation::source_pure_unit_calls::validate_source_pure_unit_calls(program)
        .map_err(|failure| error(failure.to_string()))?;
    crate::translation::source_unit_clone_derivations::validate_source_unit_clone_derivations(
        program,
    )
    .map_err(|failure| error(failure.to_string()))?;
    crate::translation::source_empty_options::validate_source_empty_options(program)
        .map_err(|failure| error(failure.to_string()))?;
    crate::translation::source_primitive_owners::validate_source_primitive_owners(program)
        .map_err(|failure| error(failure.to_string()))?;
    crate::translation::owned_local_state::validate_owned_local_state(program)
        .map_err(|failure| error(failure.to_string()))?;
    crate::translation::source_type_namespaces::validate_source_type_namespaces(program)
        .map_err(|failure| error(failure.to_string()))?;
    let data_names = program
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Decl::Data(declaration) => Some(declaration.full_name.as_str()),
            _ => None,
        })
        .collect();
    validate_type_records(
        &serde_json::to_value(program).map_err(|failure| error(failure.to_string()))?,
        &data_names,
        program.source_language.as_str(),
    )?;
    if program.schema_version != 1 {
        return Err(error("the linked value is not a checked typed program"));
    }
    if !program.imports.is_empty() || program.output_threaded || program.aborts_threaded {
        return Err(error(
            "the Rust typed links adapter does not yet recheck imported or threaded programs",
        ));
    }
    let mut items = Vec::new();
    let mut seen = HashSet::new();
    for item in &program.items {
        if let Item::Module { .. } = item {
            items.push(loop_modules::project_module(item, program, &[], &mut seen)?);
            continue;
        }
        let Item::Decl { full_name } = item else {
            return Err(error(
                "the Rust typed links adapter does not yet recheck module items",
            ));
        };
        if !seen.insert(full_name.clone()) {
            return Err(error("the linked declaration item occurs more than once"));
        }
        let declaration = program
            .declaration(full_name)
            .ok_or_else(|| error("the linked declaration item has no declaration"))?;
        if let Decl::Data(data) = declaration {
            if !data.module_path.is_empty() {
                return Err(error(
                    "the Rust typed links adapter does not yet recheck qualified declarations",
                ));
            }
            items.push(declarations::project_data(data, program)?);
            continue;
        }
        let Decl::Fn(function) = declaration else {
            return Err(error(
                "the Rust typed links adapter does not yet recheck theorem declarations",
            ));
        };
        if !function.module_path.is_empty() {
            return Err(error(
                "the Rust typed links adapter does not yet recheck qualified declarations",
            ));
        }
        items.push(loop_modules::project_function(function, program)?);
    }
    if seen.len() != program.declarations.len() {
        return Err(error(
            "the linked declaration mapping and ordered items disagree",
        ));
    }
    let main = program
        .main
        .as_ref()
        .map(|main| -> Result<SMain> {
            let effects = main
                .effects
                .iter()
                .map(|effect| match effect {
                    Effect::Print { expr, span } => Ok(SEffect::Print {
                        expr: expression(expr, program)?,
                        style: ShowStyle::JsConsole,
                        span: *span,
                    }),
                    Effect::Assert { prop, span } => Ok(SEffect::Assert {
                        prop: serde_json::from_value(project_assertion_proposition(
                            serde_json::to_value(prop)
                                .map_err(|failure| error(failure.to_string()))?,
                            program,
                        )?)
                        .map_err(|failure| error(failure.to_string()))?,
                        span: *span,
                    }),
                    Effect::Let {
                        name,
                        value,
                        constant,
                        span,
                        source_pure_unit_call,
                    } => Ok(SEffect::Let {
                        name: name.clone(),
                        ty: Some(project_type(&value.ty, program)?),
                        value: expression(value, program)?,
                        constant: *constant,
                        span: *span,

                        source_pure_unit_call: source_pure_unit_call.clone(),
                    }),
                    _ => Err(error(
                        "the Rust typed links adapter does not yet recheck this main effect",
                    )),
                })
                .collect::<Result<_>>()?;
            Ok(SMain {
                effects,
                span: main.span,
                sequential_async: main.sequential_async,
                source_runtime_worker: main.source_runtime_worker.clone(),
            })
        })
        .transpose()?;
    let surface = SProgram {
        reference_semantics: program.reference_semantics.then(|| true.into()),
        language: program.source_language,
        items,
        main,
        imports: Vec::new(),
        externals: program.externals.clone(),
        source_pure_unit_calls: program.source_pure_unit_calls.clone(),
        source_runtime_shell: program.source_runtime_shell.clone(),
    };
    let mut checked =
        check_projected_program(&surface).map_err(|failure| error(failure.to_string()))?;
    checked.utf16_strings = program.utf16_strings;
    if checked != *program {
        let retained_records =
            serde_json::to_value(program).map_err(|failure| error(failure.to_string()))?;
        let checked_records =
            serde_json::to_value(&checked).map_err(|failure| error(failure.to_string()))?;
        let detail = read_checked_program_difference(
            &retained_records,
            &checked_records,
            "",
            0,
            &mut 100_000,
        )
        .unwrap_or_else(|| {
            "the internal Rust representation differs despite equal serialized fields".to_owned()
        });
        return Err(error(format!(
            "the linked program has inconsistent types, bindings or operation semantics: {detail}",
        )));
    }
    Ok(())
}

// This diagnostic runs only after the unchanged exact checked-IR comparison
// rejects. It never accepts, normalizes or discards a mismatching field.
fn read_checked_program_difference(
    retained: &Value,
    checked: &Value,
    path: &str,
    depth: usize,
    remaining: &mut usize,
) -> Option<String> {
    if depth > 256 || *remaining == 0 {
        return Some(format!("{path}: comparison detail resource limit"));
    }
    *remaining -= 1;
    match (retained, checked) {
        (Value::Object(retained), Value::Object(checked)) => {
            for (field, value) in retained {
                let field_path = format!("{path}/{}", field.replace('~', "~0").replace('/', "~1"));
                let Some(actual) = checked.get(field) else {
                    return Some(format!(
                        "{field_path}: retained field is absent after recheck"
                    ));
                };
                if let Some(difference) = read_checked_program_difference(
                    value,
                    actual,
                    &field_path,
                    depth + 1,
                    remaining,
                ) {
                    return Some(difference);
                }
            }
            checked
                .keys()
                .find(|field| !retained.contains_key(*field))
                .map(|field| {
                    format!(
                        "{path}/{}: recheck introduced a field",
                        field.replace('~', "~0").replace('/', "~1")
                    )
                })
        }
        (Value::Array(retained), Value::Array(checked)) => {
            if retained.len() != checked.len() {
                return Some(format!(
                    "{path}: retained length {} differs from rechecked length {}",
                    retained.len(),
                    checked.len()
                ));
            }
            retained
                .iter()
                .zip(checked)
                .enumerate()
                .find_map(|(index, (value, actual))| {
                    read_checked_program_difference(
                        value,
                        actual,
                        &format!("{path}/{index}"),
                        depth + 1,
                        remaining,
                    )
                })
        }
        _ if retained == checked => None,
        _ => Some(format!(
            "{path}: retained {} differs from rechecked {}",
            read_checked_program_value(retained),
            read_checked_program_value(checked)
        )),
    }
}

fn read_checked_program_value(value: &Value) -> String {
    match value {
        Value::Object(_) => "an object".to_owned(),
        Value::Array(_) => "an array".to_owned(),
        Value::String(text) => format!("{:?}", text.chars().take(96).collect::<String>()),
        _ => value.to_string(),
    }
}
