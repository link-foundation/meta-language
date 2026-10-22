//! The Rust frontend: reads a Rust program into the surface AST.
//!
//! It reads the subset of Rust the core can represent faithfully: modules
//! with `use` of crate items, enums with unit and tuple variants (`Box<T>`
//! fields are the recursive occurrences of a value type), free functions over
//! machine integers, `bool` and `String`, anonymous scalar Copy tuple products,
//! expression bodies with `let`, `if`, `match` and `format!`, and a `main` made
//! of `println!`, `let` and `assert!`/`assert_eq!` statements. Everything else is rejected with a
//! precise obligation.
//!
//! Mirrors `js/src/translation/rust.js`.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::aborts::rust_macro_message;
use super::diagnostics::{Result, TranslationError, type_error, unsupported};
use super::lexer::{Token, TokenCursor, TokenKind, describe, tokenize};
use super::surface::{
    BinaryOp, Flavor, Rounding, SArrayItem, SComparison, SCtor, SData, SEffect, SExpr, SField, SFn,
    SItem, SMain, SModule, SNode, SParam, SPattern, SPatternNode, SProgram, SProp, SPropNode, SRow,
    ShowStyle, UnaryOp,
};
use super::types::{BOOL, FLOAT, STRING, Type, UNIT, rust_fixed_type};
use super::{Language, Span};

mod array_rules;
mod cast_syntax_rules;
mod completion_rules;
pub(crate) mod const_function_rules;
pub(crate) mod const_functions;
mod diagnostic_attributes;
pub(crate) mod empty_option_rules;
mod empty_options;
mod expressions;
mod float_predicate_rules;
mod float_predicates;
pub(super) mod indirection_rules;
mod items;
mod owned_local_state;
pub(crate) mod owned_local_state_rules;
pub(crate) mod owned_tuple_call_rules;
pub(crate) mod primitive_owner_rules;
pub(crate) mod primitive_owners;
mod product_rules;
pub(crate) mod pure_unit_call_rules;
mod pure_unit_calls;
mod read_only_query_rules;
mod scalar_loops;
mod scalar_parameter_rules;
mod scalar_parameters;
mod shared_array_handles;
mod source_origins;
pub(crate) mod type_namespaces;

use source_origins::{SourceDeclaration, SourceOrigin, SourceOrigins};

const ACCEPTED_DERIVES: [&str; 8] = [
    "Clone",
    "Copy",
    "Debug",
    "PartialEq",
    "Eq",
    "Hash",
    "PartialOrd",
    "Ord",
];
const ACCEPTED_LINTS: [&str; 6] = ["allow", "warn", "deny", "expect", "must_use", "inline"];
const RESERVED_ITEMS: [&str; 10] = [
    "impl",
    "trait",
    "struct",
    "union",
    "static",
    "type",
    "extern",
    "macro_rules",
    "async",
    "unsafe",
];
const UNSUPPORTED_TYPES: [&str; 10] = [
    "str", "char", "f32", "Vec", "Option", "Result", "Rc", "Arc", "HashMap", "Self",
];

fn comparison_op(value: &str) -> Option<BinaryOp> {
    Some(match value {
        "==" => BinaryOp::Eq,
        "!=" => BinaryOp::Ne,
        "<" => BinaryOp::Lt,
        "<=" => BinaryOp::Le,
        ">" => BinaryOp::Gt,
        ">=" => BinaryOp::Ge,
        _ => return None,
    })
}

fn additive_op(value: &str) -> Option<BinaryOp> {
    Some(match value {
        "+" => BinaryOp::Add,
        "-" => BinaryOp::Sub,
        _ => return None,
    })
}

fn multiplicative_op(value: &str) -> Option<BinaryOp> {
    Some(match value {
        "*" => BinaryOp::Mul,
        "/" => BinaryOp::Div,
        "%" => BinaryOp::Rem,
        _ => return None,
    })
}

mod source_profiles;
pub(crate) use source_profiles::parse_rust_with_loop_profiles;
pub use source_profiles::{
    parse_rust, parse_rust_with_array_result_model, parse_rust_with_completion_model,
    parse_rust_with_empty_option_source_model, parse_rust_with_primitive_owner_model,
    parse_rust_with_pure_unit_call_model, parse_rust_with_runtime_shell_model,
    parse_rust_with_scalar_loop_model, parse_rust_with_shared_array_handle_source_model,
    parse_rust_with_source_models, parse_rust_with_source_profiles,
    parse_rust_with_source_signature_model, parse_rust_with_type_namespace_model,
};

