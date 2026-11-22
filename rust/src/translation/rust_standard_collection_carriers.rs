//! Checked declaration adapters for concrete Rust standard collection carriers.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

use super::Span;
use super::diagnostics::{TranslationError, unsupported};
use super::ir::{Decl, Program};
use super::rust_standard_collection_value_packet::{
    RustStandardCollectionOrigin, ml_string::Text, read_rust_standard_collection_kind,
    read_rust_standard_collection_source_origin_error, resolve_rust_standard_collection_owner,
};
use super::types::Type;

type Result<T> = std::result::Result<T, TranslationError>;

/// The actual checked library declaration that owns a projected source role.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CollectionLibraryOwner {
    model: String,
    entry_path: String,
    source_sha256: String,
    declaration: String,
    role: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CollectionKernelLibrary {
    model: String,
    entry_path: String,
    source_sha256: String,
    kernel_declaration: String,
    packet_declaration: String,
}

/// Concrete source type identity carried by a nominal wrapper over shared state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CollectionCarrier {
    model: String,
    origin: Value,
    key_domain: String,
    value_domain: String,
    pointer_width: u32,
    concrete_identity: String,
    /// Actual shared nominal state type.
    pub(crate) kernel_type: Type,
    kernel_library: CollectionKernelLibrary,
}

#[derive(Deserialize)]
struct SourceTypeOwner<Argument> {
    kind: String,
    path: Vec<String>,
    arguments: Vec<Argument>,
}

#[derive(Deserialize)]
struct RejectedChildOwner {
    kind: String,
    path: Vec<String>,
}

type ScalarOwner = SourceTypeOwner<RejectedChildOwner>;
type NativeScalarOwner = (Vec<(Text, Vec<Text>)>, Text, Vec<Text>);

fn text_list(values: &[String]) -> Vec<Text> {
    values.iter().map(|value| Text::from_text(value)).collect()
}

