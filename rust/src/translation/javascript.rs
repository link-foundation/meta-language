//! JavaScript frontend for the portable core.
//!
//! It reads the subset of modern JavaScript the core can represent
//! faithfully: Number and `BigInt` arithmetic, strings and booleans; top-level functions
//! and object-literal namespaces of methods whose types come from `JSDoc`
//! (`@param`, `@returns`, and `@typedef` unions of `{ $: 'tag', … }` object
//! types for data types) or, where `JSDoc` is silent, are inferred from how the
//! program uses them; bodies made of `const`, `if`, `return`, `throw` and
//! `switch` statements; and a top level of `console.log`, `const` and
//! `node:assert` statements, which are the program's effects. A function
//! whose leading statements throw on a negative argument takes a natural
//! number. Everything else is rejected with a precise obligation.
//!
//! Mirrors `js/src/translation/javascript.js`.

use std::collections::{HashMap, HashSet};

use super::diagnostics::{Result, TranslationError, type_error, unsupported};
use super::javascript_line_terminator_rules::accept_java_script_line_terminator;
use super::lexer::{
    Comment, Source, Token, TokenCursor, TokenKind, describe, is_js_space, tokenize,
};
use super::surface::{
    BinaryOp, External, Guard, SArrayItem, SComparison, SCtor, SData, SEffect, SExpr, SField, SFn,
    SImport, SImportName, SItem, SMain, SModule, SNode, SParam, SPattern, SPatternNode, SProgram,
    SProp, SPropNode, SRow, STagTest, ShowStyle, UnaryOp,
};
use super::types::{BOOL, FLOAT, INT, NAT, STRING, Type, array};
use super::{Language, Span};

mod array_bindings;
mod array_methods;
mod callables;
mod console;
mod constructed_guarded_array_writes;
mod declarations;
mod expressions;
mod flow;
mod guarded_private_array_writes;
mod imperative;
mod imports;
mod infer;
mod lexical_bindings;
mod loops;
mod lowering;
mod math;
mod object_properties;
mod owned_array_append;
mod product_source_rules;
mod products;
mod record_parameters;
mod reference_analysis;
mod scalar_loops;
mod statements;
mod surface_rules;
mod tag_context;
use self::flow::{imperative, statement_uses};
use self::imperative::lower_imperative;
use self::lowering::{assertion_kind, guarded_parameter, lower, prop_of};

const ROOT: &str = "crate";
const ASSIGNMENTS: [&str; 16] = [
    "=", "+=", "-=", "*=", "/=", "%=", "**=", "<<=", ">>=", ">>>=", "&=", "|=", "^=", "&&=", "||=",
    "??=",
];
/// `x op= e` is `x = x op e`; `+=` adds numbers or concatenates strings like `+`.
const COMPOUND: [(&str, BinaryOp); 7] = [
    ("+=", BinaryOp::Plus),
    ("-=", BinaryOp::Sub),
    ("*=", BinaryOp::Mul),
    ("/=", BinaryOp::Div),
    ("%=", BinaryOp::Rem),
    ("&&=", BinaryOp::And),
    ("||=", BinaryOp::Or),
];
const ERRORS: [&str; 3] = ["Error", "RangeError", "TypeError"];
const GLOBALS: [&str; 24] = [
    "Math",
    "Number",
    "BigInt",
    "parseInt",
    "parseFloat",
    "JSON",
    "Array",
    "Object",
    "Symbol",
    "Date",
    "Promise",
    "Reflect",
    "Proxy",
    "Map",
    "Set",
    "WeakMap",
    "WeakSet",
    "globalThis",
    "process",
    "require",
    "console",
    "setTimeout",
    "isNaN",
    "isFinite",
];
/// The own properties of `Object.prototype`. The JavaScript frontend looks
/// assertion methods up in a plain object, so these names inherit a truthy
/// entry and read as `notDeepStrictEqual`.
const OBJECT_PROTOTYPE: [&str; 12] = [
    "constructor",
    "__defineGetter__",
    "__defineSetter__",
    "hasOwnProperty",
    "__lookupGetter__",
    "__lookupSetter__",
    "isPrototypeOf",
    "propertyIsEnumerable",
    "toString",
    "valueOf",
    "__proto__",
    "toLocaleString",
];