/// Read explicit owned String local state and its exact source interface.
///
/// # Errors
/// Refuses unknown models and unproved source ownership or local state.
pub fn parse_rust_with_owned_local_state_model(
    source: &str,
    model: &str,
    source_signature_model: &str,
) -> Result<SProgram> {
    parse_rust_with_selected_profiles(
        source,
        SourceProfiles {
            local_value_state: model,
            source_signature: source_signature_model,
            ..SourceProfiles::default()
        },
    )
}

/// Read owned const qualification on closed primitive Rust predicates.
///
/// # Errors
/// Refuses unknown models and unsupported source or compilation context.
pub fn parse_rust_with_const_function_model(
    source: &str,
    model: &str,
    source_signature_model: &str,
) -> Result<SProgram> {
    parse_rust_with_selected_profiles(
        source,
        SourceProfiles {
            const_function: model,
            source_signature: source_signature_model,
            ..SourceProfiles::default()
        },
    )
}

#[derive(Clone, Copy, Default)]
struct SourceProfiles<'a> {
    shared_array_handle_source: &'a str,
    const_function: &'a str,
    array_result: &'a str,
    completion: &'a str,
    source_signature: &'a str,
    scalar_loop: &'a str,
    primitive_owner: &'a str,
    empty_option_source: &'a str,
    type_namespace: &'a str,
    pure_unit_call: &'a str,
    local_value_state: &'a str,
}

