//! The Lean 4 frontend for the portable core.
//!
//! It reads the layout-sensitive subset that the core can represent
//! faithfully: namespaces, inductive types, definitions (including
//! equation-compiler style), `match`, `if`, `let`, theorems with their tactic
//! structure, and a `main : IO Unit` of `IO.println` effects. Everything else
//! is rejected with a precise obligation.
//!
//! Mirrors `js/src/translation/lean.js`.

use super::diagnostics::{type_error, unsupported, Result, TranslationError};
use super::lexer::{describe, is_js_space, tokenize_source, Source, Token, TokenCursor, TokenKind};
use super::surface::{
    BinaryOp, SBinder, SComparison, SCtor, SData, SEffect, SExpr, SField, SFn, SItem, SMain,
    SModule, SNode, SParam, SPattern, SPatternNode, SProgram, SProp, SPropNode, ShowStyle,
};
use super::types::{Type, BOOL, INT, NAT, STRING, UNIT};
use super::{Language, Span};

mod expressions;
mod proofs;

const ITEM_KEYWORDS: [&str; 28] = [
    "namespace",
    "end",
    "inductive",
    "def",
    "theorem",
    "lemma",
    "structure",
    "class",
    "instance",
    "abbrev",
    "open",
    "section",
    "variable",
    "universe",
    "import",
    "mutual",
    "partial",
    "private",
    "protected",
    "noncomputable",
    "set_option",
    "example",
    "axiom",
    "opaque",
    "macro",
    "syntax",
    "notation",
    "attribute",
];

fn is_item_keyword(value: &str) -> bool {
    ITEM_KEYWORDS.contains(&value)
}

fn builtin_type(name: &str) -> Option<Type> {
    match name {
        "Nat" => Some(NAT),
        "Int" => Some(INT),
        "Bool" => Some(BOOL),
        "String" => Some(STRING),
        "Unit" => Some(UNIT),
        _ => None,
    }
}

/// Lean types the portable core has no counterpart for.
fn is_unsupported_type(name: &str) -> bool {
    let fixed = name
        .strip_prefix('U')
        .unwrap_or(name)
        .strip_prefix("Int")
        .is_some_and(|bits| matches!(bits, "8" | "16" | "32" | "64"));
    fixed
        || matches!(
            name,
            "USize" | "Float" | "Char" | "List" | "Array" | "Option" | "Prop" | "Type" | "Sort"
        )
}

struct Level {
    ops: &'static [&'static str],
    prec: u32,
    op: Option<BinaryOp>,
    right: bool,
}

const BINARY: [Level; 6] = [
    Level {
        ops: &["||"],
        prec: 30,
        op: Some(BinaryOp::Or),
        right: false,
    },
    Level {
        ops: &["&&"],
        prec: 35,
        op: Some(BinaryOp::And),
        right: false,
    },
    Level {
        ops: &["==", "!=", "=", "≠", "<", "<=", "≤", ">", ">=", "≥"],
        prec: 50,
        op: None,
        right: false,
    },
    Level {
        ops: &["++"],
        prec: 65,
        op: Some(BinaryOp::Concat),
        right: true,
    },
    Level {
        ops: &["+", "-"],
        prec: 65,
        op: None,
        right: false,
    },
    Level {
        ops: &["*", "/", "%"],
        prec: 70,
        op: None,
        right: false,
    },
];

fn operator_name(op: &str) -> Option<BinaryOp> {
    Some(match op {
        "==" | "=" => BinaryOp::Eq,
        "!=" | "≠" => BinaryOp::Ne,
        "<" => BinaryOp::Lt,
        "<=" | "≤" => BinaryOp::Le,
        ">" => BinaryOp::Gt,
        ">=" | "≥" => BinaryOp::Ge,
        "+" => BinaryOp::Add,
        "-" => BinaryOp::Sub,
        "*" => BinaryOp::Mul,
        "/" => BinaryOp::Div,
        "%" => BinaryOp::Rem,
        _ => return None,
    })
}

type Relation = fn(SComparison) -> SPropNode;

fn prop_relation(op: &str) -> Option<Relation> {
    Some(match op {
        "=" => SPropNode::Eq,
        "≠" => SPropNode::Ne,
        "<" => SPropNode::Lt,
        "≤" | "<=" => SPropNode::Le,
        ">" => SPropNode::Gt,
        "≥" | ">=" => SPropNode::Ge,
        _ => return None,
    })
}

