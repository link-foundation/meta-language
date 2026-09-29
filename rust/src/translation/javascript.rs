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

use std::collections::HashSet;

use super::diagnostics::{Result, TranslationError, type_error, unsupported};
use super::lexer::{
    Comment, Source, Token, TokenCursor, TokenKind, describe, is_js_space, tokenize,
};
use super::surface::{
    BinaryOp, SArrayItem, SComparison, SCtor, SData, SEffect, SExpr, SField, SFn, SItem, SMain,
    SModule, SNode, SParam, SPattern, SPatternNode, SProgram, SProp, SPropNode, SRow, STagTest,
    ShowStyle, UnaryOp,
};
use super::types::{BOOL, FLOAT, INT, NAT, STRING, Type, array};
use super::{Language, Span};

mod console;
mod declarations;
mod expressions;
mod flow;
mod imperative;
mod infer;
mod loops;
mod lowering;
mod math;
mod statements;
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
    let tokens = tokenize(source, Language::JavaScript)?;
    JavaScriptParser::new(source, tokens.tokens, &tokens.comments).file()
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
    returns: Option<String>,
    range: Span,
}

/// A `JSDoc` tag: `@tag {type} name`.
struct JsDocTag {
    tag: String,
    ty: Option<String>,
    name: Option<String>,
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
}

/// Where a block-declaration scan stops: the end of the file or the block's `}`.
#[derive(Clone, Copy)]
enum ScanEnd {
    File,
    Brace,
}

struct JavaScriptParser {
    source: Source,
    cursor: TokenCursor,
    docs: Vec<Comment>,
    /// Tags of every @typedef data type, for `switch (x.$)`.
    data_types: Vec<SData>,
    scope: Scope,
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
    /// Functions and data types that loops and joins of statements lower to, and their count.
    generated: Vec<SItem>,
    generated_count: usize,
}

impl JavaScriptParser {
    fn new(source: &str, tokens: Vec<Token>, comments: &[Comment]) -> Self {
        Self {
            source: Source::new(source),
            cursor: TokenCursor::new(tokens, Language::JavaScript),
            docs: comments
                .iter()
                .filter(|comment| comment.text.starts_with("/**") && comment.text != "/**/")
                .cloned()
                .collect(),
            data_types: Vec::new(),
            scope: Scope::default(),
            assertion: None,
            async_names: HashSet::new(),
            in_async: true,
            unawaited: Vec::new(),
            sequential_async: false,
            jumps: Jumps::default(),
            top_level: false,
            generated: Vec::new(),
            generated_count: 0,
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

/// `^[ \t]*([A-Za-z_$][\w$]*)`.
fn jsdoc_name(text: &[char]) -> Option<String> {
    let start = text
        .iter()
        .take_while(|&&ch| ch == ' ' || ch == '\t')
        .count();
    let first = *text.get(start)?;
    if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
        return None;
    }
    Some(
        text[start..]
            .iter()
            .take_while(|&&ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '$')
            .collect(),
    )
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
