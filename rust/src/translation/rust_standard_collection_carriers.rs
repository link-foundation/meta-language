//! Checked declaration adapters for concrete Rust standard collection carriers.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

use super::Span;
use super::diagnostics::{TranslationError, unsupported};
use super::ir::{Decl, Program};
use super::rust_standard_collection_library::RustStandardCollectionSourceLibrary;
use super::rust_standard_collection_value_packet::{
    RustStandardCollectionOrigin, ml_string::Text, read_rust_standard_collection_kind,
    read_rust_standard_collection_source_origin_error, resolve_rust_standard_collection_owner,
};
use super::source_signatures::SourceOwner;
use super::surface::{SCtor, SData, SExpr, SField, SItem, SProgram};
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
    pub(crate) origin: Value,
    key_domain: String,
    value_domain: String,
    pointer_width: u32,
    pub(crate) concrete_identity: String,
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

pub(crate) trait CollectionSourceDeclarations {
    fn program(&self) -> &Program;
    fn source_program(&self) -> &SProgram;
    fn callable_names(&self) -> [&str; 3];
    fn state_and_ordering_names(&self) -> (&str, &str);
    fn entry_path(&self) -> &'static str;
    fn source_sha256(&self) -> &str;
    fn invalid(&self, reason: SExpr) -> SExpr;
    fn resource(&self, state: SExpr, reason: SExpr) -> SExpr;
}

pub(crate) fn create_source_carrier(
    library: &dyn CollectionSourceDeclarations,
    name: String,
    owner: &str,
    domains: &[String],
    source_version_minor: f64,
    span: Option<Span>,
) -> Result<(SData, SourceOwner)> {
    let set = read_rust_standard_collection_kind(&Text::from_text(owner)) == Text::from_text("set");
    let (key, value) = match domains {
        [key] if set => (key.as_str(), "unit"),
        [key, value] if !set => (key.as_str(), value.as_str()),
        _ => {
            return Err(fail(
                "collection carriers require their complete concrete source domains",
                span,
            ));
        }
    };
    let arguments: Vec<_> = domains
        .iter()
        .map(|domain| {
            let (kind, path) = if domain == "String" {
                (
                    "library",
                    vec!["std".to_owned(), "string".to_owned(), domain.clone()],
                )
            } else {
                ("primitive", vec![domain.clone()])
            };
            SourceOwner {
                kind: kind.to_owned(),
                path,
                arguments: Vec::new(),
            }
        })
        .collect();
    let scalar_facts: Vec<_> = domains
        .iter()
        .zip(&arguments)
        .map(|(domain, owner)| {
            let mut fact =
                serde_json::json!({"proven": true, "wrappers": [], "sourceOwner": owner});
            if domain == "String" {
                fact["textSourceModel"] = Value::String("owned string".to_owned());
            }
            fact
        })
        .collect();
    let source_owner = SourceOwner {
        kind: "library".to_owned(),
        path: owner.split("::").map(str::to_owned).collect(),
        arguments,
    };
    let identity = format!("{owner}<{}>@64", domains.join(","));
    let origin = serde_json::json!({
        "$": "rustStandardCollectionOrigin", "ordinary": true, "proven": true,
        "shadowed": false, "typeIdentity": identity, "resolvedOwner": owner,
        "sourceOwner": source_owner, "arguments": scalar_facts, "sourceVersionMinor": source_version_minor,
        "pointerWidth": 64, "customHasher": false, "customEquality": false
    });
    let admitted = CollectionOrigin::deserialize(&origin).map_err(|_| {
        fail(
            "collection source origin cannot be represented completely",
            span,
        )
    })?;
    let reason = read_rust_standard_collection_source_origin_error(
        admitted.native(),
        &Text::from_text(key),
        &Text::from_text(value),
        64.0,
    );
    if !reason.0.is_empty() {
        return Err(fail(&reason.to_string(), span));
    }
    let (state, _) = library.state_and_ordering_names();
    let [packet, _, _] = library.callable_names();
    let state_type = Type::Named {
        path: std::iter::once("crate".to_owned())
            .chain(state.split('.').map(str::to_owned))
            .collect(),
        span: None,
    };
    let facet = Box::new(CollectionCarrier {
        model: "Rust closed standard collections 1".to_owned(),
        origin,
        key_domain: key.to_owned(),
        value_domain: value.to_owned(),
        pointer_width: 64,
        concrete_identity: identity,
        kernel_type: state_type.clone(),
        kernel_library: CollectionKernelLibrary {
            model: "Rust standard collection kernel library 1".to_owned(),
            entry_path: library.entry_path().to_owned(),
            source_sha256: library.source_sha256().to_owned(),
            kernel_declaration: state.to_owned(),
            packet_declaration: packet.to_owned(),
        },
    });
    Ok((
        SData {
            name,
            ctors: vec![SCtor {
                name: "CollectionState".to_owned(),
                fields: vec![SField {
                    name: Some("state".to_owned()),
                    ty: state_type,
                    span: None,
                    optional: false,
                    rocq_type: None,
                }],
            }],
            span,
            generated: false,
            source_standard_collection_library_owner: None,
            source_standard_collection_carrier: Some(facet),
            source_unit_clone_derivation: None,
            source_field_signatures: None,
        },
        source_owner,
    ))
}

