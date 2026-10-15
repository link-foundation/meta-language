//! Adapt the supported typed IR to the existing checker, preserving policies.
use super::projection::{project as expression, project_type};
use super::{Result, error, typed_program_links_schema};
use crate::translation::check::check_program;
use crate::translation::ir::{Decl, Effect, Item, Program};
use crate::translation::primitive_value_rules::{
    accept_linked_text_conversion_kind, read_number_display_refusal,
};
use crate::translation::surface::{SEffect, SMain, SProgram, ShowStyle};
use crate::translation::typed_program_link_rules::{
    accept_program_division_rounding, accept_program_expression_operation,
    accept_program_machine_integer_width, accept_program_optional_absence,
};
use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;
use std::sync::OnceLock;

mod declarations;
mod loop_modules;

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
        {
            return Err(error(
                "the linked program contains an unrecognized expression operation",
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
        for child in fields.values() {
            validate_type_records(child, data_names, source_language)?;
        }
    } else if let Some(values) = value.as_array() {
        for child in values {
            validate_type_records(child, data_names, source_language)?;
        }
    }
    Ok(())
}

pub(super) fn validate(program: &Program) -> Result<()> {
    crate::translation::source_const_functions::validate_const_functions(program)
        .map_err(|failure| error(failure.to_string()))?;
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
    if !program.imports.is_empty()
        || !program.externals.is_empty()
        || program.output_threaded
        || program.aborts_threaded
    {
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
            })
        })
        .transpose()?;
    let surface = SProgram {
        language: program.source_language,
        items,
        main,
        imports: Vec::new(),
        externals: Vec::new(),
        source_pure_unit_calls: program.source_pure_unit_calls.clone(),
    };
    let checked = check_program(&surface).map_err(|failure| error(failure.to_string()))?;
    if checked != *program {
        return Err(error(
            "the linked program has inconsistent types, bindings or operation semantics",
        ));
    }
    Ok(())
}
