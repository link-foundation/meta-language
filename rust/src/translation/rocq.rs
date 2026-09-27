//! The Rocq frontend for the portable core.
//!
//! It reads the vernacular subset the core can represent faithfully:
//! modules, inductive types, `Definition`, `Fixpoint` and measure-based
//! `Function` declarations over `nat`, `N`, `Z`, `bool` and `string`,
//! theorems with their tactic structure, and a program
//! `Definition main : list string` whose elements are the lines the program
//! prints. Everything else is rejected with a precise obligation.
//!
//! Mirrors `js/src/translation/rocq.js`.

use std::collections::HashMap;

use super::diagnostics::{type_error, unsupported, Result, TranslationError};
use super::lexer::{describe, is_js_space, tokenize_source, Source, Token, TokenCursor, TokenKind};
use super::surface::{
    BinaryOp, Flavor, Rounding, SBinder, SComparison, SCtor, SData, SEffect, SExpr, SField, SFn,
    SItem, SMain, SModule, SNode, SParam, SPattern, SPatternNode, SProgram, SProof, SProp,
    SPropNode, SRow, SRule, SSplit, SSplitCase, SStep, STheorem, ShowStyle, UnaryOp,
};
use super::types::{Type, BOOL, INT, NAT, STRING, UNIT};
use super::{Language, Span};

mod portable;
mod proofs;
mod terms;
mod vernacular;
use self::portable::{
    check_portable, collect_forall_types, pattern_value, scope_prop, walk_output, with_scope,
};

const THEOREM_KEYWORDS: &[&str] = &[
    "Theorem",
    "Lemma",
    "Example",
    "Fact",
    "Remark",
    "Corollary",
    "Proposition",
];
const UNSUPPORTED_COMMANDS: &[&str] = &[
    "Section",
    "Variable",
    "Variables",
    "Hypothesis",
    "Context",
    "Record",
    "Structure",
    "Class",
    "Instance",
    "CoInductive",
    "CoFixpoint",
    "Axiom",
    "Parameter",
    "Notation",
    "Infix",
    "Ltac",
    "Program",
    "Let",
    "Arguments",
    "Hint",
    "Set",
    "Unset",
    "Global",
    "Local",
    "Module Type",
    "Include",
    "Export",
    "Canonical",
    "Coercion",
    "Scheme",
    "Equations",
    "Declare",
];
const EXPRESSION_STOP: &[&str] = &["then", "else", "with", "end", "in", "as", "return"];
const EVAL_STRATEGIES: &[&str] = &[
    "vm_compute",
    "compute",
    "cbv",
    "lazy",
    "native_compute",
    "cbn",
    "simpl",
];

fn builtin_type(name: &str) -> Option<Type> {
    match name {
        "nat" | "N" => Some(NAT),
        "Z" => Some(INT),
        "bool" => Some(BOOL),
        "string" => Some(STRING),
        "unit" => Some(UNIT),
        _ => None,
    }
}

/// Rocq types with no portable counterpart.
const OUTSIDE_CORE_TYPES: &[&str] = &[
    "list",
    "option",
    "prod",
    "sum",
    "positive",
    "ascii",
    "Type",
    "Set",
    "Prop",
    "sig",
    "sigT",
    "Q",
    "R",
    "vector",
    "int",
    "uint",
    "float",
    "PrimInt63.int",
    "Uint63.int",
];

// Library functions of the portable core, by Rocq name. Rounding follows
// the Stdlib definition: `Z.div`/`Z.modulo` floor, `Z.quot`/`Z.rem` truncate,
// and the natural-number divisions are total with `x / 0 = 0`.
fn binary_function(name: &str) -> Option<(BinaryOp, Option<Rounding>)> {
    Some(match name {
        "N.add" | "Z.add" | "Nat.add" | "plus" => (BinaryOp::Add, None),
        "N.sub" | "Z.sub" | "Nat.sub" | "minus" => (BinaryOp::Sub, None),
        "N.mul" | "Z.mul" | "Nat.mul" | "mult" => (BinaryOp::Mul, None),
        "N.div" | "Nat.div" => (BinaryOp::Div, None),
        "Z.div" => (BinaryOp::Div, Some(Rounding::Floor)),
        "N.modulo" | "Nat.modulo" => (BinaryOp::Rem, None),
        "Z.modulo" => (BinaryOp::Rem, Some(Rounding::Floor)),
        "Z.quot" => (BinaryOp::Div, Some(Rounding::Trunc)),
        "Z.rem" => (BinaryOp::Rem, Some(Rounding::Trunc)),
        "N.eqb" | "Z.eqb" | "Nat.eqb" | "String.eqb" | "Bool.eqb" | "eqb" => (BinaryOp::Eq, None),
        "N.ltb" | "Z.ltb" | "Nat.ltb" => (BinaryOp::Lt, None),
        "N.leb" | "Z.leb" | "Nat.leb" => (BinaryOp::Le, None),
        "Z.gtb" => (BinaryOp::Gt, None),
        "Z.geb" => (BinaryOp::Ge, None),
        "andb" => (BinaryOp::And, None),
        "orb" => (BinaryOp::Or, None),
        "String.append" => (BinaryOp::Concat, None),
        _ => return None,
    })
}

