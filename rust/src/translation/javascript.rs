//! JavaScript frontend for the portable core.
//!
//! It reads the subset of modern JavaScript the core can represent
//! faithfully: `BigInt` arithmetic, strings and booleans; top-level functions
//! and object-literal namespaces of methods whose types come from `JSDoc`
//! (`@param`, `@returns`, and `@typedef` unions of `{ $: 'tag', … }` object
//! types for data types); bodies made of `const`, `if`, `return`, `throw` and
//! `switch` statements; and a top level of `console.log`, `const` and
//! `node:assert` statements, which are the program's effects. A function
//! whose leading statements throw on a negative argument takes a natural
//! number. Everything else is rejected with a precise obligation.
//!
//! Mirrors `js/src/translation/javascript.js`.

use std::collections::HashSet;

use super::decimal::Decimal;
use super::diagnostics::{type_error, unsupported, Result, TranslationError};
use super::lexer::{
    describe, is_js_space, tokenize, Comment, Source, Token, TokenCursor, TokenKind,
};
use super::surface::{
    BinaryOp, SComparison, SCtor, SData, SEffect, SExpr, SField, SFn, SItem, SMain, SModule, SNode,
    SParam, SPattern, SPatternNode, SProgram, SProp, SPropNode, SRow, STagTest, ShowStyle, UnaryOp,
};
use super::types::{Type, BOOL, INT, NAT, STRING};
use super::{Language, Span};

