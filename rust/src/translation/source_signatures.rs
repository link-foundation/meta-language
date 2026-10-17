//! Owned, opt-in facts from actual Rust function source headers.
//! Value type checking remains in the existing checker; pure owner and
//! wrapper decisions are translated from the shared JavaScript policy.
use super::diagnostics::{Result, type_error};
use super::rust::indirection_rules::accept_rust_source_lifetime_binding;
use super::source_signature_rules::{
    accept_rust_source_signature_model, accept_rust_source_signature_owner_kind,
    accept_rust_source_signature_primitive, accept_rust_source_signature_wrapper,
};
use super::types::Type;
use serde::{Deserialize, Serialize};

/// A resolved owner of a source type, including owned generic children.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceOwner {
    /// Primitive, declaration, standard library, tuple, unit or slice.
    pub kind: String,
    /// Actual resolved owner segments, never a source text envelope.
    pub path: Vec<String>,
    /// Parsed child type owners in source argument order.
    pub arguments: Vec<Self>,
}

/// One owned source parameter or result type and its erased qualifiers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceTypeSignature {
    /// The value type resolved by the existing checker.
    #[serde(rename = "type")]
    pub ty: Type,
    /// Pointer layers in their actual source order.
    pub wrappers: Vec<String>,
    /// One named or elided lifetime per reference layer.
    pub lifetimes: Vec<Option<String>>,
    /// Actual header owner, including resolved declaration identities.
    pub owner: SourceOwner,
}

/// A source function signature, owned by its checked declaration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceSignature {
    /// Explicit selected source interpretation.
    pub model: String,
    /// The source header language.
    pub language: String,
    /// Actual parameter positions, matched to the function parameters.
    pub parameters: Vec<SourceTypeSignature>,
    /// Actual result position, matched to the function result.
    pub result: SourceTypeSignature,
    /// Parsed function lifetime binders in declaration order.
    pub lifetime_bindings: Vec<String>,
}

fn fail(message: &str) -> super::diagnostics::TranslationError {
    type_error(message, None)
}

fn validate_owner(owner: &SourceOwner, ty: &Type, depth: usize, visits: &mut usize) -> Result<()> {
    *visits += 1;
    if *visits > 100_000 || depth > 256 {
        return Err(fail(
            "an owned Rust source signature exceeds its resource limit",
        ));
    }
    if !accept_rust_source_signature_owner_kind(&owner.kind)
        || owner.path.iter().any(String::is_empty)
    {
        return Err(fail("an owned Rust source type has an invalid owner"));
    }
    let path = owner.path.join("::");
    match owner.kind.as_str() {
        "primitive" => {
            let (bits, signed) = match ty {
                Type::Fixed { bits, signed } => (bits.to_string(), *signed),
                _ => (String::new(), false),
            };
            if owner.path.len() != 1
                || !owner.arguments.is_empty()
                || !accept_rust_source_signature_primitive(ty.kind(), &path, &bits, signed)
            {
                return Err(fail(
                    "an owned Rust source primitive differs from its value type",
                ));
            }
        }
        "declaration" => {
            if owner.path.is_empty()
                || !owner.arguments.is_empty()
                || !matches!(ty, Type::Data {name} if *name == owner.path.join("."))
            {
                return Err(fail(
                    "an owned Rust source declaration differs from its value type",
                ));
            }
        }
        "unit" => {
            if !owner.path.is_empty() || !owner.arguments.is_empty() || *ty != Type::Unit {
                return Err(fail(
                    "an owned Rust source unit differs from its value type",
                ));
            }
        }
        "tuple" => {
            let Type::Tuple { elements } = ty else {
                return Err(fail(
                    "an owned Rust source tuple differs from its value type",
                ));
            };
            if !owner.path.is_empty() || owner.arguments.len() != elements.len() {
                return Err(fail(
                    "an owned Rust source tuple differs from its value type",
                ));
            }
            for (child, element) in owner.arguments.iter().zip(elements) {
                validate_owner(child, element, depth + 1, visits)?;
            }
        }
        "slice" | "library" if owner.kind == "slice" || path == "std::vec::Vec" => {
            let Type::Array { element, .. } = ty else {
                return Err(fail(
                    "an owned Rust source array differs from its value type",
                ));
            };
            if (owner.kind == "slice" && !owner.path.is_empty()) || owner.arguments.len() != 1 {
                return Err(fail(
                    "an owned Rust source array differs from its value type",
                ));
            }
            validate_owner(&owner.arguments[0], element, depth + 1, visits)?;
        }
        "library" if path == "std::boxed::Box" || path == "alloc::boxed::Box" => {
            if owner.arguments.len() != 1 {
                return Err(fail("an owned Rust source box has invalid arguments"));
            }
            validate_owner(&owner.arguments[0], ty, depth + 1, visits)?;
        }
        "library" if path == "std::string::String" => {
            if !owner.arguments.is_empty() || *ty != Type::String {
                return Err(fail(
                    "an owned Rust source string differs from its value type",
                ));
            }
        }
        _ => return Err(fail("an owned Rust source type has an unsupported owner")),
    }
    Ok(())
}