/// Reads a Lean program.
///
/// # Errors
/// On syntax errors and on constructs outside the portable core.
pub fn parse_lean(source: &str) -> Result<SProgram> {
    let source = Source::new(source);
    let mut tokens = tokenize_source(&source, Language::Lean)?.tokens;
    annotate_layout(&mut tokens, source.units());
    LeanParser::new(tokens, &source).file()
}

/// Fills in each token's line, column and whether it starts its line.
pub fn annotate_layout(tokens: &mut [Token], source: &[u16]) {
    let mut line = 0;
    let mut line_start = 0;
    let mut cursor = 0;
    let mut previous_line = None;
    for token in tokens {
        while cursor < token.start {
            if source.get(cursor) == Some(&u16::from(b'\n')) {
                line += 1;
                line_start = cursor + 1;
            }
            cursor += 1;
        }
        token.line = line;
        token.col = token.start - line_start;
        token.first = previous_line != Some(line);
        previous_line = Some(line);
    }
}

/// The range from `from` up to the start of `to` (the end of input adds nothing).
#[must_use]
pub fn span(from: &Token, to: &Token) -> Span {
    let reach = if to.kind == TokenKind::Eof {
        from.end
    } else {
        to.start
    };
    Span::new(from.start, from.end.max(reach))
}

/// A token's column, comparable with the `-1` that no column reaches.
fn column(token: &Token) -> i64 {
    i64::try_from(token.col).unwrap_or(i64::MAX)
}

fn syntax_at(message: String, token: &Token) -> TranslationError {
    TranslationError::syntax(message, Some(token.span()))
}

/// `source.slice(start, end).trim()` as JavaScript trims.
fn js_trimmed(units: &[u16]) -> String {
    let start = units
        .iter()
        .position(|&unit| !is_js_space(unit))
        .unwrap_or(units.len());
    let end = units
        .iter()
        .rposition(|&unit| !is_js_space(unit))
        .map_or(start, |index| index + 1);
    String::from_utf16_lossy(&units[start..end.max(start)])
}

const fn expr(node: SNode, at: Span) -> SExpr {
    SExpr::new(node, Some(at))
}

const fn pattern(node: SPatternNode, at: Option<Span>) -> SPattern {
    SPattern { node, span: at }
}

/// A qualified pattern head: `Tree.node`, or `.node` resolved against the scrutinee.
struct PatternHead {
    path: Vec<String>,
    dot: bool,
}

enum Definition {
    Fn(Box<SFn>),
    Main(SMain, Span),
}

struct LeanParser<'a> {
    cursor: TokenCursor,
    source: &'a Source,
    bounds: Vec<i64>,
}

impl<'a> LeanParser<'a> {
    fn new(tokens: Vec<Token>, source: &'a Source) -> Self {
        Self {
            cursor: TokenCursor::new(tokens, Language::Lean),
            source,
            bounds: vec![-1],
        }
    }

    fn bound(&self) -> i64 {
        self.bounds.last().copied().unwrap_or(-1)
    }