/// What an `assert.method(…)` call asserts.
#[derive(Clone, Copy, PartialEq, Eq)]
enum AssertionKind {
    Eq,
    Ne,
    Deep,
    NotDeep,
}

/// Reads a JavaScript ES module into the surface AST.
///
/// # Errors
///
/// On syntax errors, type errors in `JSDoc` annotations and constructs outside
/// the portable core.
pub fn parse_javascript(source: &str) -> Result<SProgram> {
    parse_javascript_in(source, &ModuleContext::default())
}

/// Read actual emitted scalar loops under the selected Number/Boolean interface.
///
/// # Errors
/// Refuses unknown models and unproved state, snapshot or interface semantics.
pub fn parse_javascript_with_scalar_loop_model(source: &str, model: &str) -> Result<SProgram> {
    if !super::scalar_loop_rules::accept_rust_scalar_loop_model(model) {
        return Err(unsupported(
            "JavaScript scalar loop model",
            "the selected model is not registered",
            None,
        ));
    }
    let tokens = tokenize(source, Language::JavaScript)?;
    let mut parser = JavaScriptParser::new(source, tokens.tokens, &tokens.comments);
    model.clone_into(&mut parser.scalar_loop_model);
    parser.file()
}

/// What self-translation tells the frontend about the module of the item it reads.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModuleContext {
    /// The signatures of the module's other items and of the items it
    /// imports, which the item may call and read.
    pub externals: Vec<External>,
    /// The module's directory inside its crate, as path segments; with it,
    /// relative imports are items of the crate.
    pub module_directory: Option<Vec<String>>,
}

/// [`parse_javascript`] for one item of a module in `context`.
///
/// # Errors
///
/// As [`parse_javascript`].
pub fn parse_javascript_in(source: &str, context: &ModuleContext) -> Result<SProgram> {
    parse_javascript_bound_in(source, context, &HashMap::new(), false)
}

pub(crate) fn parse_javascript_bound_in(
    source: &str,
    context: &ModuleContext,
    literal_bindings: &HashMap<String, SExpr>,
    readonly_module_constants: bool,
) -> Result<SProgram> {
    let tokens = tokenize(source, Language::JavaScript)?;
    let mut parser = JavaScriptParser::new(source, tokens.tokens, &tokens.comments);
    parser.literal_bindings.clone_from(literal_bindings);
    parser.readonly_constants.module = readonly_module_constants;
    parser.externals.clone_from(&context.externals);
    parser
        .module_directory
        .clone_from(&context.module_directory);
    parser.file()
}

/// Locals in scope, the ones `let` or a parameter makes assignable, and
/// names of the current block still in their temporal dead zone.
#[derive(Clone, Default)]
struct Scope {
    locals: HashSet<String>,
    tdz: HashSet<String>,
    mutable: HashSet<String>,
}

/// Enclosing loops and switches, for `break` and `continue`.
#[derive(Clone, Copy, Default)]
struct Jumps {
    loops: usize,
    switches: usize,
}

/// The local name of the imported `node:assert` module.
struct Assertion {
    name: String,
    strict: bool,
}

/// A `JSDoc` block's `@param` types, in order, and its `@returns` type.
struct JsDoc {
    params: Vec<(String, String)>,
    optional_params: HashSet<String>,
    returns: Option<String>,
    range: Span,
}

/// A `JSDoc` tag: `@tag {type} name`.
struct JsDocTag {
    tag: String,
    ty: Option<String>,
    name: Option<String>,
    optional: bool,
}

