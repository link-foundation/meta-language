//! The Rust emitter: writes a checked program as Rust with its translation contract.
//!
//! Naturals and integers are `ml::Big`, an unbounded integer the translation
//! carries in its own prelude, so no value is ever narrowed; machine integers
//! stay Rust machine integers with checked arithmetic, which panics exactly
//! where the source aborts; a JavaScript Number is an `f64`, printed by
//! `ml_number::js_number` exactly as JavaScript prints it. Data types are enums
//! whose data-typed fields are boxed. Every value is owned: a variable is cloned where it is consumed.
//! Theorems cannot be proved in Rust: each becomes an executable property,
//! checked over a bounded domain by `--ml-check-theorems`, while the proof
//! stays discharged by the source kernel.
//!
//! Mirrors `js/src/translation/emit-rust.js`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use super::decimal::Decimal;
use super::diagnostics::{Result, unsupported};
use super::emit_common::{CtorStyle, EmitOptions, EmitState, Emitted};
use super::frontend_rules::{
    accept_constant_emission, constant_binding_form, render_constant_binding,
};
use super::ir::{
    Binder, ByZero, Decl, Effect, Expr, LitValue, Node, Pattern, Program, Prop, Scope, Semantics,
    rename_function, rename_main, rename_theorem, tail_loop,
};
use super::surface::{BinaryOp, External, Flavor, Rounding, SImport, UnaryOp};
use super::types::Type;
use super::{Language, Span};

mod array_moves;
mod array_state_methods;
mod arrays;
mod callables;
mod declarations;
mod expressions;
mod externals;
mod math;
mod nominal_array_loans;
mod number_conversions;
mod number_prelude;
mod optional_values;
mod prelude;
mod primitive_methods;
mod record_writes;
mod records;
mod reference_array_projections;
mod reference_callbacks;
mod reference_methods;

use self::externals::external_name;
use self::prelude::Prelude;

const KEYWORDS: &[&str] = &[
    "ml_unicode_case",
    "ml_utf16_text",
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "async",
    "await",
    "dyn",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "try",
    "gen",
    "union",
    "raw",
    // Names the emitted code relies on.
    "ml",
    "main",
    "std",
    "core",
    "alloc",
    "String",
    "Vec",
    "Box",
    "Option",
    "Some",
    "None",
    "Ok",
    "Err",
    "Result",
    "bool",
    "str",
    "char",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "usize",
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "isize",
    "f32",
    "f64",
    "Big",
    "Clone",
    "Debug",
    "PartialEq",
    "Eq",
    "ToString",
    "ml_main",
    "ml_check_theorems",
    "ml_number",
    "ml_record_property_order",
];

/// A legal Rust identifier for a source name.
fn ident(name: &str) -> String {
    let mut result: String = name
        .chars()
        .map(|char| {
            if char.is_ascii_alphanumeric() || char == '_' {
                char
            } else {
                '_'
            }
        })
        .collect();
    if result.starts_with(|char: char| char.is_ascii_digit())
        || result.starts_with("__")
        || result == "_"
        || result.is_empty()
    {
        result = format!("x{result}");
    }
    if KEYWORDS.contains(&result.as_str()) {
        result.push('_');
    }
    result
}

/// `camelCase` and `HTTPServer` split into `camel_case` and `http_server`.
fn snake(name: &str) -> String {
    // `/([a-z0-9])([A-Z])/g` inserts `_` between a lower-case letter or digit and a capital.
    let chars: Vec<char> = name.chars().collect();
    let mut first = Vec::with_capacity(chars.len());
    let mut index = 0;
    while index < chars.len() {
        let char = chars[index];
        let next = chars.get(index + 1).copied();
        if (char.is_ascii_lowercase() || char.is_ascii_digit())
            && next.is_some_and(|next| next.is_ascii_uppercase())
        {
            first.extend([char, '_', next.unwrap_or(char)]);
            index += 2;
        } else {
            first.push(char);
            index += 1;
        }
    }
    // `/([A-Z]+)([A-Z][a-z])/g` splits a run of capitals before its last one
    // when a lower-case letter follows.
    let mut second = String::with_capacity(first.len() + 4);
    let mut index = 0;
    while index < first.len() {
        let run = first[index..]
            .iter()
            .take_while(|char| char.is_ascii_uppercase())
            .count();
        let after = first.get(index + run).copied();
        if run >= 2 && after.is_some_and(|after| after.is_ascii_lowercase()) {
            second.extend(&first[index..index + run - 1]);
            second.push('_');
            second.extend(&first[index + run - 1..=index + run]);
            index += run + 1;
        } else {
            second.push(first[index]);
            index += 1;
        }
    }
    ident(&second.to_lowercase())
}

