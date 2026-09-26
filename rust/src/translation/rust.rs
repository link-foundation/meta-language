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

use super::diagnostics::{type_error, unsupported, Result, TranslationError};
use super::lexer::{describe, tokenize, Token, TokenCursor, TokenKind};
use super::surface::{
    BinaryOp, Flavor, Rounding, SComparison, SCtor, SData, SEffect, SExpr, SField, SFn, SItem,
    SMain, SModule, SNode, SParam, SPattern, SPatternNode, SProgram, SProp, SPropNode, SRow,
    ShowStyle, UnaryOp,
};
use super::types::{rust_fixed_type, Type, BOOL, STRING, UNIT};
use super::{Language, Span};

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

    fn file(mut self) -> Result<SProgram> {
        self.inner_attributes()?;
        let items = self.items(&[], ItemsEnd::File)?;
        let Some(main) = self.main.take() else {
            return Err(TranslationError::syntax(
                "a Rust program needs fn main",
                Some(self.cursor.peek().span()),
            ));
        };
        Ok(SProgram {
            language: Language::Rust,
            items,
            main: Some(main),
        })
    }

    fn inner_attributes(&mut self) -> Result<()> {
        while self.cursor.is("#") && self.cursor.is_at("!", 1) {
            let start = self.cursor.advance();
            self.cursor.advance();
            self.attribute_body(&start)?;
        }
        Ok(())
    }

    fn attributes(&mut self) -> Result<()> {
        while self.cursor.is("#") && self.cursor.is_at("[", 1) {
            let start = self.cursor.advance();
            self.attribute_body(&start)?;
        }
        Ok(())
    }

    /// `#[derive(Clone, …)]` and lint attributes do not change behaviour.
    fn attribute_body(&mut self, start: &Token) -> Result<()> {
        let c = &mut self.cursor;
        c.expect("[", Some("attribute"))?;
        let name = c.identifier(Some("attribute"))?.value;
        if name == "derive" {
            c.expect("(", Some("derive"))?;
            while !c.is(")") {
                let derived = c.identifier(Some("derive"))?;
                if !ACCEPTED_DERIVES.contains(&derived.value.as_str()) {
                    return Err(unsupported(
                        &format!("derive({})", derived.value),
                        "only derives that add no behaviour are portable",
                        span(&derived, &derived),
                    ));
                }
                if c.eat(",").is_none() {
                    break;
                }
            }
            c.expect(")", Some("derive"))?;
        } else if ACCEPTED_LINTS.contains(&name.as_str()) {
            let mut depth = 0_i64;
            while !c.at_end() && (depth != 0 || !c.is("]")) {
                if c.is("(") {
                    depth += 1;
                }
                if c.is(")") {
                    depth -= 1;
                }
                c.advance();
            }
        } else {
            return Err(unsupported(
                &format!("#[{name}]"),
                "attributes that change compilation are outside the portable core",
                span(start, c.peek()),
            ));
        }
        c.expect("]", Some("attribute"))?;
        Ok(())
    }

    fn items(&mut self, path: &[String], end: ItemsEnd) -> Result<Vec<SItem>> {
        let mut items = Vec::new();
        loop {
            let done = match end {
                ItemsEnd::File => self.cursor.at_end(),
                ItemsEnd::Module => self.cursor.is("}"),
            };
            if done {
                return Ok(items);
            }
            self.attributes()?;
            let start = self.peek();
            self.module_path = path.to_vec();
            self.visibility()?;
            let c = &mut self.cursor;
            if c.is("mod") {
                c.advance();
                let name = c.identifier(Some("mod"))?.value;
                if c.is(";") {
                    return Err(unsupported(
                        "out-of-line module",
                        "the program must be a single file",
                        span(&start, c.peek()),
                    ));
                }
                c.expect("{", Some("mod"))?;
                let module_span = span(&start, c.peek());
                let mut inner = path.to_vec();
                inner.push(name.clone());
                let module_items = self.items(&inner, ItemsEnd::Module)?;
                self.cursor.expect("}", Some("mod"))?;
                items.push(SItem::Module(SModule {
                    name,
                    items: module_items,
                    span: module_span,
                }));
                continue;
            }
            if c.is("use") {
                self.use_declaration(path)?;
                continue;
            }
            if c.is("enum") {
                items.push(self.enum_item()?);
                continue;
            }
            if c.is("fn") || c.is("const") {
                if c.is("fn") && c.is_at("main", 1) && path.is_empty() {
                    self.main = Some(self.main_fn()?);
                    continue;
                }
                let item = if c.is("fn") {
                    self.function(path)?
                } else {
                    self.const_item(path)?
                };
                items.push(SItem::Fn(item));
                continue;
            }
            if start.kind == TokenKind::Identifier && RESERVED_ITEMS.contains(&start.value.as_str())
            {
                return Err(unsupported(
                    &format!("Rust {} item", start.value),
                    &format!("{} items are outside the portable core", start.value),
                    span(&start, &start),
                ));
            }
            if c.at_end() {
                // `path.at(-1)` of the file's empty path reads `undefined`.
                let module = path.last().map_or("undefined", String::as_str);
                return Err(TranslationError::syntax(
                    format!("module {module} is not closed"),
                    Some(start.span()),
                ));
            }
            return Err(Self::fail("expected a Rust item", c.peek()));
        }
    }

    fn visibility(&mut self) -> Result<()> {
        let c = &mut self.cursor;
        if c.eat("pub").is_none() {
            return Ok(());
        }
        if c.is("(") {
            c.advance();
            c.eat("in");
            // The end of input stops the scan, so an unclosed `pub(` is reported rather than looping.
            while !c.is(")") && !c.at_end() {
                c.advance();
            }
            c.expect(")", Some("visibility"))?;
        }
        Ok(())
    }

    /// `use super::Tree;`, `use crate::a::{b, c as d};`: each imported name is
    /// an alias for its absolute path inside the importing module.
    fn use_declaration(&mut self, path: &[String]) -> Result<()> {
        let start = self.cursor.advance();
        self.aliases.entry(path.join(".")).or_default();
        self.use_tree(&[], &start, path)?;
        self.cursor.expect(";", Some("use"))?;
        Ok(())
    }

    fn use_tree(&mut self, prefix: &[String], start: &Token, path: &[String]) -> Result<()> {
        if self.cursor.eat("{").is_some() {
            while !self.cursor.is("}") {
                self.use_tree(prefix, start, path)?;
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect("}", Some("use"))?;
            return Ok(());
        }
        let c = &mut self.cursor;
        if c.is("*") {
            return Err(unsupported(
                "glob import",
                "name each imported item explicitly",
                span(start, c.peek()),
            ));
        }
        let segment = c.identifier(Some("use"))?.value;
        let mut full = prefix.to_vec();
        full.push(segment.clone());
        if c.eat("::").is_some() {
            return self.use_tree(&full, start, path);
        }
        if !["crate", "super", "self"].contains(&full[0].as_str()) {
            return Err(unsupported(
                &format!("use {}", full.join("::")),
                "only items of this crate can be imported; the standard library is outside the portable core",
                span(start, c.peek()),
            ));
        }
        let alias = if c.eat("as").is_some() {
            c.identifier(Some("use alias"))?.value
        } else {
            segment
        };
        let target = absolute(&full, path, span(start, c.peek()))?;
        self.aliases
            .entry(path.join("."))
            .or_default()
            .insert(alias, target);
        Ok(())
    }

    /// A `use` alias is visible only in the module that declares it.
    fn resolve(&self, segments: &[String], path: &[String]) -> Vec<String> {
        self.aliases
            .get(&path.join("."))
            .and_then(|aliases| aliases.get(&segments[0]))
            .map_or_else(
                || segments.to_vec(),
                |target| target.iter().chain(&segments[1..]).cloned().collect(),
            )
    }

    fn enum_item(&mut self) -> Result<SItem> {
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("enum"))?.value;
        if self.cursor.is("<") {
            return Err(unsupported(
                "generic enum",
                "type parameters are outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        self.cursor.expect("{", Some("enum"))?;
        let mut ctors = Vec::new();
        while !self.cursor.is("}") {
            self.attributes()?;
            let variant = self.cursor.identifier(Some("variant"))?;
            let mut fields = Vec::new();
            if self.cursor.eat("(").is_some() {
                while !self.cursor.is(")") {
                    fields.push(SField {
                        name: None,
                        ty: self.parse_type()?,
                        rocq_type: None,
                        span: None,
                    });
                    if self.cursor.eat(",").is_none() {
                        break;
                    }
                }
                self.cursor.expect(")", Some("variant"))?;
            } else if self.cursor.is("{") {
                return Err(unsupported(
                    "struct variant",
                    "use a tuple variant; named fields are outside the Rust frontend",
                    span(&variant, self.cursor.peek()),
                ));
            } else if self.cursor.is("=") {
                return Err(unsupported(
                    "explicit discriminant",
                    "discriminant values are outside the portable core",
                    span(&variant, self.cursor.peek()),
                ));
            }
            ctors.push(SCtor {
                name: variant.value,
                fields,
            });
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("enum"))?;
        Ok(SItem::Data(SData {
            name,
            ctors,
            span: span(&start, self.cursor.peek()),
        }))
    }

    /// Types: machine integers, `bool`, `String`, `&str`, `()`, crate types,
    /// and `&T`/`Box<T>`, which denote the value `T` since every portable value
    /// is immutable.
    fn parse_type(&mut self) -> Result<Type> {
        let token = self.peek();
        if self.cursor.eat("&").is_some() {
            if self.cursor.is("'") {
                self.cursor.advance();
                let lifetime = self.cursor.identifier(Some("lifetime"))?;
                if lifetime.value != "static" {
                    return Err(unsupported(
                        "lifetime parameter",
                        "only 'static references are portable",
                        span(&token, &lifetime),
                    ));
                }
            }
            if self.cursor.is("mut") {
                return Err(unsupported(
                    "mutable reference",
                    "mutation is outside the portable core",
                    span(&token, self.cursor.peek()),
                ));
            }
            if self.cursor.is("str") {
                self.cursor.advance();
                return Ok(STRING);
            }
            return self.parse_type();
        }
        if self.cursor.eat("(").is_some() {
            if self.cursor.eat(")").is_some() {
                return Ok(UNIT);
            }
            return Err(unsupported(
                "tuple type",
                "tuples are outside the portable core",
                span(&token, self.cursor.peek()),
            ));
        }
        let segments = self.path_segments("type")?;
        let name = segments.join("::");
        if name == "Box" {
            self.cursor.expect("<", Some("Box"))?;
            let inner = self.parse_type()?;
            self.cursor.expect(">", Some("Box"))?;
            return Ok(inner);
        }
        if self.cursor.is("<") {
            return Err(unsupported(
                &format!("type {name}<…>"),
                "generic types are outside the portable core",
                span(&token, self.cursor.peek()),
            ));
        }
        if segments.len() == 1 {
            if let Some(fixed) = rust_fixed_type(&name) {
                return Ok(fixed);
            }
        }
        if name == "bool" {
            return Ok(BOOL);
        }
        if name == "String" {
            return Ok(STRING);
        }
        if UNSUPPORTED_TYPES.contains(&name.as_str()) {
            return Err(unsupported(
                &format!("Rust type {name}"),
                "outside the portable core",
                span(&token, &token),
            ));
        }
        Ok(Type::Named {
            path: self.resolve(&segments, &self.module_path),
            span: span(&token, &token),
        })
    }

    fn path_segments(&mut self, context: &str) -> Result<Vec<String>> {
        let c = &mut self.cursor;
        let mut segments = vec![c.identifier(Some(context))?.value];
        while c.is("::") && c.peek_at(1).kind == TokenKind::Identifier {
            c.advance();
            segments.push(c.advance().value);
        }
        Ok(segments)
    }

    fn function(&mut self, path: &[String]) -> Result<SFn> {
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("fn"))?.value;
        if self.cursor.is("<") {
            return Err(unsupported(
                "generic function",
                "type parameters are outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        self.cursor.expect("(", Some("fn"))?;
        let mut params = Vec::new();
        while !self.cursor.is(")") {
            let param_token = self.peek();
            if self.cursor.is("mut") {
                return Err(unsupported(
                    "mutable parameter",
                    "mutation is outside the portable core",
                    span(&param_token, &param_token),
                ));
            }
            if self.cursor.is("self") || self.cursor.is("&") {
                return Err(unsupported(
                    "method",
                    "methods are outside the Rust frontend; declare a free function",
                    span(&param_token, &param_token),
                ));
            }
            let param_name = self.cursor.identifier(Some("parameter"))?.value;
            self.cursor.expect(":", Some("parameter"))?;
            let ty = self.parse_type()?;
            params.push(SParam {
                name: param_name,
                ty: Some(ty),
                span: span(&param_token, self.cursor.peek()),
                guard: None,
                rocq_type: None,
            });
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect(")", Some("fn"))?;
        if self.cursor.eat("->").is_none() {
            return Err(unsupported(
                "function without a result",
                &format!("{name} returns (); only value-returning functions are portable"),
                span(&start, self.cursor.peek()),
            ));
        }
        let ret = self.parse_type()?;
        if self.cursor.is("where") {
            return Err(unsupported(
                "where clause",
                "trait bounds are outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        let body = self.block(path)?;
        Ok(SFn {
            name,
            params,
            ret: Some(ret),
            body,
            span: span(&start, self.cursor.peek()),
        })
    }

    /// `const N: T = e;` is a function without parameters.
    fn const_item(&mut self, path: &[String]) -> Result<SFn> {
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("const"))?.value;
        self.cursor.expect(":", Some("const"))?;
        let ret = self.parse_type()?;
        self.cursor.expect("=", Some("const"))?;
        let body = self.expr(path, Options::default())?;
        self.cursor.expect(";", Some("const"))?;
        Ok(SFn {
            name,
            params: Vec::new(),
            ret: Some(ret),
            body,
            span: span(&start, self.cursor.peek()),
        })
    }

    /// A block `{ let x = e; …; tail }` is a chain of lets ending in its tail
    /// expression. Statements with effects are not values of the portable core.
    fn block(&mut self, path: &[String]) -> Result<SExpr> {
        let open = self.cursor.expect("{", Some("block"))?;
        let mut lets = Vec::new();
        let mut tail: Option<SExpr> = None;
        while !self.cursor.is("}") {
            let token = self.peek();
            if self.cursor.is("let") {
                lets.push(self.let_statement(path)?);
                continue;
            }
            if self.cursor.is("return") {
                return Err(unsupported(
                    "early return",
                    "write the result as the tail expression of the block",
                    span(&token, &token),
                ));
            }
            if token.kind == TokenKind::Identifier
                && ["while", "loop", "for"].contains(&token.value.as_str())
            {
                return Err(unsupported(
                    &format!("{} loop", token.value),
                    "loops over mutable state are outside the portable core; use recursion",
                    span(&token, &token),
                ));
            }
            let expr = self.expr(
                path,
                Options {
                    statement: true,
                    no_struct: false,
                },
            )?;
            if self.cursor.eat(";").is_some() {
                if matches!(expr.node, SNode::Abort { .. }) {
                    tail = Some(expr);
                    continue;
                }
                return Err(unsupported(
                    "expression statement",
                    "statements with effects are outside the portable core in function bodies",
                    expr.span,
                ));
            }
            let ends_block_statement = is_if_or_match(&expr);
            let expr_span = expr.span;
            tail = Some(expr);
            if !self.cursor.is("}") {
                if ends_block_statement {
                    return Err(unsupported(
                        "expression statement",
                        "a block value must be its last expression",
                        expr_span,
                    ));
                }
                return Err(Self::fail("expected }", self.cursor.peek()));
            }
        }
        let close = self.cursor.expect("}", Some("block"))?;
        let Some(tail) = tail else {
            return Err(unsupported(
                "block without a value",
                "the block evaluates to (), which is not a portable result",
                span(&open, &close),
            ));
        };
        Ok(lets.into_iter().rev().fold(tail, |body, binding| {
            SExpr::new(
                SNode::Let {
                    name: binding.name,
                    ty: binding.ty,
                    value: Box::new(binding.value),
                    body: Box::new(body),
                },
                binding.span,
            )
        }))
    }

    fn let_statement(&mut self, path: &[String]) -> Result<Binding> {
        let start = self.cursor.advance();
        if self.cursor.is("mut") {
            return Err(unsupported(
                "mutable binding",
                "mutation is outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        if self.cursor.is("(") {
            return Err(unsupported(
                "destructuring let",
                "tuples are outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        let name = self.cursor.identifier(Some("let"))?.value;
        let ty = if self.cursor.eat(":").is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };
        if self.cursor.eat("=").is_none() {
            return Err(unsupported(
                "uninitialised binding",
                "every binding needs its value",
                span(&start, self.cursor.peek()),
            ));
        }
        let value = self.expr(path, Options::default())?;
        if self.cursor.is("else") {
            return Err(unsupported(
                "let-else",
                "refutable bindings are outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        self.cursor.expect(";", Some("let"))?;
        Ok(Binding {
            name,
            ty,
            value,
            span: span(&start, self.cursor.peek()),
        })
    }

    fn main_fn(&mut self) -> Result<SMain> {
        let start = self.cursor.advance();
        self.cursor.expect("main", Some("fn main"))?;
        self.cursor.expect("(", Some("fn main"))?;
        self.cursor.expect(")", Some("fn main"))?;
        if self.cursor.is("->") {
            return Err(unsupported(
                "main with a result",
                "main must return ()",
                span(&start, self.cursor.peek()),
            ));
        }
        self.module_path = Vec::new();
        self.cursor.expect("{", Some("fn main"))?;
        let mut effects = Vec::new();
        while !self.cursor.is("}") {
            let token = self.peek();
            if self.cursor.is("let") {
                let binding = self.let_statement(&[])?;
                effects.push(SEffect::Let {
                    name: binding.name,
                    ty: binding.ty,
                    value: binding.value,
                    span: binding.span,
                });
                continue;
            }
            if token.kind == TokenKind::Macro {
                effects.push(self.main_macro()?);
                self.cursor.expect(";", Some(&token.value))?;
                continue;
            }
            return Err(unsupported(
                "main statement",
                "main may only print, bind and assert",
                span(&token, &token),
            ));
        }
        self.cursor.expect("}", Some("fn main"))?;
        Ok(SMain {
            effects,
            span: span(&start, self.cursor.peek()),
        })
    }

    fn main_macro(&mut self) -> Result<SEffect> {
        let token = self.cursor.advance();
        let mut args = self.macro_arguments(&token, &[])?;
        match token.value.as_str() {
            "println" => {
                let expr = if args.is_empty() {
                    SExpr::new(
                        SNode::Str {
                            value: String::new(),
                        },
                        None,
                    )
                } else {
                    format(args, &token)?
                };
                Ok(SEffect::Print {
                    expr,
                    style: ShowStyle::Rust,
                    span: span(&token, self.cursor.peek()),
                })
            }
            "assert" => {
                if args.len() != 1 {
                    return Err(unsupported(
                        "assert! message",
                        "custom assertion messages are not kept",
                        span(&token, self.cursor.peek()),
                    ));
                }
                Ok(SEffect::Assert {
                    prop: prop_of(args.remove(0)),
                    span: span(&token, self.cursor.peek()),
                })
            }
            "assert_eq" | "assert_ne" => {
                if args.len() != 2 {
                    return Err(unsupported(
                        &format!("{}! message", token.value),
                        "custom assertion messages are not kept",
                        span(&token, self.cursor.peek()),
                    ));
                }
                let right = args.remove(1);
                let comparison = SComparison {
                    left: args.remove(0),
                    right,
                    reference: false,
                };
                let node = if token.value == "assert_eq" {
                    SPropNode::Eq(comparison)
                } else {
                    SPropNode::Ne(comparison)
                };
                Ok(SEffect::Assert {
                    prop: SProp { node, span: None },
                    span: span(&token, self.cursor.peek()),
                })
            }
            _ => Err(unsupported(
                &format!("{}!", token.value),
                "main may only use println!, assert!, assert_eq! and assert_ne!",
                span(&token, &token),
            )),
        }
    }

    /// Macro arguments, parsed as expressions; the first one of a format macro is its literal.
    fn macro_arguments(&mut self, token: &Token, path: &[String]) -> Result<Vec<SExpr>> {
        let close = match self.cursor.peek().value.as_str() {
            "(" => ")",
            "[" => "]",
            "{" => "}",
            _ => {
                return Err(Self::fail(
                    &format!("expected ( after {}!", token.value),
                    self.cursor.peek(),
                ))
            }
        };
        self.cursor.advance();
        let mut args = Vec::new();
        while !self.cursor.is(close) {
            args.push(self.expr(path, Options::default())?);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor
            .expect(close, Some(&format!("{}!", token.value)))?;
        Ok(args)
    }

    /// Expressions, by Rust precedence.
    fn expr(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        self.or(path, options)
    }

    fn or(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.and(path, options)?;
        while self.cursor.is("||") {
            let token = self.cursor.advance();
            let right = self.and(path, Options::default())?;
            left = binary(BinaryOp::Or, left, right, &token);
        }
        Ok(left)
    }

    fn and(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.comparison(path, options)?;
        while self.cursor.is("&&") {
            let token = self.cursor.advance();
            let right = self.comparison(path, Options::default())?;
            left = binary(BinaryOp::And, left, right, &token);
        }
        Ok(left)
    }

    fn comparison(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let left = self.additive(path, options)?;
        let token = self.peek();
        let c = &self.cursor;
        let shift = (token.value == "<" || token.value == ">")
            && c.is_at(&token.value, 1)
            && c.peek_at(1).start == token.end;
        let block_statement = options.statement && is_if_or_match(&left);
        if token.kind == TokenKind::Punct
            && (shift || ["|", "^", "&"].contains(&token.value.as_str()))
            && !block_statement
        {
            let operator = if shift {
                token.value.repeat(2)
            } else {
                token.value.clone()
            };
            return Err(unsupported(
                &format!("operator {operator}"),
                "bitwise operators are outside the portable core",
                span(&token, c.peek_at(usize::from(shift))),
            ));
        }
        if token.kind == TokenKind::Punct {
            if let Some(op) = comparison_op(&token.value) {
                self.cursor.advance();
                let right = self.additive(path, Options::default())?;
                let after = self.cursor.peek();
                if after.kind == TokenKind::Punct && comparison_op(&after.value).is_some() {
                    return Err(Self::fail("comparison operators cannot be chained", after));
                }
                return Ok(binary(op, left, right, &token));
            }
        }
        Ok(left)
    }

    fn additive(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.multiplicative(path, options)?;
        loop {
            let token = self.peek();
            let Some(op) = operator(&token, additive_op) else {
                return Ok(left);
            };
            if options.statement && is_if_or_match(&left) {
                return Ok(left);
            }
            self.cursor.advance();
            let right = self.multiplicative(path, Options::default())?;
            left = binary(op, left, right, &token);
        }
    }

    fn multiplicative(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.cast(path, options)?;
        loop {
            let token = self.peek();
            let Some(op) = operator(&token, multiplicative_op) else {
                return Ok(left);
            };
            if options.statement && is_if_or_match(&left) {
                return Ok(left);
            }
            self.cursor.advance();
            let right = self.cast(path, Options::default())?;
            left = binary(op, left, right, &token);
        }
    }

    /// `e as T` converts between integer types when every value fits.
    fn cast(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut value = self.unary(path, options)?;
        while self.cursor.is("as") {
            let token = self.cursor.advance();
            let to = self.parse_type()?;
            let range = joined(value.span, span(&token, self.cursor.peek()), &token);
            value = SExpr::new(
                SNode::Cast {
                    arg: Box::new(value),
                    to,
                    from: None,
                    flavor: Flavor::Exact,
                },
                range,
            );
        }
        Ok(value)
    }

    fn unary(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let token = self.peek();
        for (symbol, op) in [("-", UnaryOp::Neg), ("!", UnaryOp::Not)] {
            if self.cursor.eat(symbol).is_some() {
                let arg = self.unary(path, Options::default())?;
                return Ok(SExpr::new(
                    SNode::Unary {
                        op,
                        arg: Box::new(arg),
                    },
                    span(&token, self.cursor.peek()),
                ));
            }
        }
        // References and dereferences of immutable values denote the value.
        if self.cursor.is("&") || self.cursor.is("&&") {
            self.cursor.advance();
            if self.cursor.is("mut") {
                return Err(unsupported(
                    "mutable reference",
                    "mutation is outside the portable core",
                    span(&token, self.cursor.peek()),
                ));
            }
            return self.unary(path, Options::default());
        }
        if self.cursor.eat("*").is_some() {
            return self.unary(path, Options::default());
        }
        self.postfix(path, options)
    }

    fn postfix(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut value = self.primary(path, options)?;
        loop {
            let token = self.peek();
            if self.cursor.is(".") {
                self.cursor.advance();
                let method = self.cursor.identifier(Some("method"))?;
                if !self.cursor.is("(") {
                    return Err(unsupported(
                        "field access",
                        &format!("field {} is outside the portable core", method.value),
                        span(&token, &method),
                    ));
                }
                self.cursor.advance();
                let args = self.call_arguments(path)?;
                self.cursor.expect(")", Some(&method.value))?;
                value = method_call(value, &method, args, span(&token, self.cursor.peek()))?;
                continue;
            }
            if self.cursor.is("?") {
                return Err(unsupported(
                    "? operator",
                    "Result and Option are outside the portable core",
                    span(&token, &token),
                ));
            }
            if self.cursor.is("[") {
                return Err(unsupported(
                    "indexing",
                    "collections are outside the portable core",
                    span(&token, &token),
                ));
            }
            return Ok(value);
        }
    }

    /// Comma-separated expressions up to (not including) the closing `)`.
    fn call_arguments(&mut self, path: &[String]) -> Result<Vec<SExpr>> {
        let mut args = Vec::new();
        while !self.cursor.is(")") {
            args.push(self.expr(path, Options::default())?);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        Ok(args)
    }

    #[allow(clippy::too_many_lines)] // one branch per primary form, as in rust.js
    fn primary(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let token = self.peek();
        if token.kind == TokenKind::Number {
            self.cursor.advance();
            let ty = if token.suffix.is_empty() {
                None
            } else {
                let Some(fixed) = rust_fixed_type(&token.suffix) else {
                    return Err(unsupported(
                        &format!("literal suffix {}", token.suffix),
                        "only integer literals are portable",
                        span(&token, &token),
                    ));
                };
                Some(fixed)
            };
            return Ok(SExpr::new(
                SNode::Num {
                    value: token.value.clone(),
                    ty,
                    negative: false,
                },
                span(&token, &token),
            ));
        }
        if token.kind == TokenKind::String {
            self.cursor.advance();
            return Ok(SExpr::new(
                SNode::Str {
                    value: token.value.clone(),
                },
                span(&token, &token),
            ));
        }
        if token.kind == TokenKind::Macro {
            self.cursor.advance();
            return self.expression_macro(&token, path);
        }
        let c = &self.cursor;
        if c.is("'") {
            return Err(unsupported(
                "character literal",
                "characters are outside the portable core",
                span(&token, &token),
            ));
        }
        if c.is("(") {
            self.cursor.advance();
            if self.cursor.eat(")").is_some() {
                return Ok(SExpr::new(SNode::Unit, span(&token, &token)));
            }
            let inner = self.expr(path, Options::default())?;
            if self.cursor.is(",") {
                return Err(unsupported(
                    "tuple",
                    "tuples are outside the portable core",
                    span(&token, self.cursor.peek()),
                ));
            }
            self.cursor.expect(")", Some("parenthesised expression"))?;
            return Ok(inner);
        }
        if c.is("{") {
            let mut block = self.block(path)?;
            block.block = true;
            return Ok(block);
        }
        if c.is("if") {
            return self.if_expr(path);
        }
        if c.is("match") {
            return self.match_expr(path);
        }
        if c.is("|") || c.is("||") || c.is("move") {
            return Err(unsupported(
                "closure",
                "higher-order values are outside the portable core",
                span(&token, &token),
            ));
        }
        if c.is("true") || c.is("false") {
            self.cursor.advance();
            return Ok(SExpr::new(
                SNode::Bool {
                    value: token.value == "true",
                },
                span(&token, &token),
            ));
        }
        if ["unsafe", "loop", "while", "for", "return", "break"]
            .iter()
            .any(|keyword| c.is(keyword))
        {
            return Err(unsupported(
                &format!("{} expression", token.value),
                "outside the portable core",
                span(&token, &token),
            ));
        }
        if token.kind == TokenKind::Identifier {
            return self.path_expression(&token, path, options);
        }
        Err(Self::fail("expected an expression", self.cursor.peek()))
    }

    /// A name, a call of a function or constructor, or a conversion (`u64::from(x)`).
    fn path_expression(
        &mut self,
        token: &Token,
        path: &[String],
        options: Options,
    ) -> Result<SExpr> {
        let segments = self.path_segments("expression")?;
        let name = segments.join("::");
        let c = &self.cursor;
        if c.is("::") && c.is_at("<", 1) {
            return Err(unsupported(
                "turbofish",
                "generic functions are outside the portable core",
                span(token, c.peek()),
            ));
        }
        let last_is_upper = segments
            .last()
            .and_then(|segment| segment.chars().next())
            .is_some_and(|first| first.is_ascii_uppercase());
        if c.is("{") && last_is_upper && !options.no_struct {
            return Err(unsupported(
                "struct literal",
                "named fields are outside the Rust frontend",
                span(token, c.peek()),
            ));
        }
        if !c.is("(") {
            return Ok(SExpr::new(
                SNode::Name {
                    path: self.resolve(&segments, path),
                },
                span(token, token),
            ));
        }
        self.cursor.advance();
        let mut args = self.call_arguments(path)?;
        self.cursor.expect(")", Some("call"))?;
        let range = span(token, self.cursor.peek());
        if name == "Box::new" {
            if args.len() != 1 {
                return Err(type_error("Box::new expects 1 argument", range));
            }
            return Ok(args.remove(0));
        }
        if name == "String::from" {
            if args.len() != 1 {
                return Err(type_error("String::from expects 1 argument", range));
            }
            return Ok(SExpr::new(
                SNode::ToString {
                    arg: Box::new(args.remove(0)),
                },
                range,
            ));
        }
        let fixed = rust_fixed_type(&segments[0]);
        if segments.len() == 2 && segments[1] == "from" {
            if let Some(to) = fixed.clone() {
                if args.len() != 1 {
                    return Err(type_error(format!("{name} expects 1 argument"), range));
                }
                return Ok(SExpr::new(
                    SNode::Cast {
                        arg: Box::new(args.remove(0)),
                        to,
                        from: None,
                        flavor: Flavor::Exact,
                    },
                    range,
                ));
            }
        }
        if segments.len() == 2 && fixed.is_some() {
            return Err(unsupported(&name, "outside the portable core", range));
        }
        if ["std", "core", "alloc"].contains(&segments[0].as_str()) {
            return Err(unsupported(
                &name,
                "the standard library is outside the portable core",
                range,
            ));
        }
        let callee = self.resolve(&segments, path);
        if args.is_empty() {
            return Ok(SExpr::new(SNode::Name { path: callee }, range));
        }
        Ok(SExpr::new(
            SNode::App {
                func: Box::new(SExpr::new(SNode::Name { path: callee }, span(token, token))),
                args,
            },
            range,
        ))
    }

    fn expression_macro(&mut self, token: &Token, path: &[String]) -> Result<SExpr> {
        let args = self.macro_arguments(token, path)?;
        let range = span(token, self.cursor.peek());
        match token.value.as_str() {
            "format" => {
                let mut formatted = format(args, token)?;
                formatted.span = range;
                Ok(formatted)
            }
            "panic" | "unreachable" | "unimplemented" | "todo" => {
                if args.len() > 1 {
                    return Err(unsupported(
                        &format!("{}! arguments", token.value),
                        "only a literal message is portable",
                        range,
                    ));
                }
                let message = match args.into_iter().next() {
                    None => token.value.clone(),
                    Some(SExpr {
                        node: SNode::Str { value },
                        ..
                    }) => value,
                    Some(_) => {
                        return Err(unsupported(
                            &format!("{}! message", token.value),
                            "the message must be a literal",
                            range,
                        ))
                    }
                };
                Ok(SExpr::new(SNode::Abort { message }, range))
            }
            _ => Err(unsupported(
                &format!("{}!", token.value),
                "outside the portable core",
                range,
            )),
        }
    }

    fn if_expr(&mut self, path: &[String]) -> Result<SExpr> {
        let token = self.cursor.advance();
        if self.cursor.is("let") {
            return Err(unsupported(
                "if let",
                "write a match",
                span(&token, self.cursor.peek()),
            ));
        }
        let cond = self.expr(
            path,
            Options {
                statement: false,
                no_struct: true,
            },
        )?;
        let then = self.block(path)?;
        if self.cursor.eat("else").is_none() {
            return Err(unsupported(
                "if without else",
                "the value of an if must be defined on both branches",
                span(&token, self.cursor.peek()),
            ));
        }
        let otherwise = if self.cursor.is("if") {
            self.if_expr(path)?
        } else {
            self.block(path)?
        };
        Ok(SExpr::new(
            SNode::If {
                cond: Box::new(cond),
                then: Box::new(then),
                otherwise: Box::new(otherwise),
            },
            span(&token, self.cursor.peek()),
        ))
    }

    fn match_expr(&mut self, path: &[String]) -> Result<SExpr> {
        let token = self.cursor.advance();
        let scrutinee = self.expr(
            path,
            Options {
                statement: false,
                no_struct: true,
            },
        )?;
        self.cursor.expect("{", Some("match"))?;
        let mut rows = Vec::new();
        while !self.cursor.is("}") {
            let arm_start = self.peek();
            let mut aliases = Vec::new();
            self.cursor.eat("|");
            let mut alternatives = vec![self.pattern(path, &mut aliases)?];
            while self.cursor.eat("|").is_some() {
                alternatives.push(self.pattern(path, &mut aliases)?);
            }
            if self.cursor.is("if") {
                return Err(unsupported(
                    "match guard",
                    "guards are outside the portable pattern language",
                    span(&arm_start, self.cursor.peek()),
                ));
            }
            self.cursor.expect("=>", Some("match arm"))?;
            let mut body = self.expr(path, Options::default())?;
            if body.block || is_if_or_match(&body) {
                self.cursor.eat(",");
            } else if !self.cursor.is("}") {
                self.cursor.expect(",", Some("match arm"))?;
            }
            if alternatives.len() > 1 && !aliases.is_empty() {
                return Err(unsupported(
                    "binding in an or-pattern",
                    "bind the value in each alternative separately",
                    span(&arm_start, self.cursor.peek()),
                ));
            }
            for alias in aliases.into_iter().rev() {
                let body_span = body.span;
                body = SExpr::new(
                    SNode::Let {
                        name: alias.name,
                        ty: None,
                        value: Box::new(alias.value),
                        body: Box::new(body),
                    },
                    body_span,
                );
            }
            // `A | B => e` is one row per alternative with the same body.
            let row_span = span(&arm_start, self.cursor.peek());
            for pattern in alternatives {
                rows.push(SRow {
                    patterns: vec![pattern],
                    body: body.clone(),
                    span: row_span,
                });
            }
        }
        self.cursor.expect("}", Some("match"))?;
        if rows.is_empty() {
            return Err(unsupported(
                "empty match",
                "uninhabited types are outside the portable core",
                span(&token, self.cursor.peek()),
            ));
        }
        Ok(SExpr::new(
            SNode::Match {
                scrutinees: vec![scrutinee],
                rows,
            },
            span(&token, self.cursor.peek()),
        ))
    }

    /// Patterns: `_`, bindings, literals, `Enum::Variant(p, …)`, `&p` and `name @ p`.
    #[allow(clippy::too_many_lines)] // one branch per pattern form, as in rust.js
    fn pattern(&mut self, path: &[String], aliases: &mut Vec<Alias>) -> Result<SPattern> {
        let token = self.peek();
        if self.cursor.eat("&").is_some() {
            return self.pattern(path, aliases);
        }
        if self.cursor.eat("_").is_some() {
            return Ok(SPattern {
                node: SPatternNode::Wild,
                span: None,
            });
        }
        if self.cursor.is("ref") || self.cursor.is("mut") {
            return Err(unsupported(
                &format!("{} binding", token.value),
                "bindings are by value in the portable core",
                span(&token, &token),
            ));
        }
        if self.cursor.is("-") && self.cursor.is_kind_at(TokenKind::Number, 1) {
            self.cursor.advance();
            let number = self.cursor.advance();
            return Ok(SPattern {
                node: SPatternNode::NumLit {
                    value: number.value.clone(),
                    negative: true,
                },
                span: span(&token, &number),
            });
        }
        if token.kind == TokenKind::Number {
            self.cursor.advance();
            if self.cursor.is("..") || self.cursor.is("..=") {
                return Err(unsupported(
                    "range pattern",
                    "ranges are outside the portable pattern language",
                    span(&token, self.cursor.peek()),
                ));
            }
            return Ok(SPattern {
                node: SPatternNode::NumLit {
                    value: token.value.clone(),
                    negative: false,
                },
                span: span(&token, &token),
            });
        }
        if token.kind == TokenKind::String {
            return Err(unsupported(
                "string pattern",
                "match on &str is outside the Rust frontend",
                span(&token, &token),
            ));
        }
        if self.cursor.is("true") || self.cursor.is("false") {
            self.cursor.advance();
            return Ok(SPattern {
                node: SPatternNode::BoolLit {
                    value: token.value == "true",
                    negative: false,
                },
                span: span(&token, &token),
            });
        }
        if self.cursor.is("(") {
            return Err(unsupported(
                "tuple pattern",
                "tuples are outside the portable core",
                span(&token, &token),
            ));
        }
        if token.kind != TokenKind::Identifier {
            return Err(unsupported(
                "pattern",
                &format!(
                    "{} is outside the portable pattern language",
                    describe(&token)
                ),
                span(&token, &token),
            ));
        }
        let segments = self.path_segments("pattern")?;
        if self.cursor.eat("@").is_some() {
            if segments.len() != 1 {
                return Err(Self::fail("expected a binding before @", &token));
            }
            let inner = self.pattern(path, aliases)?;
            let value = pattern_value(&inner, span(&token, self.cursor.peek()))?;
            aliases.push(Alias {
                name: segments[0].clone(),
                value,
            });
            return Ok(inner);
        }
        if self.cursor.is("{") {
            return Err(unsupported(
                "struct pattern",
                "named fields are outside the Rust frontend",
                span(&token, self.cursor.peek()),
            ));
        }
        let resolved = self.resolve(&segments, path);
        if self.cursor.eat("(").is_some() {
            let mut args = Vec::new();
            while !self.cursor.is(")") {
                if self.cursor.is("..") {
                    let rest = self.cursor.peek();
                    return Err(unsupported(
                        "rest pattern",
                        "name every field",
                        span(rest, rest),
                    ));
                }
                args.push(self.pattern(path, aliases)?);
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect(")", Some("pattern"))?;
            return Ok(SPattern {
                node: SPatternNode::Ctor {
                    path: resolved,
                    args,
                },
                span: span(&token, self.cursor.peek()),
            });
        }
        if segments.len() == 1 && resolved.len() == 1 {
            return Ok(SPattern {
                node: SPatternNode::BindOrCtor {
                    name: segments[0].clone(),
                },
                span: span(&token, &token),
            });
        }
        Ok(SPattern {
            node: SPatternNode::Ctor {
                path: resolved,
                args: Vec::new(),
            },
            span: span(&token, &token),
        })
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
    match method.value.as_str() {
        "clone" | "to_owned" => {
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
            ))
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
