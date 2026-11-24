//! Admission of the complete generated Rust standard collection source library.

use std::collections::HashMap;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use super::check::check_program;
use super::ir::{Decl, Expr, FnDecl, Node, Program};
use super::javascript::{has_source_library_typedef, parse_ordinary_javascript_source_library};
use super::lexer::Source;
use super::source_library_closure::{Closure, compose};
use super::surface::{SExpr, SProgram};
use super::types::Type;
use crate::LinkNetwork;
use crate::structured_data_links::{
    StructuredDataLinksLimits, read_structured_data_lino_with_limits,
};
use crate::typed_program_links::{
    TypedProgramLinksLimits, decode_typed_program_links_with_limits, typed_program_links_schema,
};

const ENTRY: &str = "js/src/translation/rust-standard-collection-value-packet.js";
const ASSET: &str = "js/src/data/rust-standard-collection-value-packet-program.lino";

/// Budget for reading the library and rechecking its complete declarations.
#[derive(Clone, Copy, Debug)]
pub struct RustStandardCollectionLibraryLimits {
    /// Maximum bytes of asset text or original source closure.
    pub byte_limit: usize,
    /// Maximum Links nodes and structurally owned checked values.
    pub node_limit: usize,
    /// Maximum edges of the admitted canonical Links network.
    pub reference_limit: usize,
    /// Maximum structural nesting in checked declarations and receipts.
    pub depth_limit: usize,
}

impl Default for RustStandardCollectionLibraryLimits {
    fn default() -> Self {
        Self {
            byte_limit: 48 * 1024 * 1024,
            node_limit: 1_000_000,
            reference_limit: 4_000_000,
            depth_limit: 256,
        }
    }
}

/// Distinct continuations that keep invalid and exhausted source outcomes.
pub struct RustStandardCollectionSourceHandlers<Invalid, Resource> {
    /// Lower an actual invalid reason to the caller's source error continuation.
    pub invalid: Invalid,
    /// Lower the retained state and reason to the caller's resource continuation.
    pub resource: Resource,
}

/// The checked collection library, with the independently captured source bodies.
pub struct RustStandardCollectionSourceLibrary<Invalid, Resource> {
    program: Program,
    source_program: SProgram,
    source_sha256: String,
    packet: String,
    kernel: String,
    equality: String,
    ordering: String,
    state: String,
    packet_type: String,
    constructors: [String; 3],
    handlers: RustStandardCollectionSourceHandlers<Invalid, Resource>,
}

impl<Invalid, Resource> RustStandardCollectionSourceLibrary<Invalid, Resource> {
    /// The checked complete linked declarations.
    #[must_use]
    pub const fn program(&self) -> &Program {
        &self.program
    }

    /// The independently captured surface declarations.
    #[must_use]
    pub const fn source_program(&self) -> &SProgram {
        &self.source_program
    }

    /// Digest of the complete recomposed source.
    #[must_use]
    pub fn source_sha256(&self) -> &str {
        &self.source_sha256
    }

    /// Qualified packet, kernel, and equality declarations, in order.
    #[must_use]
    pub fn callable_names(&self) -> [&str; 3] {
        [&self.packet, &self.kernel, &self.equality]
    }

    /// Qualified nominal state and ordering declarations bound by admission.
    #[must_use]
    pub fn state_and_ordering_names(&self) -> (&str, &str) {
        (&self.state, &self.ordering)
    }

    /// Original JavaScript entry path used to capture the complete source closure.
    #[must_use]
    pub const fn entry_path(&self) -> &'static str {
        ENTRY
    }

    /// Qualified result datatype and Normal, Invalid and Resource constructors.
    #[must_use]
    pub fn transition_type(&self) -> (&str, [&str; 3]) {
        (
            &self.packet_type,
            self.constructors.each_ref().map(String::as_str),
        )
    }

    /// Caller-owned lowering continuations for invalid and exhausted results.
    #[must_use]
    pub const fn handlers(&self) -> &RustStandardCollectionSourceHandlers<Invalid, Resource> {
        &self.handlers
    }
}

fn fail(reason: impl std::fmt::Display) -> String {
    format!("Rust standard collection library asset: {reason}")
}

fn digest(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}