/// `my_type` and `my-type` become `MyType`.
fn camel(name: &str) -> String {
    let joined: String = name
        .split(|char: char| !char.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |first| {
                let mut part = first.to_uppercase().to_string();
                part.extend(chars);
                part
            })
        })
        .collect();
    ident(if joined.is_empty() { "T" } else { &joined })
}

/// Emits a checked program as Rust.
///
/// # Errors
/// On constructs the target cannot express faithfully.
pub fn emit_rust(program: &Program) -> Result<Emitted> {
    emit_rust_with_options(program, RustEmissionOptions::default())
}

/// Optional explicit source-interface interpretations for Rust emission.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RustEmissionOptions<'a> {
    /// Preserve primitive immutable-reference headers under the selected model.
    pub source_header_model: Option<&'a str>,
}

/// Emit a checked program with an explicitly selected source-header model.
///
/// # Errors
/// Refuses unsupported models, interfaces and contexts before rendering.
pub fn emit_rust_with_options(
    program: &Program,
    options: RustEmissionOptions<'_>,
) -> Result<Emitted> {
    emit_rust_context(program, options, false, false)
}

/// Emit immutable array values by consuming each proven final lexical use.
/// Historical low-level copy profiles remain available through [`emit_rust`].
///
/// # Errors
/// On constructs or explicit source interfaces the target cannot preserve.
pub fn emit_rust_with_array_consumption(
    program: &Program,
    options: RustEmissionOptions<'_>,
) -> Result<Emitted> {
    emit_rust_context(program, options, false, true)
}

pub(crate) fn emit_rust_with_representation(
    program: &Program,
    utf16_strings: bool,
) -> Result<Emitted> {
    emit_rust_context(program, RustEmissionOptions::default(), utf16_strings, true)
}

fn emit_rust_context(
    program: &Program,
    options: RustEmissionOptions<'_>,
    utf16_strings: bool,
    array_consumption: bool,
) -> Result<Emitted> {
    emit_rust_context_owned(program, options, utf16_strings, array_consumption, false)
        .map(|(emitted, _)| emitted)
}

pub(crate) fn emit_rust_with_definition_ownership(
    program: &Program,
    utf16_strings: bool,
) -> Result<(Emitted, Option<DefinitionOwnership>)> {
    emit_rust_context_owned(
        program,
        RustEmissionOptions::default(),
        utf16_strings,
        true,
        true,
    )
}

/// Actual ordered external targets selected by this normal Rust emitter.
pub(crate) fn read_rust_external_targets(program: &Program) -> Vec<OwnedExternalTarget> {
    program
        .externals
        .iter()
        .filter(|external| program.declaration(external.name()).is_none())
        .map(|external| OwnedExternalTarget {
            source: external.name().to_owned(),
            kind: match external {
                External::Function { .. } => "fn",
                External::Constant { .. } => "const",
            },
            target: external_name(external, external.name()),
        })
        .collect()
}