fn parse_rust_with_selected_profiles(
    source: &str,
    profiles: SourceProfiles<'_>,
) -> Result<SProgram> {
    let SourceProfiles {
        shared_array_handle_source: shared_array_handle_source_model,
        const_function: const_function_model,
        array_result: array_result_model,
        completion: completion_model,
        source_signature: source_signature_model,
        scalar_loop: scalar_loop_model,
        primitive_owner: primitive_owner_model,
        empty_option_source: empty_option_source_model,
        type_namespace: type_namespace_model,
        pure_unit_call: pure_unit_call_model,
        local_value_state: local_value_state_model,
    } = profiles;
    if !super::rust_shared_array_handle_rules::accept_rust_shared_array_handle_source_model(
        shared_array_handle_source_model,
    ) {
        return Err(unsupported(
            "Rust shared array handle source model",
            "select the legacy model or standard Rust shared array identity handles 1",
            None,
        ));
    }
    if !shared_array_handle_source_model.is_empty() {
        if [
            const_function_model,
            array_result_model,
            completion_model,
            source_signature_model,
            scalar_loop_model,
            primitive_owner_model,
            empty_option_source_model,
            type_namespace_model,
            pure_unit_call_model,
            local_value_state_model,
        ]
        .iter()
        .any(|model| !model.is_empty())
        {
            return Err(unsupported(
                "Rust shared array handle source model",
                "other source interpretations require independent composition proofs",
                None,
            ));
        }
        let units = u32::try_from(source.encode_utf16().take(65_537).count()).unwrap_or(u32::MAX);
        if !super::rust_shared_array_handle_rules::accept_rust_shared_array_handle_source_size(
            f64::from(units),
        ) {
            return Err(unsupported(
                "Rust shared array handle source",
                "the profile requires its complete source within 65536 UTF-16 units",
                None,
            ));
        }
    }
    if !const_function_rules::accept_rust_const_function_model(const_function_model) {
        return Err(unsupported(
            "Rust const function model",
            "select the legacy model or closed pure Rust const predicates 1",
            None,
        ));
    }
    if !const_function_model.is_empty()
        && [
            array_result_model,
            completion_model,
            scalar_loop_model,
            primitive_owner_model,
            empty_option_source_model,
            type_namespace_model,
            local_value_state_model,
            pure_unit_call_model,
        ]
        .iter()
        .any(|model| !model.is_empty())
    {
        return Err(unsupported(
            "Rust const function model",
            "other source interpretations require independent composition proofs",
            None,
        ));
    }
    if !pure_unit_call_rules::accept_rust_pure_unit_call_model(pure_unit_call_model) {
        return Err(unsupported(
            "Rust pure Unit-call model",
            "select the legacy model, closed pure Rust unit calls 1 or closed public pure Rust unit calls 1",
            None,
        ));
    }
    if !pure_unit_call_model.is_empty()
        && [
            array_result_model,
            completion_model,
            source_signature_model,
            scalar_loop_model,
            primitive_owner_model,
            empty_option_source_model,
            type_namespace_model,
            local_value_state_model,
        ]
        .iter()
        .any(|model| !model.is_empty())
    {
        return Err(unsupported(
            "Rust pure Unit-call model",
            "the closed Unit-call interpretation requires its independent source profile",
            None,
        ));
    }
    if !owned_local_state_rules::accept_rust_owned_local_state_model(local_value_state_model)
        && !owned_tuple_call_rules::accept_rust_owned_tuple_call_model(local_value_state_model)
    {
        return Err(unsupported(
            "Rust owned local state model",
            "select the original move profile or canonical String copy profile",
            None,
        ));
    }
    if !super::type_namespace_rules::accept_rust_type_namespace_model(type_namespace_model) {
        return Err(unsupported(
            "Rust type namespace model",
            "select the legacy model or resolved Rust type namespace 1",
            None,
        ));
    }
    if !primitive_owner_rules::accept_rust_primitive_owner_model(primitive_owner_model) {
        return Err(unsupported(
            "Rust primitive owner model",
            "select the legacy model or standard primitive Rust binary64 owner 1",
            None,
        ));
    }
    if !super::scalar_loop_rules::accept_rust_scalar_loop_model(scalar_loop_model) {
        return Err(unsupported(
            "Rust scalar loop model",
            "select the legacy model or closed primitive scalar loop 1",
            None,
        ));
    }
    if !array_rules::accept_rust_source_array_result_model(array_result_model) {
        return Err(unsupported(
            "Rust array result model",
            "select the legacy model or immutable fresh scalar vector results 1",
            None,
        ));
    }
    if !completion_rules::accept_rust_completion_model(completion_model) {
        return Err(unsupported(
            "Rust completion model",
            "select the legacy model or closed guarded function returns 1",
            None,
        ));
    }
    if !super::source_signature_rules::accept_rust_source_signature_model(source_signature_model) {
        return Err(unsupported(
            "Rust source signature model",
            "select the legacy model or Rust source function signature 1",
            None,
        ));
    }
    if !empty_option_rules::accept_rust_empty_option_source_model(empty_option_source_model) {
        return Err(unsupported(
            "Rust empty option source model",
            "select the legacy model or closed empty primitive Rust option 1",
            None,
        ));
    }
    if !empty_option_source_model.is_empty()
        && [
            array_result_model,
            completion_model,
            source_signature_model,
            scalar_loop_model,
            primitive_owner_model,
            type_namespace_model,
            local_value_state_model,
        ]
        .iter()
        .any(|model| !model.is_empty())
    {
        return Err(unsupported(
            "Rust empty option source model",
            "the closed empty interpretation requires its independent source profile",
            None,
        ));
    }
    let tokens = tokenize(source, Language::Rust)?.tokens;
    let original_source = (!const_function_model.is_empty()
        || !shared_array_handle_source_model.is_empty())
    .then(|| Arc::<str>::from(source));
    let mut declarations = RustParser::new(
        tokens.clone(),
        SourceOrigins::new(None, array_result_model),
        profiles,
        true,
        original_source.clone(),
    );
    declarations.file()?;
    let mut origins = std::mem::replace(
        &mut declarations.origins,
        SourceOrigins::new(None, array_result_model),
    );
    origins.resolve_catalog_types(|segments, path| declarations.resolve(segments, path));
    let mut source_origins = SourceOrigins::new(Some(origins.catalog), array_result_model);
    source_origins.collecting = true;
    source_origins.unit_clone_facts = origins.unit_clone_facts;
    let mut syntax = RustParser::new(
        tokens.clone(),
        source_origins,
        profiles,
        false,
        original_source.clone(),
    );
    syntax.aliases = declarations.aliases;
    syntax.primitive_imports = declarations.primitive_imports;
    let surface = syntax.file()?;
    // Preserve the existing checker's rejection and syntax-tree contract;
    // every public Rust source pipeline checks this identical Surface again.
    if super::check::check_program(&surface).is_err() && shared_array_handle_source_model.is_empty()
    {
        return Ok(surface);
    }
    let mut parser = RustParser::new(
        tokens,
        SourceOrigins::new(Some(syntax.origins.catalog), array_result_model),
        profiles,
        false,
        original_source,
    );
    parser.origins.unit_clone_facts = syntax.origins.unit_clone_facts;
    parser.aliases = syntax.aliases;
    parser.primitive_imports = syntax.primitive_imports;
    parser.file()
}

