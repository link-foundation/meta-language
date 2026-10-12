//! Adapt the supported typed IR to the existing checker, preserving policies.
use super::projection::project as expression;
use super::{Result, error, typed_program_links_schema};
use crate::translation::check::check_program;
use crate::translation::ir::{Decl, Effect, Item, Program};
use crate::translation::surface::{SEffect, SFn, SItem, SMain, SParam, SProgram, ShowStyle};
use crate::translation::typed_program_link_rules::{
    accept_program_division_rounding, accept_program_expression_operation,
    accept_program_machine_integer_width, accept_program_optional_absence,
};
use regex::Regex;
use serde_json::Value;
use std::sync::OnceLock;

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

fn validate_type_records(value: &Value) -> Result<()> {
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
            Some("named" | "literal" | "hypothesis" | "output" | "data")
        ) {
            return Err(error(
                "the Rust typed links adapter does not yet recheck unresolved or threaded value types",
            ));
        }
        if value["kind"] == "fixed" {
            let width = value["bits"]
                .as_u64()
                .map(|width| width.to_string())
                .unwrap_or_default();
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
            validate_type_records(child)?;
        }
    } else if let Some(values) = value.as_array() {
        for child in values {
            validate_type_records(child)?;
        }
    }
    Ok(())
}

pub(super) fn validate(program: &Program) -> Result<()> {
    validate_type_records(
        &serde_json::to_value(program).map_err(|failure| error(failure.to_string()))?,
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
    for item in &program.items {
        let Item::Decl { full_name } = item else {
            return Err(error(
                "the Rust typed links adapter does not yet recheck module items",
            ));
        };
        let Some(Decl::Fn(function)) = program.declaration(full_name) else {
            return Err(error(
                "the Rust typed links adapter currently rechecks function declarations",
            ));
        };
        if !function.module_path.is_empty() {
            return Err(error(
                "the Rust typed links adapter does not yet recheck qualified declarations",
            ));
        }
        items.push(SItem::Fn(SFn {
            name: function.name.clone(),
            params: function
                .params
                .iter()
                .map(|parameter| SParam {
                    name: parameter.name.clone(),
                    ty: Some(parameter.ty.clone()),
                    default_value: parameter.default_value.clone(),
                    guard: parameter.guard.clone(),
                    span: None,
                    rocq_type: None,
                })
                .collect(),
            ret: Some(function.ret.clone()),
            body: expression(&function.body)?,
            span: function.span,
            generated: function.generated,
        }));
    }
    if items.len() != program.declarations.len() {
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
                        expr: expression(expr)?,
                        style: ShowStyle::JsConsole,
                        span: *span,
                    }),
                    Effect::Let {
                        name,
                        value,
                        constant,
                        span,
                    } => Ok(SEffect::Let {
                        name: name.clone(),
                        ty: Some(value.ty.clone()),
                        value: expression(value)?,
                        constant: *constant,
                        span: *span,
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
    };
    let checked = check_program(&surface).map_err(|failure| error(failure.to_string()))?;
    if checked != *program {
        return Err(error(
            "the linked program has inconsistent types, bindings or operation semantics",
        ));
    }
    Ok(())
}