fn emit_rust_context_owned(
    program: &Program,
    options: RustEmissionOptions<'_>,
    utf16_strings: bool,
    array_consumption: bool,
    record_ownership: bool,
) -> Result<(Emitted, Option<DefinitionOwnership>)> {
    let prepared = super::ordered_record_preparation::prepare(program)?;
    let program = prepared.as_ref();
    super::class_owners::validate_program(program)?;
    super::source_type_namespaces::validate_source_type_namespaces(program)?;
    super::source_const_functions::validate_const_functions(program)?;
    super::owned_local_state::validate_owned_local_state(program)?;
    if let Some(runtime) = super::runtime_shell::validate_program(program)? {
        if options.source_header_model.is_some() {
            return Err(crate::translation::diagnostics::unsupported(
                "closed Rust runtime shell",
                "target header changes require an independent runtime composition proof",
                None,
            ));
        }
        return super::runtime_shell::emit_rust(program, &runtime).map(|emitted| (emitted, None));
    }
    let mut emitter = rust_emitter(program);
    if record_ownership {
        emitter.definition_ownership = Some(DefinitionOwnership {
            blocks: Vec::new(),
            external_targets: read_rust_external_targets(program),
        });
    }
    emitter.array_consumption = array_consumption;
    emitter.utf16_strings =
        super::program_string_representation_rules::select_program_string_representation(
            program.utf16_strings.unwrap_or(false),
            utf16_strings,
            primitive_methods::program_requires_method_units(program),
        );
    if emitter.utf16_strings {
        emitter.state.encode("utf16-strings", "strings are sequences of UTF-16 code units, retaining isolated surrogates and JavaScript offsets; output replaces isolated surrogates as Node writes UTF-8");
    }
    if let Some(model) = options.source_header_model {
        if super::rust_scalar_source_interface_rules::accept_rust_scalar_source_interface_model(
            model,
        ) || super::rust_scalar_mathematical_interface_rules::accept_rust_scalar_mathematical_interface_model(model)
            || super::rust_textual_source_interface_rules::accept_rust_textual_source_interface_model(model)
            || super::rust_string_predicate_interface_rules::accept_rust_string_predicate_interface_model(model)
            || super::rust_constructed_string_interface_rules::accept_rust_constructed_string_interface_model(model)
            || super::rust_displayed_string_interface_rules::accept_rust_displayed_string_interface_model(model) {
            let headers =
                super::scalar_source_interface::read_rust_scalar_source_interface(program, model)?;
            emitter.source_headers = headers.clone();
            emitter.borrowed_call_targets = Some(headers);
            emitter.scalar_source_interface = true;
            emitter.state.encode(
                &super::rust_scalar_source_interface_rules::read_rust_scalar_source_encoding(),
                &super::rust_scalar_source_interface_rules::read_rust_scalar_source_statement(),
            );
            if super::rust_scalar_mathematical_interface_rules::accept_rust_scalar_mathematical_interface_model(model) {
                emitter.state.encode(
                    &super::rust_scalar_mathematical_interface_rules::read_rust_scalar_mathematical_encoding(),
                    &super::rust_scalar_mathematical_interface_rules::read_rust_scalar_mathematical_statement(),
                );
            }
            if super::rust_textual_source_interface_rules::accept_rust_textual_source_interface_model(model)
                || super::rust_string_predicate_interface_rules::accept_rust_string_predicate_interface_model(model)
                || super::rust_constructed_string_interface_rules::accept_rust_constructed_string_interface_model(model)
                || super::rust_displayed_string_interface_rules::accept_rust_displayed_string_interface_model(model) {
                emitter.state.encode(
                    &super::rust_textual_source_interface_rules::read_rust_textual_source_encoding(),
                    &super::rust_textual_source_interface_rules::read_rust_textual_source_statement(),
                );
            }
            if super::rust_string_predicate_interface_rules::accept_rust_string_predicate_interface_model(model)
                || super::rust_constructed_string_interface_rules::accept_rust_constructed_string_interface_model(model)
                || super::rust_displayed_string_interface_rules::accept_rust_displayed_string_interface_model(model) {
                emitter.state.encode(
                    &super::rust_string_predicate_interface_rules::read_rust_string_predicate_encoding(),
                    &super::rust_string_predicate_interface_rules::read_rust_string_predicate_statement(),
                );
            }
            if super::rust_constructed_string_interface_rules::accept_rust_constructed_string_interface_model(model)
                || super::rust_displayed_string_interface_rules::accept_rust_displayed_string_interface_model(model) {
                emitter.state.encode(
                    &super::rust_constructed_string_interface_rules::read_rust_constructed_string_encoding(),
                    &super::rust_constructed_string_interface_rules::read_rust_constructed_string_statement(),
                );
            }
            if super::rust_displayed_string_interface_rules::accept_rust_displayed_string_interface_model(model) {
                emitter.state.encode(
                    &super::rust_displayed_string_interface_rules::read_rust_displayed_string_encoding(),
                    &super::rust_displayed_string_interface_rules::read_rust_displayed_string_statement(),
                );
            }
        } else if super::borrowed_call_interface_rules::accept_rust_borrowed_call_interface_model(
            model,
        ) {
            let interface =
                super::borrowed_call_interface::read_rust_borrowed_call_interface(program, model)?;
            emitter.source_headers = interface.headers;
            emitter.borrowed_call_targets = Some(interface.targets);
        } else {
            emitter.source_headers =
                super::source_header_emission::read_rust_source_header_emission(program, model)?;
        }
    }
    emitter.file_owned()
}

