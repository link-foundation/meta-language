//! Files, imports, `JSDoc` typedefs, functions, namespaces and blocks.

use std::collections::HashMap;

use super::infer::infer_javascript_product_types;
use super::infer::infer_javascript_types;
use super::loops::reserved;
use super::math::global;
use super::products::{has_javascript_products, validate_javascript_products};
use super::{
    BOOL, FLOAT, Guard, HashSet, INT, JavaScriptParser, JsDoc, Language, NAT, ROOT, Result, SCtor,
    SData, SField, SFn, SItem, SMain, SModule, SParam, SProgram, STRING, ScanEnd, Scope, Span,
    Stmt, Token, TokenCursor, TokenKind, TranslationError, Type, array, describe,
    guarded_parameter, imperative, is_identifier_name, is_js_space, js_trim, jsdoc_tags, lower,
    lower_imperative, non_empty, span, statement_uses, tokenize, type_error, unsupported,
};
use crate::translation::frontend_rules::{
    find_default_parameter_reference, read_java_script_lexical_declaration_step,
};
use crate::translation::primitive_value_rules::accept_object_property_definition;
use crate::translation::surface::SExpr;

mod file;
mod jsdoc;
use jsdoc::{jsdoc_type, promised};

/// How a function's body is written: a block, or an arrow's block or expression.
#[derive(Clone)]
enum Body {
    Block,
    Arrow,
    RecordParameters(Box<super::record_parameters::RecordParameters>),
    RecordArrowParameters(Box<super::record_parameters::RecordParameters>),
}

