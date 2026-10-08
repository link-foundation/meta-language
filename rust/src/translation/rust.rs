//! The Rust frontend: reads a Rust program into the surface AST.
//!
//! It reads the subset of Rust the core can represent faithfully: modules
//! with `use` of crate items, enums with unit and tuple variants (`Box<T>`
//! fields are the recursive occurrences of a value type), free functions over
//! machine integers, `bool` and `String`, expression bodies with `let`, `if`,
//! `match` and `format!`, and a `main` made of `println!`, `let` and
//! `assert!`/`assert_eq!` statements. Everything else is rejected with a
//! precise obligation.
//!
//! Mirrors `js/src/translation/rust.js`.

use std::collections::HashMap;

use super::aborts::rust_macro_message;
use super::diagnostics::{Result, TranslationError, type_error, unsupported};
use super::lexer::{Token, TokenCursor, TokenKind, describe, tokenize};
use super::surface::{
    BinaryOp, Flavor, Rounding, SComparison, SCtor, SData, SEffect, SExpr, SField, SFn, SItem,
    SMain, SModule, SNode, SParam, SPattern, SPatternNode, SProgram, SProp, SPropNode, SRow,
    ShowStyle, UnaryOp,
};
use super::types::{BOOL, STRING, Type, UNIT, rust_fixed_type};
use super::{Language, Span};

mod expressions;
mod items;

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
const UNSUPPORTED_TYPES: [&str; 11] = [
    "str", "char", "f32", "f64", "Vec", "Option", "Result", "Rc", "Arc", "HashMap", "Self",
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

/// Reads a Rust program.
///
/// # Errors
/// On syntax errors and on constructs outside the portable core.
pub fn parse_rust(source: &str) -> Result<SProgram> {
    let tokens = tokenize(source, Language::Rust)?.tokens;
    RustParser::new(tokens).file()
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

struct RustParser {
    cursor: TokenCursor,
    /// `use` aliases, per module path.
    aliases: HashMap<String, HashMap<String, Vec<String>>>,
    module_path: Vec<String>,
    main: Option<SMain>,
}

impl RustParser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            cursor: TokenCursor::new(tokens, Language::Rust),
            aliases: HashMap::new(),
            module_path: Vec::new(),
            main: None,
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