/// Check ownership and positional type agreement without source or provenance.
///
/// # Errors
/// Refuses mismatched headers, unknown owners and unsupported qualifier facts.
pub fn validate_source_signature(
    signature: &SourceSignature,
    parameters: &[Type],
    result: &Type,
    language: &str,
) -> Result<()> {
    if signature.model.is_empty()
        || !accept_rust_source_signature_model(&signature.model)
        || signature.language != "Rust"
        || language != "Rust"
    {
        return Err(fail(
            "an owned Rust source signature has an unsupported model or language",
        ));
    }
    let unique: std::collections::HashSet<_> = signature.lifetime_bindings.iter().collect();
    if signature.parameters.len() != parameters.len()
        || unique.len() != signature.lifetime_bindings.len()
        || signature
            .lifetime_bindings
            .iter()
            .any(|name| !accept_rust_source_lifetime_binding(name, false, false))
    {
        return Err(fail(
            "an owned Rust source signature has inconsistent parameter or lifetime bindings",
        ));
    }
    let mut visits = 0;
    for (position, expected) in signature
        .parameters
        .iter()
        .zip(parameters)
        .chain(std::iter::once((&signature.result, result)))
    {
        if !position.ty.same(expected) {
            return Err(fail(
                "an owned Rust source signature differs from its function type",
            ));
        }
        if position
            .wrappers
            .iter()
            .any(|wrapper| !accept_rust_source_signature_wrapper(wrapper))
            || position.lifetimes.len()
                != position
                    .wrappers
                    .iter()
                    .filter(|wrapper| *wrapper == "reference")
                    .count()
            || position.lifetimes.iter().flatten().any(|lifetime| {
                lifetime != "static"
                    && lifetime != "_"
                    && !signature.lifetime_bindings.contains(lifetime)
            })
        {
            return Err(fail(
                "an owned Rust source type has inconsistent wrapper or lifetime facts",
            ));
        }
        validate_owner(&position.owner, &position.ty, 0, &mut visits)?;
        let mut boxed = 0;
        let mut leaf = &position.owner;
        while leaf.kind == "library"
            && ["std::boxed::Box", "alloc::boxed::Box"].contains(&leaf.path.join("::").as_str())
        {
            boxed += 1;
            leaf = &leaf.arguments[0];
        }
        if boxed
            != position
                .wrappers
                .iter()
                .filter(|wrapper| *wrapper == "box")
                .count()
            || ((leaf.kind == "slice" || (leaf.kind == "primitive" && leaf.path[0] == "str"))
                && !position
                    .wrappers
                    .iter()
                    .any(|wrapper| wrapper == "reference"))
        {
            return Err(fail(
                "an owned Rust source type has inconsistent owner and wrapper facts",
            ));
        }
    }
    Ok(())
}