/// Where a sequence of items ends: at the end of the file or at a module's `}`.
#[derive(Clone, Copy)]
enum ItemsEnd {
    File,
    Module,
}

/// Parsing context an expression inherits from its position.
#[derive(Clone, Copy, Default)]
struct Options {
    /// The expression starts a statement, so a leading `if`/`match` ends there.
    statement: bool,
    /// `{` opens a block, not a struct literal (`if`/`match` heads).
    no_struct: bool,
}

/// A `name @ pattern` binding, bound around the arm's body.
struct Alias {
    name: String,
    value: SExpr,
}

/// A parsed `let` statement.
struct Binding {
    name: String,
    ty: Option<Type>,
    value: SExpr,
    span: Option<Span>,
}

/// An irrefutable source pattern, lowered to ordinary ordered value bindings.
enum ProductBindingPattern {
    Name(Token),
    Wildcard(Token),
    Tuple {
        items: Vec<Self>,
        span: Option<Span>,
    },
    Unit {
        span: Option<Span>,
    },
}

impl ProductBindingPattern {
    const fn source_span(&self) -> Option<Span> {
        match self {
            Self::Name(token) | Self::Wildcard(token) => Some(token.span()),
            Self::Tuple { span, .. } | Self::Unit { span } => *span,
        }
    }
}

struct RustParser {
    shared_array_handle_source_model: String,
    shared_array_state: shared_array_handles::SharedArrayState,
    const_function_model: String,
    const_function_keyword: Option<Token>,
    original_source: Option<Arc<str>>,
    diagnostic_attribute_columns: Option<diagnostic_attributes::DiagnosticAttributeColumns>,
    cursor: TokenCursor,
    /// `use` aliases, per module path.
    aliases: HashMap<String, HashMap<String, Vec<String>>>,
    module_path: Vec<String>,
    crate_declarations: HashSet<String>,
    main: Option<SMain>,
    origins: SourceOrigins,
    completion_model: String,
    source_signature_model: String,
    scalar_loop_model: String,
    primitive_owner_model: String,
    empty_option_source_model: String,
    type_namespace_model: String,
    pure_unit_call_model: String,
    pure_unit_public_token: Option<Token>,
    local_value_state_model: String,
    owned_tuple_call_discovery: Option<()>,
    local_value_copies: HashSet<Span>,
    local_value_constructors: HashMap<Span, Span>,
    type_namespace_frames: Vec<Vec<Vec<String>>>,
    /// Selected initial catalog discovery, never a later collecting pass.
    type_namespace_discovery: Option<()>,
    empty_option_result: Option<serde_json::Value>,
    empty_option_main_seen: bool,
    primitive_owner_frames: Vec<Vec<Option<serde_json::Value>>>,
    primitive_constant_sites: Vec<serde_json::Value>,
    primitive_import_names: HashSet<String>,
    primitive_imports: Vec<serde_json::Value>,
    scalar_loop_active: bool,
    source_owner_frames: Vec<Vec<super::source_signatures::SourceOwner>>,
    reserved_binding_names: HashSet<String>,
    next_product_binding: usize,
    lifetime_bindings: HashSet<String>,
    lifetime_header: bool,
}

