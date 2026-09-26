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
    Fn(SFn),
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

    fn file(&mut self) -> Result<SProgram> {
        let mut main = None;
        let items = self.items(&mut main, &[])?;
        Ok(SProgram {
            language: Language::Rocq,
            items,
            main,
        })
    }

    fn items(&mut self, main: &mut Option<SMain>, path: &[String]) -> Result<Vec<SItem>> {
        let mut items = Vec::new();
        while !self.cursor.at_end() {
            let token = self.peek().clone();
            if self.is("End") {
                let Some(open) = path.last() else {
                    return Err(self.fail_here("unmatched End"));
                };
                self.cursor.advance();
                let name = self.identifier("End")?.value;
                if &name != open {
                    return Err(TranslationError::syntax(
                        format!("End {name} closes module {open}"),
                        Some(token.span()),
                    ));
                }
                self.end_sentence("End")?;
                return Ok(items);
            }
            if self.is("Module") {
                self.cursor.advance();
                if self.is("Type") || self.is("Import") || self.is("Export") {
                    return Err(unsupported(
                        &format!("Rocq Module {}", self.peek().value),
                        "module types and functors are outside the portable core",
                        Some(span(&token, self.peek())),
                    ));
                }
                let name = self.identifier("Module")?.value;
                if name.contains('.') {
                    return Err(unsupported(
                        "qualified module name",
                        "declare one module per level",
                        Some(span(&token, &token)),
                    ));
                }
                if !self.is(".") {
                    return Err(unsupported(
                        "module signature or functor",
                        "only plain modules are portable",
                        Some(span(&token, self.peek())),
                    ));
                }
                self.end_sentence("Module")?;
                let module_span = span(&token, self.peek());
                let mut inner = path.to_vec();
                inner.push(name.clone());
                let module_items = self.items(main, &inner)?;
                items.push(SItem::Module(SModule {
                    name,
                    items: module_items,
                    span: Some(module_span),
                }));
                continue;
            }
            if self.ignored_command()? {
                continue;
            }
            if self.is("Inductive") {
                items.push(SItem::Data(self.inductive()?));
                continue;
            }
            if self.is("Definition") || self.is("Fixpoint") || self.is("Function") {
                match self.definition(path)? {
                    Definition::Main(program, item_span) => {
                        if !path.is_empty() {
                            return Err(unsupported(
                                "main inside a module",
                                "main must be declared at the top level",
                                Some(item_span),
                            ));
                        }
                        if main.is_some() {
                            return Err(type_error("duplicate main", Some(item_span)));
                        }
                        *main = Some(program);
                    }
                    Definition::Fn(item) => items.push(SItem::Fn(item)),
                }
                continue;
            }
            if token.kind == TokenKind::Identifier
                && THEOREM_KEYWORDS.contains(&token.value.as_str())
            {
                items.push(SItem::Theorem(self.theorem()?));
                continue;
            }
            if self.is("Eval") || self.is("Compute") {
                self.evaluation()?;
                continue;
            }
            if token.kind == TokenKind::Identifier
                && (UNSUPPORTED_COMMANDS.contains(&token.value.as_str())
                    || token.value.starts_with(|ch: char| ch.is_ascii_uppercase()))
            {
                return Err(unsupported(
                    &format!("Rocq {} command", token.value),
                    "outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            return Err(self.fail_here("expected a Rocq command"));
        }
        if let Some(open) = path.last() {
            return Err(TranslationError::syntax(
                format!("module {open} is not closed"),
                Some(self.peek().span()),
            ));
        }
        Ok(items)
    }

    /// Library loading and notation scopes do not change the program's meaning
    /// in the portable core: operators are resolved by their operand types.
    fn ignored_command(&mut self) -> Result<bool> {
        let command = self.peek().value.clone();
        if self.is("From") {
            self.cursor.advance();
            self.identifier("From")?;
            self.expect("Require", "From")?;
            return self.skip_sentence(&command);
        }
        if self.is("Require")
            || (self.is("Import") && self.cursor.is_at("ListNotations", 1))
            || (self.is("Open") && self.cursor.is_at("Scope", 1))
            || (self.is("Local") && self.cursor.is_at("Open", 1) && self.cursor.is_at("Scope", 2))
        {
            return self.skip_sentence(&command);
        }
        Ok(false)
    }

    fn skip_sentence(&mut self, command: &str) -> Result<bool> {
        while !self.cursor.at_end() && !self.is(".") {
            self.cursor.advance();
        }
        self.end_sentence(command)?;
        Ok(true)
    }

    /// `Eval <strategy> in main.` or `Compute main.` displays the program output.
    fn evaluation(&mut self) -> Result<()> {
        let token = self.cursor.advance();
        if token.value == "Eval" {
            let strategy = self.identifier("Eval")?.value;
            if !EVAL_STRATEGIES.contains(&strategy.as_str()) {
                return Err(unsupported(
                    &format!("Eval {strategy}"),
                    "unknown reduction strategy",
                    Some(span(&token, self.peek())),
                ));
            }
            self.expect("in", "Eval")?;
        }
        if !self.is("main") {
            return Err(unsupported(
                "evaluation command",
                "only the evaluation of main is part of the program output",
                Some(span(&token, self.peek())),
            ));
        }
        self.cursor.advance();
        self.end_sentence(&token.value)
    }

    fn inductive(&mut self) -> Result<SData> {
        let start = self.cursor.advance();
        let name = self.identifier("Inductive")?.value;
        if name.contains('.') {
            return Err(unsupported(
                "qualified inductive name",
                "declare inductives inside their module",
                Some(span(&start, &start)),
            ));
        }
        if self.is("(") || self.is("{") {
            return Err(unsupported(
                "parameterised inductive",
                "type parameters are outside the portable core",
                Some(span(&start, self.peek())),
            ));
        }
        if self.cursor.eat(":").is_some() {
            let sort = self.identifier("Inductive sort")?;
            if !["Type", "Set"].contains(&sort.value.as_str()) {
                return Err(unsupported(
                    &format!("inductive in {}", sort.value),
                    "only data types are portable",
                    Some(span(&start, &sort)),
                ));
            }
        }
        self.expect(":=", "Inductive")?;
        let mut ctors = Vec::new();
        self.cursor.eat("|");
        loop {
            let ctor_start = self.peek().clone();
            let ctor_name = self.identifier("constructor")?.value;
            let mut fields = Vec::new();
            while self.is("(") {
                fields.extend(self.binder_group()?.into_iter().map(|binder| SField {
                    name: Some(binder.name),
                    ty: binder.ty.unwrap_or(UNIT),
                    rocq_type: binder.rocq_type,
                    span: binder.span,
                }));
            }
            if self.cursor.eat(":").is_some() {
                let mut types = self.arrow_type()?;
                let result = types.pop();
                let constructs =
                    matches!(&result, Some(Type::Named { path, .. }) if path.join(".") == name);
                if !constructs {
                    return Err(unsupported(
                        "indexed constructor",
                        &format!("{ctor_name} must construct {name}"),
                        Some(span(&ctor_start, self.peek())),
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
            if self.cursor.eat("|").is_none() {
                break;
            }
        }
        if self.is("with") {
            return Err(unsupported(
                "mutual inductive",
                "mutually inductive types are outside the portable core",
                Some(span(&start, self.peek())),
            ));
        }
        self.end_sentence("Inductive")?;
        Ok(SData {
            name,
            ctors,
            span: Some(span(&start, self.peek())),
        })
    }

    fn binder_group(&mut self) -> Result<Vec<SBinder>> {
        let open = self.expect("(", "binder")?;
        let mut names = Vec::new();
        while self.cursor.is_kind(TokenKind::Identifier) {
            names.push(self.cursor.advance().value);
        }
        if names.is_empty() {
            return Err(self.fail_here("expected binder names"));
        }
        self.expect(":", "binder")?;
        let rocq_type = self.peek().value.clone();
        let ty = self.parse_type()?;
        self.expect(")", "binder")?;
        let group_span = span(&open, self.peek());
        Ok(names
            .into_iter()
            .map(|name| SBinder {
                name,
                ty: Some(ty.clone()),
                rocq_type: Some(rocq_type.clone()),
                span: Some(group_span),
            })
            .collect())
    }

    fn binders(&mut self) -> Result<Vec<SBinder>> {
        let mut result = Vec::new();
        while self.is("(") {
            result.extend(self.binder_group()?);
        }
        if self.is("{") && !self.cursor.is_at("struct", 1) && !self.cursor.is_at("measure", 1) {
            return Err(unsupported(
                "implicit binder",
                "implicit binders are outside the portable core",
                Some(span(self.peek(), self.peek())),
            ));
        }
        if self.is("`") {
            return Err(unsupported(
                "generalised binder",
                "outside the portable core",
                Some(span(self.peek(), self.peek())),
            ));
        }
        Ok(result)
    }

    fn arrow_type(&mut self) -> Result<Vec<Type>> {
        let mut types = vec![self.parse_type()?];
        while self.cursor.eat("->").is_some() {
            types.push(self.parse_type()?);
        }
        Ok(types)
    }

    fn product_type(&self, token: &Token) -> TranslationError {
        unsupported(
            "product type",
            "tuples are outside the portable core",
            Some(span(token, self.peek())),
        )
    }

    fn parse_type(&mut self) -> Result<Type> {
        let token = self.peek().clone();
        if self.cursor.eat("(").is_some() {
            let mut types = self.arrow_type()?;
            if self.is("*") {
                return Err(self.product_type(&token));
            }
            self.expect(")", "type")?;
            if types.len() != 1 {
                return Err(unsupported(
                    "function type",
                    "higher-order values are outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            return Ok(types.remove(0));
        }
        let name = self.identifier("type")?.value;
        if let Some(builtin) = builtin_type(&name) {
            if self.is("*") {
                return Err(self.product_type(&token));
            }
            return Ok(builtin);
        }
        if OUTSIDE_CORE_TYPES.contains(&name.as_str()) {
            return Err(unsupported(
                &format!("Rocq type {name}"),
                "outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        let next = self.peek();
        if next.kind == TokenKind::Identifier
            && !["with", "end", "in", "then", "else"].contains(&next.value.as_str())
        {
            return Err(unsupported(
                "type application",
                &format!("{name} {} is outside the portable core", next.value),
                Some(span(&token, next)),
            ));
        }
        if self.is("*") {
            return Err(self.product_type(&token));
        }
        Ok(Type::Named {
            path: name.split('.').map(str::to_owned).collect(),
            span: Some(span(&token, &token)),
        })
    }

    fn definition(&mut self, path: &[String]) -> Result<Definition> {
        let start = self.cursor.advance();
        let keyword = start.value.clone();
        let name_token = self.identifier(&keyword)?;
        let name = name_token.value.clone();
        if name.contains('.') {
            return Err(unsupported(
                "qualified definition name",
                "declare definitions inside their module",
                Some(span(&name_token, &name_token)),
            ));
        }
        let params = self.binders()?;
        let annotation = self.recursion_annotation(&keyword, &params, &start)?;
        if !self.is(":") {
            return Err(unsupported(
                "definition without a result type",
                &format!("{name} needs an explicit result type"),
                Some(span(&start, self.peek())),
            ));
        }
        self.cursor.advance();
        if name == "main"
            && path.is_empty()
            && keyword == "Definition"
            && self.is("list")
            && self.cursor.is_at("string", 1)
        {
            return self.main(&start);
        }
        let ret = self.parse_type()?;
        self.expect(":=", &keyword)?;
        let body = self.expr()?;
        check_portable(&body)?;
        if self.is("with") {
            return Err(unsupported(
                "mutual fixpoint",
                "mutually recursive definitions are outside the portable core",
                Some(span(&start, self.peek())),
            ));
        }
        self.end_sentence(&keyword)?;
        if keyword == "Function" {
            self.skip_obligations(&start, annotation)?;
        }
        Ok(Definition::Fn(SFn {
            name,
            params: params
                .into_iter()
                .map(|binder| SParam {
                    name: binder.name,
                    ty: binder.ty,
                    span: binder.span,
                    guard: None,
                    rocq_type: binder.rocq_type,
                })
                .collect(),
            ret: Some(ret),
            body,
            span: Some(span(&start, self.peek())),
        }))
    }

    /// `{struct x}` names the structural argument and `{measure N.to_nat x}`
    /// justifies a `Function`; the portable core finds its own decreasing
    /// argument and every target re-establishes termination.
    fn recursion_annotation(
        &mut self,
        keyword: &str,
        params: &[SBinder],
        start: &Token,
    ) -> Result<Option<Annotation>> {
        if !self.is("{") {
            if keyword == "Function" {
                return Err(unsupported(
                    "Function without a measure",
                    "Function needs {measure …} or {struct …}",
                    Some(span(start, self.peek())),
                ));
            }
            return Ok(None);
        }
        let open = self.cursor.advance();
        let is_param = |name: &str| params.iter().any(|param| param.name == name);
        let kind = self.identifier("recursion annotation")?.value;
        if kind == "struct" {
            if keyword == "Definition" {
                return Err(Self::fail("struct annotation on a Definition", &open));
            }
            let arg = self.identifier("struct")?.value;
            if !is_param(&arg) {
                return Err(type_error(
                    format!("struct argument {arg} is not a parameter"),
                    Some(open.span()),
                ));
            }
            self.expect("}", "struct")?;
            return Ok(Some(Annotation::Struct));
        }
        if kind == "measure" && keyword == "Function" {
            let measure_fn = self.identifier("measure")?.value;
            // `measureFn in {}` also holds for the names on `Object.prototype`.
            if !["N.to_nat", "Z.to_nat"].contains(&measure_fn.as_str())
                && !object_prototype_key(&measure_fn)
            {
                if !is_param(&measure_fn) {
                    return Err(unsupported(
                        &format!("measure {measure_fn}"),
                        "only N.to_nat x, Z.to_nat x or a nat argument are portable measures",
                        Some(span(&open, self.peek())),
                    ));
                }
                self.expect("}", "measure")?;
                return Ok(Some(Annotation::Measure));
            }
            let arg = self.identifier("measure")?.value;
            if !is_param(&arg) {
                return Err(type_error(
                    format!("measure argument {arg} is not a parameter"),
                    Some(open.span()),
                ));
            }
            self.expect("}", "measure")?;
            return Ok(Some(Annotation::Measure));
        }
        Err(unsupported(
            &format!("{{{kind} …}}"),
            "only struct and measure annotations are portable",
            Some(span(&open, self.peek())),
        ))
    }

    /// The termination obligations of a `Function` are the target's to discharge again.
    fn skip_obligations(&mut self, start: &Token, annotation: Option<Annotation>) -> Result<()> {
        if annotation != Some(Annotation::Measure) {
            return Ok(());
        }
        if !self.is("Proof") {
            return Err(unsupported(
                "Function without its obligation proof",
                "the measure obligations must be proved",
                Some(span(start, self.peek())),
            ));
        }
        self.cursor.advance();
        self.end_sentence("Proof")?;
        while !self.cursor.at_end()
            && !((self.is("Defined") || self.is("Qed")) && self.cursor.is_at(".", 1))
        {
            if self.is("Admitted") || self.is("admit") {
                return Err(unsupported(
                    "admitted obligation",
                    "incomplete proofs cannot be translated",
                    Some(span(self.peek(), self.peek())),
                ));
            }
            self.cursor.advance();
        }
        if self.cursor.at_end() {
            return Err(self.fail_here("expected Defined"));
        }
        self.cursor.advance();
        self.end_sentence("Defined")
    }

    fn main(&mut self, start: &Token) -> Result<Definition> {
        self.expect("list", "main")?;
        self.expect("string", "main")?;
        self.expect(":=", "main")?;
        let body = self.expr()?;
        self.end_sentence("main")?;
        let mut effects = Vec::new();
        walk_output(body, &mut effects)?;
        let main_span = span(start, self.peek());
        Ok(Definition::Main(
            SMain {
                effects,
                span: Some(main_span),
            },
            main_span,
        ))
    }

    fn theorem(&mut self) -> Result<STheorem> {
        let start = self.cursor.advance();
        let keyword = start.value.clone();
        let name = self.identifier(&keyword)?.value;
        let binders = self.binders()?;
        self.expect(":", &keyword)?;
        let prop = self.prop()?;
        self.end_sentence(&keyword)?;
        let proof_start = self.peek().clone();
        if self.cursor.eat("Proof").is_none() {
            return Err(unsupported(
                "proof term",
                "only tactic proofs are reconstructed",
                Some(span(&proof_start, &proof_start)),
            ));
        }
        self.end_sentence("Proof")?;
        let body_start = self.peek().start;
        let mut types = ProofTypes::new();
        for binder in &binders {
            if let Some(rocq_type) = &binder.rocq_type {
                types.insert(binder.name.clone(), rocq_type.clone());
            }
        }
        collect_forall_types(&prop, &mut types);
        let steps = self.proof_block(&types, &|parser: &Self| {
            parser.is("Qed") || parser.is("Defined") || parser.is("Admitted") || parser.is("Abort")
        })?;
        let end = self.peek().clone();
        if self.is("Admitted") || self.is("Abort") {
            return Err(unsupported(
                &format!("Rocq {}", end.value),
                "incomplete proofs cannot be translated",
                Some(span(&end, &end)),
            ));
        }
        self.cursor.advance();
        self.end_sentence(&end.value)?;
        let proof = SProof {
            steps,
            source: Some(js_trim(&self.source, body_start, end.start)),
            source_language: Language::Rocq,
        };
        Ok(STheorem {
            name,
            binders,
            prop,
            proof,
            span: Some(span(&start, self.peek())),
        })
    }

    /// A tactic script: sentences, where a sentence that splits a goal is
    /// followed either by `;` tactics for every subgoal or by one bullet or
    /// brace block per subgoal.
    fn proof_block(&mut self, types: &ProofTypes, done: Done<'_>) -> Result<Vec<SStep>> {
        let mut steps = Vec::new();
        while !self.cursor.at_end() && !done(self) {
            let mut sentence = self.tactic_chain(types)?;
            self.end_sentence("tactic")?;
            let split = sentence
                .iter()
                .position(|step| matches!(step, SStep::Induction(_) | SStep::Cases(_)));
            let Some(split) = split else {
                steps.extend(sentence);
                continue;
            };
            if split == sentence.len() - 1 && !done(self) {
                // One block per subgoal, in constructor order.
                let blocks = self.subgoal_blocks(types, done)?;
                if let SStep::Induction(step) | SStep::Cases(step) = &mut sentence[split] {
                    let count = blocks.len();
                    let mut blocks = blocks.into_iter();
                    for case in &mut step.cases {
                        case.steps = blocks.next().unwrap_or_default();
                    }
                    let first = step.cases.len();
                    for (offset, block) in blocks.enumerate() {
                        step.cases.push(SSplitCase {
                            ctor: None,
                            index: Some(first + offset),
                            binds: Vec::new(),
                            steps: block,
                        });
                    }
                    step.blocks = Some(count);
                }
            }
            steps.extend(sentence);
            if !done(self) {
                return Err(unsupported(
                    "tactics after a case split",
                    "every subgoal must be closed inside its bullet or brace block",
                    Some(span(self.peek(), self.peek())),
                ));
            }
        }
        Ok(steps)
    }

    fn subgoal_blocks(&mut self, types: &ProofTypes, done: Done<'_>) -> Result<Vec<Vec<SStep>>> {
        let mut blocks = Vec::new();
        if let Some(bullet) = self.bullet() {
            while self.bullet().is_some_and(|next| next.text == bullet.text) {
                self.consume_bullet();
                let nested = |parser: &Self| {
                    done(parser)
                        || parser.is("}")
                        || parser.bullet().is_some_and(|next| {
                            next.text == bullet.text || next.depth < bullet.depth
                        })
                };
                blocks.push(self.proof_block(types, &nested)?);
            }
            return Ok(blocks);
        }
        if self.is("{") {
            while self.cursor.eat("{").is_some() {
                blocks.push(self.proof_block(types, &|parser: &Self| parser.is("}"))?);
                self.expect("}", "subgoal block")?;
            }
            return Ok(blocks);
        }
        Err(unsupported(
            "unstructured subgoals",
            "close each subgoal of a case split inside a bullet or brace block, or with ;",
            Some(span(self.peek(), self.peek())),
        ))
    }

    /// A bullet is a run of adjacent `-`, `+` or `*` at the start of a sentence.
    fn bullet(&self) -> Option<Bullet> {
        let first = self.peek();
        if first.kind != TokenKind::Punct || !["-", "+", "*"].contains(&first.value.as_str()) {
            return None;
        }
        let mut text = first.value.clone();
        let mut offset = 1;
        loop {
            let token = self.cursor.peek_at(offset);
            if token.kind != TokenKind::Punct
                || token.value != first.value
                || token.start != self.cursor.peek_at(offset - 1).end
            {
                break;
            }
            text.push_str(&first.value);
            offset += 1;
        }
        let depth = text.encode_utf16().count();
        Some(Bullet {
            text,
            length: offset,
            depth,
        })
    }

    fn consume_bullet(&mut self) {
        let length = self.bullet().map_or(0, |bullet| bullet.length);
        for _ in 0..length {
            self.cursor.advance();
        }
    }

    fn tactic_chain(&mut self, types: &ProofTypes) -> Result<Vec<SStep>> {
        let mut steps = self.tactic(types)?;
        while self.cursor.eat(";").is_some() {
            steps.extend(self.tactic(types)?);
        }
        Ok(steps)
    }

    fn no_target(&self, token: &Token, value: &str) -> Result<()> {
        if self.is("in") || self.is("at") {
            return Err(unsupported(
                &format!("{value} {}", self.peek().value),
                "hypothesis rewriting is outside the portable proof model",
                Some(span(token, self.peek())),
            ));
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)] // one arm per tactic, as in the JavaScript runtime
    fn tactic(&mut self, types: &ProofTypes) -> Result<Vec<SStep>> {
        let token = self.peek().clone();
        if token.kind != TokenKind::Identifier {
            return Err(self.fail_here("expected a tactic"));
        }
        self.cursor.advance();
        let value = token.value.clone();
        match value.as_str() {
            "reflexivity" | "vm_compute" | "native_compute" | "compute" | "cbv" | "lazy"
            | "trivial" | "easy" | "auto" | "congruence" | "discriminate" => {
                self.no_target(&token, &value)?;
                Ok(vec![SStep::Compute {
                    tactic: Some(value),
                }])
            }
            "lia" | "nia" | "omega" | "ring" => Ok(vec![SStep::Arith {
                tactic: Some(value),
            }]),
            "intro" | "intros" => {
                let mut names = Vec::new();
                while self.cursor.is_kind(TokenKind::Identifier) {
                    names.push(self.cursor.advance().value);
                }
                Ok(vec![SStep::Intro { names }])
            }
            "simpl" | "cbn" => {
                let mut names = Vec::new();
                if self.cursor.eat("[").is_some() {
                    while !self.is("]") {
                        names.push(self.identifier(&value)?.value);
                    }
                    self.expect("]", &value)?;
                } else {
                    while self.cursor.is_kind(TokenKind::Identifier)
                        && !self.is("in")
                        && !self.is("at")
                    {
                        names.push(self.cursor.advance().value);
                    }
                }
                self.no_target(&token, &value)?;
                if names.is_empty() {
                    return Ok(vec![SStep::Simp {
                        rules: Vec::new(),
                        only: Some(false),
                        tactic: Some(value),
                    }]);
                }
                Ok(vec![SStep::Unfold {
                    names,
                    tactic: Some(value),
                }])
            }
            "unfold" => {
                let mut names = vec![self.identifier("unfold")?.value];
                while self.cursor.eat(",").is_some() {
                    names.push(self.identifier("unfold")?.value);
                }
                self.no_target(&token, &value)?;
                Ok(vec![SStep::Unfold {
                    names,
                    tactic: None,
                }])
            }
            "rewrite" => {
                let mut rules = Vec::new();
                loop {
                    // The Rocq lexer reads `<-` as `<` then `-`, so this never matches.
                    let reverse = self.cursor.eat("<-").is_some();
                    self.cursor.eat("?");
                    self.cursor.eat("!");
                    rules.push(SRule {
                        name: self.identifier("rewrite rule")?.value,
                        reverse,
                    });
                    if self.cursor.eat(",").is_none() {
                        break;
                    }
                }
                self.no_target(&token, &value)?;
                Ok(vec![SStep::Rewrite {
                    rules,
                    only: None,
                    tactic: Some(value),
                }])
            }
            "now" => {
                let mut inner = self.tactic(types)?;
                inner.push(SStep::Compute {
                    tactic: Some("easy".to_owned()),
                });
                Ok(inner)
            }
            "induction" | "destruct" => {
                let variable = self.identifier(&value)?.value;
                let mut names = None;
                let mut using = None;
                loop {
                    if self.cursor.eat("as").is_some() {
                        names = Some(self.intro_pattern()?);
                        continue;
                    }
                    if self.cursor.eat("using").is_some() {
                        using = Some(self.identifier("using")?.value);
                        continue;
                    }
                    break;
                }
                let rocq_type = types.get(&variable).map(String::as_str);
                let peano = using.as_deref() == Some("N.peano_ind");
                let here = span(&token, self.peek());
                if rocq_type == Some("N") && value == "induction" && !peano {
                    return Err(unsupported(
                        "binary induction on N",
                        "N is split as zero and successor only with `using N.peano_ind`",
                        Some(here),
                    ));
                }
                if rocq_type == Some("N") && value == "destruct" {
                    return Err(unsupported(
                        "binary case analysis on N",
                        "destructing N yields N0 and Npos, which have no portable counterpart",
                        Some(here),
                    ));
                }
                if rocq_type == Some("Z") {
                    return Err(unsupported(
                        &format!("{value} on Z"),
                        "integers are not split in the portable proof model",
                        Some(here),
                    ));
                }
                if let Some(using) = using.as_deref().filter(|_| !peano) {
                    // An empty `using` name is falsy in JavaScript, but the
                    // lexer never produces an empty identifier.
                    return Err(unsupported(
                        &format!("induction using {using}"),
                        "custom induction principles are outside the portable proof model",
                        Some(here),
                    ));
                }
                let cases = names
                    .unwrap_or_default()
                    .into_iter()
                    .enumerate()
                    .map(|(index, binds)| SSplitCase {
                        ctor: None,
                        index: Some(index),
                        binds: binds.into_iter().map(Some).collect(),
                        steps: Vec::new(),
                    })
                    .collect();
                let split = SSplit {
                    variable,
                    cases,
                    positional: true,
                    blocks: None,
                };
                Ok(vec![if value == "induction" {
                    SStep::Induction(split)
                } else {
                    SStep::Cases(split)
                }])
            }
            _ => Err(unsupported(
                &format!("Rocq tactic {value}"),
                "outside the portable proof model",
                Some(span(&token, &token)),
            )),
        }
    }

    /// `[ | k ih ]`: one list of names per constructor, in declaration order.
    fn intro_pattern(&mut self) -> Result<Vec<Vec<String>>> {
        self.expect("[", "intro pattern")?;
        let mut alternatives = vec![Vec::new()];
        while !self.is("]") {
            if self.cursor.eat("|").is_some() {
                alternatives.push(Vec::new());
                continue;
            }
            let last = alternatives.len() - 1;
            if self.cursor.eat("_").is_some() {
                alternatives[last].push("_".to_owned());
                continue;
            }
            let token = self.peek();
            if token.kind != TokenKind::Identifier {
                return Err(unsupported(
                    "intro pattern",
                    &format!("{} is outside the portable proof model", describe(token)),
                    Some(span(token, token)),
                ));
            }
            alternatives[last].push(self.cursor.advance().value);
        }
        self.expect("]", "intro pattern")?;
        Ok(alternatives)
    }

    fn prop(&mut self) -> Result<SProp> {
        let token = self.peek().clone();
        if self.cursor.eat("forall").is_some() {
            let mut binders = Vec::new();
            if self.is("(") {
                while self.is("(") {
                    binders.extend(self.binder_group()?);
                }
            } else {
                let mut names = Vec::new();
                while self.cursor.is_kind(TokenKind::Identifier) && !self.is(":") {
                    names.push(self.cursor.advance().value);
                }
                self.expect(":", "forall")?;
                let rocq_type = self.peek().value.clone();
                let ty = self.parse_type()?;
                binders.extend(names.into_iter().map(|name| SBinder {
                    name,
                    ty: Some(ty.clone()),
                    rocq_type: Some(rocq_type.clone()),
                    span: None,
                }));
            }
            self.expect(",", "forall")?;
            let body = self.prop()?;
            return Ok(SProp {
                node: SPropNode::Forall {
                    binders,
                    body: Box::new(body),
                },
                span: Some(span(&token, self.peek())),
            });
        }
        if self.is("exists") {
            return Err(unsupported(
                "existential",
                "existential statements are outside the portable proof model",
                Some(span(&token, &token)),
            ));
        }
        let left = self.prop_or()?;
        if self.cursor.eat("->").is_some() {
            let right = self.prop()?;
            return Ok(connective(SPropNode::Implies {
                left: Box::new(left),
                right: Box::new(right),
            }));
        }
        if self.is("<->") {
            return Err(unsupported(
                "logical equivalence",
                "state both implications separately",
                Some(span(self.peek(), self.peek())),
            ));
        }
        Ok(left)
    }

    fn prop_or(&mut self) -> Result<SProp> {
        let left = self.prop_and()?;
        if self.cursor.eat("\\/").is_some() {
            let right = self.prop_or()?;
            return Ok(connective(SPropNode::Or {
                left: Box::new(left),
                right: Box::new(right),
            }));
        }
        Ok(left)
    }

    fn prop_and(&mut self) -> Result<SProp> {
        let left = self.prop_unary()?;
        if self.cursor.eat("/\\").is_some() {
            let right = self.prop_and()?;
            return Ok(connective(SPropNode::And {
                left: Box::new(left),
                right: Box::new(right),
            }));
        }
        Ok(left)
    }

    fn prop_unary(&mut self) -> Result<SProp> {
        let token = self.peek().clone();
        if self.cursor.eat("~").is_some() {
            let arg = self.prop_unary()?;
            return Ok(connective(SPropNode::Not { arg: Box::new(arg) }));
        }
        if self.is("forall") {
            return self.prop();
        }
        if self.is("(") && self.prop_parenthesised() {
            self.cursor.advance();
            let inner = self.prop()?;
            self.expect(")", "proposition")?;
            return self.prop_scope(inner);
        }
        if self.is("True") || self.is("False") {
            return Err(unsupported(
                &token.value,
                "propositional constants are outside the portable proof model",
                Some(span(&token, &token)),
            ));
        }
        let left = self.expr_at(69)?;
        let relation = self.peek().clone();
        if relation.kind == TokenKind::Punct && is_prop_relation(&relation.value) {
            self.cursor.advance();
            let right = self.expr_at(69)?;
            let comparison = SComparison {
                left,
                right,
                reference: false,
            };
            return Ok(SProp {
                node: prop_relation(&relation.value, comparison)
                    .unwrap_or_else(|| unreachable_relation(&relation.value)),
                span: Some(span(&token, self.peek())),
            });
        }
        let left_span = left.span;
        let truth = SExpr::new(SNode::Bool { value: true }, None);
        let expr = SExpr::new(
            SNode::Binary {
                op: BinaryOp::Eq,
                left: Box::new(left),
                right: Box::new(truth),
                rounding: None,
            },
            left_span,
        );
        Ok(SProp {
            node: SPropNode::Bool { expr },
            span: Some(span(&token, self.peek())),
        })
    }

    /// `(P)%Z` delimits the literals of a whole proposition.
    fn prop_scope(&mut self, inner: SProp) -> Result<SProp> {
        Ok(match self.scope_delimiter()? {
            Some(scope) => scope_prop(inner, &scope),
            None => inner,
        })
    }

    fn prop_parenthesised(&self) -> bool {
        let mut depth = 0_i64;
        let mut offset = 0;
        loop {
            // Token values are compared whatever their kind, strings included.
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
                && [
                    "/\\", "\\/", "->", "~", "forall", "=", "<>", "<", "<=", ">", ">=",
                ]
                .contains(&token.value.as_str())
            {
                return true;
            }
            offset += 1;
        }
    }

    fn expr(&mut self) -> Result<SExpr> {
        self.expr_at(200)
    }

    /// Terms, by Rocq notation level: `let`, `if` and `match` at 200, the
    /// boolean relations at 70, `::` and `++` at 60, `+ - ||` at 50, `* / mod
    /// &&` at 40, unary minus at 35, and application.
    fn expr_at(&mut self, level: u32) -> Result<SExpr> {
        let token = self.peek().clone();
        if self.is("if") {
            self.cursor.advance();
            let cond = self.expr()?;
            self.expect("then", "if")?;
            let then = self.expr()?;
            self.expect("else", "if")?;
            let otherwise = self.expr()?;
            return Ok(SExpr::new(
                SNode::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                },
                Some(span(&token, self.peek())),
            ));
        }
        if self.is("let") {
            self.cursor.advance();
            if self.is("(") || self.is("'") {
                return Err(unsupported(
                    "destructuring let",
                    "tuples are outside the portable core",
                    Some(span(&token, self.peek())),
                ));
            }
            let name = self.identifier("let")?.value;
            if self.is("(") {
                return Err(unsupported(
                    "local function",
                    "let-bound functions are outside the portable core",
                    Some(span(&token, self.peek())),
                ));
            }
            let ty = if self.cursor.eat(":").is_some() {
                Some(self.parse_type()?)
            } else {
                None
            };
            self.expect(":=", "let")?;
            let value = self.expr()?;
            self.expect("in", "let")?;
            let body = self.expr()?;
            return Ok(SExpr::new(
                SNode::Let {
                    name,
                    ty,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                Some(span(&token, self.peek())),
            ));
        }
        if self.is("fun") {
            return Err(unsupported(
                "lambda",
                "higher-order values are outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        if self.is("fix") {
            return Err(unsupported(
                "local fixpoint",
                "local recursive functions are outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        self.relation(level)
    }

    fn relation(&mut self, level: u32) -> Result<SExpr> {
        let left = self.infix(60)?;
        let token = self.peek().clone();
        if level >= 70 && token.kind == TokenKind::Punct {
            if let Some(op) = boolean_relation(&token.value) {
                self.cursor.advance();
                let right = self.infix(60)?;
                return Ok(binary(op, left, right, &token));
            }
        }
        Ok(left)
    }

    /// Levels 60 and below: `::`, `++` (right), `+ - ||` and `* / mod &&` (left).
    fn infix(&mut self, level: u32) -> Result<SExpr> {
        if level == 60 {
            let left = self.infix(50)?;
            let token = self.peek().clone();
            if self.is("::") {
                self.cursor.advance();
                let tail = self.cons()?;
                let cons_span = joined(&left, &tail, &token);
                return Ok(SExpr::new(
                    SNode::Cons {
                        head: Box::new(left),
                        tail: Box::new(tail),
                    },
                    Some(cons_span),
                ));
            }
            if self.is("++") {
                self.cursor.advance();
                let right = self.infix(60)?;
                return Ok(binary(BinaryOp::Concat, left, right, &token));
            }
            return Ok(left);
        }
        let mut left = if level == 50 {
            self.infix(40)?
        } else {
            self.prefix()?
        };
        loop {
            let token = self.peek().clone();
            let Some(op) = notation(level, &token) else {
                return Ok(left);
            };
            self.cursor.advance();
            let right = if level == 50 {
                self.infix(40)?
            } else {
                self.prefix()?
            };
            left = binary(op, left, right, &token);
        }
    }

    /// The tail of `::` may itself be a `let` of the program output.
    fn cons(&mut self) -> Result<SExpr> {
        if self.is("let") || self.is("if") || self.is("match") {
            return self.expr();
        }
        self.infix(60)
    }

    fn prefix(&mut self) -> Result<SExpr> {
        let token = self.peek().clone();
        if self.is("-") {
            self.cursor.advance();
            let arg = self.prefix()?;
            return Ok(SExpr::new(
                SNode::Unary {
                    op: UnaryOp::Neg,
                    arg: Box::new(arg),
                },
                Some(span(&token, self.peek())),
            ));
        }
        self.application()
    }

    fn argument_start(&self) -> bool {
        let token = self.peek();
        if token.kind == TokenKind::Identifier {
            let value = token.value.as_str();
            return !EXPRESSION_STOP.contains(&value)
                && value != "mod"
                && !["if", "let", "match", "fun", "fix"].contains(&value);
        }
        matches!(token.kind, TokenKind::Number | TokenKind::String)
            || token.value == "("
            || token.value == "["
    }

    fn application(&mut self) -> Result<SExpr> {
        let token = self.peek().clone();
        if self.is("match") {
            return self.match_expr();
        }
        let head = self.atom()?;
        let mut args = Vec::new();
        while self.argument_start() {
            args.push(self.atom()?);
        }
        self.apply_head(head, args, &token)
    }

    #[allow(clippy::too_many_lines)] // one branch per library family, as in the JavaScript runtime
    fn apply_head(&self, head: SExpr, args: Vec<SExpr>, token: &Token) -> Result<SExpr> {
        let range = span(token, self.peek());
        let node = |node: SNode| SExpr::new(node, Some(range));
        let name = match &head.node {
            SNode::Name { path } if !path.is_empty() => Some(path.join(".")),
            _ => None,
        };
        if let Some(name) = name {
            if let Some((op, rounding)) = binary_function(&name) {
                let [left, right] = arity::<2>(&name, args, range)?;
                return Ok(node(SNode::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding,
                }));
            }
            if name == "negb" || name == "Z.opp" {
                let [arg] = arity::<1>(&name, args, range)?;
                let op = if name == "negb" {
                    UnaryOp::Not
                } else {
                    UnaryOp::Neg
                };
                return Ok(node(SNode::Unary {
                    op,
                    arg: Box::new(arg),
                }));
            }
            let cast = if NAT_TO_INT.contains(&name.as_str()) {
                Some((INT, NAT, Flavor::Exact))
            } else if INT_TO_NAT.contains(&name.as_str()) {
                Some((NAT, INT, Flavor::Clamp))
            } else if NAT_IDENTITY.contains(&name.as_str()) {
                Some((NAT, NAT, Flavor::Exact))
            } else {
                None
            };
            if let Some((to, from, flavor)) = cast {
                let [arg] = arity::<1>(&name, args, range)?;
                return Ok(node(SNode::Cast {
                    arg: Box::new(arg),
                    to,
                    from: Some(from),
                    flavor,
                }));
            }
            let predecessor = PREDECESSOR.contains(&name.as_str());
            if predecessor || SUCCESSOR.contains(&name.as_str()) {
                let [arg] = arity::<1>(&name, args, range)?;
                let op = if predecessor {
                    BinaryOp::Sub
                } else {
                    BinaryOp::Add
                };
                let one = node(SNode::Num {
                    value: "1".to_owned(),
                    ty: name.starts_with("Z.").then_some(INT),
                    negative: false,
                });
                return Ok(node(SNode::Binary {
                    op,
                    left: Box::new(arg),
                    right: Box::new(one),
                    rounding: None,
                }));
            }
            if let Some((_, accepted)) = DECIMAL_STRINGS.iter().find(|(key, _)| *key == name) {
                let [inner] = arity::<1>(&name, args, range)?;
                if let SNode::App { func, args } = inner.node {
                    let converts = matches!(&func.node, SNode::Name { path } if accepted.contains(&path.join(".").as_str()));
                    if converts && args.len() == 1 {
                        if let Some(arg) = args.into_iter().next() {
                            return Ok(node(SNode::ToString { arg: Box::new(arg) }));
                        }
                    }
                }
                return Err(unsupported(
                    &name,
                    &format!("only {name} ({} x) renders a number", accepted.join(" | ")),
                    Some(range),
                ));
            }
            if name == "tt" && args.is_empty() {
                return Ok(node(SNode::Unit));
            }
            if name == "true" || name == "false" {
                if !args.is_empty() {
                    return Err(type_error(format!("{name} is not a function"), Some(range)));
                }
                return Ok(node(SNode::Bool {
                    value: name == "true",
                }));
            }
            // A name contained in a `DECIMAL_STRINGS` key (`String.Nil`, …)
            // escapes this check, as in the JavaScript runtime.
            if outside_core(&name) && !DECIMAL_STRINGS.iter().any(|(key, _)| key.contains(&name)) {
                if !args.is_empty() && decimal_conversion(&name) {
                    return Ok(node(SNode::App {
                        func: Box::new(head),
                        args,
                    }));
                }
                return Err(unsupported(
                    &format!("Rocq library function {name}"),
                    "outside the portable core",
                    Some(range),
                ));
            }
            if name == "nil" && args.is_empty() {
                return Ok(node(SNode::Nil));
            }
        }
        if args.is_empty() {
            return Ok(head);
        }
        if !matches!(head.node, SNode::Name { .. }) {
            return Err(unsupported(
                "higher-order application",
                "only named functions can be applied",
                Some(range),
            ));
        }
        Ok(node(SNode::App {
            func: Box::new(head),
            args,
        }))
    }

    fn match_expr(&mut self) -> Result<SExpr> {
        let token = self.cursor.advance();
        let mut scrutinees = vec![self.expr()?];
        while self.cursor.eat(",").is_some() {
            scrutinees.push(self.expr()?);
        }
        if self.is("as") || self.is("in") || self.is("return") {
            return Err(unsupported(
                "dependent match",
                "return clauses are outside the portable core",
                Some(span(&token, self.peek())),
            ));
        }
        self.expect("with", "match")?;
        let mut rows = Vec::new();
        self.cursor.eat("|");
        if !self.is("end") {
            loop {
                let bar = self.peek().clone();
                let mut aliases = Vec::new();
                let mut patterns = vec![self.pattern(&mut aliases)?];
                while self.cursor.eat(",").is_some() {
                    patterns.push(self.pattern(&mut aliases)?);
                }
                if patterns.len() != scrutinees.len() {
                    return Err(TranslationError::syntax(
                        format!(
                            "alternative has {} patterns, expected {}",
                            patterns.len(),
                            scrutinees.len()
                        ),
                        Some(bar.span()),
                    ));
                }
                self.expect("=>", "alternative")?;
                let mut body = self.expr()?;
                for (name, value) in aliases.into_iter().rev() {
                    let body_span = body.span;
                    body = SExpr::new(
                        SNode::Let {
                            name,
                            ty: None,
                            value: Box::new(value),
                            body: Box::new(body),
                        },
                        body_span,
                    );
                }
                rows.push(SRow {
                    patterns,
                    body,
                    span: Some(span(&bar, self.peek())),
                });
                if self.cursor.eat("|").is_none() {
                    break;
                }
            }
        }
        self.expect("end", "match")?;
        if rows.is_empty() {
            return Err(self.fail_here("expected match alternatives"));
        }
        let match_span = span(&token, self.peek());
        self.scoped(SExpr::new(
            SNode::Match { scrutinees, rows },
            Some(match_span),
        ))
    }

    /// Patterns: constructors (`O`, `S k`, `node l v r`), numerals, `_`,
    /// variables, parentheses and `p as x`. An alias becomes a `let` of the
    /// value the pattern matched, rebuilt from its fields.
    fn pattern(&mut self, aliases: &mut Vec<(String, SExpr)>) -> Result<SPattern> {
        let token = self.peek().clone();
        let pattern = if token.kind == TokenKind::Identifier && token.value != "_" {
            self.cursor.advance();
            let path: Vec<String> = token.value.split('.').map(str::to_owned).collect();
            let mut args = Vec::new();
            while self.pattern_argument_start() {
                args.push(self.pattern_atom(aliases)?);
            }
            if args.is_empty() && path.len() == 1 {
                SPattern {
                    node: SPatternNode::BindOrCtor {
                        name: token.value.clone(),
                    },
                    span: Some(span(&token, &token)),
                }
            } else {
                SPattern {
                    node: SPatternNode::Ctor { path, args },
                    span: Some(span(&token, self.peek())),
                }
            }
        } else {
            self.pattern_atom(aliases)?
        };
        while self.cursor.eat("as").is_some() {
            let name = self.identifier("as")?.value;
            let value = pattern_value(&pattern, span(&token, self.peek()))?;
            aliases.push((name, value));
        }
        Ok(pattern)
    }

    fn pattern_argument_start(&self) -> bool {
        let token = self.peek();
        (token.kind == TokenKind::Identifier
            && !["as", "with", "end"].contains(&token.value.as_str()))
            || token.kind == TokenKind::Number
            || self.is("(")
            || self.is("_")
    }

    fn pattern_atom(&mut self, aliases: &mut Vec<(String, SExpr)>) -> Result<SPattern> {
        let token = self.peek().clone();
        if self.cursor.eat("_").is_some() {
            return Ok(SPattern {
                node: SPatternNode::Wild,
                span: None,
            });
        }
        if token.kind == TokenKind::Number {
            self.cursor.advance();
            if self.is("%") {
                return Err(unsupported(
                    "scoped numeral pattern",
                    "numeral patterns on N or Z match binary constructors, which have no portable counterpart",
                    Some(span(&token, self.peek())),
                ));
            }
            return Ok(SPattern {
                node: SPatternNode::NumLit {
                    value: token.value.clone(),
                    negative: false,
                },
                span: Some(span(&token, &token)),
            });
        }
        if self.cursor.eat("(").is_some() {
            let inner = self.pattern(aliases)?;
            if self.is("|") {
                return Err(unsupported(
                    "or-pattern",
                    "disjunctive patterns are outside the portable core",
                    Some(span(&token, self.peek())),
                ));
            }
            self.expect(")", "pattern")?;
            return Ok(inner);
        }
        if token.kind == TokenKind::Identifier {
            self.cursor.advance();
            let path: Vec<String> = token.value.split('.').map(str::to_owned).collect();
            let node = if path.len() == 1 {
                if token.value == "true" || token.value == "false" {
                    SPatternNode::BoolLit {
                        value: token.value == "true",
                        negative: false,
                    }
                } else {
                    SPatternNode::BindOrCtor {
                        name: token.value.clone(),
                    }
                }
            } else {
                SPatternNode::Ctor {
                    path,
                    args: Vec::new(),
                }
            };
            return Ok(SPattern {
                node,
                span: Some(span(&token, &token)),
            });
        }
        if token.kind == TokenKind::String {
            return Err(unsupported(
                "string pattern",
                "string patterns are outside the portable core in Rocq",
                Some(span(&token, &token)),
            ));
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

    fn atom(&mut self) -> Result<SExpr> {
        let token = self.peek().clone();
        if token.kind == TokenKind::Number {
            self.cursor.advance();
            return self.scoped(SExpr::new(
                SNode::Num {
                    value: token.value.clone(),
                    ty: None,
                    negative: false,
                },
                Some(span(&token, &token)),
            ));
        }
        if token.kind == TokenKind::String {
            self.cursor.advance();
            return self.scoped(SExpr::new(
                SNode::Str {
                    value: token.value.clone(),
                },
                Some(span(&token, &token)),
            ));
        }
        if self.cursor.eat("[").is_some() {
            let mut items = Vec::new();
            while !self.is("]") {
                items.push(self.expr()?);
                if self.cursor.eat(";").is_none() {
                    break;
                }
            }
            self.expect("]", "list")?;
            return Ok(SExpr::new(
                SNode::List { items },
                Some(span(&token, self.peek())),
            ));
        }
        if self.cursor.eat("(").is_some() {
            if self.cursor.eat(")").is_some() {
                return Ok(SExpr::new(SNode::Unit, Some(span(&token, &token))));
            }
            let inner = self.expr()?;
            if self.cursor.eat(":").is_some() {
                let ty = self.parse_type()?;
                self.expect(")", "type ascription")?;
                let ascribed_span = Some(span(&token, self.peek()));
                if let SNode::Num {
                    value, negative, ..
                } = inner.node
                {
                    return self.scoped(SExpr {
                        node: SNode::Num {
                            value,
                            ty: Some(ty),
                            negative,
                        },
                        span: ascribed_span,
                        block: inner.block,
                        tag_test: None,
                    });
                }
                return self.scoped(SExpr::new(
                    SNode::Cast {
                        arg: Box::new(inner),
                        to: ty,
                        from: None,
                        flavor: Flavor::Exact,
                    },
                    ascribed_span,
                ));
            }
            if self.is(",") {
                return Err(unsupported(
                    "tuple",
                    "product values are outside the portable core",
                    Some(span(&token, self.peek())),
                ));
            }
            self.expect(")", "parenthesised expression")?;
            return self.scoped(inner);
        }
        if token.kind == TokenKind::Identifier {
            if self.is("match") {
                return self.match_expr();
            }
            self.cursor.advance();
            if token.value == "_" {
                return Err(unsupported(
                    "implicit argument hole",
                    "outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            return self.scoped(SExpr::new(
                SNode::Name {
                    path: token.value.split('.').map(str::to_owned).collect(),
                },
                Some(span(&token, &token)),
            ));
        }
        Err(self.fail_here("expected an expression"))
    }

    fn scope_delimiter(&mut self) -> Result<Option<String>> {
        if !self.is("%") {
            return Ok(None);
        }
        self.cursor.advance();
        let scope = self.identifier("scope")?;
        if !["N", "nat", "Z", "string", "bool", "list", "type"].contains(&scope.value.as_str()) {
            return Err(unsupported(
                &format!("%{} scope", scope.value),
                "outside the portable core",
                Some(span(&scope, &scope)),
            ));
        }
        Ok(Some(scope.value))
    }

    /// `e%N`, `(e)%Z`: the delimiting scope decides the type of notation numerals.
    fn scoped(&mut self, node: SExpr) -> Result<SExpr> {
        Ok(match self.scope_delimiter()? {
            Some(scope) => with_scope(node, &scope),
            None => node,
        })
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

/// Lists only appear as the program output: the first list node, in the
/// JavaScript runtime's depth-first key order, is rejected.
fn check_portable(node: &SExpr) -> Result<()> {
    let children: Vec<&SExpr> = match &node.node {
        SNode::Cons { .. } | SNode::Nil | SNode::List { .. } => {
            return Err(unsupported(
                "list value",
                "lists are outside the portable core except as the program output",
                node.span,
            ));
        }
        SNode::Binary { left, right, .. } => vec![left, right],
        SNode::If {
            cond,
            then,
            otherwise,
        } => vec![cond, then, otherwise],
        SNode::Let { value, body, .. } => vec![value, body],
        SNode::Match { scrutinees, rows } => scrutinees
            .iter()
            .chain(rows.iter().map(|row| &row.body))
            .collect(),
        SNode::App { func, args } => std::iter::once(func.as_ref()).chain(args).collect(),
        SNode::Unary { arg, .. } | SNode::Cast { arg, .. } | SNode::ToString { arg } => {
            vec![arg]
        }
        _ => Vec::new(),
    };
    children.into_iter().try_for_each(check_portable)
}

/// The program output: a list of lines built with `::`, `[ ; ]` and `let`.
fn walk_output(node: SExpr, effects: &mut Vec<SEffect>) -> Result<()> {
    let node_span = node.span;
    match node.node {
        SNode::Nil => Ok(()),
        SNode::List { items } => {
            for item in items {
                effects.push(print_effect(item)?);
            }
            Ok(())
        }
        SNode::Cons { head, tail } => {
            effects.push(print_effect(*head)?);
            walk_output(*tail, effects)
        }
        SNode::Let {
            name,
            ty,
            value,
            body,
        } => {
            check_portable(&value)?;
            effects.push(SEffect::Let {
                name,
                ty,
                value: *value,
                span: node_span,
            });
            walk_output(*body, effects)
        }
        _ => Err(unsupported(
            "program output",
            "main must be a list of lines built with ::, [ ; ] and let",
            node_span,
        )),
    }
}

fn print_effect(expr: SExpr) -> Result<SEffect> {
    check_portable(&expr)?;
    let span = expr.span;
    Ok(SEffect::Print {
        expr,
        style: ShowStyle::Rocq,
        span,
    })
}

/// Numerals in notation positions take the delimited scope; arguments of
/// applications keep the scope of their parameter type, which the checker
/// infers from the signature, exactly as Rocq's argument scopes do.
fn with_scope(node: SExpr, scope: &str) -> SExpr {
    match scope_type(scope) {
        Some(ty) => scope_expr(node, &ty),
        None => node,
    }
}

fn scope_expr(node: SExpr, ty: &Type) -> SExpr {
    let visit = |inner: Box<SExpr>| Box::new(scope_expr(*inner, ty));
    let scoped = match node.node {
        SNode::Num {
            value,
            ty: None,
            negative,
        } => SNode::Num {
            value,
            ty: Some(ty.clone()),
            negative,
        },
        SNode::Unary { op, arg } => SNode::Unary {
            op,
            arg: visit(arg),
        },
        SNode::Binary {
            op:
                op @ (BinaryOp::Eq
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge
                | BinaryOp::Ne
                | BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div
                | BinaryOp::Rem),
            left,
            right,
            rounding,
        } => SNode::Binary {
            op,
            left: visit(left),
            right: visit(right),
            rounding,
        },
        SNode::If {
            cond,
            then,
            otherwise,
        } => SNode::If {
            cond,
            then: visit(then),
            otherwise: visit(otherwise),
        },
        SNode::Let {
            name,
            ty: let_type,
            value,
            body,
        } => SNode::Let {
            name,
            ty: let_type,
            value,
            body: visit(body),
        },
        SNode::Match { scrutinees, rows } => SNode::Match {
            scrutinees,
            rows: rows
                .into_iter()
                .map(|row| SRow {
                    body: scope_expr(row.body, ty),
                    ..row
                })
                .collect(),
        },
        other => other,
    };
    SExpr {
        node: scoped,
        ..node
    }
}

/// The literals of a whole proposition take the delimited scope.
fn scope_prop(prop: SProp, scope: &str) -> SProp {
    let visit = |inner: Box<SProp>| Box::new(scope_prop(*inner, scope));
    let compare = |comparison: SComparison| SComparison {
        left: with_scope(comparison.left, scope),
        right: with_scope(comparison.right, scope),
        reference: comparison.reference,
    };
    let node = match prop.node {
        SPropNode::Forall { binders, body } => SPropNode::Forall {
            binders,
            body: visit(body),
        },
        SPropNode::Not { arg } => SPropNode::Not { arg: visit(arg) },
        SPropNode::Bool { expr } => SPropNode::Bool { expr },
        SPropNode::And { left, right } => SPropNode::And {
            left: visit(left),
            right: visit(right),
        },
        SPropNode::Or { left, right } => SPropNode::Or {
            left: visit(left),
            right: visit(right),
        },
        SPropNode::Implies { left, right } => SPropNode::Implies {
            left: visit(left),
            right: visit(right),
        },
        SPropNode::Eq(comparison) => SPropNode::Eq(compare(comparison)),
        SPropNode::Ne(comparison) => SPropNode::Ne(compare(comparison)),
        SPropNode::Lt(comparison) => SPropNode::Lt(compare(comparison)),
        SPropNode::Le(comparison) => SPropNode::Le(compare(comparison)),
        SPropNode::Gt(comparison) => SPropNode::Gt(compare(comparison)),
        SPropNode::Ge(comparison) => SPropNode::Ge(compare(comparison)),
    };
    SProp { node, ..prop }
}

/// The value a fully bound pattern matched, for `p as x`.
fn pattern_value(pattern: &SPattern, range: Span) -> Result<SExpr> {
    let node = |node: SNode| SExpr::new(node, Some(range));
    match &pattern.node {
        SPatternNode::BindOrCtor { name } => Ok(node(SNode::Name {
            path: vec![name.clone()],
        })),
        SPatternNode::NumLit { value, .. } => Ok(node(SNode::Num {
            value: value.clone(),
            ty: None,
            negative: false,
        })),
        SPatternNode::Ctor { path, args } => {
            let name = path.join(".");
            if (name == "S" || name == "Nat.succ") && args.len() == 1 {
                let left = pattern_value(&args[0], range)?;
                let one = node(SNode::Num {
                    value: "1".to_owned(),
                    ty: None,
                    negative: false,
                });
                return Ok(node(SNode::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(left),
                    right: Box::new(one),
                    rounding: None,
                }));
            }
            let args = args
                .iter()
                .map(|arg| pattern_value(arg, range))
                .collect::<Result<Vec<_>>>()?;
            Ok(node(SNode::App {
                func: Box::new(node(SNode::Name { path: path.clone() })),
                args,
            }))
        }
        _ => Err(unsupported(
            "as-pattern",
            "an aliased pattern must bind every field",
            Some(range),
        )),
    }
}

fn collect_forall_types(prop: &SProp, types: &mut ProofTypes) {
    if let SPropNode::Forall { binders, body } = &prop.node {
        for binder in binders {
            if let Some(rocq_type) = &binder.rocq_type {
                types.insert(binder.name.clone(), rocq_type.clone());
            }
        }
        collect_forall_types(body, types);
    }
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