const ROOT: &str = "crate";
const ASSIGNMENTS: [&str; 16] = [
    "=", "+=", "-=", "*=", "/=", "%=", "**=", "<<=", ">>=", ">>>=", "&=", "|=", "^=", "&&=", "||=",
    "??=",
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
const NUMBER_REASON: &str =
    "numbers are IEEE-754 doubles, which are outside the portable core; use BigInt literals such as 5n";

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

/// Locals in scope, and names of the current block still in their temporal dead zone.
#[derive(Clone, Default)]
struct Scope {
    locals: HashSet<String>,
    tdz: HashSet<String>,
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
enum Stmt {
    Const {
        name: String,
        value: SExpr,
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
        span: Span,
    },
}

struct Switch {
    /// The switch subject: the discriminant, or the object of `x.$`.
    scrutinee: SExpr,
    /// The subject's name and data type in a tag switch.
    tag: Option<(String, SData)>,
    clauses: Vec<Clause>,
    span: Span,
}

struct Clause {
    tests: Vec<CaseTest>,
    body: Vec<Stmt>,
    span: Span,
}

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

    fn file(mut self) -> Result<SProgram> {
        let mut items = self.typedefs()?;
        let mut effects = Vec::new();
        if self.cursor.is_kind(TokenKind::String) && self.cursor.peek().value == "use strict" {
            self.cursor.advance();
            self.cursor.eat(";");
        }
        self.scope = Scope {
            locals: HashSet::new(),
            tdz: self.block_declarations(self.cursor.index, ScanEnd::File),
        };
        while !self.cursor.at_end() {
            let start = self.peek();
            if self.cursor.is("import") {
                self.import_declaration()?;
                continue;
            }
            if self.cursor.eat("export").is_some()
                && !self.cursor.is("function")
                && !self.cursor.is("const")
            {
                return Err(unsupported(
                    &format!("export {}", describe(self.cursor.peek())),
                    "only function and const declarations are exported",
                    Some(self.to_here(&start)),
                ));
            }
            if self.cursor.is("async") {
                return Err(unsupported(
                    "async function",
                    "asynchronous code is outside the portable core",
                    Some(self.to_here(&start)),
                ));
            }
            if self.cursor.is("function") {
                items.push(SItem::Fn(self.function_declaration(&start)?));
                continue;
            }
            if self.cursor.is("const")
                && self.cursor.is_kind_at(TokenKind::Identifier, 1)
                && self.cursor.is_at("=", 2)
                && self.cursor.is_at("{", 3)
                && !(self.cursor.is_at("$", 4) && self.cursor.is_at(":", 5))
            {
                if !effects.is_empty() {
                    let name = self.peek_at(1);
                    return Err(unsupported(
                        "namespace after a top-level statement",
                        &format!(
                            "the statements before const {} would run before it is initialised; declare every namespace first",
                            name.value
                        ),
                        Some(span(&start, &name)),
                    ));
                }
                items.push(SItem::Module(self.namespace(&start)?));
                continue;
            }
            effects.push(self.main_statement()?);
        }
        Ok(SProgram {
            language: Language::JavaScript,
            items,
            main: Some(SMain {
                effects,
                span: Some(Span::new(0, self.source.len())),
            }),
        })
    }

    /// `import assert from 'node:assert/strict'` is the only portable import.
    fn import_declaration(&mut self) -> Result<()> {
        let start = self.cursor.advance();
        let local;
        let mut strict = false;
        if self.cursor.eat("{").is_some() {
            let imported = self.cursor.identifier(Some("import"))?;
            if imported.value != "strict" || self.cursor.eat("as").is_none() {
                return Err(unsupported(
                    &format!("import {{ {} }}", imported.value),
                    "import the assertion module as a whole, e.g. import assert from 'node:assert/strict'",
                    Some(self.to_here(&start)),
                ));
            }
            local = self.cursor.identifier(Some("import"))?;
            self.cursor.expect("}", Some("import"))?;
            strict = true;
        } else if self.cursor.is("*") {
            return Err(unsupported(
                "namespace import",
                "only node:assert can be imported",
                Some(self.to_here(&start)),
            ));
        } else {
            local = self.cursor.identifier(Some("import"))?;
        }
        self.cursor.expect("from", Some("import"))?;
        let module = self.cursor.advance();
        if module.kind != TokenKind::String {
            return Err(Self::fail("expected a module name", &module));
        }
        self.cursor.eat(";");
        if module.value == "node:assert/strict" || module.value == "assert/strict" {
            strict = true;
        } else if module.value != "node:assert" && module.value != "assert" {
            return Err(unsupported(
                &format!("import from '{}'", module.value),
                "modules other than node:assert are outside the portable core",
                Some(span(&start, &module)),
            ));
        }
        if self.assertion.is_some() {
            return Err(unsupported(
                "second assertion import",
                "import node:assert once",
                Some(span(&start, &module)),
            ));
        }
        self.scope.tdz.remove(&local.value);
        self.assertion = Some(Assertion {
            name: local.value,
            strict,
        });
        Ok(())
    }

    /// `@typedef {{ $: 'leaf' } | { $: 'node', left: Tree, value: bigint }} Tree`
    /// declares a data type: each alternative is a constructor named by its
    /// `$` tag, and its other properties are the constructor's fields.
    fn typedefs(&mut self) -> Result<Vec<SItem>> {
        let mut items = Vec::new();
        for comment in self.docs.clone() {
            for tag in jsdoc_tags(&comment)? {
                if tag.tag != "typedef" {
                    continue;
                }
                let range = Span::new(comment.start, comment.end);
                let (Some(text), Some(name)) = (non_empty(tag.ty), tag.name) else {
                    return Err(TranslationError::syntax(
                        "@typedef needs a type and a name",
                        Some(range),
                    ));
                };
                let ctors = self.typedef_ctors(&text, &name, range)?;
                if self.data_types.iter().any(|data| data.name == name) {
                    return Err(type_error(
                        format!("duplicate @typedef {name}"),
                        Some(range),
                    ));
                }
                self.data_types.push(SData {
                    name: name.clone(),
                    ctors: ctors.clone(),
                    span: None,
                });
                items.push(SItem::Data(SData {
                    name,
                    ctors,
                    span: Some(range),
                }));
            }
        }
        Ok(items)
    }

    /// The alternatives of a `@typedef` union type.
    fn typedef_ctors(&self, text: &str, name: &str, range: Span) -> Result<Vec<SCtor>> {
        let mut c = TokenCursor::new(
            tokenize(text, Language::JavaScript)?.tokens,
            Language::JavaScript,
        );
        let mut ctors: Vec<SCtor> = Vec::new();
        loop {
            if !c.is("{") {
                return Err(unsupported(
                    &format!("@typedef {name}"),
                    "a portable data type is a union of { $: 'tag', … } object types",
                    Some(range),
                ));
            }
            c.advance();
            let mut tag = None;
            let mut fields = Vec::new();
            while !c.is("}") {
                let key = c.identifier(Some("object type"))?;
                c.expect(":", Some("object type"))?;
                if key.value == "$" {
                    let value = c.advance();
                    if value.kind != TokenKind::String {
                        return Err(unsupported(
                            &format!("@typedef {name}"),
                            "the $ tag must be a string literal type",
                            Some(range),
                        ));
                    }
                    tag = Some(value.value);
                } else {
                    let ty = c.identifier(Some("field type"))?;
                    if !c.is(",") && !c.is(";") && !c.is("}") {
                        return Err(unsupported(
                            &format!("field type of {}", key.value),
                            "field types are bigint, boolean, string or a @typedef name",
                            Some(range),
                        ));
                    }
                    fields.push(SField {
                        name: Some(key.value),
                        ty: self.ty(&ty.value, range)?,
                    });
                }
                if c.eat(",").is_none() && c.eat(";").is_none() {
                    break;
                }
            }
            c.expect("}", Some("object type"))?;
            let Some(tag) = tag else {
                return Err(unsupported(
                    &format!("@typedef {name}"),
                    "every alternative needs a $ tag",
                    Some(range),
                ));
            };
            if ctors.iter().any(|ctor| ctor.name == tag) {
                return Err(type_error(
                    format!("duplicate tag {tag} in {name}"),
                    Some(range),
                ));
            }
            ctors.push(SCtor { name: tag, fields });
            if c.eat("|").is_none() {
                break;
            }
        }
        if !c.at_end() {
            return Err(unsupported(
                &format!("@typedef {name}"),
                "a portable data type is a union of object types",
                Some(range),
            ));
        }
        Ok(ctors)
    }

    #[allow(clippy::unused_self)] // a method, as in the JavaScript frontend
    fn ty(&self, text: &str, range: Span) -> Result<Type> {
        let name = js_trim(text);
        match name {
            "bigint" => return Ok(INT),
            "boolean" => return Ok(BOOL),
            "string" => return Ok(STRING),
            "number" => {
                return Err(unsupported(
                    "JavaScript number",
                    "numbers are IEEE-754 doubles, which are outside the portable core; use bigint",
                    Some(range),
                ))
            }
            _ => {}
        }
        let reserved = [
            "object",
            "any",
            "unknown",
            "void",
            "undefined",
            "null",
            "Object",
            "Function",
            "Array",
            "symbol",
        ];
        if !is_identifier_name(name) || reserved.contains(&name) {
            return Err(unsupported(
                &format!("JSDoc type {{{name}}}"),
                "portable types are bigint, boolean, string and @typedef data types",
                Some(range),
            ));
        }
        Ok(Type::Named {
            path: vec![ROOT.to_owned(), name.to_owned()],
            span: Some(range),
        })
    }

    /// The `JSDoc` block directly before a declaration.
    fn jsdoc_for(&self, token: &Token) -> Result<Option<JsDoc>> {
        let doc = self.docs.iter().rev().find(|comment| {
            comment.end <= token.start
                && self.source.units()[comment.end..token.start]
                    .iter()
                    .all(|&unit| is_js_space(unit))
        });
        let Some(doc) = doc else {
            return Ok(None);
        };
        let range = Span::new(doc.start, doc.end);
        let mut params: Vec<(String, String)> = Vec::new();
        let mut returns = None;
        for tag in jsdoc_tags(doc)? {
            if tag.tag == "param" {
                let (Some(ty), Some(name)) = (non_empty(tag.ty), tag.name) else {
                    return Err(TranslationError::syntax(
                        "@param needs a type and a name",
                        Some(range),
                    ));
                };
                if params.iter().any(|(param, _)| *param == name) {
                    return Err(type_error(format!("duplicate @param {name}"), Some(range)));
                }
                params.push((name, ty));
            } else if tag.tag == "returns" || tag.tag == "return" {
                returns = tag.ty;
            }
        }
        Ok(Some(JsDoc {
            params,
            returns,
            range,
        }))
    }

    fn function_declaration(&mut self, start: &Token) -> Result<SFn> {
        self.cursor
            .expect("function", Some("function declaration"))?;
        if self.cursor.is("*") {
            return Err(unsupported(
                "generator function",
                "generators are outside the portable core",
                Some(self.to_here(start)),
            ));
        }
        let name = self.cursor.identifier(Some("function"))?;
        self.function_rest(start, &name)
    }

    /// Parameters and body of a function or method; `doc_token` is where its `JSDoc` ends.
    fn function_rest(&mut self, doc_token: &Token, name_token: &Token) -> Result<SFn> {
        let doc = self.jsdoc_for(doc_token)?;
        let name = name_token.value.clone();
        self.cursor.expect("(", Some(&name))?;
        let mut tokens = Vec::new();
        while !self.cursor.is(")") {
            let token = self.peek();
            if self.cursor.is("...") {
                return Err(unsupported(
                    "rest parameter",
                    "functions take a fixed number of arguments",
                    Some(span(&token, &token)),
                ));
            }
            if self.cursor.is("{") || self.cursor.is("[") {
                return Err(unsupported(
                    "destructured parameter",
                    "name each parameter",
                    Some(span(&token, &token)),
                ));
            }
            let param = self.cursor.identifier(Some("parameter"))?;
            if self.cursor.is("=") {
                return Err(unsupported(
                    "default parameter",
                    "every argument must be passed",
                    Some(self.to_here(&param)),
                ));
            }
            tokens.push(param);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect(")", Some(&name))?;
        let place = self.to_here(doc_token);
        let Some(doc) = doc else {
            return Err(unsupported(
                &format!("untyped function {name}"),
                "declare its types with JSDoc @param and @returns tags",
                Some(place),
            ));
        };
        let mut params = Vec::new();
        for token in &tokens {
            let Some((_, text)) = doc.params.iter().find(|(param, _)| *param == token.value) else {
                return Err(unsupported(
                    &format!("untyped parameter {}", token.value),
                    "give every parameter a JSDoc @param type",
                    Some(span(token, token)),
                ));
            };
            params.push(SParam {
                name: token.value.clone(),
                ty: Some(self.ty(text, doc.range)?),
                span: Some(span(token, token)),
                guard: None,
                rocq_type: None,
            });
        }
        for (documented, _) in &doc.params {
            if !params.iter().any(|param| param.name == *documented) {
                return Err(type_error(
                    format!("@param {documented} is not a parameter of {name}"),
                    Some(doc.range),
                ));
            }
        }
        let Some(returns) = non_empty(doc.returns) else {
            return Err(unsupported(
                &format!("function {name} without @returns"),
                "declare the result type with a JSDoc @returns tag",
                Some(place),
            ));
        };
        let ret = self.ty(&returns, doc.range)?;
        let outer = std::mem::replace(
            &mut self.scope,
            Scope {
                locals: params.iter().map(|param| param.name.clone()).collect(),
                tdz: HashSet::new(),
            },
        );
        let statements = self.block_statements();
        self.scope = outer;
        let statements = statements?;
        // `if (n < 0n) throw …` as a leading statement makes `n` a natural number.
        let mut index = 0;
        while index < statements.len() {
            let Some(param) = guarded_parameter(&statements[index], &params) else {
                break;
            };
            params[param].ty = Some(NAT);
            params[param].guard = Some(true);
            index += 1;
        }
        let body = lower(&statements[index..], self.to_here(name_token))?;
        Ok(SFn {
            name,
            params,
            ret: Some(ret),
            body,
            span: Some(self.to_here(doc_token)),
        })
    }

    /// `const Name = { method(…) { … }, Nested: { … } };` is a module.
    fn namespace(&mut self, start: &Token) -> Result<SModule> {
        self.cursor.expect("const", Some("namespace"))?;
        let name = self.cursor.identifier(Some("namespace"))?;
        self.cursor.expect("=", Some("namespace"))?;
        self.scope.tdz.remove(&name.value);
        let module = self.namespace_body(name.value, start)?;
        self.cursor.eat(";");
        Ok(module)
    }

    fn namespace_body(&mut self, name: String, start: &Token) -> Result<SModule> {
        self.cursor.expect("{", Some("namespace"))?;
        let mut items = Vec::new();
        while !self.cursor.is("}") {
            let member = self.peek();
            if member.kind != TokenKind::Identifier {
                return Err(unsupported(
                    &format!("namespace member {}", describe(&member)),
                    "members are methods or nested namespaces with identifier names",
                    Some(span(&member, &member)),
                ));
            }
            if ["get", "set", "async", "static"].contains(&member.value.as_str())
                && self.cursor.is_kind_at(TokenKind::Identifier, 1)
            {
                return Err(unsupported(
                    &format!("{} member", member.value),
                    "accessors and asynchronous methods are outside the portable core",
                    Some(span(&member, self.cursor.peek_at(1))),
                ));
            }
            self.cursor.advance();
            if self.cursor.is("(") {
                items.push(SItem::Fn(self.function_rest(&member, &member)?));
            } else if self.cursor.eat(":").is_some() {
                if self.cursor.is("{") {
                    items.push(SItem::Module(
                        self.namespace_body(member.value.clone(), &member)?,
                    ));
                } else {
                    return Err(unsupported(
                        &format!("namespace property {}", member.value),
                        "namespaces hold methods, name(…) { … }, and nested namespaces only",
                        Some(self.to_here(&member)),
                    ));
                }
            } else {
                return Err(self.fail_here(&format!("expected ( or : after {}", member.value)));
            }
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("namespace"))?;
        Ok(SModule {
            name,
            items,
            span: Some(self.to_here(start)),
        })
    }

    /// Names a block declares with `const`, which are in their temporal dead zone until declared.
    fn block_declarations(&mut self, from: usize, end: ScanEnd) -> HashSet<String> {
        let mut names = HashSet::new();
        let saved = self.cursor.index;
        self.cursor.index = from;
        let mut depth = 0_i64;
        loop {
            let done = match end {
                ScanEnd::File => self.cursor.at_end(),
                ScanEnd::Brace => self.cursor.is("}"),
            };
            if self.cursor.at_end() || (depth == 0 && done) {
                break;
            }
            let token = self.cursor.advance();
            let punct = token.kind == TokenKind::Punct;
            if punct && ["{", "(", "["].contains(&token.value.as_str()) {
                depth += 1;
            } else if punct && ["}", ")", "]"].contains(&token.value.as_str()) {
                depth -= 1;
            } else if depth == 0 && token.kind == TokenKind::Identifier && token.value == "const" {
                let c = &self.cursor;
                if c.peek().kind == TokenKind::Identifier {
                    names.insert(c.peek().value.clone());
                } else if c.is("{") {
                    let mut offset = 1;
                    while !c.is_at("}", offset) && !c.is_kind_at(TokenKind::Eof, offset) {
                        if c.peek_at(offset).kind == TokenKind::Identifier
                            && (c.is_at(",", offset + 1) || c.is_at("}", offset + 1))
                        {
                            names.insert(c.peek_at(offset).value.clone());
                        }
                        offset += 1;
                    }
                }
            }
        }
        self.cursor.index = saved;
        names
    }

    fn declare_local(&mut self, name: &str) {
        self.scope.locals.insert(name.to_owned());
        self.scope.tdz.remove(name);
    }

    fn block_statements(&mut self) -> Result<Vec<Stmt>> {
        self.cursor.expect("{", Some("block"))?;
        let mut tdz = self.scope.tdz.clone();
        tdz.extend(self.block_declarations(self.cursor.index, ScanEnd::Brace));
        let inner = Scope {
            locals: self.scope.locals.clone(),
            tdz,
        };
        let outer = std::mem::replace(&mut self.scope, inner);
        let statements = self.block_body();
        self.scope = outer;
        statements
    }

    fn block_body(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.cursor.is("}") {
            if self.cursor.at_end() {
                return Err(self.fail_here("expected }"));
            }
            statements.push(self.statement()?);
        }
        self.cursor.expect("}", Some("block"))?;
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt> {
        let token = self.peek();
        if self.cursor.is("const") {
            return self.const_statement();
        }
        if self.cursor.is("let") || self.cursor.is("var") {
            return Err(unsupported(
                &format!("{} declaration", token.value),
                "mutable bindings are outside the portable core; use const",
                Some(span(&token, &token)),
            ));
        }
        if self.cursor.is("return") {
            self.cursor.advance();
            let next = self.cursor.peek().start;
            let line_break = token.end < next
                && self.source.units()[token.end..next].contains(&u16::from(b'\n'));
            if self.cursor.is(";") || self.cursor.is("}") || line_break {
                return Err(unsupported(
                    "return without a value",
                    "the function would return undefined, which is not a portable value",
                    Some(self.to_here(&token)),
                ));
            }
            let expr = self.expr()?;
            self.cursor.eat(";");
            return Ok(Stmt::Return {
                expr,
                span: self.to_here(&token),
            });
        }
        if self.cursor.is("throw") {
            return self.throw_statement();
        }
        if self.cursor.is("if") {
            return self.if_statement();
        }
        if self.cursor.is("switch") {
            return self.switch_statement().map(Stmt::Switch);
        }
        if self.cursor.is("{") {
            let body = self.block_statements()?;
            return Ok(Stmt::Block {
                body,
                inline: false,
                span: self.to_here(&token),
            });
        }
        if self.cursor.eat(";").is_some() {
            return Ok(Stmt::Empty);
        }
        if token.kind == TokenKind::Identifier {
            if ["for", "while", "do"].contains(&token.value.as_str()) {
                return Err(unsupported(
                    &format!("{} loop", token.value),
                    "loops over mutable state are outside the portable core; use recursion",
                    Some(span(&token, &token)),
                ));
            }
            let statements = [
                "break", "continue", "try", "function", "class", "label", "with", "debugger",
            ];
            if statements.contains(&token.value.as_str()) {
                return Err(unsupported(
                    &format!("{} statement", token.value),
                    "outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
        }
        self.expr()?;
        self.cursor.eat(";");
        Ok(Stmt::Expr {
            span: self.to_here(&token),
        })
    }

    fn const_statement(&mut self) -> Result<Stmt> {
        let start = self.cursor.advance();
        if self.cursor.is("{") {
            return self.destructuring(&start);
        }
        let (name, value, span) = self.const_binding(&start)?;
        Ok(Stmt::Const { name, value, span })
    }

    /// `const name = value;` after the `const`.
    fn const_binding(&mut self, start: &Token) -> Result<(String, SExpr, Span)> {
        if self.cursor.is("[") {
            return Err(unsupported(
                "array destructuring",
                "arrays are outside the portable core",
                Some(self.to_here(start)),
            ));
        }
        let name = self.cursor.identifier(Some("const"))?;
        if self.cursor.eat("=").is_none() {
            return Err(self.fail_here("expected = in const"));
        }
        let value = self.expr()?;
        if self.cursor.is(",") {
            return Err(unsupported(
                "several declarators",
                "declare one binding per const",
                Some(self.to_here(start)),
            ));
        }
        self.cursor.eat(";");
        self.declare_local(&name.value);
        Ok((name.value, value, self.to_here(start)))
    }

    /// `const { left, value: v } = t;` binds fields of a switch subject.
    fn destructuring(&mut self, start: &Token) -> Result<Stmt> {
        self.cursor.expect("{", Some("destructuring"))?;
        let mut pairs = Vec::new();
        while !self.cursor.is("}") {
            if self.cursor.is("...") {
                let token = self.peek();
                return Err(unsupported(
                    "rest property",
                    "name every field",
                    Some(span(&token, &token)),
                ));
            }
            let key = self.cursor.identifier(Some("destructuring"))?;
            let local = if self.cursor.eat(":").is_some() {
                self.cursor.identifier(Some("destructuring"))?
            } else {
                key.clone()
            };
            if self.cursor.is("=") {
                return Err(unsupported(
                    "default value",
                    "every field has a value",
                    Some(self.to_here(&key)),
                ));
            }
            pairs.push((key.value.clone(), local.value.clone(), span(&key, &local)));
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("destructuring"))?;
        self.cursor.expect("=", Some("destructuring"))?;
        let object = self.expr()?;
        self.cursor.eat(";");
        let whole = self.to_here(start);
        let statements = pairs
            .iter()
            .map(|(field, name, place)| Stmt::Const {
                name: name.clone(),
                value: node(
                    SNode::Field {
                        object: Box::new(object.clone()),
                        field: field.clone(),
                    },
                    *place,
                ),
                span: whole,
            })
            .collect();
        for (_, name, _) in &pairs {
            self.declare_local(name);
        }
        Ok(Stmt::Block {
            body: statements,
            inline: true,
            span: whole,
        })
    }

    fn throw_statement(&mut self) -> Result<Stmt> {
        let start = self.cursor.advance();
        if self.cursor.eat("new").is_none() {
            return Err(unsupported(
                "throw of a non-error value",
                "throw new Error('…'), which aborts the program",
                Some(self.to_here(&start)),
            ));
        }
        let kind = self.cursor.identifier(Some("throw"))?;
        if !ERRORS.contains(&kind.value.as_str()) {
            return Err(unsupported(
                &format!("throw new {}", kind.value),
                "throw an Error, RangeError or TypeError",
                Some(span(&kind, &kind)),
            ));
        }
        self.cursor.expect("(", Some("throw"))?;
        let message = if self.cursor.is(")") {
            String::new()
        } else {
            let token = self.cursor.advance();
            if token.kind != TokenKind::String {
                return Err(unsupported(
                    "computed error message",
                    "the message of an abort is a string literal",
                    Some(span(&token, &token)),
                ));
            }
            token.value
        };
        self.cursor.expect(")", Some("throw"))?;
        self.cursor.eat(";");
        Ok(Stmt::Throw {
            message,
            span: self.to_here(&start),
        })
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        let start = self.cursor.advance();
        self.cursor.expect("(", Some("if"))?;
        let cond = self.expr()?;
        self.cursor.expect(")", Some("if"))?;
        let then = self.branch()?;
        let otherwise = if self.cursor.eat("else").is_some() {
            Some(self.branch()?)
        } else {
            None
        };
        Ok(Stmt::If {
            cond,
            then,
            otherwise,
            span: self.to_here(&start),
        })
    }

    fn branch(&mut self) -> Result<Vec<Stmt>> {
        if self.cursor.is("{") {
            return self.block_statements();
        }
        if self.cursor.is("const") || self.cursor.is("let") || self.cursor.is("function") {
            return Err(self.fail_here("expected a statement"));
        }
        Ok(vec![self.statement()?])
    }

    /// `switch (x.$) { case 'tag': … }` matches the constructors of a data
    /// type; `switch (n) { case 0n: … default: … }` matches literals.
    fn switch_statement(&mut self) -> Result<Switch> {
        let start = self.cursor.advance();
        self.cursor.expect("(", Some("switch"))?;
        let discriminant = self.expr()?;
        self.cursor.expect(")", Some("switch"))?;
        self.cursor.expect("{", Some("switch"))?;
        let tag_object = match discriminant.node {
            SNode::Field {
                ref object,
                ref field,
            } if field == "$" => Some(object.as_ref().clone()),
            _ => None,
        };
        let tagged = tag_object.is_some();
        if let Some(object) = &tag_object {
            if object.simple_name().is_none() {
                return Err(unsupported(
                    "switch on the tag of an expression",
                    "bind the value with const and switch on its tag",
                    discriminant.span,
                ));
            }
        }
        let mut clauses = Vec::new();
        let mut tests = Vec::new();
        while !self.cursor.is("}") {
            let clause_start = self.peek();
            if self.cursor.eat("default").is_some() {
                tests.push(CaseTest::Default {
                    span: span(&clause_start, &clause_start),
                });
            } else {
                self.cursor.expect("case", Some("switch"))?;
                tests.push(self.case_test(tagged)?);
            }
            self.cursor.expect(":", Some("case"))?;
            if self.cursor.is("case") || self.cursor.is("default") {
                continue;
            }
            let mut body = Vec::new();
            while !self.cursor.is("case") && !self.cursor.is("default") && !self.cursor.is("}") {
                if self.cursor.is("const") || self.cursor.is("let") {
                    let token = self.peek();
                    return Err(unsupported(
                        "declaration in a case clause",
                        "case clauses share one scope; wrap the case body in braces",
                        Some(span(&token, &token)),
                    ));
                }
                body.push(self.statement()?);
            }
            clauses.push(Clause {
                tests: std::mem::take(&mut tests),
                body,
                span: self.to_here(&clause_start),
            });
        }
        if !tests.is_empty() {
            clauses.push(Clause {
                tests,
                body: Vec::new(),
                span: self.to_here(&start),
            });
        }
        self.cursor.expect("}", Some("switch"))?;
        let whole = self.to_here(&start);
        let (scrutinee, tag) = match tag_object {
            Some(object) => {
                let subject = object.simple_name().unwrap_or_default().to_owned();
                let data = self.switch_data(&clauses, whole)?;
                (object, Some((subject, data)))
            }
            None => (discriminant, None),
        };
        Ok(Switch {
            scrutinee,
            tag,
            clauses,
            span: whole,
        })
    }

    fn case_test(&mut self, tagged: bool) -> Result<CaseTest> {
        let token = self.peek();
        if tagged {
            if !self.cursor.is_kind(TokenKind::String) {
                return Err(unsupported(
                    "case test",
                    "the cases of a tag switch are string literal tags",
                    Some(span(&token, &token)),
                ));
            }
            self.cursor.advance();
            return Ok(CaseTest::Tag {
                tag: token.value.clone(),
                span: span(&token, &token),
            });
        }
        let negative = self.cursor.eat("-").is_some();
        let value = self.peek();
        if value.kind == TokenKind::Number {
            self.cursor.advance();
            return Ok(CaseTest::NumLit {
                value: bigint(&value)?,
                negative,
                span: span(&token, &value),
            });
        }
        if !negative && (self.cursor.is("true") || self.cursor.is("false")) {
            self.cursor.advance();
            return Ok(CaseTest::BoolLit {
                value: token.value == "true",
                span: span(&token, &token),
            });
        }
        Err(unsupported(
            "case test",
            "the cases of a value switch are BigInt or boolean literals",
            Some(span(&token, &value)),
        ))
    }

    /// The @typedef whose tags a tag switch names.
    fn switch_data(&self, clauses: &[Clause], place: Span) -> Result<SData> {
        let tags: Vec<&str> = clauses
            .iter()
            .flat_map(|clause| &clause.tests)
            .filter_map(|test| match test {
                CaseTest::Tag { tag, .. } => Some(tag.as_str()),
                _ => None,
            })
            .collect();
        let candidates: Vec<&SData> = self
            .data_types
            .iter()
            .filter(|data| tags.iter().all(|tag| has_ctor(data, tag)))
            .collect();
        match candidates.as_slice() {
            [data] => Ok((*data).clone()),
            [] => Err(type_error(
                format!("no @typedef has tags {}", tags.join(", ")),
                Some(place),
            )),
            _ => Err(type_error(
                format!("tags {} are ambiguous between data types", tags.join(", ")),
                Some(place),
            )),
        }
    }

    fn main_statement(&mut self) -> Result<SEffect> {
        let token = self.peek();
        if self.cursor.is("const") {
            if self.cursor.is_at("{", 1) {
                return Err(unsupported(
                    "top-level destructuring",
                    "bind each value with const",
                    Some(span(&token, self.cursor.peek_at(1))),
                ));
            }
            let start = self.cursor.advance();
            let (name, value, span) = self.const_binding(&start)?;
            return Ok(SEffect::Let {
                name,
                ty: None,
                value,
                span: Some(span),
            });
        }
        if self.cursor.is("console") && self.cursor.is_at(".", 1) {
            return self.console_statement();
        }
        if let Some(assertion) = &self.assertion {
            if self.cursor.is(&assertion.name) {
                return self.assert_statement();
            }
        }
        let statements = [
            "let", "var", "if", "for", "while", "do", "switch", "try", "throw", "class", "return",
        ];
        if token.kind == TokenKind::Identifier && statements.contains(&token.value.as_str()) {
            return Err(unsupported(
                &format!("top-level {} statement", token.value),
                "the top level may only print with console.log, bind with const and assert",
                Some(span(&token, &token)),
            ));
        }
        Err(unsupported(
            "top-level expression statement",
            "a statement that discards its value has no portable effect",
            Some(span(&token, &token)),
        ))
    }

    fn console_statement(&mut self) -> Result<SEffect> {
        let start = self.cursor.advance();
        self.cursor.expect(".", Some("console"))?;
        let method = self.cursor.identifier(Some("console"))?;
        if method.value != "log" {
            return Err(unsupported(
                &format!("console.{}", method.value),
                "console.log, which writes a line to standard output, is the portable output",
                Some(span(&start, &method)),
            ));
        }
        let mut args = self.arguments("console.log")?;
        self.cursor.eat(";");
        let place = self.to_here(&start);
        if args.len() > 1 {
            return Err(unsupported(
                "console.log with several arguments",
                "several arguments are formatted by util.format; pass one string",
                Some(place),
            ));
        }
        let expr = args.pop().unwrap_or_else(|| {
            node(
                SNode::Str {
                    value: String::new(),
                },
                place,
            )
        });
        Ok(SEffect::Print {
            expr,
            style: ShowStyle::JsConsole,
            span: Some(place),
        })
    }

    fn assert_statement(&mut self) -> Result<SEffect> {
        let start = self.cursor.advance();
        let method = if self.cursor.eat(".").is_some() {
            Some(self.cursor.identifier(Some("assertion"))?.value)
        } else {
            None
        };
        let mut args = self.arguments("assertion")?;
        self.cursor.eat(";");
        let place = self.to_here(&start);
        let (local, strict) = self
            .assertion
            .as_ref()
            .map_or((String::new(), false), |assertion| {
                (assertion.name.clone(), assertion.strict)
            });
        let name = method
            .as_ref()
            .map_or_else(|| local.clone(), |method| format!("{local}.{method}"));
        if method.is_none() || method.as_deref() == Some("ok") {
            if args.len() != 1 {
                return Err(unsupported(
                    &format!("{name} message"),
                    "custom assertion messages are not kept",
                    Some(place),
                ));
            }
            let mut prop = prop_of(args.remove(0));
            prop.span = Some(place);
            return Ok(SEffect::Assert {
                prop,
                span: Some(place),
            });
        }
        let Some(kind) = assertion_kind(method.as_deref().unwrap_or_default(), strict) else {
            return Err(unsupported(
                &name,
                "the portable assertions are assert, ok, strictEqual, notStrictEqual and deepStrictEqual",
                Some(place),
            ));
        };
        let Ok([left, right]) = <[SExpr; 2]>::try_from(args) else {
            return Err(unsupported(
                &format!("{name} message"),
                "custom assertion messages are not kept",
                Some(place),
            ));
        };
        let (reference, equal) = match kind {
            AssertionKind::Eq => (true, true),
            AssertionKind::Ne => (true, false),
            AssertionKind::Deep => (false, true),
            AssertionKind::NotDeep => (false, false),
        };
        let comparison = SComparison {
            left,
            right,
            reference,
        };
        let prop = SProp {
            node: if equal {
                SPropNode::Eq(comparison)
            } else {
                SPropNode::Ne(comparison)
            },
            span: Some(place),
        };
        Ok(SEffect::Assert {
            prop,
            span: Some(place),
        })
    }

    fn arguments(&mut self, context: &str) -> Result<Vec<SExpr>> {
        self.cursor.expect("(", Some(context))?;
        let mut args = Vec::new();
        while !self.cursor.is(")") {
            if self.cursor.is("...") {
                let token = self.peek();
                return Err(unsupported(
                    "spread argument",
                    "pass each argument",
                    Some(span(&token, &token)),
                ));
            }
            args.push(self.expr()?);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect(")", Some(context))?;
        Ok(args)
    }

    /// Expressions, by JavaScript precedence.
    fn expr(&mut self) -> Result<SExpr> {
        let start = self.peek();
        if start.kind == TokenKind::Identifier && self.cursor.is_at("=>", 1) {
            return Err(unsupported(
                "arrow function",
                "functions as values are outside the portable core",
                Some(span(&start, self.cursor.peek_at(1))),
            ));
        }
        let cond = self.or()?;
        let next = self.cursor.peek();
        if next.kind == TokenKind::Punct && ASSIGNMENTS.contains(&next.value.as_str()) {
            return Err(unsupported(
                "assignment",
                "mutation is outside the portable core",
                Some(self.to_here(&start)),
            ));
        }
        if self.cursor.eat("?").is_none() {
            return Ok(cond);
        }
        let then = self.expr()?;
        self.cursor.expect(":", Some("conditional expression"))?;
        let otherwise = self.expr()?;
        let place = joined(&cond, &otherwise, &start);
        Ok(node(
            SNode::If {
                cond: Box::new(cond),
                then: Box::new(then),
                otherwise: Box::new(otherwise),
            },
            place,
        ))
    }

    fn or(&mut self) -> Result<SExpr> {
        let mut left = self.and()?;
        loop {
            let token = self.peek();
            if self.cursor.is("??") {
                return Err(unsupported(
                    "nullish coalescing",
                    "null and undefined are outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            if self.cursor.eat("||").is_none() {
                return Ok(left);
            }
            let right = self.and()?;
            left = binary(BinaryOp::Or, left, right, &token);
        }
    }

    fn and(&mut self) -> Result<SExpr> {
        let mut left = self.bitwise()?;
        while self.cursor.is("&&") {
            let token = self.cursor.advance();
            let right = self.bitwise()?;
            left = binary(BinaryOp::And, left, right, &token);
        }
        Ok(left)
    }

    fn bitwise(&mut self) -> Result<SExpr> {
        let left = self.equality()?;
        let token = self.peek();
        if token.kind == TokenKind::Punct && ["|", "^", "&"].contains(&token.value.as_str()) {
            return Err(unsupported(
                &format!("bitwise {}", token.value),
                "bitwise operators are outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        Ok(left)
    }

    fn equality(&mut self) -> Result<SExpr> {
        let mut left = self.relational()?;
        loop {
            let token = self.peek();
            if self.cursor.is("==") || self.cursor.is("!=") {
                return Err(unsupported(
                    &format!("loose equality {}", token.value),
                    "use === or !==",
                    Some(span(&token, &token)),
                ));
            }
            let op = match (token.kind, token.value.as_str()) {
                (TokenKind::Punct, "===") => BinaryOp::Eq,
                (TokenKind::Punct, "!==") => BinaryOp::Ne,
                _ => return Ok(left),
            };
            self.cursor.advance();
            let right = self.relational()?;
            left = if is_tag_field(&left) || is_tag_field(&right) {
                self.tag_test(op, left, right, &token)?
            } else {
                binary(op, left, right, &token)
            };
        }
    }

    /// `x.$ === 'tag'` tests the alternative of a data value.
    fn tag_test(&self, op: BinaryOp, left: SExpr, right: SExpr, token: &Token) -> Result<SExpr> {
        let place = joined(&left, &right, token);
        let (field, tag) = if is_tag_field(&left) {
            (left, right)
        } else {
            (right, left)
        };
        let SNode::Field { object, .. } = field.node else {
            return Ok(field);
        };
        let SNode::Str { value: tag } = tag.node else {
            return Err(unsupported(
                "computed tag test",
                "compare the $ tag with a string literal",
                Some(place),
            ));
        };
        let candidates: Vec<&SData> = self
            .data_types
            .iter()
            .filter(|data| has_ctor(data, &tag))
            .collect();
        let data = match candidates.as_slice() {
            [data] => (*data).clone(),
            [] => {
                return Err(type_error(
                    format!("no @typedef has tag {tag}"),
                    Some(place),
                ))
            }
            _ => {
                return Err(type_error(
                    format!("tag {tag} is ambiguous between data types"),
                    Some(place),
                ))
            }
        };
        let arity = data
            .ctors
            .iter()
            .find(|ctor| ctor.name == tag)
            .map_or(0, |ctor| ctor.fields.len());
        let pattern = SPattern {
            node: SPatternNode::Ctor {
                path: vec![ROOT.to_owned(), data.name.clone(), tag.clone()],
                args: (0..arity).map(|_| wild(place)).collect(),
            },
            span: Some(place),
        };
        let equal = op == BinaryOp::Eq;
        let rows = vec![
            SRow {
                patterns: vec![pattern],
                body: node(SNode::Bool { value: equal }, place),
                span: Some(place),
            },
            SRow {
                patterns: vec![wild(place)],
                body: node(SNode::Bool { value: !equal }, place),
                span: Some(place),
            },
        ];
        let mut expr = node(
            SNode::Match {
                scrutinees: vec![object.as_ref().clone()],
                rows,
            },
            place,
        );
        expr.tag_test = Some(Box::new(STagTest {
            object: *object,
            tag,
            data,
            negated: op == BinaryOp::Ne,
        }));
        Ok(expr)
    }

    fn relational(&mut self) -> Result<SExpr> {
        let mut left = self.shift()?;
        loop {
            let token = self.peek();
            if self.cursor.is("in") || self.cursor.is("instanceof") {
                return Err(unsupported(
                    &format!("{} operator", token.value),
                    "objects are outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            let op = match (token.kind, token.value.as_str()) {
                (TokenKind::Punct, "<") => BinaryOp::Lt,
                (TokenKind::Punct, "<=") => BinaryOp::Le,
                (TokenKind::Punct, ">") => BinaryOp::Gt,
                (TokenKind::Punct, ">=") => BinaryOp::Ge,
                _ => return Ok(left),
            };
            self.cursor.advance();
            let right = self.shift()?;
            left = binary(op, left, right, &token);
        }
    }

    fn shift(&mut self) -> Result<SExpr> {
        let left = self.additive()?;
        let token = self.peek();
        if self.cursor.is("<<") || self.cursor.is(">>") || self.cursor.is(">>>") {
            return Err(unsupported(
                &format!("shift {}", token.value),
                "bit shifts are outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        Ok(left)
    }

    /// `+` adds numbers and concatenates strings; the checker decides which by the operand types.
    fn additive(&mut self) -> Result<SExpr> {
        let mut left = self.multiplicative()?;
        loop {
            let token = self.peek();
            if !self.cursor.is("+") && !self.cursor.is("-") {
                return Ok(left);
            }
            self.cursor.advance();
            let right = self.multiplicative()?;
            let op = if token.value == "+" {
                BinaryOp::Plus
            } else {
                BinaryOp::Sub
            };
            left = binary(op, left, right, &token);
        }
    }

    fn multiplicative(&mut self) -> Result<SExpr> {
        let mut left = self.exponent()?;
        loop {
            let token = self.peek();
            let op = match (token.kind, token.value.as_str()) {
                (TokenKind::Punct, "*") => BinaryOp::Mul,
                (TokenKind::Punct, "/") => BinaryOp::Div,
                (TokenKind::Punct, "%") => BinaryOp::Rem,
                _ => return Ok(left),
            };
            self.cursor.advance();
            let right = self.exponent()?;
            left = binary(op, left, right, &token);
        }
    }

    fn exponent(&mut self) -> Result<SExpr> {
        let left = self.unary()?;
        if self.cursor.is("**") {
            let token = self.peek();
            return Err(unsupported(
                "exponentiation",
                "powers are outside the portable core; use recursion",
                Some(span(&token, &token)),
            ));
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<SExpr> {
        let token = self.peek();
        for (symbol, op) in [("!", UnaryOp::Not), ("-", UnaryOp::Neg)] {
            if self.cursor.eat(symbol).is_some() {
                let arg = self.unary()?;
                let place = Span::new(token.start, arg.span.map_or(token.end, |arg| arg.end));
                return Ok(node(
                    SNode::Unary {
                        op,
                        arg: Box::new(arg),
                    },
                    place,
                ));
            }
        }
        let here = Some(span(&token, &token));
        if self.cursor.is("+") {
            return Err(unsupported(
                "unary +",
                "unary plus throws a TypeError on BigInt values",
                here,
            ));
        }
        if self.cursor.is("~") {
            return Err(unsupported(
                "bitwise ~",
                "bitwise operators are outside the portable core",
                here,
            ));
        }
        if self.cursor.is("++") || self.cursor.is("--") {
            return Err(unsupported(
                &format!("{} operator", token.value),
                "mutation is outside the portable core",
                here,
            ));
        }
        if token.kind == TokenKind::Identifier
            && ["typeof", "void", "delete", "await", "yield"].contains(&token.value.as_str())
        {
            return Err(unsupported(
                &format!("{} operator", token.value),
                "outside the portable core",
                here,
            ));
        }
        self.postfix()
    }

    fn postfix(&mut self) -> Result<SExpr> {
        let mut expr = self.primary()?;
        loop {
            let token = self.peek();
            let start = expr.span.map_or(token.start, |place| place.start);
            if self.cursor.is(".") && self.cursor.is_kind_at(TokenKind::Identifier, 1) {
                self.cursor.advance();
                let field = self.cursor.advance();
                if self.cursor.is("(") {
                    if field.value != "toString" {
                        return Err(unsupported(
                            &format!("method call .{}()", field.value),
                            "methods of values are outside the portable core",
                            Some(self.to_here(&token)),
                        ));
                    }
                    let args = self.arguments("toString")?;
                    if !args.is_empty() {
                        return Err(unsupported(
                            "toString with a radix",
                            "only decimal text is portable",
                            Some(self.to_here(&token)),
                        ));
                    }
                    let place = Span::new(start, self.to_here(&token).end);
                    expr = node(
                        SNode::ToString {
                            arg: Box::new(expr),
                        },
                        place,
                    );
                    continue;
                }
                let place = Span::new(start, span(&field, &field).end);
                expr = node(
                    SNode::Field {
                        object: Box::new(expr),
                        field: field.value,
                    },
                    place,
                );
                continue;
            }
            let here = Some(span(&token, &token));
            if self.cursor.is("?.") {
                return Err(unsupported(
                    "optional chaining",
                    "null and undefined are outside the portable core",
                    here,
                ));
            }
            if self.cursor.is("[") {
                return Err(unsupported(
                    "indexing",
                    "arrays and computed properties are outside the portable core",
                    here,
                ));
            }
            if self.cursor.is("(") {
                return Err(unsupported(
                    "call of a computed function",
                    "only named functions can be called",
                    here,
                ));
            }
            if self.cursor.is_kind(TokenKind::Template) {
                return Err(unsupported(
                    "tagged template",
                    "template tags are functions as values",
                    here,
                ));
            }
            if self.cursor.is("++") || self.cursor.is("--") {
                return Err(unsupported(
                    &format!("{} operator", token.value),
                    "mutation is outside the portable core",
                    here,
                ));
            }
            return Ok(expr);
        }
    }

    fn primary(&mut self) -> Result<SExpr> {
        let token = self.peek();
        match token.kind {
            TokenKind::Number => {
                self.cursor.advance();
                if self.cursor.is(".")
                    && self.cursor.is_kind_at(TokenKind::Number, 1)
                    && self.cursor.peek_at(1).start == self.cursor.peek().end
                {
                    return Err(unsupported(
                        "JavaScript number",
                        NUMBER_REASON,
                        Some(span(&token, self.cursor.peek_at(1))),
                    ));
                }
                return Ok(node(
                    SNode::Num {
                        value: bigint(&token)?,
                        ty: None,
                        negative: false,
                    },
                    span(&token, &token),
                ));
            }
            TokenKind::String => {
                self.cursor.advance();
                return Ok(node(
                    SNode::Str {
                        value: token.value.clone(),
                    },
                    span(&token, &token),
                ));
            }
            TokenKind::Template => {
                self.cursor.advance();
                return self.template(&token);
            }
            TokenKind::Identifier => return self.identifier(),
            _ => {}
        }
        let here = Some(span(&token, &token));
        if self.cursor.is("(") {
            let close = self.matching(self.cursor.index);
            if self
                .cursor
                .tokens
                .get(close + 1)
                .is_some_and(|after| after.value == "=>")
            {
                return Err(unsupported(
                    "arrow function",
                    "functions as values are outside the portable core",
                    here,
                ));
            }
            self.cursor.advance();
            let mut inner = self.expr()?;
            self.cursor.expect(")", Some("parenthesised expression"))?;
            inner.span = Some(self.to_here(&token));
            return Ok(inner);
        }
        if self.cursor.is("{") {
            return self.object_literal();
        }
        if self.cursor.is("[") {
            return Err(unsupported(
                "array literal",
                "arrays are outside the portable core",
                here,
            ));
        }
        if self.cursor.is("/") {
            return Err(unsupported(
                "regular expression",
                "outside the portable core",
                here,
            ));
        }
        Err(self.fail_here("expected an expression"))
    }

    fn matching(&self, index: usize) -> usize {
        let tokens = &self.cursor.tokens;
        let mut depth = 0_i64;
        for (at, token) in tokens.iter().enumerate().skip(index) {
            if token.kind != TokenKind::Punct {
                continue;
            }
            if ["(", "[", "{"].contains(&token.value.as_str()) {
                depth += 1;
            }
            if [")", "]", "}"].contains(&token.value.as_str()) {
                depth -= 1;
                if depth == 0 {
                    return at;
                }
            }
        }
        tokens.len() - 1
    }

    /// Template literals concatenate their text with `String(value)` of each substitution.
    fn template(&mut self, token: &Token) -> Result<SExpr> {
        let whole = span(token, token);
        let mut pieces = Vec::new();
        for part in &token.parts {
            if !part.text.is_empty() {
                pieces.push(node(
                    SNode::Str {
                        value: part.text.clone(),
                    },
                    whole,
                ));
            }
            if let Some(expression) = &part.expression {
                let value = self.subexpression(&expression.source, expression.offset)?;
                let place = value.span;
                pieces.push(SExpr::new(
                    SNode::Show {
                        arg: Box::new(value),
                        style: ShowStyle::JsTemplate,
                    },
                    place,
                ));
            }
        }
        let mut pieces = pieces.into_iter();
        let Some(first) = pieces.next() else {
            return Ok(node(
                SNode::Str {
                    value: String::new(),
                },
                whole,
            ));
        };
        Ok(pieces.fold(first, |left, right| {
            node(
                SNode::Binary {
                    op: BinaryOp::Concat,
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding: None,
                },
                whole,
            )
        }))
    }

    fn subexpression(&mut self, source: &str, offset: usize) -> Result<SExpr> {
        let mut tokens = tokenize(source, Language::JavaScript)?.tokens;
        for token in &mut tokens {
            token.start += offset;
            token.end += offset;
        }
        let outer = std::mem::replace(
            &mut self.cursor,
            TokenCursor::new(tokens, Language::JavaScript),
        );
        let expr = self.substitution();
        self.cursor = outer;
        expr
    }

    fn substitution(&mut self) -> Result<SExpr> {
        let expr = self.expr()?;
        if !self.cursor.at_end() {
            return Err(self.fail_here("expected } after the template substitution"));
        }
        Ok(expr)
    }

    /// `{ $: 'node', left, value: v }` constructs the `node` alternative of a data type.
    fn object_literal(&mut self) -> Result<SExpr> {
        let start = self.cursor.advance();
        let mut tag = None;
        let mut fields: Vec<(String, SExpr)> = Vec::new();
        while !self.cursor.is("}") {
            let key = self.peek();
            let here = Some(span(&key, &key));
            if self.cursor.is("...") {
                return Err(unsupported("object spread", "name every field", here));
            }
            if key.kind != TokenKind::Identifier {
                return Err(unsupported(
                    &format!("object key {}", describe(&key)),
                    "keys are identifiers",
                    here,
                ));
            }
            self.cursor.advance();
            if key.value == "$" {
                self.cursor.expect(":", Some("object"))?;
                let value = self.cursor.advance();
                if value.kind != TokenKind::String {
                    return Err(unsupported(
                        "computed tag",
                        "the $ tag is a string literal",
                        Some(span(&value, &value)),
                    ));
                }
                tag = Some(value.value);
            } else {
                if fields.iter().any(|(name, _)| *name == key.value) {
                    return Err(type_error(format!("duplicate field {}", key.value), here));
                }
                if self.cursor.is("(") {
                    return Err(unsupported(
                        "method in an object value",
                        "functions as values are outside the portable core",
                        here,
                    ));
                }
                let value = if self.cursor.eat(":").is_some() {
                    self.expr()?
                } else {
                    self.reference(&key)?
                };
                // Assigning `__proto__` sets the prototype of the JavaScript
                // frontend's field object rather than adding a field.
                if key.value != "__proto__" {
                    fields.push((key.value, value));
                }
            }
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("object"))?;
        let Some(tag) = tag else {
            return Err(unsupported(
                "object without a $ tag",
                "a portable object is an alternative of a @typedef data type",
                Some(self.to_here(&start)),
            ));
        };
        Ok(node(
            SNode::CtorObject { tag, fields },
            self.to_here(&start),
        ))
    }

    fn identifier(&mut self) -> Result<SExpr> {
        let token = self.peek();
        let here = Some(span(&token, &token));
        if token.value == "true" || token.value == "false" {
            self.cursor.advance();
            return Ok(node(
                SNode::Bool {
                    value: token.value == "true",
                },
                span(&token, &token),
            ));
        }
        if ["null", "undefined", "NaN", "Infinity"].contains(&token.value.as_str()) {
            return Err(unsupported(&token.value, "outside the portable core", here));
        }
        let expressions = [
            "this",
            "super",
            "new",
            "function",
            "class",
            "import",
            "arguments",
        ];
        if expressions.contains(&token.value.as_str()) {
            return Err(unsupported(
                &format!("{} expression", token.value),
                "outside the portable core",
                here,
            ));
        }
        self.cursor.advance();
        if self.scope.locals.contains(&token.value) || self.scope.tdz.contains(&token.value) {
            return self.reference(&token);
        }
        if self
            .assertion
            .as_ref()
            .is_some_and(|assertion| assertion.name == token.value)
        {
            return Err(unsupported(
                "assertion in an expression",
                "assertions are top-level statements",
                here,
            ));
        }
        // A global: a function, or a namespace path to a method, which must be called.
        let mut segments = vec![token.value.clone()];
        while self.cursor.is(".") && self.cursor.is_kind_at(TokenKind::Identifier, 1) {
            self.cursor.advance();
            segments.push(self.cursor.advance().value);
        }
        let name = segments.join(".");
        let place = self.to_here(&token);
        if !self.cursor.is("(") {
            return Err(unsupported(
                &format!("function value {name}"),
                "functions and namespaces are only portable when a function is called",
                Some(place),
            ));
        }
        let mut args = self.arguments(&name)?;
        let called = self.to_here(&token);
        if name == "String" && args.len() == 1 {
            let arg = args.remove(0);
            return Ok(node(SNode::ToString { arg: Box::new(arg) }, called));
        }
        if name == "Object.freeze" && args.len() == 1 {
            let mut arg = args.remove(0);
            arg.span = Some(called);
            return Ok(arg);
        }
        if GLOBALS.contains(&segments[0].as_str()) || name == "String" {
            return Err(unsupported(
                &name,
                "the JavaScript standard library is outside the portable core",
                Some(called),
            ));
        }
        let mut path = vec![ROOT.to_owned()];
        path.extend(segments);
        if args.is_empty() {
            return Ok(node(SNode::Name { path }, called));
        }
        Ok(node(
            SNode::App {
                func: Box::new(node(SNode::Name { path }, place)),
                args,
            },
            called,
        ))
    }

    /// A local variable; reading one before its declaration throws in JavaScript.
    fn reference(&self, token: &Token) -> Result<SExpr> {
        let here = span(token, token);
        if self.scope.tdz.contains(&token.value) {
            return Err(unsupported(
                &format!("use of {} before its declaration", token.value),
                "the binding is in its temporal dead zone, where reading it throws a ReferenceError",
                Some(here),
            ));
        }
        if !self.scope.locals.contains(&token.value) {
            return Err(unsupported(
                &format!("function value {}", token.value),
                "functions are only portable when called",
                Some(here),
            ));
        }
        Ok(node(
            SNode::Name {
                path: vec![token.value.clone()],
            },
            here,
        ))
    }
}

/// What `assert.method` asserts: the lookup is in plain objects, so the names
/// of `Object.prototype` find an inherited function and read as `notDeep`.
fn assertion_kind(method: &str, strict: bool) -> Option<AssertionKind> {
    let kind = match method {
        "strictEqual" => AssertionKind::Eq,
        "notStrictEqual" => AssertionKind::Ne,
        "deepStrictEqual" => AssertionKind::Deep,
        "notDeepStrictEqual" => AssertionKind::NotDeep,
        _ if OBJECT_PROTOTYPE.contains(&method) => AssertionKind::NotDeep,
        "equal" if strict => AssertionKind::Eq,
        "notEqual" if strict => AssertionKind::Ne,
        "deepEqual" if strict => AssertionKind::Deep,
        "notDeepEqual" if strict => AssertionKind::NotDeep,
        _ => return None,
    };
    Some(kind)
}

/// Statements to one expression: `const` is `let`, `if` with a returning
/// branch selects between the branch and the rest, and every path must end
/// in `return` or `throw`.
fn lower(statements: &[Stmt], place: Span) -> Result<SExpr> {
    let list: Vec<&Stmt> = statements
        .iter()
        .flat_map(|statement| match statement {
            Stmt::Block {
                body, inline: true, ..
            } => body.iter().collect(),
            other => vec![other],
        })
        .filter(|statement| !matches!(statement, Stmt::Empty))
        .collect();
    lower_at(&list, 0, place)
}

const fn statement_span(statement: &Stmt) -> Option<Span> {
    match statement {
        Stmt::Const { span, .. }
        | Stmt::Return { span, .. }
        | Stmt::Throw { span, .. }
        | Stmt::Block { span, .. }
        | Stmt::If { span, .. }
        | Stmt::Expr { span } => Some(*span),
        Stmt::Switch(node) => Some(node.span),
        Stmt::Empty => None,
    }
}

fn unreachable(list: &[&Stmt], index: usize) -> Result<()> {
    if index + 1 < list.len() {
        return Err(unsupported(
            "unreachable statement",
            "statements after return, throw or a complete if are never executed",
            statement_span(list[index + 1]),
        ));
    }
    Ok(())
}

fn lower_at(list: &[&Stmt], index: usize, place: Span) -> Result<SExpr> {
    let Some(statement) = list.get(index) else {
        return Err(unsupported(
            "missing return",
            "the function can finish without returning and return undefined, which is not a portable value",
            Some(place),
        ));
    };
    match statement {
        Stmt::Const { name, value, span } => Ok(node(
            SNode::Let {
                name: name.clone(),
                ty: None,
                value: Box::new(value.clone()),
                body: Box::new(lower_at(list, index + 1, place)?),
            },
            *span,
        )),
        Stmt::Return { expr, .. } => {
            unreachable(list, index)?;
            Ok(expr.clone())
        }
        Stmt::Throw { message, span } => {
            unreachable(list, index)?;
            Ok(node(
                SNode::Abort {
                    message: message.clone(),
                },
                *span,
            ))
        }
        Stmt::Block { body, span, .. } => {
            if !terminates(body) {
                return Err(unsupported(
                    "block without a result",
                    "the block must end in return or throw",
                    Some(*span),
                ));
            }
            unreachable(list, index)?;
            lower(body, *span)
        }
        Stmt::If {
            cond,
            then,
            otherwise,
            span,
        } => {
            if !terminates(then) || otherwise.as_ref().is_some_and(|branch| !terminates(branch)) {
                return Err(unsupported(
                    "if branch without a result",
                    "every branch must end in return or throw; statements without effects are outside the portable core",
                    Some(*span),
                ));
            }
            let then = lower(then, *span)?;
            if let Some(test) = tag_condition(cond) {
                if otherwise.is_some() {
                    unreachable(list, index)?;
                }
                let other = match otherwise {
                    Some(branch) => lower(branch, *span)?,
                    None => lower_at(list, index + 1, place)?,
                };
                return lower_tag_if(test, then, other, *span);
            }
            let other = if let Some(branch) = otherwise {
                unreachable(list, index)?;
                lower(branch, *span)?
            } else {
                lower_at(list, index + 1, place)?
            };
            Ok(node(
                SNode::If {
                    cond: Box::new(cond.clone()),
                    then: Box::new(then),
                    otherwise: Box::new(other),
                },
                *span,
            ))
        }
        Stmt::Switch(switch) => lower_switch(switch, list, index, place),
        Stmt::Expr { span } => Err(unsupported(
            "expression statement",
            "statements with effects are outside the portable core in function bodies",
            Some(*span),
        )),
        Stmt::Empty => Err(TranslationError::syntax("unknown statement empty", None)),
    }
}

fn terminates(statements: &[Stmt]) -> bool {
    let last = statements
        .iter()
        .rev()
        .find(|statement| !matches!(statement, Stmt::Empty | Stmt::Block { inline: true, .. }));
    match last {
        Some(Stmt::Return { .. } | Stmt::Throw { .. }) => true,
        Some(Stmt::Block { body, .. }) => terminates(body),
        Some(Stmt::If {
            then,
            otherwise: Some(otherwise),
            ..
        }) => terminates(then) && terminates(otherwise),
        Some(Stmt::Switch(node)) => switch_terminates(node),
        _ => false,
    }
}

fn switch_terminates(node: &Switch) -> bool {
    if !node.clauses.iter().all(|clause| terminates(&clause.body)) {
        return false;
    }
    if node.clauses.iter().any(|clause| {
        clause
            .tests
            .iter()
            .any(|test| matches!(test, CaseTest::Default { .. }))
    }) {
        return true;
    }
    let Some((_, data)) = &node.tag else {
        return false;
    };
    let tags = switch_tags(node);
    data.ctors
        .iter()
        .all(|ctor| tags.contains(&ctor.name.as_str()))
}

/// The tags a switch's cases name.
fn switch_tags(node: &Switch) -> Vec<&str> {
    node.clauses
        .iter()
        .flat_map(|clause| &clause.tests)
        .filter_map(|test| match test {
            CaseTest::Tag { tag, .. } => Some(tag.as_str()),
            _ => None,
        })
        .collect()
}

/// The subject, data type and covered tags of a tag switch or tag test.
struct TagSwitch<'a> {
    subject: &'a str,
    data: &'a SData,
    covered: Vec<&'a str>,
}

/// A switch is a match. In a tag switch, `x.field` of the subject in a case
/// is a pattern variable of that case's constructor; the variables get
/// names that nothing in the case body uses, so no reference is captured.
fn lower_switch(node: &Switch, list: &[&Stmt], index: usize, place: Span) -> Result<SExpr> {
    for clause in &node.clauses {
        if !terminates(&clause.body) {
            return Err(unsupported(
                "case without a result",
                "every case must end in return or throw; falling through is outside the portable core",
                Some(clause.span),
            ));
        }
    }
    let tag_switch = node.tag.as_ref().map(|(subject, data)| TagSwitch {
        subject,
        data,
        covered: switch_tags(node),
    });
    let mut rows = Vec::new();
    // `default` applies only when no case matches, wherever it stands.
    let mut defaults = Vec::new();
    let mut has_default = false;
    for clause in &node.clauses {
        for test in &clause.tests {
            let body = lower(&clause.body, clause.span)?;
            match (test, &tag_switch) {
                (CaseTest::Default { span }, tag_switch) => {
                    has_default = true;
                    defaults = match tag_switch {
                        Some(tag_switch) => default_rows(tag_switch, body, clause.span)?,
                        None => vec![SRow {
                            patterns: vec![wild(*span)],
                            body,
                            span: Some(clause.span),
                        }],
                    };
                }
                (CaseTest::Tag { tag, span }, Some(tag_switch)) => {
                    rows.push(tag_row(tag_switch, tag, *span, &body, clause.span)?);
                }
                (
                    CaseTest::NumLit {
                        value,
                        negative,
                        span,
                    },
                    _,
                ) => rows.push(SRow {
                    patterns: vec![SPattern {
                        node: SPatternNode::NumLit {
                            value: value.clone(),
                            negative: *negative,
                        },
                        span: Some(*span),
                    }],
                    body,
                    span: Some(clause.span),
                }),
                (CaseTest::BoolLit { value, span }, _) => rows.push(SRow {
                    patterns: vec![SPattern {
                        node: SPatternNode::BoolLit {
                            value: *value,
                            negative: false,
                        },
                        span: Some(*span),
                    }],
                    body,
                    span: Some(clause.span),
                }),
                (CaseTest::Tag { .. }, None) => {}
            }
        }
    }
    rows.extend(defaults);
    let at_end = index + 1 == list.len();
    if switch_terminates(node) {
        unreachable(list, index)?;
    } else if !at_end && !has_default {
        rows.push(SRow {
            patterns: vec![wild(node.span)],
            body: lower_at(list, index + 1, place)?,
            span: Some(node.span),
        });
    } else if has_default {
        unreachable(list, index)?;
    }
    Ok(SExpr::new(
        SNode::Match {
            scrutinees: vec![node.scrutinee.clone()],
            rows,
        },
        Some(node.span),
    ))
}

/// The `default` of a tag switch covers the remaining alternatives; when it
/// reads fields of the subject, it is one row per remaining alternative.
fn default_rows(tag_switch: &TagSwitch, body: SExpr, place: Span) -> Result<Vec<SRow>> {
    if !reads_fields(&body, tag_switch.subject) {
        return Ok(vec![SRow {
            patterns: vec![wild(place)],
            body,
            span: Some(place),
        }]);
    }
    tag_switch
        .data
        .ctors
        .iter()
        .filter(|ctor| !tag_switch.covered.contains(&ctor.name.as_str()))
        .map(|ctor| tag_row(tag_switch, &ctor.name, place, &body, place))
        .collect()
}

/// `x.$ === 'tag'` on a local `x`: the data type and the tag, when it narrows `x`.
fn tag_condition(cond: &SExpr) -> Option<&STagTest> {
    cond.tag_test
        .as_deref()
        .filter(|test| test.object.simple_name().is_some())
}

/// `if (x.$ === 'tag') A else B` is a match in which `A` reads the fields of `tag`.
fn lower_tag_if(test: &STagTest, then: SExpr, otherwise: SExpr, place: Span) -> Result<SExpr> {
    let tag_switch = TagSwitch {
        subject: test.object.simple_name().unwrap_or_default(),
        data: &test.data,
        covered: vec![test.tag.as_str()],
    };
    let (matching, other) = if test.negated {
        (otherwise, then)
    } else {
        (then, otherwise)
    };
    let mut rows = vec![tag_row(&tag_switch, &test.tag, place, &matching, place)?];
    rows.extend(default_rows(&tag_switch, other, place)?);
    Ok(node(
        SNode::Match {
            scrutinees: vec![test.object.clone()],
            rows,
        },
        place,
    ))
}

fn tag_row(
    tag_switch: &TagSwitch,
    tag: &str,
    test_span: Span,
    case_body: &SExpr,
    place: Span,
) -> Result<SRow> {
    let subject = tag_switch.subject;
    let data = tag_switch.data;
    let Some(ctor) = data.ctors.iter().find(|candidate| candidate.name == tag) else {
        return Err(type_error(
            format!("{} has no tag {tag}", data.name),
            Some(test_span),
        ));
    };
    let has_field = |field: &str| {
        ctor.fields
            .iter()
            .any(|candidate| candidate.name.as_deref() == Some(field))
    };
    // Field names to pattern variables, in order.
    let mut binders: Vec<(String, String)> = Vec::new();
    // `const { left, value } = t;` at the start of the case names the pattern variables itself.
    let mut rest = case_body;
    let mut direct: Vec<&str> = Vec::new();
    while let SNode::Let {
        name, value, body, ..
    } = &rest.node
    {
        let SNode::Field { object, field } = &value.node else {
            break;
        };
        if object.simple_name() != Some(subject)
            || !has_field(field)
            || binders.iter().any(|(bound, _)| bound == field)
            || name == subject
            || direct.contains(&name.as_str())
        {
            break;
        }
        binders.push((field.clone(), name.clone()));
        direct.push(name);
        rest = body;
    }
    let mut rebound = HashSet::new();
    collect_binders(rest, &mut rebound);
    if direct.iter().any(|name| rebound.contains(*name)) {
        binders.clear();
        rest = case_body;
    }
    let mut body = rest.clone();
    let mut used = HashSet::new();
    collect_names(&body, &mut used);
    for (_, name) in &binders {
        used.insert(name.clone());
    }
    let mut binder_for = |field: &str| -> Result<String> {
        if !has_field(field) {
            return Err(type_error(
                format!(
                    "the {tag} alternative of {} has no field {field}",
                    data.name
                ),
                Some(place),
            ));
        }
        if let Some((_, name)) = binders.iter().find(|(bound, _)| bound == field) {
            return Ok(name.clone());
        }
        let mut name = if used.contains(field) || field == subject {
            format!("{subject}_{field}")
        } else {
            field.to_owned()
        };
        let mut index = 2;
        while used.contains(&name) {
            name = format!("{subject}_{field}{index}");
            index += 1;
        }
        used.insert(name.clone());
        binders.push((field.to_owned(), name.clone()));
        Ok(name)
    };
    substitute_fields(&mut body, subject, &mut binder_for)?;
    let args = ctor
        .fields
        .iter()
        .map(|field| {
            let bound = binders
                .iter()
                .find(|(name, _)| Some(name.as_str()) == field.name.as_deref());
            SPattern {
                node: bound.map_or(SPatternNode::Wild, |(_, name)| SPatternNode::BindOrCtor {
                    name: name.clone(),
                }),
                span: Some(test_span),
            }
        })
        .collect();
    Ok(SRow {
        patterns: vec![SPattern {
            node: SPatternNode::Ctor {
                path: vec![ROOT.to_owned(), data.name.clone(), tag.to_owned()],
                args,
            },
            span: Some(test_span),
        }],
        body,
        span: Some(place),
    })
}

/// The subexpressions the JavaScript frontend's generic traversals visit.
/// They skip keys named `span` and `type`, so constructor-object fields with
/// those names are skipped too.
fn children(expr: &SExpr) -> Vec<&SExpr> {
    let mut out: Vec<&SExpr> = Vec::new();
    match &expr.node {
        SNode::App { func, args } => {
            out.push(func);
            out.extend(args);
        }
        SNode::Field { object, .. } => out.push(object),
        SNode::Unary { arg, .. } | SNode::ToString { arg } | SNode::Show { arg, .. } => {
            out.push(arg);
        }
        SNode::Binary { left, right, .. } => {
            out.push(left);
            out.push(right);
        }
        SNode::If {
            cond,
            then,
            otherwise,
        } => {
            out.push(cond);
            out.push(then);
            out.push(otherwise);
        }
        SNode::Let { value, body, .. } => {
            out.push(value);
            out.push(body);
        }
        SNode::Match { scrutinees, rows } => {
            out.extend(scrutinees);
            out.extend(rows.iter().map(|row| &row.body));
        }
        SNode::CtorObject { fields, .. } => {
            out.extend(
                fields
                    .iter()
                    .filter(|(name, _)| name != "span" && name != "type")
                    .map(|(_, value)| value),
            );
        }
        _ => {}
    }
    if let Some(test) = &expr.tag_test {
        out.push(&test.object);
    }
    out
}

/// The names patterns bind.
fn pattern_names(pattern: &SPattern, names: &mut HashSet<String>) {
    match &pattern.node {
        SPatternNode::BindOrCtor { name } => {
            names.insert(name.clone());
        }
        SPatternNode::Ctor { args, .. } => {
            for arg in args {
                pattern_names(arg, names);
            }
        }
        _ => {}
    }
}

fn row_pattern_names(expr: &SExpr, names: &mut HashSet<String>) {
    if let SNode::Match { rows, .. } = &expr.node {
        for pattern in rows.iter().flat_map(|row| &row.patterns) {
            pattern_names(pattern, names);
        }
    }
}

/// Every name a surface expression binds or mentions.
fn collect_names(expr: &SExpr, names: &mut HashSet<String>) {
    match &expr.node {
        SNode::Name { path } if path.len() == 1 => {
            names.insert(path[0].clone());
        }
        SNode::Let { name, .. } => {
            names.insert(name.clone());
        }
        _ => {}
    }
    row_pattern_names(expr, names);
    for child in children(expr) {
        collect_names(child, names);
    }
}

/// Every name a surface expression binds.
fn collect_binders(expr: &SExpr, names: &mut HashSet<String>) {
    if let SNode::Let { name, .. } = &expr.node {
        names.insert(name.clone());
    }
    row_pattern_names(expr, names);
    for child in children(expr) {
        collect_binders(child, names);
    }
}

/// Whether an expression reads a field of `subject`.
fn reads_fields(expr: &SExpr, subject: &str) -> bool {
    if let SNode::Field { object, .. } = &expr.node {
        if object.simple_name() == Some(subject) {
            return true;
        }
    }
    children(expr)
        .into_iter()
        .any(|child| reads_fields(child, subject))
}

/// Replaces `subject.field` by the case's pattern variables, up to a rebinding of `subject`.
fn substitute_fields(
    expr: &mut SExpr,
    subject: &str,
    binder_for: &mut dyn FnMut(&str) -> Result<String>,
) -> Result<()> {
    if let SNode::Field { object, field } = &expr.node {
        if object.simple_name() == Some(subject) {
            let name = binder_for(field)?;
            expr.node = SNode::Name { path: vec![name] };
            return Ok(());
        }
    }
    let mut each = |child: &mut SExpr| substitute_fields(child, subject, binder_for);
    match &mut expr.node {
        SNode::Let {
            name, value, body, ..
        } => {
            each(value)?;
            if name != subject {
                each(body)?;
            }
        }
        SNode::Match { scrutinees, rows } => {
            for scrutinee in scrutinees {
                each(scrutinee)?;
            }
            for row in rows {
                let mut bound = HashSet::new();
                for pattern in &row.patterns {
                    pattern_names(pattern, &mut bound);
                }
                if !bound.contains(subject) {
                    each(&mut row.body)?;
                }
            }
        }
        SNode::App { func, args } => {
            each(func)?;
            for arg in args {
                each(arg)?;
            }
        }
        SNode::Field { object, .. } => each(object)?,
        SNode::Unary { arg, .. } | SNode::ToString { arg } | SNode::Show { arg, .. } => each(arg)?,
        SNode::Binary { left, right, .. } => {
            each(left)?;
            each(right)?;
        }
        SNode::If {
            cond,
            then,
            otherwise,
        } => {
            each(cond)?;
            each(then)?;
            each(otherwise)?;
        }
        SNode::CtorObject { fields, .. } => {
            for (name, value) in fields {
                if name != "span" && name != "type" {
                    each(value)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

/// A proposition from an asserted condition; `===` on objects compares identities.
fn prop_of(expr: SExpr) -> SProp {
    let node = match expr.node {
        SNode::Binary {
            op, left, right, ..
        } if matches!(
            op,
            BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge
        ) =>
        {
            let comparison = SComparison {
                left: *left,
                right: *right,
                reference: matches!(op, BinaryOp::Eq | BinaryOp::Ne),
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

/// The parameter a leading `if (n < 0n) throw …` restricts to the naturals.
fn guarded_parameter(statement: &Stmt, params: &[SParam]) -> Option<usize> {
    let Stmt::If {
        cond,
        then,
        otherwise: None,
        ..
    } = statement
    else {
        return None;
    };
    let then: Vec<&Stmt> = then
        .iter()
        .filter(|inner| !matches!(inner, Stmt::Empty))
        .collect();
    if !matches!(then.as_slice(), [Stmt::Throw { .. }]) {
        return None;
    }
    let SNode::Binary {
        op, left, right, ..
    } = &cond.node
    else {
        return None;
    };
    let is_zero = |expr: &SExpr| matches!(&expr.node, SNode::Num { value, .. } if value == "0");
    let variable = match op {
        BinaryOp::Lt if is_zero(right) => left,
        BinaryOp::Gt if is_zero(left) => right,
        _ => return None,
    };
    let name = variable.simple_name()?;
    params
        .iter()
        .position(|param| param.name == name)
        .filter(|&index| params[index].ty == Some(INT))
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

/// `5n`, `0x1fn`: `BigInt` literals. Plain numbers are doubles and are rejected.
fn bigint(token: &Token) -> Result<String> {
    if token.value == "0" {
        if let Some(radix) = radix_suffix(&token.suffix) {
            let literal = format!("0{radix}");
            // `BigInt('0x')` throws a SyntaxError with this message.
            return Decimal::parse(&literal)
                .map(|value| value.to_string())
                .ok_or_else(|| {
                    TranslationError::syntax(format!("Cannot convert {literal} to a BigInt"), None)
                });
        }
    }
    if token.suffix == "n" {
        return Ok(token.value.clone());
    }
    Err(unsupported(
        "JavaScript number",
        NUMBER_REASON,
        Some(span(token, token)),
    ))
}

/// The radix letter and digits of a suffix matching `^([xob])([0-9a-f]*)n$`.
fn radix_suffix(suffix: &str) -> Option<&str> {
    let body = suffix.strip_suffix('n')?;
    let mut chars = body.chars();
    let radix = chars.next()?;
    (matches!(radix, 'x' | 'o' | 'b')
        && chars.all(|ch| ch.is_ascii_digit() || ('a'..='f').contains(&ch)))
    .then_some(body)
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