impl<Invalid, Resource> CollectionSourceDeclarations
    for RustStandardCollectionSourceLibrary<Invalid, Resource>
where
    Invalid: Fn(SExpr) -> SExpr,
    Resource: Fn(SExpr, SExpr) -> SExpr,
{
    fn program(&self) -> &Program {
        self.program()
    }

    fn source_program(&self) -> &SProgram {
        self.source_program()
    }

    fn callable_names(&self) -> [&str; 3] {
        self.callable_names()
    }

    fn state_and_ordering_names(&self) -> (&str, &str) {
        self.state_and_ordering_names()
    }

    fn entry_path(&self) -> &'static str {
        self.entry_path()
    }

    fn source_sha256(&self) -> &str {
        self.source_sha256()
    }

    fn invalid(&self, reason: SExpr) -> SExpr {
        (self.handlers().invalid)(reason)
    }

    fn resource(&self, state: SExpr, reason: SExpr) -> SExpr {
        (self.handlers().resource)(state, reason)
    }
}

pub(crate) fn project_source_library(
    program: &mut SProgram,
    library: &dyn CollectionSourceDeclarations,
    carriers: Vec<SItem>,
) -> Result<()> {
    fn qualify(path: &str, name: &str) -> String {
        if path.is_empty() {
            name.to_owned()
        } else {
            format!("{path}.{name}")
        }
    }

    fn reject_conflicts(items: &[SItem], path: &str, library: &Program) -> Result<()> {
        for item in items {
            let name = qualify(path, item.name());
            if let SItem::Module(module) = item {
                reject_conflicts(&module.items, &name, library)?;
            } else if library.declaration(&name).is_some() {
                return Err(fail(
                    "a source declaration cannot replace an actual projected collection library body",
                    item.span(),
                ));
            }
        }
        Ok(())
    }

    fn attach_owners(
        items: &mut [SItem],
        path: &str,
        roles: &mut HashMap<&str, &str>,
        entry_path: &str,
        source_sha256: &str,
    ) -> Result<()> {
        for item in items {
            let name = qualify(path, item.name());
            if let SItem::Module(module) = item {
                attach_owners(&mut module.items, &name, roles, entry_path, source_sha256)?;
                continue;
            }
            let Some(role) = roles.remove(name.as_str()) else {
                continue;
            };
            let owner = Box::new(CollectionLibraryOwner {
                model: "Rust standard collection projected library owner 1".to_owned(),
                entry_path: entry_path.to_owned(),
                source_sha256: source_sha256.to_owned(),
                declaration: name,
                role: role.to_owned(),
            });
            match item {
                SItem::Data(data) if role == "state" => {
                    data.source_standard_collection_library_owner = Some(owner);
                }
                SItem::Fn(function) if role != "state" => {
                    function.source_standard_collection_library_owner = Some(owner);
                }
                _ => {
                    return Err(fail(
                        "a projected collection role requires its actual source declaration",
                        item.span(),
                    ));
                }
            }
        }
        Ok(())
    }

    reject_conflicts(&program.items, "", library.program())?;
    let [packet, kernel, equality] = library.callable_names();
    let (state, ordering) = library.state_and_ordering_names();
    let mut roles = HashMap::from([
        (state, "state"),
        (kernel, "kernel"),
        (packet, "packet"),
        (equality, "equality"),
        (ordering, "ordering"),
    ]);
    let mut items = library.source_program().items.clone();
    attach_owners(
        &mut items,
        "",
        &mut roles,
        library.entry_path(),
        library.source_sha256(),
    )?;
    if !roles.is_empty() {
        return Err(fail(
            "the source projection requires all actual collection library declarations",
            None,
        ));
    }
    items.extend(carriers);
    items.append(&mut program.items);
    program.items = items;
    Ok(())
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