const NAT_TO_INT: &[&str] = &["Z.of_N", "Z.of_nat"];
const INT_TO_NAT: &[&str] = &["Z.to_N", "Z.to_nat"];
const NAT_IDENTITY: &[&str] = &["N.of_nat", "N.to_nat", "Nat.of_N"];
const PREDECESSOR: &[&str] = &["N.pred", "Nat.pred", "pred", "Z.pred"];
const SUCCESSOR: &[&str] = &["N.succ", "Nat.succ", "S", "Z.succ"];
// `NilEmpty.string_of_uint (N.to_uint n)` and `NilZero.string_of_int
// (Z.to_int z)` are the Stdlib's decimal renderings, the same text as the
// other languages' number printing.
const DECIMAL_STRINGS: &[(&str, &[&str])] = &[
    ("NilEmpty.string_of_uint", &["N.to_uint", "Nat.to_uint"]),
    (
        "DecimalString.NilEmpty.string_of_uint",
        &["N.to_uint", "Nat.to_uint"],
    ),
    ("NilZero.string_of_int", &["Z.to_int"]),
    ("DecimalString.NilZero.string_of_int", &["Z.to_int"]),
];

/// `/^(?:N|Z|Nat|String|Pos|List|Bool|Ascii)\./`: the Stdlib modules whose
/// other functions are outside the portable core.
fn outside_core(name: &str) -> bool {
    ["N", "Z", "Nat", "String", "Pos", "List", "Bool", "Ascii"]
        .iter()
        .any(|module| {
            name.strip_prefix(module)
                .is_some_and(|rest| rest.starts_with('.'))
        })
}

/// `/^(?:N|Z|Nat)\.to_u?int$/`: the decimal conversions `DECIMAL_STRINGS` consume.
fn decimal_conversion(name: &str) -> bool {
    matches!(
        name,
        "N.to_int" | "N.to_uint" | "Z.to_int" | "Z.to_uint" | "Nat.to_int" | "Nat.to_uint"
    )
}

/// The names `measureFn in {}` finds on `Object.prototype` (Node 24).
fn object_prototype_key(name: &str) -> bool {
    matches!(
        name,
        "constructor"
            | "__defineGetter__"
            | "__defineSetter__"
            | "hasOwnProperty"
            | "__lookupGetter__"
            | "__lookupSetter__"
            | "isPrototypeOf"
            | "propertyIsEnumerable"
            | "toString"
            | "valueOf"
            | "__proto__"
            | "toLocaleString"
    )
}

/// Levels 50 (`|| + -`) and 40 (`&& * / mod`) of the notation table.
fn notation(level: u32, token: &Token) -> Option<BinaryOp> {
    if token.kind != TokenKind::Punct && token.value != "mod" {
        return None;
    }
    match (level, token.value.as_str()) {
        (50, "||") => Some(BinaryOp::Or),
        (50, "+") => Some(BinaryOp::Add),
        (50, "-") => Some(BinaryOp::Sub),
        (40, "&&") => Some(BinaryOp::And),
        (40, "*") => Some(BinaryOp::Mul),
        (40, "/") => Some(BinaryOp::Div),
        (40, "mod") => Some(BinaryOp::Rem),
        _ => None,
    }
}

fn boolean_relation(value: &str) -> Option<BinaryOp> {
    match value {
        "=?" => Some(BinaryOp::Eq),
        "<?" => Some(BinaryOp::Lt),
        "<=?" => Some(BinaryOp::Le),
        _ => None,
    }
}

fn prop_relation(value: &str, comparison: SComparison) -> Option<SPropNode> {
    Some(match value {
        "=" => SPropNode::Eq(comparison),
        "<>" => SPropNode::Ne(comparison),
        "<" => SPropNode::Lt(comparison),
        "<=" => SPropNode::Le(comparison),
        ">" => SPropNode::Gt(comparison),
        ">=" => SPropNode::Ge(comparison),
        _ => return None,
    })
}