impl RustParser {
    fn new(
        tokens: Vec<Token>,
        mut origins: SourceOrigins,
        profiles: SourceProfiles<'_>,
        initial_discovery: bool,
        original_source: Option<Arc<str>>,
    ) -> Self {
        let reserved_binding_names = tokens
            .iter()
            .filter(|token| token.kind == TokenKind::Identifier)
            .map(|token| token.value.clone())
            .collect();
        origins.owned_local_values = !profiles.local_value_state.is_empty();
        origins.shared_array_handles = !profiles.shared_array_handle_source.is_empty();
        Self {
            shared_array_handle_source_model: profiles.shared_array_handle_source.to_owned(),
            shared_array_state: shared_array_handles::SharedArrayState::default(),
            const_function_model: profiles.const_function.to_owned(),
            const_function_keyword: None,
            original_source,
            diagnostic_attribute_columns: None,
            cursor: TokenCursor::new(tokens, Language::Rust),
            aliases: HashMap::new(),
            module_path: Vec::new(),
            crate_declarations: HashSet::new(),
            main: None,
            origins,
            completion_model: profiles.completion.to_owned(),
            source_signature_model: if profiles.source_signature.is_empty()
                && (!profiles.type_namespace.is_empty()
                    || !profiles.local_value_state.is_empty()
                    || !profiles.const_function.is_empty())
            {
                "Rust source function signature 1".to_owned()
            } else {
                profiles.source_signature.to_owned()
            },
            scalar_loop_model: profiles.scalar_loop.to_owned(),
            primitive_owner_model: profiles.primitive_owner.to_owned(),
            empty_option_source_model: profiles.empty_option_source.to_owned(),
            type_namespace_model: profiles.type_namespace.to_owned(),
            pure_unit_call_model: profiles.pure_unit_call.to_owned(),
            pure_unit_public_token: None,
            local_value_state_model: profiles.local_value_state.to_owned(),
            owned_tuple_call_discovery:
                (owned_tuple_call_rules::accept_rust_owned_tuple_call_model(
                    profiles.local_value_state,
                ) && initial_discovery)
                    .then_some(()),
            local_value_copies: HashSet::new(),
            local_value_constructors: HashMap::new(),
            type_namespace_frames: Vec::new(),
            type_namespace_discovery: (!profiles.type_namespace.is_empty() && initial_discovery)
                .then_some(()),
            empty_option_result: None,
            empty_option_main_seen: false,
            primitive_owner_frames: Vec::new(),
            primitive_constant_sites: Vec::new(),
            primitive_import_names: HashSet::new(),
            primitive_imports: Vec::new(),
            scalar_loop_active: false,
            source_owner_frames: Vec::new(),
            reserved_binding_names,
            next_product_binding: 0,
            lifetime_bindings: HashSet::new(),
            lifetime_header: false,
        }
    }

    fn peek(&self) -> Token {
        self.cursor.peek().clone()
    }

    fn fail(message: &str, token: &Token) -> TranslationError {
        TranslationError::syntax(
            format!("{message} but found {}", describe(token)),
            Some(token.span()),
        )
    }
}

/// `format!("{} and {name}", a)`: `{}` takes the next argument, `{name}` a
/// variable in scope, `{{`/`}}` are braces; every value is rendered with
/// Display, which for integers is its decimal text.
fn format(args: Vec<SExpr>, token: &Token) -> Result<SExpr> {
    let mut args = args.into_iter();
    let template = args.next();
    let values: Vec<SExpr> = args.collect();
    let Some(SExpr {
        node: SNode::Str { value: source },
        span: template_span,
        ..
    }) = template
    else {
        return Err(unsupported(
            &format!("{}!", token.value),
            "the format string must be a literal",
            span(token, token),
        ));
    };
    let chars: Vec<char> = source.chars().collect();
    let mut pieces: Vec<SExpr> = Vec::new();
    let mut text = String::new();
    let mut next = 0;
    let mut index = 0;
    while index < chars.len() {
        let char = chars[index];
        let following = chars.get(index + 1).copied();
        if char == '{' && following == Some('{') {
            text.push('{');
            index += 2;
            continue;
        }
        if char == '}' && following == Some('}') {
            text.push('}');
            index += 2;
            continue;
        }
        if char == '}' {
            return Err(TranslationError::syntax(
                "unmatched } in format string",
                template_span,
            ));
        }
        if char != '{' {
            text.push(char);
            index += 1;
            continue;
        }
        let Some(end) = chars[index..]
            .iter()
            .position(|&candidate| candidate == '}')
            .map(|offset| index + offset)
        else {
            return Err(TranslationError::syntax(
                "unclosed { in format string",
                template_span,
            ));
        };
        let spec: String = chars[index + 1..end].iter().collect();
        if spec.contains(':') {
            return Err(unsupported(
                &format!("format spec {{{spec}}}"),
                "only plain {} Display formatting is portable",
                template_span,
            ));
        }
        let value = if spec.is_empty() {
            let value = values.get(next).cloned();
            next += 1;
            value.ok_or_else(|| {
                TranslationError::syntax("format string has more {} than arguments", template_span)
            })?
        } else if spec.bytes().all(|byte| byte.is_ascii_digit()) {
            // A position too large for an index names no argument either.
            let position = spec.parse::<usize>().ok();
            let value = position.and_then(|position| values.get(position).cloned());
            let (Some(position), Some(value)) = (position, value) else {
                return Err(TranslationError::syntax(
                    format!("format argument {spec} is missing"),
                    template_span,
                ));
            };
            next = next.max(position + 1);
            value
        } else if is_identifier(&spec) {
            SExpr::new(SNode::Name { path: vec![spec] }, template_span)
        } else {
            return Err(unsupported(
                &format!("format argument {{{spec}}}"),
                "outside the portable core",
                template_span,
            ));
        };
        if !text.is_empty() {
            pieces.push(SExpr::new(
                SNode::Str {
                    value: std::mem::take(&mut text),
                },
                template_span,
            ));
        }
        let value_span = value.span;
        pieces.push(SExpr::new(
            SNode::Show {
                arg: Box::new(value),
                style: ShowStyle::Rust,
            },
            value_span,
        ));
        index = end + 1;
    }
    if next < values.len() {
        return Err(TranslationError::syntax(
            "format string has fewer {} than arguments",
            template_span,
        ));
    }
    if !text.is_empty() || pieces.is_empty() {
        pieces.push(SExpr::new(SNode::Str { value: text }, template_span));
    }
    let mut pieces = pieces.into_iter();
    let first = pieces
        .next()
        .unwrap_or_else(|| unreachable!("a piece was pushed"));
    Ok(pieces.fold(first, |left, right| {
        SExpr::new(
            SNode::Binary {
                op: BinaryOp::Concat,
                left: Box::new(left),
                right: Box::new(right),
                rounding: None,
            },
            template_span,
        )
    }))
}