fn read_sources<'a>(receipt: &'a Value, byte_limit: usize) -> Result<Closure<'a>, String> {
    let inputs = receipt["inputs"]
        .as_array()
        .ok_or_else(|| fail("complete bounded source inputs are required"))?;
    if inputs.is_empty() || inputs.len() > 128 {
        return Err(fail(
            "source closure needs at most 128 actual source modules",
        ));
    }
    let mut size = 0;
    let inputs = inputs
        .iter()
        .map(|input| {
            let path = input["path"]
                .as_str()
                .ok_or_else(|| fail("every source input retains its actual logical path"))?;
            let source = input["source"]
                .as_str()
                .ok_or_else(|| fail("every source input retains its actual text"))?;
            size += source.len();
            if size > byte_limit {
                return Err(fail("source closure exceeds its byte budget"));
            }
            Ok((path, source))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let closure = compose(ENTRY, &inputs).map_err(fail)?;
    if receipt["source"].as_str() != Some(closure.source.as_str())
        || receipt["sourceSha256"].as_str() != Some(digest(&closure.source).as_str())
        || receipt["bytes"].as_u64() != Some(closure.source.len() as u64)
    {
        return Err(fail("source closure differs from actual recomposition"));
    }
    Ok(closure)
}

fn calls(body: &Expr) -> impl Iterator<Item = &str> {
    let mut pending = vec![body];
    std::iter::from_fn(move || {
        while let Some(term) = pending.pop() {
            pending.extend(term.children());
            if let Node::Call { func, .. } = &term.node {
                return Some(func.as_str());
            }
        }
        None
    })
}

fn read_owners(closure: &Closure<'_>, program: &Program) -> Result<Vec<Value>, String> {
    if program
        .main
        .as_ref()
        .is_some_and(|main| !main.effects.is_empty())
    {
        return Err(fail(
            "source dependency closure needs declaration-only modules",
        ));
    }
    let source = Source::new(&closure.source);
    let mut modules = HashMap::new();
    let mut rows = Vec::new();
    for declaration in &program.declarations {
        let name = declaration.full_name();
        let span = declaration.span().ok_or_else(|| {
            fail(format!(
                "source dependency needs a represented span for {name}"
            ))
        })?;
        let current = closure
            .modules
            .iter()
            .find(|module| module.range.start <= span.start && span.start < module.range.end)
            .ok_or_else(|| {
                fail(format!(
                    "source dependency needs a represented declaration owner for {name}"
                ))
            })?;
        modules.insert(name, current);
        let generated = match declaration {
            Decl::Fn(function) => function.generated,
            Decl::Data(data) => data.generated,
            _ => {
                return Err(fail(
                    "source dependencies need function or datatype declarations",
                ));
            }
        };
        let mut row = json!({"name": name, "path": current.path, "generated": generated});
        if !generated {
            if matches!(declaration, Decl::Data(_)) {
                let text = source.slice(span.start, span.end);
                if !has_source_library_typedef(&text, span, name) {
                    return Err(fail(format!(
                        "datatype {name} needs its complete documentation declaration"
                    )));
                }
                row["declarationSource"] = json!(text);
                row["span"] = json!(span);
            } else {
                row["sourceName"] =
                    json!(current.declarations.get(name).ok_or_else(|| fail(format!(
                        "{name} has no actual original module owner"
                    )))?);
            }
        }
        rows.push(row);
    }
    for declaration in &program.declarations {
        if let Decl::Fn(function) = declaration {
            let owner = modules[function.full_name.as_str()];
            for callee in calls(&function.body) {
                if modules
                    .get(callee)
                    .is_none_or(|module| module.path != owner.path)
                    && !owner.imports.contains(callee)
                {
                    return Err(fail(format!(
                        "{} cannot capture {callee} from another module",
                        owner.path
                    )));
                }
            }
        }
    }
    Ok(rows)
}

fn callable<'a>(
    program: &'a Program,
    owners: &[Value],
    source_name: &str,
    suffix: &str,
) -> Result<&'a FnDecl, String> {
    let mut matching = owners.iter().filter(|row| {
        row.get("sourceName").unwrap_or(&row["name"]).as_str() == Some(source_name)
            && row["path"]
                .as_str()
                .is_some_and(|path| path.ends_with(suffix))
    });
    let name = matching.next().and_then(|row| row["name"].as_str());
    if let Some(Decl::Fn(function)) = name.and_then(|name| program.declaration(name))
        && matching.next().is_none()
    {
        return Ok(function);
    }
    Err(fail(format!(
        "one complete actual source callable owner is required for {source_name}"
    )))
}