impl JavaScriptParser {
    /// `@typedef {{ $: 'leaf' } | { $: 'node', left: Tree, value: bigint }} Tree`
    /// declares a data type: each alternative is a constructor named by its
    /// `$` tag, and its other properties are the constructor's fields.
    pub(super) fn typedefs(&mut self) -> Result<Vec<SItem>> {
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
                let units: Vec<u16> = text.encode_utf16().collect();
                let tagged = units.iter().enumerate().any(|(index, unit)| {
                    *unit == u16::from(b'$')
                        && units[index + 1..].iter().find(|unit| !is_js_space(**unit))
                            == Some(&u16::from(b':'))
                });
                if super::surface_rules::read_java_script_documentation_type_form(tagged) == "alias"
                {
                    if self.type_aliases.contains_key(&name)
                        || self.data_types.iter().any(|data| data.name == name)
                    {
                        return Err(type_error(
                            format!("duplicate @typedef {name}"),
                            Some(range),
                        ));
                    }
                    self.type_aliases.insert(name, text);
                    continue;
                }
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
                    generated: false,
                    source_unit_clone_derivation: None,
                    source_field_signatures: None,
                });
                items.push(SItem::Data(SData {
                    name,
                    ctors,
                    span: Some(range),
                    generated: false,
                    source_unit_clone_derivation: None,
                    source_field_signatures: None,
                }));
            }
        }
        Ok(items)
    }

    /// The alternatives of a `@typedef` union type.
    pub(super) fn typedef_ctors(&self, text: &str, name: &str, range: Span) -> Result<Vec<SCtor>> {
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
                    self.require_scalar_string_token(&value, "datatype constructor tag")?;
                    tag = Some(value.value);
                } else {
                    let mut ty = c.identifier(Some("field type"))?.value;
                    if c.is("<") {
                        // Array<T>, whose closing brackets may be read as one >> token.
                        let mut depth = 0i32;
                        loop {
                            let token = c.advance();
                            for char in token.value.chars() {
                                depth += match char {
                                    '<' => 1,
                                    '>' => -1,
                                    _ => 0,
                                };
                            }
                            ty.push_str(&token.value);
                            if depth <= 0 || c.at_end() {
                                break;
                            }
                        }
                    }
                    while c.is("[") {
                        c.advance();
                        c.expect("]", Some("field type"))?;
                        ty.push_str("[]");
                    }
                    if !c.is(",") && !c.is(";") && !c.is("}") {
                        return Err(unsupported(
                            &format!("field type of {}", key.value),
                            "field types are number, bigint, boolean, string, a @typedef name or an array T[] of one",
                            Some(range),
                        ));
                    }
                    fields.push(SField {
                        name: Some(key.value),
                        ty: self.ty(&ty, range)?,
                        rocq_type: None,
                        span: None,
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

    pub(super) fn ty(&self, text: &str, range: Span) -> Result<Type> {
        jsdoc_type(text, range, &self.type_aliases)
    }

    /// The `JSDoc` block directly before a declaration.
    pub(super) fn jsdoc_for(&self, token: &Token) -> Result<Option<JsDoc>> {
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
        let mut optional_params = HashSet::new();
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
                if tag.optional {
                    optional_params.insert(name.clone());
                }
                params.push((name, ty));
            } else if tag.tag == "returns" || tag.tag == "return" {
                returns = tag.ty;
            }
        }
        Ok(Some(JsDoc {
            params,
            optional_params,
            returns,
            range,
        }))
    }

    pub(super) fn function_declaration(&mut self, start: &Token, is_async: bool) -> Result<SFn> {
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
        global(&name)?;
        self.lexical.declaration(&name.value, name.span(), 0);
        self.function_rest(start, &name, is_async, true)
    }

    /// Parameters and body of a function or method; `doc_token` is where its `JSDoc` ends.
    pub(super) fn function_rest(
        &mut self,
        doc_token: &Token,
        name_token: &Token,
        is_async: bool,
        named_record_parameters: bool,
    ) -> Result<SFn> {
        let doc = self.jsdoc_for(doc_token)?;
        self.cursor.expect("(", Some(&name_token.value))?;
        let arrays = self.read_array_formal_parameters(
            if named_record_parameters {
                "ordinary named function declaration"
            } else {
                "ordinary namespace object method"
            },
            doc.as_ref(),
            is_async,
        )?;
        let records = if arrays.is_some() {
            arrays
        } else if named_record_parameters {
            self.read_named_record_parameters(doc.as_ref(), is_async)?
        } else {
            self.read_record_namespace_parameters(doc.as_ref(), is_async)?
        };
        if let Some(records) = records {
            return self.function_body(
                doc_token,
                name_token,
                doc,
                &[],
                &Body::RecordParameters(Box::new(records)),
                is_async,
            );
        }
        let tokens = self.parameters(&name_token.value)?;
        self.function_body(doc_token, name_token, doc, &tokens, &Body::Block, is_async)
    }

    /// `const name = (…) => …` or `const name = function (…) { … }` before any
    /// top-level statement: a function, as nothing can call it before it is
    /// initialised. An arrow's expression body is the value it returns.
    pub(super) fn const_function(&mut self, start: &Token) -> Result<SFn> {
        let doc = self.jsdoc_for(start)?;
        self.cursor.expect("const", Some("function"))?;
        let name_token = self.cursor.identifier(Some("function"))?;
        global(&name_token)?;
        self.lexical
            .declaration(&name_token.value, name_token.span(), 1);
        self.cursor.expect("=", Some("function"))?;
        self.scope.tdz.remove(&name_token.value);
        let is_async = self.cursor.is("async") && !self.cursor.is_at("=>", 1);
        if is_async {
            self.cursor.advance();
        }
        let function = if self.cursor.eat("function").is_some() {
            if self.cursor.is("*") {
                return Err(unsupported(
                    "generator function",
                    "generators are outside the portable core",
                    Some(self.to_here(start)),
                ));
            }
            if self.cursor.is_kind(TokenKind::Identifier) {
                let inner = self.peek();
                if inner.value != name_token.value {
                    return Err(unsupported(
                        &format!("function expression {}", inner.value),
                        &format!(
                            "its own name is visible only inside it; call it {}",
                            name_token.value
                        ),
                        Some(span(&inner, &inner)),
                    ));
                }
                self.cursor.advance();
            }
            self.cursor.expect("(", Some(&name_token.value))?;
            let context=crate::translation::record_callable_parameter_rules::read_java_script_record_constant_context(true).to_string();
            let arrays = self.read_array_formal_parameters(&context, doc.as_ref(), is_async)?;
            let records = if arrays.is_some() {
                arrays
            } else {
                self.read_record_constant_parameters(doc.as_ref(), is_async, true)?
            };
            let tokens = if records.is_some() {
                Vec::new()
            } else {
                self.parameters(&name_token.value)?
            };
            let body = records.map_or(Body::Block, |records| {
                Body::RecordParameters(Box::new(records))
            });
            self.function_body(start, &name_token, doc, &tokens, &body, is_async)?
        } else {
            let mut records = None;
            let tokens = if self.cursor.eat("(").is_some() {
                let context=crate::translation::record_callable_parameter_rules::read_java_script_record_constant_context(false).to_string();
                let arrays = self.read_array_formal_parameters(&context, doc.as_ref(), is_async)?;
                records = if arrays.is_some() {
                    arrays
                } else {
                    self.read_record_constant_parameters(doc.as_ref(), is_async, false)?
                };
                if records.is_some() {
                    Vec::new()
                } else {
                    self.parameters(&name_token.value)?
                }
            } else {
                vec![(self.cursor.identifier(Some("parameter"))?, None)]
            };
            self.cursor.expect("=>", Some(&name_token.value))?;
            let body = records.map_or(Body::Arrow, |records| {
                Body::RecordArrowParameters(Box::new(records))
            });
            self.function_body(start, &name_token, doc, &tokens, &body, is_async)?
        };
        self.cursor.eat(";");
        Ok(function)
    }

    /// Names of the top-level async functions, which every call must await.
    fn async_declarations(&self, index: usize) -> HashSet<String> {
        let tokens = &self.cursor.tokens;
        let value = |at: usize| tokens.get(at).map(|token| token.value.as_str());
        let identifier = |at: usize| {
            tokens
                .get(at)
                .filter(|token| token.kind == TokenKind::Identifier)
        };
        let mut names = HashSet::new();
        let mut depth = 0_i64;
        for (at, token) in tokens.iter().enumerate().skip(index) {
            if token.kind == TokenKind::Punct {
                match token.value.as_str() {
                    "(" | "[" | "{" => depth += 1,
                    ")" | "]" | "}" => depth -= 1,
                    _ => {}
                }
            }
            if depth != 0 || token.kind != TokenKind::Identifier {
                continue;
            }
            if token.value == "async"
                && value(at + 1) == Some("function")
                && let Some(name) = identifier(at + 2)
            {
                names.insert(name.value.clone());
            }
            if token.value == "const"
                && value(at + 2) == Some("=")
                && value(at + 3) == Some("async")
                && value(at + 4) != Some("=>")
                && self.starts_function(at + 3)
                && let Some(name) = identifier(at + 1)
            {
                names.insert(name.value.clone());
            }
        }
        names
    }

    /// Whether the tokens from `index` are a function expression or an arrow
    /// function, either maybe async.
    pub(super) fn starts_function(&self, index: usize) -> bool {
        let tokens = &self.cursor.tokens;
        let Some(mut token) = tokens.get(index) else {
            return false;
        };
        if token.kind == TokenKind::Identifier
            && !crate::translation::javascript_identifier_rules::accept_java_script_source_keyword_token(&token.value, &token.raw)
            && tokens.get(index + 1).map(|next| next.value.as_str()) != Some("=>") { return false; }
        let mut index = index;
        if token.kind == TokenKind::Identifier
            && token.value == "async"
            && tokens.get(index + 1).map(|next| next.value.as_str()) != Some("=>")
        {
            index += 1;
            let Some(next) = tokens.get(index) else {
                return false;
            };
            token = next;
        }
        let kind = match token.kind {
            TokenKind::Identifier => "identifier",
            TokenKind::Punct => "punct",
            _ => "",
        };
        let after_group = if token.kind == TokenKind::Punct && token.value == "(" {
            tokens.get(self.matching(index) + 1)
        } else {
            None
        };
        super::surface_rules::accept_java_script_function_start(
            kind,
            &token.value,
            tokens.get(index + 1).map_or("", |next| next.value.as_str()),
            after_group.map_or("", |next| next.value.as_str()),
        )
    }

    /// A parameter list after its `(`, through its `)`.
    pub(super) fn parameters(&mut self, name: &str) -> Result<Vec<(Token, Option<SExpr>)>> {
        let mut tokens: Vec<(Token, Option<SExpr>)> = Vec::new();
        let binding_names = self.parameter_binding_names();
        let start = self.cursor.index;
        let close = self.matching(start.saturating_sub(1));
        let source = self.cursor.tokens[start..=close].to_vec();
        let ordinary_kinds = source
            .iter()
            .map(|token| super::record_parameters::token_kind(token).to_owned())
            .collect::<Vec<_>>();
        let ordinary_values = source
            .iter()
            .map(|token| token.value.clone())
            .collect::<Vec<_>>();
        let census=crate::translation::sequential_parameter_default_rules::plan_ordinary_parameter_binding_positions(&ordinary_kinds,&ordinary_values,0.0);
        let positions = census.get(1..).unwrap_or_default();
        let bound_names = positions
            .iter()
            .map(|at| {
                source[crate::translation::frontend_rules::ml_array::number_index(*at)
                    .expect("actual ordinary binder")]
                .value
                .clone()
            })
            .collect::<Vec<_>>();
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
            let default_value = if self.cursor.eat("=").is_some() {
                let remaining = &self.cursor.tokens[self.cursor.index..start + source.len()];
                let kinds = remaining
                    .iter()
                    .map(|token| {
                        match token.kind {
                            TokenKind::Identifier => "identifier",
                            TokenKind::Eof => "eof",
                            _ => "other",
                        }
                        .to_owned()
                    })
                    .collect();
                let values = remaining.iter().map(|token| token.value.clone()).collect();
                let parameters = tokens
                    .iter()
                    .map(|(token, _)| token.value.clone())
                    .collect();
                let reference = find_default_parameter_reference(kinds, values, parameters);
                if reference >= 0.0 {
                    use crate::translation::array_formal_parameter_rules::{
                        self as rules, ml_string::Text,
                    };
                    use crate::translation::frontend_rules::ml_array::number_index;
                    let first = self.cursor.index - start;
                    let kinds = ordinary_kinds
                        .iter()
                        .map(|text| Text::from_text(text))
                        .collect::<Vec<_>>();
                    let values = ordinary_values
                        .iter()
                        .map(|text| Text::from_text(text))
                        .collect::<Vec<_>>();
                    let first_number = serde_json::Number::from(first)
                        .as_f64()
                        .expect("actual initializer token");
                    let last =
                        number_index(rules::find_java_script_pattern_default_expression_end(
                            &kinds,
                            &values,
                            first_number,
                            &Text::from_text(")"),
                        ))
                        .ok_or_else(|| {
                            unsupported(
                                "ordinary parameter default transport",
                                "the actual initializer needs its complete source interval",
                                Some(param.span()),
                            )
                        })?;
                    let binding = source
                        .iter()
                        .position(|token| token.start == param.start && token.end == param.end)
                        .expect("actual ordinary binding token");
                    let binding = serde_json::Number::from(binding)
                        .as_f64()
                        .expect("actual binding position");
                    let value = self.read_formal_initializer(
                        &source,
                        super::formal_initializers::FormalInitializer {
                            interval_positions: (first, last, start),
                            binding_positions: positions,
                            binding,
                            bound_names: &bound_names,
                            context: "ordinary named function declaration",
                            profile: "ordinary parameter default",
                        },
                    )?;
                    self.cursor.index = start + last;
                    Some(value)
                } else {
                    let outer_tdz = self.scope.tdz.clone();
                    self.scope.tdz.extend(binding_names.iter().cloned());
                    let value = self.with_lexical_scope(&[], 2, Self::expr);
                    self.scope.tdz = outer_tdz;
                    Some(value?)
                }
            } else {
                None
            };
            tokens.push((param, default_value));
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect(")", Some(name))?;
        Ok(tokens)
    }

    /// A function from its parameters; `body` says how its body is written.
    /// An async function is the function its body computes, and its `JSDoc`
    /// may declare the result as `Promise<T>`.
    fn function_body(
        &mut self,
        doc_token: &Token,
        name_token: &Token,
        doc: Option<JsDoc>,
        tokens: &[(Token, Option<SExpr>)],
        body: &Body,
        is_async: bool,
    ) -> Result<SFn> {
        let name = name_token.value.clone();
        if is_async {
            self.sequential_async = true;
        }
        // A type JSDoc does not declare is inferred once the whole program is read.
        let records = match body {
            Body::RecordParameters(records) | Body::RecordArrowParameters(records) => {
                Some(records.as_ref())
            }
            _ => None,
        };
        let mut params = records.map_or_else(Vec::new, |records| records.parameters.clone());
        for (token, default_value) in tokens {
            reserved(token)?;
            let optional = default_value.is_none()
                && doc
                    .as_ref()
                    .is_some_and(|doc| doc.optional_params.contains(&token.value));
            let text = doc.as_ref().and_then(|doc| {
                doc.params
                    .iter()
                    .find(|(param, _)| *param == token.value)
                    .map(|(_, text)| (text.clone(), doc.range))
            });
            let declared = text
                .map(|(text, range)| self.ty(&text, range))
                .transpose()?;
            params.push(SParam {
                name: token.value.clone(),
                optional,
                default_value: default_value.clone().or_else(|| {
                    optional.then(|| {
                        SExpr::new(
                            super::SNode::None {
                                absence: "undefined".to_owned(),
                            },
                            Some(span(token, token)),
                        )
                    })
                }),
                ty: declared.map(|declared| {
                    if optional {
                        Type::Option {
                            element: Box::new(declared),
                            absence: "undefined".to_owned(),
                        }
                    } else {
                        declared
                    }
                }),
                span: Some(span(token, token)),
                guard: None,
                rocq_type: None,
            });
        }
        if let Some(doc) = &doc {
            for (documented, _) in &doc.params {
                if !params.iter().any(|param| param.name == *documented) {
                    return Err(type_error(
                        format!("@param {documented} is not a parameter of {name}"),
                        Some(doc.range),
                    ));
                }
            }
        }
        let ret =
            match doc.and_then(|doc| non_empty(doc.returns).map(|returns| (returns, doc.range))) {
                Some((returns, range)) => {
                    let promised = if is_async { promised(&returns) } else { None };
                    Some(self.ty(promised.unwrap_or(&returns), range)?)
                }
                None => None,
            };
        let names: HashSet<String> = params.iter().map(|param| param.name.clone()).collect();
        let outer = std::mem::replace(
            &mut self.scope,
            Scope {
                locals: names.clone(),
                tdz: HashSet::new(),
                mutable: names,
            },
        );
        let outer_async = std::mem::replace(&mut self.in_async, is_async);
        let outer_jumps = std::mem::take(&mut self.jumps);
        let outer_top_level = std::mem::replace(&mut self.top_level, false);
        if records.is_none() && !self.scalar_loop_model.is_empty() && self.is_scalar_loop() {
            let names = params
                .iter()
                .map(|param| param.name.clone())
                .collect::<Vec<_>>();
            let read = self.with_lexical_scope(&names, 2, |parser| {
                parser.read_scalar_loop(&name, &params, is_async)
            });
            self.scope = outer;
            self.in_async = outer_async;
            self.jumps = outer_jumps;
            self.top_level = outer_top_level;
            let (body, proof) = read?;
            return Ok(SFn {
                semantic_ret: None,
                name,
                params,
                ret,
                body,
                source_signature: None,
                source_primitive_owners: None,
                source_empty_option: None,
                source_type_namespace: None,
                source_loop: Some(Box::new(proof)),
                source_local_state: None,
                source_const_function: None,
                source_diagnostic_attributes: None,
                span: Some(self.read_complete_declaration_span(doc_token)?),
                generated: false,
                source_pure_unit: None,
            });
        }
        let names = params
            .iter()
            .map(|param| param.name.clone())
            .collect::<Vec<_>>();
        let indexed_writes_before = self.indexed_reference_writes;
        let statements = self.with_lexical_scope(&names, 2, |parser| {
            for parameter in &params {
                parser.lexical.parameter(&parameter.name);
                parser.lexical.tag_binding(
                    &parameter.name,
                    super::tag_context::TagReceiverFact::Declared(parameter.ty.clone()),
                );
            }
            match body {
                Body::Arrow if !parser.cursor.is("{") => {
                    let token = parser.peek();
                    let expr = if is_async {
                        parser.awaited(Self::expr)
                    } else {
                        parser.expr()
                    };
                    expr.map(|expr| {
                        vec![Stmt::Return {
                            expr,
                            span: parser.to_here(&token),
                        }]
                    })
                }
                Body::RecordParameters(records) => parser.read_named_record_parameter_body(records),
                Body::RecordArrowParameters(records) => {
                    parser.read_record_parameter_body_with(records, |parser| {
                        if parser.cursor.is("{") {
                            parser.block_statements()
                        } else {
                            let token = parser.peek();
                            let expr = parser.expr()?;
                            Ok(vec![Stmt::Return {
                                expr,
                                span: parser.to_here(&token),
                            }])
                        }
                    })
                }
                _ => parser.block_statements(),
            }
        });
        self.scope = outer;
        self.in_async = outer_async;
        self.jumps = outer_jumps;
        self.top_level = outer_top_level;
        let mut statements = statements?;
        super::callables::validate_closure_captures(&statements, self.reference_state.enabled)?;
        // A forced reference context keeps the actual heap operations.
        if !crate::translation::surface_raw_metadata_rules::select_surface_reference_semantics(
            false,
            self.reference_state.forced,
            true,
        ) {
            if self.indexed_reference_writes != indexed_writes_before {
                self.eliminated_owned_array_append_operations.extend(
                    super::constructed_guarded_array_writes::lower_constructed_guarded_array_writes(
                        &mut statements,
                        &params,
                        &mut self.generated_count,
                    ),
                );
            }
            self.eliminated_owned_array_append_operations.extend(
                super::owned_array_append::lower_owned_array_appends(
                    &mut statements,
                    &params,
                    &self.owned_array_readers,
                    &self.owned_array_iteration_sources,
                ),
            );
            if self.indexed_reference_writes != indexed_writes_before {
                self.eliminated_owned_array_append_operations.extend(
                    super::guarded_private_array_writes::lower_guarded_private_array_writes(
                        &mut statements,
                        &params,
                        &mut self.generated_count,
                    ),
                );
            }
        }
        // `if (n < 0n) throw …` as a leading statement makes `n` a natural number,
        // unless the body assigns it another value.
        let assigned = statement_uses(&statements).assigned;
        let mut index = 0;
        while index < statements.len() {
            let Some(param) = guarded_parameter(&statements[index], &params) else {
                break;
            };
            if assigned.contains(&params[param].name) {
                break;
            }
            params[param].ty = Some(NAT);
            // A negative argument aborts with the guard's message, the guards in their order.
            let Stmt::If { then, .. } = &statements[index] else {
                unreachable!("a guard is an if statement")
            };
            let message = then.iter().find_map(|inner| match inner {
                Stmt::Throw { message, .. } => Some(message.clone()),
                _ => None,
            });
            params[param].guard = Some(Guard {
                message: message.expect("a guard throws"),
                order: index,
            });
            index += 1;
        }
        let place = self.read_complete_declaration_span(name_token)?;
        let tail = &statements[index..];
        let body = if imperative(tail) {
            lower_imperative(self, &name, &params, tail, place)?
        } else {
            lower(
                tail,
                place,
                &mut self.generated_count,
                &self.generated_prefix,
            )?
        };
        let declaration = SFn {
            semantic_ret: None,
            name,
            params,
            ret,
            source_signature: None,
            source_primitive_owners: None,
            source_empty_option: None,
            source_type_namespace: None,
            source_local_state: None,
            source_const_function: None,
            source_diagnostic_attributes: None,
            source_loop: None,
            body,
            span: Some(self.read_complete_declaration_span(doc_token)?),
            generated: false,
            source_pure_unit: None,
        };
        if self.owned_array_reader_namespace == 0 {
            self.owned_array_readers.insert(
                declaration.name.clone(),
                super::owned_array_append::read_owned_array_reader_parameters(
                    &declaration,
                    &self.owned_array_readers,
                    &statements,
                ),
            );
        }
        Ok(declaration)
    }

    /// `const Name = { method(…) { … }, Nested: { … } };` is a module.
    pub(super) fn namespace(&mut self, start: &Token) -> Result<SModule> {
        self.cursor.expect("const", Some("namespace"))?;
        let name = self.cursor.identifier(Some("namespace"))?;
        global(&name)?;
        self.lexical.declaration(&name.value, name.span(), 1);
        self.cursor.expect("=", Some("namespace"))?;
        self.scope.tdz.remove(&name.value);
        let module = self.namespace_body(name.value, start)?;
        self.cursor.eat(";");
        Ok(module)
    }

    pub(super) fn namespace_body(&mut self, name: String, start: &Token) -> Result<SModule> {
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
            if !accept_object_property_definition(&member.value, false, self.cursor.is(":")) {
                return Err(unsupported(
                    "object prototype definition",
                    "an uncomputed __proto__: entry is prototype-definition syntax rather than an own data property; the portable core does not represent object prototypes",
                    Some(span(&member, &member)),
                ));
            }
            if super::surface_rules::accept_java_script_namespace_method(
                "identifier",
                &self.cursor.peek().value,
            ) {
                self.function_names
                    .insert(format!("{name}.{}", member.value));
                self.owned_array_reader_namespace += 1;
                let function = self.function_rest(&member, &member, false, false);
                self.owned_array_reader_namespace -= 1;
                items.push(SItem::Fn(function?));
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

    /// Names a block declares with `const` or `let`, which are in their temporal dead zone until declared.
    pub(super) fn block_declarations(&mut self, from: usize, _end: ScanEnd) -> HashSet<String> {
        self.block_declaration_counts(from).0
    }

    fn block_declaration_counts(
        &mut self,
        from: usize,
    ) -> (HashSet<String>, HashMap<String, usize>) {
        let mut names = HashSet::new();
        let mut counts = HashMap::new();
        let mut collect = |name: String| {
            names.insert(name.clone());
            *counts.entry(name).or_insert(0) += 1;
        };
        let saved = self.cursor.index;
        self.cursor.index = from;
        // Signed Number depth retains the shared parser's malformed-delimiter
        // behavior without unsigned underflow. No token columns are copied.
        let mut depth = 0.0;
        let mut declaration = false;
        loop {
            let token = self.cursor.advance();
            let kind = match token.kind {
                TokenKind::Punct => "punct",
                TokenKind::Identifier => "identifier",
                TokenKind::Eof => "eof",
                _ => "other",
            };
            let step = crate::translation::frontend_rules::ml_array::number_index(
                read_java_script_lexical_declaration_step(kind, &token.value, depth, declaration),
            );
            match step {
                None => break,
                Some(1) => depth += 1.0,
                Some(2) => depth -= 1.0,
                Some(3 | 4) => declaration = true,
                Some(5) => declaration = false,
                _ => {}
            }
            if !matches!(step, Some(3 | 4)) {
                continue;
            }
            let binding_start = self.cursor.index;
            let c = &self.cursor;
            if c.peek().kind == TokenKind::Identifier {
                collect(c.peek().value.clone());
            } else if c.is("{") {
                for token in self.object_binding_names(c.index) {
                    collect(token.value);
                }
            } else if c.is("[") {
                for token in self.array_binding_names(c.index) {
                    collect(token.value);
                }
            }
            self.cursor.index = binding_start;
        }
        self.cursor.index = saved;
        (names, counts)
    }

    pub(super) fn declare_local(&mut self, name: &str, mutable: bool) {
        self.lexical.binding(name, None);
        self.scope.locals.insert(name.to_owned());
        self.scope.tdz.remove(name);
        if mutable {
            self.scope.mutable.insert(name.to_owned());
        } else {
            self.scope.mutable.remove(name);
        }
    }

    pub(super) fn block_statements(&mut self) -> Result<Vec<Stmt>> {
        self.cursor.expect("{", Some("block"))?;
        let mut tdz = self.scope.tdz.clone();
        let (names, counts) = self.block_declaration_counts(self.cursor.index);
        let names = names.into_iter().collect::<Vec<_>>();
        tdz.extend(names.iter().cloned());
        let inner = Scope {
            locals: self.scope.locals.clone(),
            tdz,
            mutable: self.scope.mutable.clone(),
        };
        let outer = std::mem::replace(&mut self.scope, inner);
        let statements = self.with_lexical_scope(&names, 0, |parser| {
            parser.lexical.declaration_counts(counts);
            parser.block_body()
        });
        self.scope = outer;
        statements
    }

    pub(super) fn block_body(&mut self) -> Result<Vec<Stmt>> {
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
}