fn is_prop_relation(value: &str) -> bool {
    matches!(value, "=" | "<>" | "<" | "<=" | ">" | ">=")
}

fn scope_type(scope: &str) -> Option<Type> {
    match scope {
        "N" | "nat" => Some(NAT),
        "Z" => Some(INT),
        _ => None,
    }
}

/// Reads a Rocq program.
///
/// # Errors
/// On syntax errors and on constructs outside the portable core.
pub fn parse_rocq(source: &str) -> Result<SProgram> {
    let source = Source::new(source);
    let tokens = tokenize_source(&source, Language::Rocq)?.tokens;
    RocqParser {
        cursor: TokenCursor::new(tokens, Language::Rocq),
        source,
    }
    .file()
}

/// A `Definition`, `Fixpoint` or `Function`: a function, or the program output.
enum Definition {
    Fn(Box<SFn>),
    Main(SMain, Span),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Annotation {
    Struct,
    Measure,
}

/// A run of adjacent `-`, `+` or `*` tokens.
struct Bullet {
    text: String,
    length: usize,
    depth: usize,
}

/// Whether a proof block has reached its end.
type Done<'a> = &'a dyn Fn(&RocqParser) -> bool;

/// Rocq names each proof variable's type as written, which proof steps consult.
type ProofTypes = HashMap<String, String>;

struct RocqParser {
    cursor: TokenCursor,
    source: Source,
}

impl RocqParser {
    fn fail(message: &str, token: &Token) -> TranslationError {
        TranslationError::syntax(
            format!("{message} but found {}", describe(token)),
            Some(token.span()),
        )
    }

    fn fail_here(&self, message: &str) -> TranslationError {
        Self::fail(message, self.cursor.peek())
    }

    fn peek(&self) -> &Token {
        self.cursor.peek()
    }

    fn is(&self, value: &str) -> bool {
        self.cursor.is(value)
    }

    /// `cursor.expect`, where an empty context (a JavaScript falsy string) is none.
    fn expect(&mut self, value: &str, context: &str) -> Result<Token> {
        self.cursor.expect(value, context_of(context))
    }

    fn identifier(&mut self, context: &str) -> Result<Token> {
        self.cursor.identifier(context_of(context))
    }

    /// The `.` that ends a sentence.
    fn end_sentence(&mut self, context: &str) -> Result<()> {
        self.expect(".", context).map(drop)
    }
}

/// A JavaScript context string: the empty string counts as none.
fn context_of(context: &str) -> Option<&str> {
    (!context.is_empty()).then_some(context)
}

/// The arguments of a library function of fixed arity.
fn arity<const N: usize>(name: &str, args: Vec<SExpr>, range: Span) -> Result<[SExpr; N]> {
    let count = args.len();
    <[SExpr; N]>::try_from(args).map_err(|_| {
        if count < N {
            unsupported(
                "partial application",
                &format!("{name} expects {N} arguments"),
                Some(range),
            )
        } else {
            type_error(
                format!("{name} expects {N} arguments but got {count}"),
                Some(range),
            )
        }
    })
}

/// A proposition connective, which carries no span.
const fn connective(node: SPropNode) -> SProp {
    SProp { node, span: None }
}

fn unreachable_relation(value: &str) -> SPropNode {
    unreachable!("{value} is a proposition relation")
}

fn binary(op: BinaryOp, left: SExpr, right: SExpr, token: &Token) -> SExpr {
    let binary_span = joined(&left, &right, token);
    SExpr::new(
        SNode::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
            rounding: None,
        },
        Some(binary_span),
    )
}

/// `String.prototype.trim` of `source.slice(start, end)`.
fn js_trim(source: &Source, start: usize, end: usize) -> String {
    let units = source.units();
    let end = end.min(units.len());
    let mut first = start.min(end);
    let mut last = end;
    while first < last && is_js_space(units[first]) {
        first += 1;
    }
    while last > first && is_js_space(units[last - 1]) {
        last -= 1;
    }
    source.slice(first, last)
}

fn joined(left: &SExpr, right: &SExpr, token: &Token) -> Span {
    Span {
        start: left.span.map_or(token.start, |span| span.start),
        end: right.span.map_or(token.end, |span| span.end),
    }
}

/// From the start of `from` to the start of `to` (or the end of `from`, when
/// `to` is the end of input or precedes it).
fn span(from: &Token, to: &Token) -> Span {
    let end = if to.kind == TokenKind::Eof {
        from.end
    } else {
        to.start
    };
    Span {
        start: from.start,
        end: from.end.max(end),
    }
}