    fn with_bound<T>(
        &mut self,
        column: i64,
        parse: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<T> {
        self.bounds.push(column);
        let result = parse(self);
        self.bounds.pop();
        result
    }

    /// True when the token closes the current layout block.
    fn blocked(&self, token: &Token) -> bool {
        if token.kind == TokenKind::Eof {
            return true;
        }
        if token.col == 0
            && token.first
            && token.kind == TokenKind::Identifier
            && is_item_keyword(&token.value)
        {
            return true;
        }
        token.first && column(token) <= self.bound()
    }

    fn blocked_next(&self) -> bool {
        self.blocked(self.cursor.peek())
    }

    /// Alternatives may sit at the column of the construct that opens them.
    fn outdented(&self, at: i64) -> bool {
        let token = self.cursor.peek();
        token.kind == TokenKind::Eof || (token.first && column(token) < at)
    }

    fn fail(&self, message: &str) -> TranslationError {
        Self::fail_at(message, self.cursor.peek())
    }

    fn fail_at(message: &str, token: &Token) -> TranslationError {
        syntax_at(format!("{message} but found {}", describe(token)), token)
    }

    fn peek(&self) -> Token {
        self.cursor.peek().clone()
    }

    fn span_to_next(&self, from: &Token) -> Span {
        span(from, self.cursor.peek())
    }

    fn file(mut self) -> Result<SProgram> {
        let mut items = Vec::new();
        let mut main = None;
        self.items(&mut main, &mut items, &[])?;
        Ok(SProgram {
            language: Language::Lean,
            items,
            main,
        })
    }

    fn items(
        &mut self,
        main: &mut Option<SMain>,
        items: &mut Vec<SItem>,
        path: &[String],
    ) -> Result<()> {
        while !self.cursor.at_end() {
            let token = self.peek();
            if self.cursor.is("end") {
                let Some(open) = path.last() else {
                    return Err(self.fail("unmatched end"));
                };
                self.cursor.advance();
                let name = self.qualified_name()?.join(".");
                if name != *open {
                    return Err(syntax_at(
                        format!("end {name} closes namespace {open}"),
                        &token,
                    ));
                }
                return Ok(());
            }
            if self.cursor.is("namespace") {
                self.cursor.advance();
                let mut segments = self.qualified_name()?;
                let module_span = self.span_to_next(&token);
                if segments.len() != 1 {
                    return Err(unsupported(
                        "dotted namespace",
                        "use one namespace per level",
                        Some(span(&token, &token)),
                    ));
                }
                let name = segments.remove(0);
                let mut module = SModule {
                    name: name.clone(),
                    items: Vec::new(),
                    span: Some(module_span),
                };
                let mut inner = path.to_vec();
                inner.push(name);
                self.items(main, &mut module.items, &inner)?;
                items.push(SItem::Module(module));
                continue;
            }
            if self.cursor.is("inductive") {
                items.push(SItem::Data(self.inductive()?));
                continue;
            }
            if self.cursor.is("def") {
                match self.definition(path)? {
                    Definition::Main(program_main, at) => {
                        if !path.is_empty() {
                            return Err(unsupported(
                                "namespaced main",
                                "main must be declared at the top level",
                                Some(at),
                            ));
                        }
                        if main.is_some() {
                            return Err(type_error("duplicate main", Some(at)));
                        }
                        *main = Some(program_main);
                    }
                    Definition::Fn(item) => items.push(SItem::Fn(*item)),
                }
                continue;
            }
            if self.cursor.is("theorem") {
                items.push(SItem::Theorem(self.theorem()?));
                continue;
            }
            if token.kind == TokenKind::Identifier && is_item_keyword(&token.value) {
                return Err(unsupported(
                    &format!("Lean {} declaration", token.value),
                    "outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            return Err(self.fail("expected a declaration"));
        }
        if let Some(open) = path.last() {
            return Err(syntax_at(
                format!("namespace {open} is not closed"),
                self.cursor.peek(),
            ));
        }
        Ok(())
    }

    fn qualified_name(&mut self) -> Result<Vec<String>> {
        let token = self.cursor.identifier(Some("name"))?;
        Ok(token.value.split('.').map(str::to_owned).collect())
    }

    fn inductive(&mut self) -> Result<SData> {
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("inductive"))?.value;
        if name.contains('.') {
            return Err(unsupported(
                "qualified inductive name",
                "declare inductives inside their namespace",
                Some(span(&start, &start)),
            ));
        }
        if !self.cursor.is("where") {
            if self.cursor.is("(") || self.cursor.is("{") {
                return Err(unsupported(
                    "parameterised inductive",
                    "type parameters are outside the portable core",
                    Some(self.span_to_next(&start)),
                ));
            }
            self.cursor.expect("where", Some("inductive"))?;
        }
        self.cursor.advance();
        let mut ctors = Vec::new();
        while self.cursor.is("|") {
            let bar = self.cursor.advance();
            let ctor_name = self.cursor.identifier(Some("constructor"))?.value;
            let mut fields = Vec::new();
            while self.cursor.is("(") {
                fields.extend(self.binder_group()?.into_iter().map(|(name, ty)| SField {
                    name: Some(name),
                    ty,
                    rocq_type: None,
                    span: None,
                }));
            }
            if self.cursor.eat(":").is_some() {
                let mut types = self.arrow_type()?;
                let result = types.pop();
                let constructs_self = matches!(
                    &result,
                    Some(Type::Named { path, .. }) if path.join(".") == name
                );
                if !constructs_self {
                    return Err(unsupported(
                        "indexed constructor",
                        &format!("{ctor_name} must construct {name}"),
                        Some(self.span_to_next(&bar)),
                    ));
                }
                fields.extend(types.into_iter().map(|ty| SField {
                    name: None,
                    ty,
                    rocq_type: None,
                    span: None,
                }));
            }
            ctors.push(SCtor {
                name: ctor_name,
                fields,
            });
        }
        if self.cursor.is("deriving") {
            self.cursor.advance();
            self.cursor.identifier(Some("deriving"))?;
            while self.cursor.eat(",").is_some() {
                self.cursor.identifier(Some("deriving"))?;
            }
        }
        Ok(SData {
            name,
            ctors,
            span: Some(self.span_to_next(&start)),
        })
    }

    fn binder_group(&mut self) -> Result<Vec<(String, Type)>> {
        let open = self.cursor.expect("(", Some("binder"))?;
        let mut names = Vec::new();
        while self.cursor.is_kind(TokenKind::Identifier) {
            names.push(self.cursor.advance().value);
        }
        if names.is_empty() {
            return Err(self.fail("expected binder names"));
        }
        self.cursor.expect(":", Some("binder"))?;
        let ty = self.ty()?;
        self.cursor.expect(")", Some("binder"))?;
        if self.cursor.is("{") || self.cursor.is("[") {
            return Err(unsupported(
                "implicit binder",
                "implicit and instance binders are outside the portable core",
                Some(self.span_to_next(&open)),
            ));
        }
        Ok(names.into_iter().map(|name| (name, ty.clone())).collect())
    }

    fn binders(&mut self) -> Result<Vec<(String, Type)>> {
        let mut result = Vec::new();
        while self.cursor.is("(") || self.cursor.is("{") || self.cursor.is("[") {
            if !self.cursor.is("(") {
                let token = self.cursor.peek();
                return Err(unsupported(
                    "implicit binder",
                    "implicit and instance binders are outside the portable core",
                    Some(span(token, token)),
                ));
            }
            result.extend(self.binder_group()?);
        }
        Ok(result)
    }

    fn arrow_type(&mut self) -> Result<Vec<Type>> {
        let mut types = vec![self.ty()?];
        while self.cursor.eat("→").is_some() || self.cursor.eat("->").is_some() {
            types.push(self.ty()?);
        }
        Ok(types)
    }

    fn ty(&mut self) -> Result<Type> {
        let token = self.peek();
        if self.cursor.eat("(").is_some() {
            let mut types = self.arrow_type()?;
            self.cursor.expect(")", Some("type"))?;
            if types.len() != 1 {
                return Err(unsupported(
                    "function type",
                    "higher-order values are outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            return Ok(types.remove(0));
        }
        let name = self.cursor.identifier(Some("type"))?.value;
        if let Some(builtin) = builtin_type(&name) {
            return Ok(builtin);
        }
        if is_unsupported_type(&name) {
            return Err(unsupported(
                &format!("Lean type {name}"),
                "outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        let next = self.cursor.peek();
        if !self.blocked(next)
            && next.kind == TokenKind::Identifier
            && !["where", ":=", "with", "deriving"].contains(&next.value.as_str())
        {
            return Err(unsupported(
                "type application",
                &format!("{name} {} is outside the portable core", next.value),
                Some(span(&token, next)),
            ));
        }
        Ok(Type::Named {
            path: name.split('.').map(str::to_owned).collect(),
            span: Some(span(&token, &token)),
        })
    }

    fn definition(&mut self, path: &[String]) -> Result<Definition> {
        let start = self.cursor.advance();
        let name_token = self.cursor.identifier(Some("def"))?;
        let name = name_token.value.clone();
        if name.contains('.') {
            return Err(unsupported(
                "qualified definition name",
                "declare definitions inside their namespace",
                Some(span(&name_token, &name_token)),
            ));
        }
        let params: Vec<SParam> = self
            .binders()?
            .into_iter()
            .map(|(name, ty)| param(name, ty))
            .collect();
        self.cursor.expect(":", Some("def"))?;
        if name == "main" && path.is_empty() && self.cursor.is("IO") {
            return self.main(&start);
        }
        let mut types = self.arrow_type()?;
        let ret = types.pop();
        let extra: Vec<SParam> = types
            .into_iter()
            .enumerate()
            .map(|(index, ty)| param(format!("ml_arg{index}"), ty))
            .collect();
        if self.cursor.eat(":=").is_some() {
            if !extra.is_empty() {
                return Err(unsupported(
                    "function-valued definition body",
                    "use binders or equations for arrow types",
                    Some(self.span_to_next(&start)),
                ));
            }
            let body = self.with_bound(0, Self::expr)?;
            self.end_decl(&start, "termination_by")?;
            return Ok(Definition::Fn(Box::new(SFn {
                name,
                params,
                ret,
                body,
                span: Some(self.span_to_next(&start)),
            })));
        }
        if !self.cursor.is("|") {
            return Err(self.fail("expected := or equations"));
        }
        if extra.is_empty() {
            return Err(unsupported(
                "equations without arrow arguments",
                "use match",
                Some(self.span_to_next(&start)),
            ));
        }
        let rows = self.alternatives(extra.len())?;
        let body = expr(
            SNode::Match {
                scrutinees: extra.iter().map(|param| SExpr::name(&param.name)).collect(),
                rows,
            },
            self.span_to_next(&start),
        );
        self.end_decl(&start, "termination_by")?;
        Ok(Definition::Fn(Box::new(SFn {
            name,
            params: params.into_iter().chain(extra).collect(),
            ret,
            body,
            span: Some(self.span_to_next(&start)),
        })))
    }

    fn end_decl(&self, start: &Token, keyword: &str) -> Result<()> {
        if self.cursor.is(keyword) || self.cursor.is("decreasing_by") {
            return Err(unsupported(
                &format!("Lean {}", self.cursor.peek().value),
                "explicit termination arguments are outside the portable core",
                Some(self.span_to_next(start)),
            ));
        }
        if !self.blocked_next() {
            return Err(self.fail("expected the end of the declaration"));
        }
        Ok(())
    }

    fn main(&mut self, start: &Token) -> Result<Definition> {
        self.cursor.expect("IO", Some("main"))?;
        self.cursor.expect("Unit", Some("main"))?;
        self.cursor.expect(":=", Some("main"))?;
        self.cursor.expect("do", Some("main"))?;
        let mut effects = Vec::new();
        let first = self.peek();
        let effect_column = if first.first { column(&first) } else { -1 };
        if effect_column <= 0 {
            return Err(unsupported(
                "inline do block",
                "write one effect per line",
                Some(span(&first, &first)),
            ));
        }
        while !self.cursor.at_end()
            && column(self.cursor.peek()) == effect_column
            && self.cursor.peek().first
        {
            let before = self.cursor.index;
            effects.push(self.with_bound(effect_column, Self::effect)?);
            if self.cursor.index == before {
                return Err(self.fail("expected an effect"));
            }
        }
        self.end_decl(start, "where")?;
        let at = self.span_to_next(start);
        Ok(Definition::Main(
            SMain {
                effects,
                span: Some(at),
            },
            at,
        ))
    }

    fn effect(&mut self) -> Result<SEffect> {
        let token = self.peek();
        if self.cursor.is("IO.println") {
            self.cursor.advance();
            let expr = self.atom()?;
            return Ok(SEffect::Print {
                expr,
                style: ShowStyle::Lean,
                span: Some(self.span_to_next(&token)),
            });
        }
        if self.cursor.is("let") {
            self.cursor.advance();
            let name = self.cursor.identifier(Some("let"))?.value;
            let ty = if self.cursor.eat(":").is_some() {
                Some(self.ty()?)
            } else {
                None
            };
            self.cursor.expect(":=", Some("let"))?;
            let value = self.expr()?;
            return Ok(SEffect::Let {
                name,
                ty,
                value,
                span: Some(self.span_to_next(&token)),
            });
        }
        Err(unsupported(
            "Lean do element",
            &format!("{} is outside the portable effect model", describe(&token)),
            Some(span(&token, &token)),
        ))
    }
}

/// A head applied to arguments, or the head alone when there are none.
fn plain_application(head: SExpr, args: Vec<SExpr>, range: Span) -> Result<SExpr> {
    if args.is_empty() {
        return Ok(head);
    }
    if !matches!(head.node, SNode::Name { .. } | SNode::DotCtor { .. }) {
        return Err(unsupported(
            "higher-order application",
            "only named functions can be applied",
            Some(range),
        ));
    }
    Ok(expr(
        SNode::App {
            func: Box::new(head),
            args,
        },
        range,
    ))
}

const fn connective(node: SPropNode) -> SProp {
    SProp { node, span: None }
}

fn bind_or_ctor(head: PatternHead, token: &Token) -> SPattern {
    let name = head.path.into_iter().next().unwrap_or_default();
    pattern(SPatternNode::BindOrCtor { name }, Some(span(token, token)))
}

const fn param(name: String, ty: Type) -> SParam {
    SParam {
        name,
        ty: Some(ty),
        span: None,
        guard: None,
        rocq_type: None,
    }
}

const fn binder(name: String, ty: Type) -> SBinder {
    SBinder {
        name,
        ty: Some(ty),
        rocq_type: None,
        span: None,
    }
}