/// The Rust emitter of `program`, with every name of the module's other items
/// reserved.
fn rust_emitter(program: &Program) -> RustEmitter<'_> {
    let mut state = EmitState::new(
        program,
        Language::Rust,
        snake,
        KEYWORDS,
        EmitOptions {
            ctor_style: CtorStyle::Data,
            type_name: Some(camel),
            value_name: Some(snake),
            ctor_name: Some(camel),
            module_segment: Some(snake),
            type_space: true,
            modules_share_type_space: true,
            ..EmitOptions::default()
        },
    );
    // A local may not shadow another item of the module that the program calls or reads.
    for external in &program.externals {
        state.reserve(external_name(external, external.name()));
    }
    RustEmitter {
        program,
        named_constructor_fields: program.source_language == super::Language::Rust
            && program.declarations.iter().any(|entry| {
                let Decl::Data(data) = entry else {
                    return false;
                };
                data.ctors.iter().any(|ctor| {
                    super::rust_source_render_interfaces::accept_rust_named_constructor_fields(
                        program.source_language.as_str(),
                        &ctor
                            .fields
                            .iter()
                            .map(|field| field.name.clone())
                            .collect::<Vec<_>>(),
                    )
                })
            }),
        state,
        temporaries: 0,
        theorem_checks: Vec::new(),
        preludes: BTreeSet::new(),
        loop_params: None,
        source_headers: BTreeMap::new(),
        borrowed_call_targets: None,
        borrowed_call_scope: None,
        scalar_source_interface: false,
        utf16_strings: false,
        standard_array: false,
        array_final_uses: BTreeSet::new(),
        array_consumption: false,
        definition_ownership: None,
    }
}

/// Emits a top-level constant with the JavaScript-generated eligibility,
/// binding form and rendering decisions, or `None` for other programs.
///
/// # Errors
/// On a constant value the target cannot express faithfully.
pub fn emit_rust_constants(program: &Program) -> Result<Option<Emitted>> {
    super::class_owners::validate_program(program)?;
    let prepared = super::ordered_record_preparation::prepare(program)?;
    let program = prepared.as_ref();
    let Some(main) = &program.main else {
        return Ok(None);
    };
    let Some(Effect::Let {
        name,
        value,
        constant,
        ..
    }) = main.effects.first()
    else {
        return Ok(None);
    };
    let legal_name = !name.starts_with("ml_")
        && !KEYWORDS.contains(&name.as_str())
        && name.bytes().enumerate().all(|(index, byte)| {
            byte.is_ascii_alphabetic() || byte == b'_' || (index > 0 && byte.is_ascii_digit())
        })
        && !name.is_empty();
    #[allow(clippy::cast_precision_loss)]
    if !accept_constant_emission(
        *constant,
        main.effects.len() as f64,
        program.declarations.len() as f64,
        legal_name,
    ) {
        return Ok(None);
    }
    let mut emitter = rust_emitter(program);
    emitter.utf16_strings = program.utf16_strings.unwrap_or(false)
        || primitive_methods::program_requires_method_units(program);
    let form = constant_binding_form(
        if matches!(value.node, Node::Lit { .. }) {
            "lit"
        } else {
            ""
        },
        value.ty.kind(),
    );
    let (ty, expression) = if form == "string" {
        let Node::Lit { value } = &value.node else {
            unreachable!("the generated string form requires a literal");
        };
        ("&str".to_owned(), rust_string(&value.text()))
    } else {
        (emitter.ty(&value.ty)?, emitter.expr(value)?)
    };
    let definition = render_constant_binding(&form, name, &ty, &expression);
    let preludes: Vec<String> = emitter
        .preludes
        .iter()
        .map(|prelude| prelude.source(emitter.standard_array))
        .collect();
    let mut output = emitter.state.finish(
        preludes
            .iter()
            .chain(std::iter::once(&definition))
            .cloned()
            .collect::<Vec<_>>()
            .join("\n\n"),
        None,
    );
    output.preludes = preludes;
    output.definitions = vec![definition];
    Ok(Some(output))
}

/// Declarations grouped by source module, in declaration order.
#[derive(Default)]
struct ModuleTree<'p> {
    items: Vec<&'p Decl>,
    modules: Vec<(String, Self)>,
}

/// A theorem the `--ml-check-theorems` runner evaluates.
#[derive(Clone)]
struct TheoremCheck {
    reference: String,
    binders: Vec<Binder>,
    source: String,
}