/// `/^[A-Za-z_][A-Za-z0-9_]*$/`.
fn is_identifier(text: &str) -> bool {
    let mut bytes = text.bytes();
    bytes
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn method_call(
    receiver: SExpr,
    method: &Token,
    args: Vec<SExpr>,
    range: Option<Span>,
) -> Result<SExpr> {
    let expect = |count: usize| {
        if args.len() == count {
            Ok(())
        } else {
            Err(type_error(
                format!(
                    "{} expects {count} arguments but got {}",
                    method.value,
                    args.len()
                ),
                range,
            ))
        }
    };
    let mapping =
        crate::translation::frontend_rules::read_string_map_operation("Rust", &method.value);
    if !mapping.is_empty() {
        expect(0)?;
        return Ok(SExpr::new(
            SNode::StringMap {
                op: mapping,
                object: Box::new(receiver),
            },
            range,
        ));
    }
    let operation =
        crate::translation::frontend_rules::read_string_test_operation("Rust", &method.value);
    if !operation.is_empty() {
        expect(1)?;
        return Ok(SExpr::new(
            SNode::StringTest {
                op: operation,
                object: Box::new(receiver),
                search: Box::new(args.into_iter().next().expect("one search argument")),
            },
            range,
        ));
    }
    match method.value.as_str() {
        "clone" | "to_owned" | "as_str" => {
            expect(0)?;
            Ok(receiver)
        }
        "to_string" => {
            expect(0)?;
            Ok(SExpr::new(
                SNode::ToString {
                    arg: Box::new(receiver),
                },
                range,
            ))
        }
        "div_euclid" | "rem_euclid" => {
            expect(1)?;
            let op = if method.value == "div_euclid" {
                BinaryOp::Div
            } else {
                BinaryOp::Rem
            };
            let right = args
                .into_iter()
                .next()
                .unwrap_or_else(|| unreachable!("one argument"));
            Ok(SExpr::new(
                SNode::Binary {
                    op,
                    left: Box::new(receiver),
                    right: Box::new(right),
                    rounding: Some(Rounding::Euclid),
                },
                range,
            ))
        }
        _ => Err(unsupported(
            &format!("method {}", method.value),
            "outside the portable core",
            range,
        )),
    }
}

/// An imported path from the crate root, so it means the same wherever it is used.
fn absolute(segments: &[String], path: &[String], range: Option<Span>) -> Result<Vec<String>> {
    let crate_root = || vec!["crate".to_owned()];
    if segments[0] == "crate" {
        return Ok(segments.to_vec());
    }
    if segments[0] == "self" {
        let mut result = crate_root();
        result.extend_from_slice(path);
        result.extend_from_slice(&segments[1..]);
        return Ok(result);
    }
    let up = segments
        .iter()
        .take_while(|segment| *segment == "super")
        .count();
    if up > path.len() {
        return Err(type_error("super beyond crate root", range));
    }
    let mut result = crate_root();
    result.extend_from_slice(&path[..path.len() - up]);
    result.extend_from_slice(&segments[up..]);
    Ok(result)
}

fn prop_of(expr: SExpr) -> SProp {
    let node = match expr.node {
        SNode::Binary {
            op:
                op @ (BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge),
            left,
            right,
            ..
        } => {
            let comparison = SComparison {
                left: *left,
                right: *right,
                reference: false,
                same_value: false,
            };
            match op {
                BinaryOp::Eq => SPropNode::Eq(comparison),
                BinaryOp::Ne => SPropNode::Ne(comparison),
                BinaryOp::Lt => SPropNode::Lt(comparison),
                BinaryOp::Le => SPropNode::Le(comparison),
                BinaryOp::Gt => SPropNode::Gt(comparison),
                _ => SPropNode::Ge(comparison),
            }
        }
        SNode::Binary {
            op: BinaryOp::And,
            left,
            right,
            ..
        } => SPropNode::And {
            left: Box::new(prop_of(*left)),
            right: Box::new(prop_of(*right)),
        },
        SNode::Binary {
            op: BinaryOp::Or,
            left,
            right,
            ..
        } => SPropNode::Or {
            left: Box::new(prop_of(*left)),
            right: Box::new(prop_of(*right)),
        },
        SNode::Unary {
            op: UnaryOp::Not,
            arg,
        } => SPropNode::Not {
            arg: Box::new(prop_of(*arg)),
        },
        node => SPropNode::Bool {
            expr: SExpr { node, ..expr },
        },
    };
    SProp { node, span: None }
}

/// The value a pattern matched, for `name @ p`.
fn pattern_value(pattern: &SPattern, range: Option<Span>) -> Result<SExpr> {
    let node = match &pattern.node {
        SPatternNode::BindOrCtor { name } => SNode::Name {
            path: vec![name.clone()],
        },
        SPatternNode::NumLit { value, negative } => SNode::Num {
            value: value.clone(),
            ty: None,
            negative: *negative,
            unit: false,
        },
        SPatternNode::BoolLit { value, .. } => SNode::Bool { value: *value },
        SPatternNode::Ctor { path, args } if args.is_empty() => SNode::Name { path: path.clone() },
        SPatternNode::Ctor { path, args } => SNode::App {
            func: Box::new(SExpr::new(SNode::Name { path: path.clone() }, range)),
            args: args
                .iter()
                .map(|arg| pattern_value(arg, range))
                .collect::<Result<_>>()?,
        },
        _ => {
            return Err(unsupported(
                "@ binding",
                "the aliased pattern must bind every field",
                range,
            ));
        }
    };
    Ok(SExpr::new(node, range))
}

const fn is_if_or_match(expr: &SExpr) -> bool {
    matches!(expr.node, SNode::If { .. } | SNode::Match { .. })
}

fn operator(token: &Token, table: fn(&str) -> Option<BinaryOp>) -> Option<BinaryOp> {
    if token.kind == TokenKind::Punct {
        table(&token.value)
    } else {
        None
    }
}

fn binary(op: BinaryOp, left: SExpr, right: SExpr, token: &Token) -> SExpr {
    let range = joined(left.span, right.span, token);
    SExpr::new(
        SNode::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
            rounding: None,
        },
        range,
    )
}

#[allow(clippy::unnecessary_wraps)] // nodes and errors take an optional span
fn joined(left: Option<Span>, right: Option<Span>, token: &Token) -> Option<Span> {
    Some(Span::new(
        left.map_or(token.start, |left| left.start),
        right.map_or(token.end, |right| right.end),
    ))
}

#[allow(clippy::unnecessary_wraps)] // nodes and errors take an optional span
fn span(from: &Token, to: &Token) -> Option<Span> {
    let to_start = if to.kind == TokenKind::Eof {
        from.end
    } else {
        to.start
    };
    Some(Span::new(from.start, from.end.max(to_start)))
}