/// Function-body statements before lowering to one expression.
#[derive(Clone)]
enum Stmt {
    Const {
        name: String,
        value: SExpr,
        span: Span,
    },
    /// `let name = value`, an assignable local.
    Let {
        name: String,
        value: SExpr,
        span: Span,
    },
    /// `name = value`, and `x op= e`, `x++` and `x--` read as one.
    Assign {
        name: String,
        value: SExpr,
        span: Span,
    },
    While {
        cond: SExpr,
        body: Vec<Self>,
        span: Span,
    },
    DoWhile {
        body: Vec<Self>,
        cond: SExpr,
        span: Span,
    },
    /// `for (init; cond; update) body`, whose `init` are `let` declarations or assignments.
    For {
        init: Vec<Self>,
        cond: Option<SExpr>,
        update: Vec<Self>,
        body: Vec<Self>,
        span: Span,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    Return {
        expr: SExpr,
        span: Span,
    },
    Throw {
        message: String,
        span: Span,
    },
    /// A braced block; `inline` for the bindings of a destructuring `const`.
    Block {
        body: Vec<Self>,
        inline: bool,
        span: Span,
    },
    Empty,
    If {
        cond: SExpr,
        then: Vec<Self>,
        otherwise: Option<Vec<Self>>,
        span: Span,
    },
    Switch(Switch),
    Expr {
        expr: SExpr,
        span: Span,
    },
    /// `console.log(expr)`.
    Print {
        expr: SExpr,
        style: ShowStyle,
        span: Span,
    },
}

#[derive(Clone)]
struct Switch {
    /// The switch subject: the discriminant, or the object of `x.$`.
    scrutinee: SExpr,
    /// The subject's name and data type in a tag switch.
    tag: Option<(String, SData)>,
    clauses: Vec<Clause>,
    span: Span,
}

#[derive(Clone)]
struct Clause {
    tests: Vec<CaseTest>,
    body: Vec<Stmt>,
    span: Span,
}

#[derive(Clone)]
enum CaseTest {
    Default {
        span: Span,
    },
    Tag {
        tag: String,
        span: Span,
    },
    NumLit {
        value: String,
        negative: bool,
        span: Span,
    },
    BoolLit {
        value: bool,
        span: Span,
    },
    Value {
        value: Box<SExpr>,
        span: Span,
    },
}

/// Where a block-declaration scan stops: the end of the file or the block's `}`.
#[derive(Clone, Copy)]
enum ScanEnd {
    File,
    Brace,
}

#[derive(Default)]
struct ReadonlyConstants {
    module: bool,
    initializer: bool,
}

struct ReferenceSemantics {
    enabled: bool,
    forced: bool,
}

struct JavaScriptParser {
    source: Source,
    cursor: TokenCursor,
    docs: Vec<Comment>,
    /// Tags of every @typedef data type, for `switch (x.$)`.
    data_types: Vec<SData>,
    type_aliases: HashMap<String, String>,
    scope: Scope,
    lexical: lexical_bindings::Trace,
    tag_function_results: HashMap<String, Option<tag_context::TagReceiverFact>>,
    literal_bindings: HashMap<String, SExpr>,
    assertion: Option<Assertion>,
    /// Async functions run sequentially: each call of one is awaited where it
    /// is made, so nothing else runs until its result is back. `await` is
    /// allowed at the top level of the module and in async functions.
    async_names: HashSet<String>,
    in_async: bool,
    unawaited: Vec<(String, Span)>,
    sequential_async: bool,
    jumps: Jumps,
    /// Whether a top-level statement is being read, where `return` is a syntax error.
    top_level: bool,
    readonly_constants: ReadonlyConstants,
    reference_state: ReferenceSemantics,
    eliminated_owned_array_append_operations: Vec<Span>,
    scalar_loop_model: String,
    /// Functions and data types that loops and joins of statements lower to, and their count.
    generated: Vec<SItem>,
    generated_count: usize,
    indexed_reference_writes: usize,
    /// The other items of the module an item may call and read.
    externals: Vec<External>,
    /// The module's directory inside its crate, which makes relative imports items of the crate.
    module_directory: Option<Vec<String>>,
    imports: Vec<SImport>,
    exported_names: HashSet<String>,
    function_names: HashSet<String>,
    owned_array_readers: owned_array_append::Readers,
    owned_array_reader_namespace: usize,
    nested_product_bindings: HashSet<Span>,
    global_binding_names: HashSet<String>,
    /// Whole actual token/context names, initialized after context installation.
    record_parameter_names: Option<HashSet<String>>,
    record_parameter_ordinal: u64,
}

impl JavaScriptParser {
    fn new(source: &str, tokens: Vec<Token>, comments: &[Comment]) -> Self {
        let reference_semantics = reference_analysis::requires_reference_semantics(&tokens, &[]);
        Self {
            source: Source::new(source),
            cursor: TokenCursor::new(tokens, Language::JavaScript),
            docs: comments
                .iter()
                .filter(|comment| comment.text.starts_with("/**") && comment.text != "/**/")
                .cloned()
                .collect(),
            data_types: Vec::new(),
            type_aliases: HashMap::new(),
            scope: Scope::default(),
            lexical: lexical_bindings::Trace::default(),
            tag_function_results: HashMap::new(),
            literal_bindings: HashMap::new(),
            assertion: None,
            async_names: HashSet::new(),
            in_async: true,
            unawaited: Vec::new(),
            sequential_async: false,
            jumps: Jumps::default(),
            top_level: false,
            readonly_constants: ReadonlyConstants::default(),
            reference_state: ReferenceSemantics {
                enabled: reference_semantics,
                forced: false,
            },
            eliminated_owned_array_append_operations: Vec::new(),
            scalar_loop_model: String::new(),
            generated: Vec::new(),
            generated_count: 0,
            indexed_reference_writes: 0,
            externals: Vec::new(),
            module_directory: None,
            imports: Vec::new(),
            exported_names: HashSet::new(),
            function_names: HashSet::new(),
            owned_array_readers: HashMap::new(),
            owned_array_reader_namespace: 0,
            nested_product_bindings: HashSet::new(),
            global_binding_names: HashSet::new(),
            record_parameter_names: None,
            record_parameter_ordinal: 0,
        }
    }

