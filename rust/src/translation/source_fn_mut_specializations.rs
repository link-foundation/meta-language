//! Checked ABI for a specialization of an actual Rust mutable callback declaration.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{
    diagnostics::{Result, type_error},
    ir::{DataDecl, FnDecl, Param},
    rust::source_versions,
    rust_fnmut_source_abi_rules::read_rust_fn_mut_source_packet_error,
    rust_fnmut_source_declaration_rules::read_rust_fn_mut_source_model_error,
    source_loop_folds::{SourceFoldSpan as SourceRange, SourceStateBinder},
    source_signatures::{
        SourceSignature, SourceTypeSignature, resolve_source_position,
        validate_source_signature_with_data,
    },
    surface::SFn,
    types::Type,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RustFnMutSpecialization {
    pub model: String,
    pub language: String,
    pub source_version: String,
    pub declaration: SourceDeclaration,
    pub type_arguments: Vec<TypeArgument>,
    pub callback: Callback,
    pub parameters: Vec<Parameter>,
    pub result: SourceTypeSignature,
    pub packet: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceDeclaration {
    pub path: Vec<String>,
    pub range: SourceRange,
    pub body_range: SourceRange,
    pub generics: Vec<GenericBinder>,
    pub formal_count: usize,
    pub callback: OriginalCallback,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenericBinder {
    pub name: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OriginalCallback {
    pub index: usize,
    pub name: String,
    pub mode: String,
    pub mutable: bool,
    pub binder: SourceRange,
    pub type_range: SourceRange,
    pub parameter_ranges: Vec<SourceRange>,
    pub result_range: SourceRange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    pub mode: String,
    pub position: SourceTypeSignature,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeArgument {
    pub name: String,
    pub position: SourceTypeSignature,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Callback {
    pub parameter: usize,
    pub parameters: Vec<SourceTypeSignature>,
    pub result: SourceTypeSignature,
    pub captures: Vec<Capture>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capture {
    pub parameter: usize,
    pub binder: SourceStateBinder,
    pub position: SourceTypeSignature,
}

fn fail(reason: &str) -> super::diagnostics::TranslationError {
    type_error(reason, None)
}

fn source_number(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}

fn is_source_integer(value: usize) -> bool {
    u64::try_from(value).is_ok_and(|value| value <= 9_007_199_254_740_991)
}

fn valid_range(range: &SourceRange) -> bool {
    range.start < range.end && is_source_integer(range.end)
}

impl SourceDeclaration {
    fn validate(&self) -> Result<()> {
        if self.path.is_empty()
            || self.path.len() > 256
            || self.path.iter().any(String::is_empty)
            || !valid_range(&self.range)
            || !valid_range(&self.body_range)
            || self.body_range.start <= self.range.start
            || self.body_range.end != self.range.end
            || !(1..=256).contains(&self.formal_count)
            || self.generics.len() > 32
        {
            return Err(fail(
                "a source FnMut specialization retains its actual complete original declaration owner and body range",
            ));
        }
        let mut names = HashSet::new();
        for value in &self.generics {
            if value.name.is_empty()
                || !names.insert(&value.name)
                || value.start <= self.range.start
                || value.start >= value.end
                || value.end >= self.body_range.start
            {
                return Err(fail(
                    "generic type binders retain their distinct actual source token origins",
                ));
            }
        }
        let callback = &self.callback;
        if callback.index >= self.formal_count
            || callback.name.is_empty()
            || !matches!(callback.mode.as_str(), "value" | "exclusive")
            || callback.mode == "value" && !callback.mutable
            || !valid_range(&callback.binder)
            || !valid_range(&callback.type_range)
            || !valid_range(&callback.result_range)
            || callback.parameter_ranges.len() > 256
            || callback.binder.start <= self.range.start
            || callback.binder.end > callback.type_range.start
            || callback.type_range.end >= self.body_range.start
            || callback.result_range.start <= callback.type_range.start
            || callback.result_range.end != callback.type_range.end
            || callback.parameter_ranges.iter().any(|range| {
                !valid_range(range)
                    || range.start <= callback.type_range.start
                    || range.end >= callback.result_range.start
            })
        {
            return Err(fail(
                "the original FnMut formal retains its actual mutable source binder, trait mode and complete header positions",
            ));
        }
        Ok(())
    }
}

pub(super) fn check_surface<'data>(
    facet: &RustFnMutSpecialization,
    function: &SFn,
    params: &[Param],
    ret: &Type,
    language: &str,
    resolve_type: impl FnMut(&Type) -> Result<Type>,
    resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<RustFnMutSpecialization> {
    if function.source_signature.is_some()
        || function.source_state_transform.is_some()
        || function.source_vector_mutation.is_some()
    {
        return Err(fail(
            "a source FnMut specialization retains its exact independent Rust interface",
        ));
    }
    check_interface(facet, params, ret, language, resolve_type, resolve_data)
}

pub(crate) fn validate_checked<'data>(
    facet: &RustFnMutSpecialization,
    function: &FnDecl,
    language: &str,
    resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<()> {
    if function.source_signature.is_some()
        || function.source_state_transform.is_some()
        || function.source_vector_mutation.is_some()
    {
        return Err(fail(
            "a source FnMut specialization retains its exact independent Rust interface",
        ));
    }
    let checked = check_interface(
        facet,
        &function.params,
        &function.ret,
        language,
        |ty| Ok(ty.clone()),
        resolve_data,
    )?;
    if &checked != facet {
        return Err(fail(
            "the linked FnMut specialization retains its exact checked source positions",
        ));
    }
    Ok(())
}

fn check_interface<'data>(
    facet: &RustFnMutSpecialization,
    params: &[Param],
    ret: &Type,
    language: &str,
    mut resolve_type: impl FnMut(&Type) -> Result<Type>,
    mut resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<RustFnMutSpecialization> {
    if language != "Rust" || facet.language != "Rust" {
        return Err(fail(
            "a source FnMut specialization retains its exact independent Rust interface",
        ));
    }
    let version = source_versions::canonical(&facet.source_version)?;
    let model_error =
        read_rust_fn_mut_source_model_error(&facet.model, source_versions::minor(&version)?);
    if !model_error.is_empty() {
        return Err(fail(&model_error));
    }
    if version != facet.source_version {
        return Err(fail(
            "a source FnMut specialization retains its actual covered canonical source version",
        ));
    }
    let declaration = &facet.declaration;
    declaration.validate()?;
    let callback = &facet.callback;
    if facet.parameters.len() != params.len()
        || params.len() > 512
        || facet.type_arguments.len() != declaration.generics.len()
        || callback.parameter != declaration.callback.index
        || callback.parameters.len() != declaration.callback.parameter_ranges.len()
        || callback.captures.len() > 256
        || facet.packet.len() > 512
    {
        return Err(fail(
            "a concrete FnMut specialization retains its complete expanded formal and generic inventories",
        ));
    }

    let modes = facet
        .parameters
        .iter()
        .map(|parameter| parameter.mode.clone())
        .collect::<Vec<_>>();
    let packet = facet
        .packet
        .iter()
        .copied()
        .map(source_number)
        .collect::<Vec<_>>();
    let packet_error = read_rust_fn_mut_source_packet_error(
        &modes,
        source_number(declaration.formal_count),
        source_number(callback.parameter),
        source_number(callback.captures.len()),
        &packet,
    );
    if !packet_error.is_empty() {
        return Err(fail(&packet_error));
    }
    let Type::Tuple { elements } = ret else {
        return Err(fail(
            "a FnMut completion publishes the original result then every actual exclusive environment root",
        ));
    };
    if elements.len() != packet.len() + 1 {
        return Err(fail(
            "a FnMut completion publishes the original result then every actual exclusive environment root",
        ));
    }
    for (&formal, output) in facet.packet.iter().zip(&elements[1..]) {
        if &params[formal].ty != output {
            return Err(fail(
                "a concrete FnMut writeback must retain its actual formal type",
            ));
        }
    }
    let Type::Function {
        params: inputs,
        ret: output,
    } = &params[callback.parameter].ty
    else {
        return Err(fail(
            "the concrete callback function carries its actual result/state packet interface",
        ));
    };
    let Type::Tuple { elements: outputs } = output.as_ref() else {
        return Err(fail(
            "the concrete callback function carries its actual result/state packet interface",
        ));
    };
    let callback_arity = callback.parameters.len();
    if inputs.len() != callback_arity + callback.captures.len()
        || outputs.len() != callback.captures.len() + 1
    {
        return Err(fail(
            "the concrete callback function carries its actual result/state packet interface",
        ));
    }

    let mut checked = facet.clone();
    let mut positions = |parameters: Vec<SourceTypeSignature>,
                         types: &[Type],
                         result: SourceTypeSignature,
                         ret: &Type|
     -> Result<_> {
        let mut signature = SourceSignature {
            model: "Rust source function signature 2".to_owned(),
            language: "Rust".to_owned(),
            parameters,
            result,
            lifetime_bindings: Vec::new(),
        };
        for position in signature
            .parameters
            .iter_mut()
            .chain(std::iter::once(&mut signature.result))
        {
            resolve_source_position(position, &mut resolve_type)?;
        }
        validate_source_signature_with_data(
            &signature,
            types,
            ret,
            language,
            &mut resolve_type,
            &mut resolve_data,
        )?;
        Ok(signature)
    };
    let signature = positions(
        facet
            .parameters
            .iter()
            .map(|row| row.position.clone())
            .collect(),
        &params
            .iter()
            .map(|parameter| parameter.ty.clone())
            .collect::<Vec<_>>(),
        facet.result.clone(),
        &elements[0],
    )?;
    let callback_signature = positions(
        callback.parameters.clone(),
        &inputs[..callback_arity],
        callback.result.clone(),
        &outputs[0],
    )?;
    if !signature.result.wrappers.is_empty() || !callback_signature.result.wrappers.is_empty() {
        return Err(fail(
            "this source FnMut specialization returns owned results and cannot release escaping source references",
        ));
    }
    for (parameter, position) in checked.parameters.iter_mut().zip(signature.parameters) {
        parameter.position = position;
    }
    checked.result = signature.result;
    checked.callback.parameters = callback_signature.parameters;
    checked.callback.result = callback_signature.result;

    let mut captured = HashSet::new();
    for (index, (row, output)) in checked
        .callback
        .captures
        .iter_mut()
        .zip(&outputs[1..])
        .enumerate()
    {
        let input = &inputs[callback_arity + index];
        if row.parameter != declaration.formal_count + index
            || row.binder.name.is_empty()
            || row.binder.identity.is_empty()
            || row.binder.start >= row.binder.end
            || !is_source_integer(row.binder.end)
            || !captured.insert(&row.binder.identity)
            || input != &params[row.parameter].ty
            || input != output
        {
            return Err(fail(
                "the callback state ABI retains distinct actual captured lexical roots and their exact packet positions",
            ));
        }
        let signature = positions(
            vec![row.position.clone()],
            std::slice::from_ref(input),
            row.position.clone(),
            input,
        )?;
        row.position = signature
            .parameters
            .into_iter()
            .next()
            .expect("one checked capture position");
        if !row.position.wrappers.is_empty() || !row.position.lifetimes.is_empty() {
            return Err(fail(
                "a mutable callback capture is its direct referent state, not an escaping stored reference",
            ));
        }
    }
    for (argument, binder) in checked.type_arguments.iter_mut().zip(&declaration.generics) {
        if argument.name != binder.name {
            return Err(fail(
                "the concrete type argument inventory follows its actual generic binder order",
            ));
        }
        let expected = resolve_type(&argument.position.ty)?;
        let mut signature = SourceSignature {
            model: "Rust source function signature 2".to_owned(),
            language: "Rust".to_owned(),
            parameters: vec![argument.position.clone()],
            result: argument.position.clone(),
            lifetime_bindings: Vec::new(),
        };
        resolve_source_position(&mut signature.parameters[0], &mut resolve_type)?;
        resolve_source_position(&mut signature.result, &mut resolve_type)?;
        validate_source_signature_with_data(
            &signature,
            std::slice::from_ref(&expected),
            &expected,
            language,
            &mut resolve_type,
            &mut resolve_data,
        )?;
        if !signature.result.wrappers.is_empty() {
            return Err(fail(
                "this closed generic FnMut profile retains direct concrete type arguments",
            ));
        }
        argument.position = signature.parameters.remove(0);
    }
    Ok(checked)
}

include!("../../tests/internal/translation/source_fn_mut_specializations.rs");
