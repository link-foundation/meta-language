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
    BinaryOp, Flavor, SBinder, SComparison, SCtor, SData, SEffect, SExpr, SField, SFn, SItem,
    SMain, SModule, SNode, SParam, SPattern, SPatternNode, SProgram, SProof, SProp, SPropNode,
    SRow, SRule, SSplit, SSplitCase, SStep, ShowStyle, UnaryOp,
};
use super::types::{Type, BOOL, INT, NAT, STRING, UNIT};
use super::{Language, Span};

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
    Fn(SFn),
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
                    Definition::Fn(item) => items.push(SItem::Fn(item)),
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
                fields.extend(types.into_iter().map(|ty| SField { name: None, ty }));
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
            return Ok(Definition::Fn(SFn {
                name,
                params,
                ret,
                body,
                span: Some(self.span_to_next(&start)),
            }));
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
        Ok(Definition::Fn(SFn {
            name,
            params: params.into_iter().chain(extra).collect(),
            ret,
            body,
            span: Some(self.span_to_next(&start)),
        }))
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

    fn theorem(&mut self) -> Result<super::surface::STheorem> {
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("theorem"))?.value;
        let binders = self
            .binders()?
            .into_iter()
            .map(|(name, ty)| binder(name, ty))
            .collect();
        self.cursor.expect(":", Some("theorem"))?;
        let prop = self.with_bound(0, Self::prop)?;
        self.cursor.expect(":=", Some("theorem"))?;
        let proof_start = self.peek();
        let steps = if self.cursor.eat("by").is_some() {
            self.tactics(&proof_start)?
        } else if self.cursor.is("rfl") {
            self.cursor.advance();
            vec![SStep::Compute { tactic: None }]
        } else {
            return Err(unsupported(
                "proof term",
                "only tactic proofs are reconstructed",
                Some(span(&proof_start, &proof_start)),
            ));
        };
        let next = self.cursor.peek();
        let end = if next.kind == TokenKind::Eof {
            self.source.len()
        } else {
            next.start
        };
        let units = self.source.units();
        let proof_source = js_trimmed(&units[proof_start.start.min(end)..end]);
        let proof = SProof {
            steps,
            source: Some(proof_source),
            source_language: Language::Lean,
        };
        self.end_decl(&start, "termination_by")?;
        Ok(super::surface::STheorem {
            name,
            binders,
            prop,
            proof,
            span: Some(self.span_to_next(&start)),
        })
    }

    // Tactic blocks keep their structure: intro, induction with per-case
    // steps, and closing steps. The emitters rebuild each case for the target
    // kernel, which then re-checks the proof.
    fn tactics(&mut self, by_token: &Token) -> Result<Vec<SStep>> {
        let first = self.cursor.peek();
        let block_column = if first.first { column(first) } else { -1 };
        let mut steps = Vec::new();
        if block_column < 0 {
            self.with_bound(column(by_token), |parser| {
                parser.tactic_sequence(&mut steps)
            })?;
            return Ok(steps);
        }
        while !self.cursor.at_end()
            && self.cursor.peek().first
            && column(self.cursor.peek()) == block_column
        {
            self.with_bound(block_column, |parser| parser.tactic_sequence(&mut steps))?;
        }
        Ok(steps)
    }

    /// Tactics joined by `;` or `<;>`.
    fn tactic_sequence(&mut self, steps: &mut Vec<SStep>) -> Result<()> {
        loop {
            steps.extend(self.tactic()?);
            if self.cursor.eat(";").is_some() || self.cursor.eat("<;>").is_some() {
                continue;
            }
            return Ok(());
        }
    }

    fn names_until_blocked(&mut self) -> Vec<String> {
        let mut names = Vec::new();
        while self.cursor.is_kind(TokenKind::Identifier) && !self.blocked_next() {
            names.push(self.cursor.advance().value);
        }
        names
    }

    fn tactic(&mut self) -> Result<Vec<SStep>> {
        let token = self.peek();
        let value = token.value.as_str();
        if token.kind != TokenKind::Identifier {
            return Err(self.fail("expected a tactic"));
        }
        self.cursor.advance();
        match value {
            "rfl" | "decide" | "trivial" => Ok(vec![SStep::Compute {
                tactic: Some(value.to_owned()),
            }]),
            "omega" => Ok(vec![SStep::Arith {
                tactic: Some(value.to_owned()),
            }]),
            "intro" | "intros" => Ok(vec![SStep::Intro {
                names: self.names_until_blocked(),
            }]),
            "unfold" => Ok(vec![SStep::Unfold {
                names: self.names_until_blocked(),
                tactic: None,
            }]),
            "rw" | "simp" | "simp_all" => {
                let only = value != "rw" && self.cursor.eat("only").is_some();
                let mut rules = Vec::new();
                if self.cursor.eat("[").is_some() {
                    while !self.cursor.is("]") {
                        let reverse =
                            self.cursor.eat("←").is_some() || self.cursor.eat("<-").is_some();
                        rules.push(SRule {
                            name: self.cursor.identifier(Some("rewrite rule"))?.value,
                            reverse,
                        });
                        if self.cursor.eat(",").is_none() {
                            break;
                        }
                    }
                    self.cursor.expect("]", Some(value))?;
                }
                if self.cursor.is("at") {
                    return Err(unsupported(
                        &format!("{value} at"),
                        "hypothesis rewriting is outside the portable proof model",
                        Some(self.span_to_next(&token)),
                    ));
                }
                let (only, tactic) = (Some(only), Some(value.to_owned()));
                Ok(vec![if value == "rw" {
                    SStep::Rewrite {
                        rules,
                        only,
                        tactic,
                    }
                } else {
                    SStep::Simp {
                        rules,
                        only,
                        tactic,
                    }
                }])
            }
            "induction" | "cases" => {
                let variable = self.cursor.identifier(Some(value))?.value;
                self.cursor.expect("with", Some(value))?;
                let mut cases = Vec::new();
                while self.cursor.is("|") && !self.outdented(column(&token)) {
                    let bar = self.cursor.advance();
                    let ctor = self.cursor.identifier(Some("case"))?.value;
                    let ctor = ctor.strip_prefix('.').unwrap_or(&ctor).to_owned();
                    let mut binds = Vec::new();
                    while self.cursor.is_kind(TokenKind::Identifier) && !self.cursor.is("=>") {
                        binds.push(Some(self.cursor.advance().value));
                    }
                    self.cursor.expect("=>", Some("case"))?;
                    let steps = self.case_tactics(&bar)?;
                    cases.push(SSplitCase {
                        ctor: Some(ctor),
                        index: None,
                        binds,
                        steps,
                    });
                }
                let split = SSplit {
                    variable,
                    cases,
                    positional: false,
                    blocks: None,
                };
                Ok(vec![if value == "induction" {
                    SStep::Induction(split)
                } else {
                    SStep::Cases(split)
                }])
            }
            // `exact`, `apply`, `constructor`, `exists`, `calc`, `have`, `show`,
            // `sorry`, `admit`, `native_decide`, `grind`, `aesop` and the rest.
            _ => Err(unsupported(
                &format!("Lean tactic {value}"),
                "outside the portable proof model",
                Some(span(&token, &token)),
            )),
        }
    }

    fn case_tactics(&mut self, bar: &Token) -> Result<Vec<SStep>> {
        let mut steps = Vec::new();
        let first = self.cursor.peek();
        if !first.first {
            self.with_bound(column(bar), |parser| parser.tactic_sequence(&mut steps))?;
            return Ok(steps);
        }
        let case_column = column(first);
        while !self.cursor.at_end()
            && self.cursor.peek().first
            && column(self.cursor.peek()) == case_column
            && case_column > column(bar)
        {
            self.with_bound(case_column, |parser| parser.tactic_sequence(&mut steps))?;
        }
        Ok(steps)
    }

    fn prop(&mut self) -> Result<SProp> {
        self.prop_implies()
    }

    fn prop_implies(&mut self) -> Result<SProp> {
        let left = self.prop_or()?;
        if self.cursor.eat("→").is_some() || self.cursor.eat("->").is_some() {
            let right = self.prop_implies()?;
            return Ok(connective(SPropNode::Implies {
                left: Box::new(left),
                right: Box::new(right),
            }));
        }
        Ok(left)
    }

    fn prop_or(&mut self) -> Result<SProp> {
        let mut left = self.prop_and()?;
        while self.cursor.eat("∨").is_some() {
            let right = self.prop_and()?;
            left = connective(SPropNode::Or {
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn prop_and(&mut self) -> Result<SProp> {
        let mut left = self.prop_unary()?;
        while self.cursor.eat("∧").is_some() {
            let right = self.prop_unary()?;
            left = connective(SPropNode::And {
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn prop_unary(&mut self) -> Result<SProp> {
        let token = self.peek();
        if self.cursor.eat("¬").is_some() {
            let arg = self.prop_unary()?;
            return Ok(connective(SPropNode::Not { arg: Box::new(arg) }));
        }
        if self.cursor.eat("∀").is_some() {
            let mut binders = Vec::new();
            while self.cursor.is("(") {
                binders.extend(self.binder_group()?);
            }
            if binders.is_empty() {
                let mut names = Vec::new();
                while self.cursor.is_kind(TokenKind::Identifier) {
                    names.push(self.cursor.advance().value);
                }
                self.cursor.expect(":", Some("∀"))?;
                let ty = self.ty()?;
                binders.extend(names.into_iter().map(|name| (name, ty.clone())));
            }
            self.cursor.expect(",", Some("∀"))?;
            let body = self.prop()?;
            return Ok(SProp {
                node: SPropNode::Forall {
                    binders: binders
                        .into_iter()
                        .map(|(name, ty)| binder(name, ty))
                        .collect(),
                    body: Box::new(body),
                },
                span: Some(self.span_to_next(&token)),
            });
        }
        if self.cursor.is("(") && self.prop_parenthesised() {
            self.cursor.advance();
            let inner = self.prop()?;
            self.cursor.expect(")", Some("proposition"))?;
            return Ok(inner);
        }
        let left = self.binary(55)?;
        let relation = self.cursor.peek();
        if relation.kind == TokenKind::Punct && !self.blocked(relation) {
            if let Some(build) = prop_relation(&relation.value) {
                self.cursor.advance();
                let right = self.binary(55)?;
                return Ok(SProp {
                    node: build(SComparison {
                        left,
                        right,
                        reference: false,
                    }),
                    span: Some(self.span_to_next(&token)),
                });
            }
        }
        Ok(SProp {
            node: SPropNode::Bool { expr: left },
            span: Some(self.span_to_next(&token)),
        })
    }

    /// A parenthesis opens a proposition when it contains a logical connective at depth one.
    fn prop_parenthesised(&self) -> bool {
        let mut depth = 0i64;
        for offset in 0.. {
            let token = self.cursor.peek_at(offset);
            if token.kind == TokenKind::Eof {
                return false;
            }
            if token.value == "(" {
                depth += 1;
            }
            if token.value == ")" {
                depth -= 1;
                if depth == 0 {
                    return false;
                }
            }
            if depth == 1
                && ["∧", "∨", "→", "¬", "∀", "=", "≠", "↔"].contains(&token.value.as_str())
            {
                return true;
            }
        }
        false
    }

    fn expr(&mut self) -> Result<SExpr> {
        let token = self.peek();
        if self.cursor.is("if") {
            self.cursor.advance();
            if self.cursor.is_kind(TokenKind::Identifier) && self.cursor.is_at(":", 1) {
                return Err(unsupported(
                    "dependent if",
                    "if h : … is outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            let cond = self.expr()?;
            self.cursor.expect("then", Some("if"))?;
            let then = self.expr()?;
            self.cursor.expect("else", Some("if"))?;
            let otherwise = self.expr()?;
            return Ok(expr(
                SNode::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                },
                self.span_to_next(&token),
            ));
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
            let value = self.with_bound(column(&token), Self::expr)?;
            self.cursor.eat(";");
            let body = self.expr()?;
            return Ok(expr(
                SNode::Let {
                    name,
                    ty,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                self.span_to_next(&token),
            ));
        }
        if self.cursor.is("match") {
            self.cursor.advance();
            let mut scrutinees = vec![self.expr()?];
            while self.cursor.eat(",").is_some() {
                scrutinees.push(self.expr()?);
            }
            self.cursor.expect("with", Some("match"))?;
            let rows = self.alternatives(scrutinees.len())?;
            return Ok(expr(
                SNode::Match { scrutinees, rows },
                self.span_to_next(&token),
            ));
        }
        if self.cursor.is("fun") || self.cursor.is("λ") {
            return Err(unsupported(
                "lambda",
                "higher-order values are outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        if self.cursor.is("do") {
            return Err(unsupported(
                "do block",
                "monadic code outside main is outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        self.binary(0)
    }

    fn alternatives(&mut self, arity: usize) -> Result<Vec<SRow>> {
        let mut rows = Vec::new();
        while self.cursor.is("|") && !self.outdented(self.bound()) {
            let bar = self.cursor.advance();
            let mut patterns = vec![self.pattern()?];
            while self.cursor.eat(",").is_some() {
                patterns.push(self.pattern()?);
            }
            if patterns.len() != arity {
                return Err(syntax_at(
                    format!(
                        "alternative has {} patterns, expected {arity}",
                        patterns.len()
                    ),
                    &bar,
                ));
            }
            self.cursor.expect("=>", Some("alternative"))?;
            let body = self.expr()?;
            rows.push(SRow {
                patterns,
                body,
                span: Some(self.span_to_next(&bar)),
            });
        }
        if rows.is_empty() {
            return Err(self.fail("expected match alternatives"));
        }
        Ok(rows)
    }

    fn pattern(&mut self) -> Result<SPattern> {
        let token = self.peek();
        let mut result = self.pattern_app()?;
        while self.cursor.is("+") {
            self.cursor.advance();
            let amount = self.cursor.advance();
            if amount.kind != TokenKind::Number || !amount.suffix.is_empty() {
                return Err(Self::fail_at(
                    "expected a numeral in an n + k pattern",
                    &amount,
                ));
            }
            result = pattern(
                SPatternNode::NatAdd {
                    inner: Box::new(result),
                    add: js_number_u64(&amount.value),
                },
                Some(self.span_to_next(&token)),
            );
        }
        Ok(result)
    }

    fn pattern_app(&mut self) -> Result<SPattern> {
        let token = self.peek();
        if token.kind == TokenKind::Identifier || self.cursor.is(".") {
            let head = self.pattern_head()?;
            let mut args = Vec::new();
            while self.pattern_argument_start() {
                args.push(self.pattern_atom()?);
            }
            if args.is_empty() && head.path.len() == 1 && !head.dot {
                return Ok(bind_or_ctor(head, &token));
            }
            return Ok(self.ctor_pattern(head, args, &token));
        }
        self.pattern_atom()
    }

    fn pattern_argument_start(&self) -> bool {
        let token = self.cursor.peek();
        if self.blocked(token) {
            return false;
        }
        token.kind == TokenKind::Identifier
            || token.kind == TokenKind::Number
            || self.cursor.is("(")
            || self.cursor.is("_")
            || self.cursor.is(".")
    }

    fn pattern_head(&mut self) -> Result<PatternHead> {
        let dot = self.cursor.eat(".").is_some();
        let path = self
            .cursor
            .identifier(Some("pattern"))?
            .value
            .split('.')
            .map(str::to_owned)
            .collect();
        Ok(PatternHead { path, dot })
    }

    fn ctor_pattern(&self, head: PatternHead, mut args: Vec<SPattern>, token: &Token) -> SPattern {
        let joined = head.path.join(".");
        let name = head.path.last().map_or("", String::as_str);
        if (joined == "Nat.succ" || (head.dot && name == "succ")) && args.len() == 1 {
            return pattern(
                SPatternNode::NatAdd {
                    inner: Box::new(args.remove(0)),
                    add: 1,
                },
                Some(self.span_to_next(token)),
            );
        }
        if (joined == "Nat.zero" || (head.dot && name == "zero")) && args.is_empty() {
            return pattern(
                SPatternNode::NumLit {
                    value: "0".to_owned(),
                    negative: false,
                },
                Some(span(token, token)),
            );
        }
        pattern(
            SPatternNode::Ctor {
                path: head.path,
                args,
            },
            Some(self.span_to_next(token)),
        )
    }

    fn pattern_atom(&mut self) -> Result<SPattern> {
        let token = self.peek();
        if self.cursor.eat("_").is_some() {
            return Ok(pattern(SPatternNode::Wild, None));
        }
        if token.kind == TokenKind::Number {
            self.cursor.advance();
            return Ok(pattern(
                SPatternNode::NumLit {
                    value: token.value.clone(),
                    negative: false,
                },
                Some(span(&token, &token)),
            ));
        }
        if self.cursor.eat("(").is_some() {
            let inner = self.pattern()?;
            self.cursor.expect(")", Some("pattern"))?;
            return Ok(inner);
        }
        if token.kind == TokenKind::Identifier || self.cursor.is(".") {
            let head = self.pattern_head()?;
            if head.path.len() == 1 && !head.dot {
                return Ok(bind_or_ctor(head, &token));
            }
            return Ok(self.ctor_pattern(head, Vec::new(), &token));
        }
        Err(unsupported(
            "pattern",
            &format!(
                "{} is outside the portable pattern language",
                describe(&token)
            ),
            Some(span(&token, &token)),
        ))
    }

    fn binary(&mut self, min_prec: u32) -> Result<SExpr> {
        let mut left = self.unary()?;
        loop {
            let token = self.cursor.peek();
            if self.blocked(token) || token.kind != TokenKind::Punct {
                return Ok(left);
            }
            let Some(level) = BINARY
                .iter()
                .find(|level| level.ops.contains(&token.value.as_str()))
            else {
                return Ok(left);
            };
            if level.prec < min_prec {
                return Ok(left);
            }
            let token = self.cursor.advance();
            let right = self.binary(if level.right {
                level.prec
            } else {
                level.prec + 1
            })?;
            let op = level
                .op
                .or_else(|| operator_name(&token.value))
                .unwrap_or(BinaryOp::Eq);
            let at = Span::new(
                left.span.map_or(token.start, |at| at.start),
                right.span.map_or(token.end, |at| at.end),
            );
            left = expr(
                SNode::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding: None,
                },
                at,
            );
        }
    }

    fn unary(&mut self) -> Result<SExpr> {
        let token = self.peek();
        let op = if self.cursor.is("-") {
            UnaryOp::Neg
        } else if self.cursor.is("!") || self.cursor.is("not") {
            UnaryOp::Not
        } else {
            return self.application();
        };
        self.cursor.advance();
        let arg = self.unary()?;
        Ok(expr(
            SNode::Unary {
                op,
                arg: Box::new(arg),
            },
            self.span_to_next(&token),
        ))
    }

    fn argument_start(&self) -> bool {
        let token = self.cursor.peek();
        if self.blocked(token) {
            return false;
        }
        match token.kind {
            TokenKind::Identifier => {
                ![
                    "then", "else", "with", "do", "at", "if", "let", "match", "fun", "by", "from",
                ]
                .contains(&token.value.as_str())
                    && !is_item_keyword(&token.value)
            }
            TokenKind::Number | TokenKind::String | TokenKind::Interpolation => true,
            _ => ["(", ".", "↑"].contains(&token.value.as_str()),
        }
    }

    fn application(&mut self) -> Result<SExpr> {
        let token = self.peek();
        let head = self.atom()?;
        let mut args = Vec::new();
        while self.argument_start() {
            args.push(self.atom()?);
        }
        self.apply_head(head, args, &token)
    }

    fn apply_head(&self, head: SExpr, args: Vec<SExpr>, token: &Token) -> Result<SExpr> {
        let range = self.span_to_next(token);
        let name = match &head.node {
            SNode::Name { path } => path.join("."),
            _ => return plain_application(head, args, range),
        };
        let one = |args: Vec<SExpr>| -> Result<Box<SExpr>> {
            let mut args = args.into_iter();
            match (args.next(), args.next()) {
                (Some(arg), None) => Ok(Box::new(arg)),
                _ => Err(type_error(
                    format!("{name} expects one argument"),
                    Some(range),
                )),
            }
        };
        let node = match name.as_str() {
            "toString" => SNode::ToString { arg: one(args)? },
            "Int.toNat" => SNode::Cast {
                arg: one(args)?,
                to: NAT,
                from: Some(INT),
                flavor: Flavor::Clamp,
            },
            "Int.ofNat" => SNode::Cast {
                arg: one(args)?,
                to: INT,
                from: Some(NAT),
                flavor: Flavor::Exact,
            },
            "Nat.succ" => SNode::Binary {
                op: BinaryOp::Add,
                left: one(args)?,
                right: Box::new(SExpr::new(
                    SNode::Num {
                        value: "1".to_owned(),
                        ty: None,
                        negative: false,
                    },
                    None,
                )),
                rounding: None,
            },
            "Nat.zero" => {
                if !args.is_empty() {
                    return Err(type_error("Nat.zero takes no arguments", Some(range)));
                }
                SNode::Num {
                    value: "0".to_owned(),
                    ty: Some(NAT),
                    negative: false,
                }
            }
            _ => {
                if ["Nat.", "Int.", "String.", "Bool."]
                    .iter()
                    .any(|prefix| name.starts_with(prefix))
                {
                    return Err(unsupported(
                        &format!("Lean library function {name}"),
                        "outside the portable core",
                        Some(range),
                    ));
                }
                return plain_application(head, args, range);
            }
        };
        Ok(expr(node, range))
    }

    fn atom(&mut self) -> Result<SExpr> {
        let token = self.peek();
        let at = span(&token, &token);
        match token.kind {
            TokenKind::Number => {
                self.cursor.advance();
                if !token.suffix.is_empty() {
                    return Err(unsupported("numeric literal suffix", &token.raw, Some(at)));
                }
                return Ok(expr(
                    SNode::Num {
                        value: token.value,
                        ty: None,
                        negative: false,
                    },
                    at,
                ));
            }
            TokenKind::String => {
                self.cursor.advance();
                return Ok(expr(SNode::Str { value: token.value }, at));
            }
            TokenKind::Interpolation => {
                self.cursor.advance();
                return self.interpolation(&token);
            }
            _ => {}
        }
        if self.cursor.eat("↑").is_some() {
            let arg = self.atom()?;
            return Ok(expr(
                SNode::Cast {
                    arg: Box::new(arg),
                    to: INT,
                    from: Some(NAT),
                    flavor: Flavor::Exact,
                },
                self.span_to_next(&token),
            ));
        }
        if self.cursor.eat(".").is_some() {
            let name = self.cursor.identifier(Some("constructor"))?.value;
            return Ok(expr(SNode::DotCtor { name }, at));
        }
        if self.cursor.eat("(").is_some() {
            if self.cursor.eat(")").is_some() {
                return Ok(expr(SNode::Unit, at));
            }
            let inner = self.with_bound(-1, Self::expr)?;
            if self.cursor.eat(":").is_some() {
                let ty = self.ty()?;
                self.cursor.expect(")", Some("type ascription"))?;
                let ascribed = self.span_to_next(&token);
                if let SNode::Num {
                    value, negative, ..
                } = &inner.node
                {
                    return Ok(expr(
                        SNode::Num {
                            value: value.clone(),
                            ty: Some(ty),
                            negative: *negative,
                        },
                        ascribed,
                    ));
                }
                if let SNode::Unary {
                    op: UnaryOp::Neg,
                    arg,
                } = &inner.node
                {
                    if let SNode::Num { value, .. } = &arg.node {
                        return Ok(expr(
                            SNode::Num {
                                value: value.clone(),
                                ty: Some(ty),
                                negative: true,
                            },
                            ascribed,
                        ));
                    }
                }
                return Ok(expr(
                    SNode::Cast {
                        arg: Box::new(inner),
                        to: ty,
                        from: None,
                        flavor: Flavor::Exact,
                    },
                    ascribed,
                ));
            }
            if self.cursor.is(",") {
                return Err(unsupported(
                    "tuple",
                    "product values are outside the portable core",
                    Some(self.span_to_next(&token)),
                ));
            }
            self.cursor.expect(")", Some("parenthesised expression"))?;
            return Ok(inner);
        }
        if token.kind == TokenKind::Identifier {
            self.cursor.advance();
            if token.value == "true" || token.value == "false" {
                return Ok(expr(
                    SNode::Bool {
                        value: token.value == "true",
                    },
                    at,
                ));
            }
            if token.value == "sorry" {
                return Err(unsupported(
                    "sorry",
                    "incomplete definitions cannot be translated",
                    Some(at),
                ));
            }
            let next = self.cursor.peek();
            if next.value == "." && next.start == token.end {
                let member = self.cursor.peek_at(1);
                return Err(unsupported(
                    "method call",
                    &format!(
                        "{}.{} is outside the portable core",
                        token.value, member.value
                    ),
                    Some(span(&token, member)),
                ));
            }
            return Ok(expr(
                SNode::Name {
                    path: token.value.split('.').map(str::to_owned).collect(),
                },
                at,
            ));
        }
        Err(self.fail("expected an expression"))
    }

    fn interpolation(&self, token: &Token) -> Result<SExpr> {
        // `token.raw.slice(3, -1)`: the text between `s!"` and the closing quote.
        let raw: Vec<u16> = token.raw.encode_utf16().collect();
        let raw = if raw.len() >= 4 {
            &raw[3..raw.len() - 1]
        } else {
            &[][..]
        };
        let mut parts = Vec::new();
        let mut text: Vec<u16> = Vec::new();
        let mut index = 0;
        let offset = token.start + 3;
        let unit = |ch: u8| u16::from(ch);
        while index < raw.len() {
            if raw[index] == unit(b'\\') {
                let escaped = raw.get(index + 1).copied();
                let replacement = match escaped.and_then(|unit| u8::try_from(unit).ok()) {
                    Some(b'n') => b'\n',
                    Some(b't') => b'\t',
                    Some(b'\\') => b'\\',
                    Some(b'"') => b'"',
                    Some(b'{') => b'{',
                    _ => {
                        let shown = escaped.map_or_else(
                            || "undefined".to_owned(),
                            |unit| String::from_utf16_lossy(&[unit]),
                        );
                        return Err(syntax_at(format!("unsupported escape \\{shown}"), token));
                    }
                };
                text.push(u16::from(replacement));
                index += 2;
                continue;
            }
            if raw[index] == unit(b'{') {
                let Some(close) = (index..raw.len()).find(|&at| raw[at] == unit(b'}')) else {
                    return Err(syntax_at("unterminated interpolation".to_owned(), token));
                };
                if !text.is_empty() {
                    parts.push(SExpr::new(
                        SNode::Str {
                            value: String::from_utf16_lossy(&text),
                        },
                        None,
                    ));
                }
                text.clear();
                let inner = &raw[index + 1..close];
                let shift = offset + index + 1;
                let mut tokens = tokenize_source(
                    &Source::new(&String::from_utf16_lossy(inner)),
                    Language::Lean,
                )?
                .tokens;
                for inner_token in &mut tokens {
                    inner_token.start += shift;
                    inner_token.end += shift;
                }
                let mut padded = vec![unit(b' '); shift];
                padded.extend_from_slice(inner);
                annotate_layout(&mut tokens, &padded);
                let mut parser = LeanParser::new(tokens, self.source);
                let arg = parser.expr()?;
                if !parser.cursor.at_end() {
                    return Err(parser.fail("unexpected token in interpolation"));
                }
                parts.push(SExpr::new(
                    SNode::Show {
                        arg: Box::new(arg),
                        style: ShowStyle::Lean,
                    },
                    None,
                ));
                index = close + 1;
                continue;
            }
            text.push(raw[index]);
            index += 1;
        }
        if !text.is_empty() || parts.is_empty() {
            parts.push(SExpr::new(
                SNode::Str {
                    value: String::from_utf16_lossy(&text),
                },
                None,
            ));
        }
        let mut parts = parts.into_iter();
        let first = parts
            .next()
            .unwrap_or_else(|| SExpr::new(SNode::Unit, None));
        Ok(parts.fold(first, |left, right| {
            expr(
                SNode::Binary {
                    op: BinaryOp::Concat,
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding: None,
                },
                span(token, token),
            )
        }))
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

/// `Number(digits)` for an `n + k` pattern's `k`; amounts beyond `u64` saturate.
fn js_number_u64(digits: &str) -> u64 {
    digits.parse().unwrap_or(u64::MAX)
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