fn scalar_owner(owner: &ScalarOwner) -> NativeScalarOwner {
    (
        owner
            .arguments
            .iter()
            .map(|child| (Text::from_text(&child.kind), text_list(&child.path)))
            .collect(),
        Text::from_text(&owner.kind),
        text_list(&owner.path),
    )
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScalarOrigin {
    proven: bool,
    wrappers: Vec<String>,
    source_owner: ScalarOwner,
    text_source_model: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CollectionOrigin {
    #[serde(rename = "$")]
    tag: String,
    ordinary: bool,
    proven: bool,
    shadowed: bool,
    type_identity: String,
    resolved_owner: String,
    source_owner: SourceTypeOwner<ScalarOwner>,
    arguments: Vec<ScalarOrigin>,
    source_version_minor: f64,
    pointer_width: f64,
    custom_hasher: bool,
    custom_equality: bool,
}

impl CollectionOrigin {
    fn native(&self) -> RustStandardCollectionOrigin {
        RustStandardCollectionOrigin::RustStandardCollectionOrigin(
            self.ordinary,
            self.proven,
            self.shadowed,
            Text::from_text(&self.type_identity),
            Text::from_text(&self.resolved_owner),
            (
                self.source_owner
                    .arguments
                    .iter()
                    .map(scalar_owner)
                    .collect(),
                Text::from_text(&self.source_owner.kind),
                text_list(&self.source_owner.path),
            ),
            self.arguments
                .iter()
                .map(|argument| {
                    (
                        argument.proven,
                        scalar_owner(&argument.source_owner),
                        argument
                            .text_source_model
                            .as_deref()
                            .map(|model| (Some(Text::from_text(model)),)),
                        text_list(&argument.wrappers),
                    )
                })
                .collect(),
            self.source_version_minor,
            self.pointer_width,
            self.custom_hasher,
            self.custom_equality,
        )
    }
}

fn fail(reason: &str, span: Option<Span>) -> TranslationError {
    unsupported("Rust standard collection source interface", reason, span)
}

fn valid_source_identity(path: &str, sha256: &str) -> bool {
    path.ends_with("/rust-standard-collection-value-packet.js")
        && path.split('/').all(|part| !matches!(part, "" | "." | ".."))
        && !path.contains('\u{005c}')
        && sha256.len() == 64
        && sha256
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn library_owner(declaration: &Decl) -> Option<&CollectionLibraryOwner> {
    match declaration {
        Decl::Data(data) => data.source_standard_collection_library_owner.as_deref(),
        Decl::Fn(function) => function.source_standard_collection_library_owner.as_deref(),
        Decl::Theorem(_) => None,
    }
}

fn check_library_owner(
    declaration: &Decl,
    owner: &CollectionLibraryOwner,
    language: &str,
) -> Result<()> {
    let (name, span) = match declaration {
        Decl::Data(data) => (&data.full_name, data.span),
        Decl::Fn(function) => (&function.full_name, function.span),
        Decl::Theorem(_) => {
            return Err(fail(
                "a theorem cannot own a projected collection declaration",
                None,
            ));
        }
    };
    if language != "Rust"
        || owner.model != "Rust standard collection projected library owner 1"
        || owner.declaration != *name
        || !valid_source_identity(&owner.entry_path, &owner.source_sha256)
    {
        return Err(fail(
            "standard collection library owner must bind its actual checked declaration and source identity",
            span,
        ));
    }
    let valid = match (owner.role.as_str(), declaration) {
        ("state", Decl::Data(data)) => matches!(data.ctors.as_slice(), [constructor]
            if constructor.name == "rustStandardCollectionState"
                && constructor.fields.iter().map(|field| field.name.as_str()).eq(["origin", "root", "keyDomain", "valueDomain", "pointerWidth", "revision", "entries"])),
        ("equality" | "ordering", Decl::Fn(function)) => {
            matches!(function.params.as_slice(), [left, right, call, budget]
            if [&left.ty, &right.ty, &call.ty].iter().all(|ty| matches!(ty, Type::Data { .. } | Type::Record { .. }))
                && left.ty == right.ty && budget.ty == Type::Float && matches!(function.ret, Type::Data { .. }))
        }
        ("packet" | "kernel", Decl::Fn(function)) => {
            let parameters = function.params.as_slice();
            let interface = matches!(parameters, [state, operation, arguments, call, ..]
                if matches!(state.ty, Type::Data { .. }) && operation.ty == Type::String
                    && matches!(arguments.ty, Type::Array { .. }) && matches!(call.ty, Type::Data { .. } | Type::Record { .. }));
            interface
                && matches!(function.ret, Type::Data { .. })
                && if owner.role == "packet" {
                    parameters.len() == 5 && parameters[4].ty == Type::String
                } else {
                    parameters.len() == 4
                }
        }
        _ => false,
    };
    if !valid {
        return Err(fail(
            "the collection library owner retains its actual complete checked declaration interface",
            span,
        ));
    }
    Ok(())
}

pub(crate) fn check_program(program: &Program) -> Result<()> {
    let language = program.source_language.as_str();
    for declaration in &program.declarations {
        if let Some(owner) = library_owner(declaration) {
            check_library_owner(declaration, owner, language)?;
        }
        let Decl::Data(data) = declaration else {
            continue;
        };
        let Some(carrier) = &data.source_standard_collection_carrier else {
            continue;
        };
        let origin = CollectionOrigin::deserialize(&carrier.origin).map_err(|_| {
            fail(
                "collection carrier retains its actual complete source origin",
                data.span,
            )
        })?;
        let library = &carrier.kernel_library;
        if language != "Rust"
            || carrier.model != "Rust closed standard collections 1"
            || carrier.pointer_width != 64
            || origin.tag != "rustStandardCollectionOrigin"
            || library.model != "Rust standard collection kernel library 1"
            || !valid_source_identity(&library.entry_path, &library.source_sha256)
        {
            return Err(fail(
                "collection carrier requires its actual complete Rust specialization facet",
                data.span,
            ));
        }
        let reason = read_rust_standard_collection_source_origin_error(
            origin.native(),
            &Text::from_text(&carrier.key_domain),
            &Text::from_text(&carrier.value_domain),
            f64::from(carrier.pointer_width),
        );
        if !reason.0.is_empty() {
            return Err(fail(&reason.to_string(), data.span));
        }
        let owner = resolve_rust_standard_collection_owner(
            &Text::from_text(&origin.resolved_owner),
            true,
            false,
        );
        let set = read_rust_standard_collection_kind(&owner) == Text::from_text("set");
        let owner = owner.to_string();
        let domains = if set {
            carrier.key_domain.clone()
        } else {
            format!("{},{}", carrier.key_domain, carrier.value_domain)
        };
        let identity = format!("{owner}<{domains}>@{}", carrier.pointer_width);
        let base = format!(
            "RustStandardCollection{}_{}_{}_{}",
            owner.rsplit("::").next().unwrap_or_default(),
            carrier.key_domain,
            carrier.value_domain,
            carrier.pointer_width
        );
        let suffix = data
            .name
            .strip_prefix(&base)
            .and_then(|suffix| suffix.strip_prefix('_'));
        if carrier.concrete_identity != identity
            || origin.type_identity != identity
            || !(data.name == base
                || suffix.is_some_and(|suffix| {
                    !suffix.is_empty()
                        && !suffix.starts_with('0')
                        && suffix.bytes().all(|byte| byte.is_ascii_digit())
                }))
        {
            return Err(fail(
                "collection nominal identity preserves its canonical owner key/value domains and target width",
                data.span,
            ));
        }
        if carrier.kernel_type
            != (Type::Data {
                name: library.kernel_declaration.clone(),
            })
            || !matches!(data.ctors.as_slice(), [constructor] if constructor.name == "CollectionState"
                && matches!(constructor.fields.as_slice(), [field] if field.name == "state" && field.ty == carrier.kernel_type))
        {
            return Err(fail(
                "collection specialization retains its actual generic shared kernel state type",
                data.span,
            ));
        }
        for (name, role) in [
            (&library.kernel_declaration, "state"),
            (&library.packet_declaration, "packet"),
        ] {
            let target = program.declaration(name);
            let owner = target.and_then(library_owner);
            if !owner.is_some_and(|owner| {
                owner.role == role
                    && owner.declaration == *name
                    && owner.entry_path == library.entry_path
                    && owner.source_sha256 == library.source_sha256
            }) || (role == "packet"
                && !matches!(target, Some(Decl::Fn(function))
                    if function.params.first().is_some_and(|param| param.ty == carrier.kernel_type)))
            {
                return Err(fail(
                    "collection specialization requires its actual kernel and packet declarations",
                    data.span,
                ));
            }
        }
    }
    Ok(())
}

pub(crate) fn check_observation<'a>(
    ty: &'a Type,
    resolve: impl Fn(&str) -> Option<&'a Decl>,
    operation: &str,
    span: Option<Span>,
) -> Result<()> {
    let mut pending = vec![ty];
    let mut seen = HashSet::new();
    let mut remaining = 65536_u32;
    while let Some(current) = pending.pop() {
        remaining = remaining.checked_sub(1).ok_or_else(|| {
            fail(
                "collection observation exceeds its actual complete type traversal budget",
                span,
            )
        })?;
        match current {
            Type::Data { name } if seen.insert(name.as_str()) => {
                if let Some(Decl::Data(data)) = resolve(name) {
                    if data.source_standard_collection_carrier.is_some() {
                        return Err(fail(
                            &format!(
                                "standard collection {operation} requires its separate actual observation semantics"
                            ),
                            span,
                        ));
                    }
                    pending.extend(
                        data.ctors
                            .iter()
                            .flat_map(|ctor| &ctor.fields)
                            .map(|field| &field.ty),
                    );
                }
            }
            Type::Array { .. }
            | Type::Option { .. }
            | Type::Set { .. }
            | Type::Tuple { .. }
            | Type::Record { .. }
            | Type::Map { .. } => pending.extend(current.children()),
            _ => {}
        }
    }
    Ok(())
}