    fn peek(&self) -> Token {
        self.cursor.peek().clone()
    }

    fn peek_at(&self, offset: usize) -> Token {
        self.cursor.peek_at(offset).clone()
    }

    /// The span from `from` to the next token.
    fn to_here(&self, from: &Token) -> Span {
        span(from, self.cursor.peek())
    }

    /// Retain the actual last consumed token after the shared interval proof.
    fn read_complete_declaration_span(&self, from: &Token) -> Result<Span> {
        let fail = || {
            TranslationError::syntax(
                "an actual completed declaration requires its last consumed ordered source token",
                Some(from.span()),
            )
        };
        let consumed = self
            .cursor
            .index
            .checked_sub(1)
            .and_then(|index| self.cursor.tokens.get(index))
            .filter(|token| token.kind != TokenKind::Eof)
            .ok_or_else(fail)?;
        // Reuse the existing integer Number transport. The shared offset rule
        // rejects any conversion outside its exact safe-integer domain.
        let numbers = [from.start, from.end, consumed.end]
            .map(|offset| serde_json::Number::from(offset).as_f64());
        let [Some(start), Some(prefix_end), Some(consumed_end)] = numbers else {
            return Err(fail());
        };
        if !crate::typed_program_links::source_span_rules::accept_complete_declaration_source_interval(
            start, prefix_end, consumed_end,
        ) {
            return Err(fail());
        }
        Ok(Span::new(from.start, consumed.end))
    }

    fn fail(message: &str, token: &Token) -> TranslationError {
        TranslationError::syntax(
            format!("{message} but found {}", describe(token)),
            Some(token.span()),
        )
    }