/// Inspectable records captured during normal complete block emission.
/// Transient validation metadata, separate from the serialized Emitted contract.
#[derive(Clone, Debug, Default)]
pub struct DefinitionOwnership {
    pub blocks: Vec<OwnedDefinitionBlock>,
    pub external_targets: Vec<OwnedExternalTarget>,
}
#[derive(Clone, Debug)]
pub struct OwnedDefinitionBlock {
    pub source: String,
    pub kind: &'static str,
    pub source_span: Option<Span>,
    pub definition_index: usize,
    pub definition: String,
    pub mapping_start: usize,
    pub mapping_end: usize,
    pub targets: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedExternalTarget {
    pub source: String,
    pub kind: &'static str,
    pub target: String,
}

struct RustEmitter<'p> {
    program: &'p Program,
    named_constructor_fields: bool,
    state: EmitState<'p>,
    definition_ownership: Option<DefinitionOwnership>,
    temporaries: usize,
    theorem_checks: Vec<TheoremCheck>,
    /// The generated modules the program uses.
    preludes: BTreeSet<Prelude>,
    /// The lifted loop being emitted as a loop, and its parameters, which
    /// each call to it assigns before the next iteration.
    loop_params: Option<(String, Vec<String>)>,
    /// Validated owned source signatures selected before interface rendering.
    source_headers: BTreeMap<String, super::source_signatures::SourceSignature>,
    /// Actual owned leaf signatures for the selected direct-call interface.
    borrowed_call_targets: Option<BTreeMap<String, super::source_signatures::SourceSignature>>,
    /// Per-function hygienic names, reserved against every argument and binder.
    borrowed_call_scope: Option<Scope<'p>>,
    /// Explicit retained scalar header and whole call-graph interpretation.
    scalar_source_interface: bool,
    /// Internal self-translation representation; actual Text values are guarded.
    utf16_strings: bool,
    /// The JavaScript source selected optional reads or standard array methods.
    standard_array: bool,
    /// Actual last consuming occurrences in the current renamed checked tree.
    array_final_uses: BTreeSet<*const Expr>,
    /// Explicit emission profile; historical low-level output retains copies.
    array_consumption: bool,
}

fn is_copy(ty: &Type, product_model: Option<&str>) -> bool {
    if matches!(ty, Type::Tuple { .. }) && product_model != Some("immutable value product") {
        return false;
    }
    let fields = match ty {
        Type::Tuple { elements } => elements
            .iter()
            .map(|item| is_copy(item, product_model))
            .collect(),
        _ => Vec::new(),
    };
    super::product_emission_rules::accept_tuple_copy_type(ty.kind(), &fields)
}

const fn comparison_operator(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Eq => "==",
        BinaryOp::Ne => "!=",
        BinaryOp::Lt => "<",
        BinaryOp::Le => "<=",
        BinaryOp::Gt => ">",
        _ => ">=",
    }
}

/// The rounding as the JavaScript runtime interpolates it.
const fn rounding_text(rounding: Option<Rounding>) -> &'static str {
    match rounding {
        Some(Rounding::Trunc) => "trunc",
        Some(Rounding::Euclid) => "euclid",
        Some(Rounding::Floor) => "floor",
        None => "undefined",
    }
}

impl RustEmitter<'_> {}

/// An owned copy of a variable.
fn own(name: &str, ty: &Type) -> String {
    if is_copy(ty, None) {
        name.to_owned()
    } else {
        format!("{name}.clone()")
    }
}

fn block(text: &str) -> String {
    format!("{{\n{}\n}}", indent(text, 1))
}

fn rust_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for char in text.chars() {
        match char {
            '"' | '\\' => {
                out.push('\\');
                out.push(char);
            }
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ if (char as u32) < 0x20 || char as u32 == 0x7f => {
                let _ = write!(out, "\\u{{{:x}}}", char as u32);
            }
            _ => out.push(char),
        }
    }
    out
}

/// Text inside a `format!`-style literal, where braces are doubled.
fn format_escape(text: &str) -> String {
    rust_escape(text).replace('{', "{{").replace('}', "}}")
}

fn rust_string(text: &str) -> String {
    format!("\"{}\"", rust_escape(text))
}

pub(super) fn indent(text: &str, depth: usize) -> String {
    let pad = "    ".repeat(depth);
    text.split('\n')
        .map(|line| {
            if line.is_empty() {
                line.to_owned()
            } else {
                format!("{pad}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