fn require_call(wrapper: &FnDecl, kernel: &FnDecl) -> Result<(), String> {
    if !calls(&wrapper.body).any(|callee| callee == kernel.full_name) {
        return Err(fail(format!(
            "{} must invoke its actual {} kernel",
            wrapper.name, kernel.name
        )));
    }
    Ok(())
}

/// Read a supplied complete program and original source receipt under explicit limits.
///
/// # Errors
/// Refuses budget exhaustion, malformed Links, incomplete declarations, and
/// source or ownership mismatches before returning any callable binding.
pub fn create_rust_standard_collection_source_library<Invalid, Resource>(
    program_text: &str,
    receipt: &Value,
    handlers: RustStandardCollectionSourceHandlers<Invalid, Resource>,
    limits: RustStandardCollectionLibraryLimits,
) -> Result<RustStandardCollectionSourceLibrary<Invalid, Resource>, String>
where
    Invalid: Fn(SExpr) -> SExpr,
    Resource: Fn(SExpr, SExpr) -> SExpr,
{
    if [
        limits.byte_limit,
        limits.node_limit,
        limits.reference_limit,
        limits.depth_limit,
    ]
    .contains(&0)
        || program_text.len() > limits.byte_limit
    {
        return Err(fail(
            "positive budgets and bounded complete program text are required",
        ));
    }
    if receipt["model"].as_str() != Some("Complete source library asset receipt 1")
        || receipt["schemaVersion"].as_u64() != Some(1)
        || receipt["entryPath"].as_str() != Some(ENTRY)
        || receipt["referenceSemantics"].as_bool() != Some(false)
    {
        return Err(fail(
            "a complete ordinary source receipt with the genuine entry owner is required",
        ));
    }
    let asset = &receipt["programAsset"];
    if asset["path"].as_str() != Some(ASSET)
        || asset["bytes"].as_u64() != Some(program_text.len() as u64)
        || asset["sha256"].as_str() != Some(digest(program_text).as_str())
    {
        return Err(fail("program bytes differ from their source receipt"));
    }
    let closure = read_sources(&receipt["sourceClosure"], limits.byte_limit)?;
    let network = LinkNetwork::from_bounded_lino(
        program_text,
        limits.byte_limit,
        limits.node_limit,
        limits.reference_limit,
    )
    .map_err(fail)?;
    let schema = typed_program_links_schema();
    let version = schema["schemaVersion"].to_string();
    let mut roots = network.links().filter(|link| {
        link.metadata().term() == schema["root"].as_str()
            && link.metadata().definition() == Some(version.as_str())
    });
    let root = roots
        .next()
        .ok_or_else(|| fail("one actual typed program root is required"))?;
    if roots.next().is_some() {
        return Err(fail("one actual typed program root is required"));
    }
    let program = decode_typed_program_links_with_limits(
        &network,
        root.id(),
        TypedProgramLinksLimits {
            node_limit: limits.node_limit,
            depth_limit: limits.depth_limit,
        },
    )
    .map_err(fail)?;
    drop(network);
    let source_program = parse_ordinary_javascript_source_library(&closure.source).map_err(fail)?;
    if check_program(&source_program).map_err(fail)? != program {
        return Err(fail(
            "actual source must recheck to the same complete linked program",
        ));
    }
    let owners = read_owners(&closure, &program)?;
    if receipt["sourceDeclarationOwners"].as_array() != Some(&owners) {
        return Err(fail(
            "source declaration ownership differs from the complete actual program",
        ));
    }
    let packet = callable(
        &program,
        &owners,
        "executeRustStandardCollectionValuePacket",
        "/rust-standard-collection-value-packet.js",
    )?;
    let kernel = callable(
        &program,
        &owners,
        "executeRustStandardCollectionOperation",
        "/rust-standard-collection-state.js",
    )?;
    let ordered = callable(
        &program,
        &owners,
        "executeRustStandardCollectionOrderedSnapshot",
        "/rust-standard-collection-ordered-snapshot.js",
    )?;
    let borrowed = callable(
        &program,
        &owners,
        "executeRustStandardCollectionOrderedBorrow",
        "/rust-standard-collection-ordered-borrow.js",
    )?;
    let consuming = callable(
        &program,
        &owners,
        "executeRustStandardCollectionOrderedMove",
        "/rust-standard-collection-ordered-borrow.js",
    )?;
    let equality = callable(
        &program,
        &owners,
        "compareRustStandardCollectionValuePackets",
        "/rust-standard-collection-value-packet.js",
    )?;
    let equality_kernel = callable(
        &program,
        &owners,
        "executeRustStandardCollectionEquality",
        "/rust-standard-collection-equality.js",
    )?;
    let ordering = callable(
        &program,
        &owners,
        "compareRustStandardCollectionOrderedValuePackets",
        "/rust-standard-collection-value-packet.js",
    )?;
    let ordering_kernel = callable(
        &program,
        &owners,
        "executeRustStandardCollectionOrdering",
        "/rust-standard-collection-ordering.js",
    )?;
    for callee in [kernel, ordered, borrowed, consuming] {
        require_call(packet, callee)?;
    }
    require_call(equality, equality_kernel)?;
    require_call(ordering, ordering_kernel)?;
    let [receiver, _, _, _, _] = packet.params.as_slice() else {
        return Err(fail("packet wrapper needs its complete concrete signature"));
    };
    let Type::Data { name: state } = &receiver.ty else {
        return Err(fail("packet wrapper needs its actual nominal state receiver"));
    };
    if [equality, ordering].iter().any(|wrapper| {
            wrapper.params.len() != 4
                || wrapper.params[..2]
                    .iter()
                    .any(|parameter| parameter.ty != packet.params[0].ty)
        })
        || ordering.name != "compareRustStandardCollectionOrderedValuePackets"
    {
        return Err(fail(
            "packet and comparison wrappers need their complete concrete signatures",
        ));
    }
    let Type::Data { name } = &packet.ret else {
        return Err(fail(
            "actual packet callable must return its concrete transition datatype",
        ));
    };
    let Some(Decl::Data(data)) = program
        .declaration(name)
        .filter(|decl| matches!(decl, Decl::Data(data) if data.ctors.len() == 3))
    else {
        return Err(fail("complete packet transition datatype is required"));
    };
    let fields: [&[&str]; 3] = [&["values", "writeback"], &["reason"], &["state", "reason"]];
    let mut constructors = [String::new(), String::new(), String::new()];
    for (index, expected) in fields.iter().enumerate() {
        let mut matching = data.ctors.iter().filter(|variant| {
            variant
                .fields
                .iter()
                .map(|field| field.name.as_str())
                .eq(expected.iter().copied())
        });
        let variant = matching
            .next()
            .ok_or_else(|| fail("actual transition fields are required"))?;
        if matching.next().is_some() {
            return Err(fail(
                "actual transition fields must have a unique constructor owner",
            ));
        }
        constructors[index] = variant.name.clone();
    }
    Ok(RustStandardCollectionSourceLibrary {
        packet: packet.full_name.clone(),
        kernel: kernel.full_name.clone(),
        equality: equality.full_name.clone(),
        ordering: ordering.full_name.clone(),
        state: state.clone(),
        packet_type: data.full_name.clone(),
        constructors,
        source_sha256: digest(&closure.source),
        program,
        source_program,
        handlers,
    })
}

/// Load the installed complete collection asset and verify its original sources.
///
/// # Errors
/// Refuses incomplete, stale, malformed or over-budget assets.
pub fn load_rust_standard_collection_source_library<Invalid, Resource>(
    handlers: RustStandardCollectionSourceHandlers<Invalid, Resource>,
    limits: RustStandardCollectionLibraryLimits,
) -> Result<RustStandardCollectionSourceLibrary<Invalid, Resource>, String>
where
    Invalid: Fn(SExpr) -> SExpr,
    Resource: Fn(SExpr, SExpr) -> SExpr,
{
    let program = include_str!("../data/rust-standard-collection-value-packet-program.lino");
    let receipt = include_str!("../data/rust-standard-collection-value-packet-program-source.lino");
    let receipt = read_structured_data_lino_with_limits(
        receipt,
        StructuredDataLinksLimits {
            node_limit: limits.node_limit,
            depth_limit: limits.depth_limit,
            byte_limit: limits.byte_limit,
        },
    )
    .map_err(fail)?;
    create_rust_standard_collection_source_library(program, &receipt, handlers, limits)
}