    fn fail_here(&self, message: &str) -> TranslationError {
        Self::fail(message, self.cursor.peek())
    }
}

/// `JSDoc` tags of a comment: `@tag {type} name`. Leading `*` of each line is
/// decoration.
fn jsdoc_tags(comment: &Comment) -> Result<Vec<JsDocTag>> {
    let units: Vec<u16> = comment.text.encode_utf16().collect();
    let inner = String::from_utf16_lossy(
        &units[3.min(units.len())..units.len().saturating_sub(2).max(3.min(units.len()))],
    );
    let text: Vec<char> = inner
        .split('\n')
        .map(|line| {
            let line = line
                .trim_start_matches(|ch: char| u16::try_from(u32::from(ch)).is_ok_and(is_js_space));
            line.strip_prefix('*').unwrap_or(line)
        })
        .collect::<Vec<_>>()
        .join("\n")
        .chars()
        .collect();
    let mut tags = Vec::new();
    let mut at = 0;
    while let Some((tag_start, tag_end)) = next_tag(&text, at) {
        let tag: String = text[tag_start + 1..tag_end].iter().collect();
        let mut index = tag_end;
        while matches!(text.get(index), Some(' ' | '\t')) {
            index += 1;
        }
        let mut ty = None;
        if text.get(index) == Some(&'{') {
            let mut depth = 0;
            let mut end = index;
            while end < text.len() {
                if text[end] == '{' {
                    depth += 1;
                }
                if text[end] == '}' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                end += 1;
            }
            if depth != 0 {
                return Err(TranslationError::syntax(
                    format!("unbalanced braces in the JSDoc type of @{tag}"),
                    Some(Span::new(comment.start, comment.end)),
                ));
            }
            ty = Some(
                text[index + 1..end]
                    .iter()
                    .map(|&ch| if ch == '\n' { ' ' } else { ch })
                    .collect(),
            );
            index = end + 1;
        }
        tags.push(JsDocTag {
            tag,
            ty,
            name: jsdoc_name(&text[index.min(text.len())..]),
            optional: text[index.min(text.len())..]
                .iter()
                .find(|&&ch| !u16::try_from(u32::from(ch)).is_ok_and(is_js_space))
                == Some(&'['),
        });
        at = index;
    }
    Ok(tags)
}

/// The next `@tag` at or after `from`: the positions of the `@` and after the letters.
fn next_tag(text: &[char], from: usize) -> Option<(usize, usize)> {
    (from..text.len()).find_map(|start| {
        if text[start] != '@' {
            return None;
        }
        let letters = text[start + 1..]
            .iter()
            .take_while(|ch| ch.is_ascii_alphabetic())
            .count();
        (letters > 0).then_some((start, start + 1 + letters))
    })
}

/// Read the shared `JSDoc` name range; convert its UTF-16 offsets at the host boundary.
fn jsdoc_name(text: &[char]) -> Option<String> {
    let tail: String = text.iter().collect();
    let units: Vec<u16> = tail.encode_utf16().collect();
    let numeric: Vec<f64> = units.iter().map(|&unit| f64::from(unit)).collect();
    let range = super::frontend_rules::find_documentation_parameter_range(&numeric);
    if range.is_empty() {
        return None;
    }
    let start = super::frontend_rules::ml_array::number_index(range[0])?;
    let end = super::frontend_rules::ml_array::number_index(range[1])?;
    Some(String::from_utf16_lossy(units.get(start..end)?))
}

/// `/^[A-Za-z_$][\w$]*$/`.
fn is_identifier_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_' || first == '$')
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '$')
}

/// `String.prototype.trim`: JavaScript whitespace and line terminators.
fn js_trim(text: &str) -> &str {
    text.trim_matches(|ch: char| u16::try_from(u32::from(ch)).is_ok_and(is_js_space))
}

/// A `JSDoc` type, where an empty one counts as missing.
fn non_empty(text: Option<String>) -> Option<String> {
    text.filter(|text| !text.is_empty())
}

/// `5n` and `0x1fn` are `BigInt` literals, which the lexer keeps as decimal
/// digits; `42`, `1.5` and `1e21` are Numbers, IEEE-754 doubles, which the
/// lexer keeps as `String(value)`.
fn number_literal(token: &Token) -> SExpr {
    node(
        SNode::Num {
            value: token.value.clone(),
            ty: (token.suffix != "n").then_some(FLOAT),
            negative: false,
            unit: false,
        },
        span(token, token),
    )
}

fn has_ctor(data: &SData, tag: &str) -> bool {
    data.ctors.iter().any(|ctor| ctor.name == tag)
}

fn is_tag_field(expr: &SExpr) -> bool {
    matches!(&expr.node, SNode::Field { field, .. } if field == "$")
}

const fn node(node: SNode, place: Span) -> SExpr {
    SExpr::new(node, Some(place))
}

const fn wild(place: Span) -> SPattern {
    SPattern {
        node: SPatternNode::Wild,
        span: Some(place),
    }
}

fn binary(op: BinaryOp, left: SExpr, right: SExpr, token: &Token) -> SExpr {
    let place = joined(&left, &right, token);
    node(
        SNode::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
            rounding: None,
        },
        place,
    )
}

fn joined(left: &SExpr, right: &SExpr, token: &Token) -> Span {
    Span::new(
        left.span.map_or(token.start, |place| place.start),
        right.span.map_or(token.end, |place| place.end),
    )
}

fn span(from: &Token, to: &Token) -> Span {
    let end = if to.kind == TokenKind::Eof {
        from.end
    } else {
        to.start
    };
    Span::new(from.start, from.end.max(end))
}
