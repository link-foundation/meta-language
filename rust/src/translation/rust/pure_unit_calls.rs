//! Thin source adapters for the shared automatically translated Unit policies.
use super::pure_unit_call_rules::{
    accept_rust_pure_unit_context, accept_rust_pure_unit_name,
    accept_rust_pure_unit_name_character, accept_rust_pure_unit_visibility,
};
use super::{
    Result, RustParser, SEffect, SExpr, SFn, SItem, SMain, SNode, SourceDeclaration, SourceOrigin,
    Token, UNIT, unsupported,
};
use crate::translation::Span;
use serde_json::{Value, json};

fn source_failure(
    detail: &str,
    site: Option<Span>,
) -> crate::translation::diagnostics::TranslationError {
    unsupported("Rust pure Unit-call source", detail, site)
}
fn ordinary_name(name: &str) -> bool {
    accept_rust_pure_unit_name(name)
        && name.chars().enumerate().all(|(index, character)| {
            accept_rust_pure_unit_name_character(f64::from(u32::from(character)), index == 0)
        })
}
const fn actual_span(start: &Token, end: &Token) -> Span {
    Span {
        start: start.start,
        end: end.end,
    }
}
impl RustParser {
    pub(super) fn pure_unit_attribute_guard(&self) -> Result<()> {
        if !self.pure_unit_call_model.is_empty() && self.cursor.is("#") {
            return Err(source_failure(
                "attributes require a separate compilation context proof",
                Some(actual_span(self.cursor.peek(), self.cursor.peek())),
            ));
        }
        Ok(())
    }
    pub(super) fn pure_unit_item_context(&mut self, path: &[String], start: &Token) -> Result<()> {
        self.pure_unit_public_token = None;
        if self.pure_unit_call_model.is_empty() {
            return Ok(());
        }
        let policy = crate::translation::source_pure_unit_calls::policy();
        if accept_rust_pure_unit_visibility(
            &self.pure_unit_call_model,
            policy["visibilityForms"]["private"].as_str().unwrap(),
        ) {
            if !path.is_empty() || !self.cursor.is("fn") {
                return Err(source_failure(
                    "only flat private pure Unit functions and one ordinary main are admitted",
                    Some(actual_span(start, self.cursor.peek())),
                ));
            }
            return Ok(());
        }
        let public_token = self.cursor.eat("pub");
        if !path.is_empty() || !self.cursor.is("fn") {
            return Err(source_failure(
                "only flat bare public pure Unit helpers and one ordinary private main are admitted",
                Some(actual_span(start, self.cursor.peek())),
            ));
        }
        let main = self.cursor.is_at("main", 1);
        if main && public_token.is_some() {
            return Err(source_failure(
                "the complete source requires an ordinary private main",
                Some(actual_span(start, self.cursor.peek())),
            ));
        }
        if !main && public_token.is_none() {
            return Err(source_failure(
                "each helper requires its actual bare pub keyword and complete owned Unit header",
                Some(actual_span(start, self.cursor.peek())),
            ));
        }
        self.pure_unit_public_token = public_token;
        Ok(())
    }
    fn pure_unit_name(&mut self, context: &str) -> Result<Token> {
        let token = self.cursor.identifier(Some(context))?;
        if !ordinary_name(&token.value) {
            return Err(source_failure(
                "the selected name requires an ordinary unreserved ASCII Rust identifier",
                Some(token.span()),
            ));
        }
        Ok(token)
    }
    fn pure_unit_token_pair(&mut self, context: &str) -> Result<()> {
        self.cursor.expect("(", Some(context))?;
        self.cursor.expect(")", Some(context))?;
        Ok(())
    }
    pub(super) fn pure_unit_function(&mut self, path: &[String]) -> Result<SFn> {
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("fn"))?.value;
        let first = self
            .pure_unit_public_token
            .clone()
            .unwrap_or_else(|| start.clone());
        if !path.is_empty() || !ordinary_name(&name) {
            return Err(source_failure(
                "pure Unit functions require an ordinary flat owned name",
                Some(actual_span(&start, self.cursor.peek())),
            ));
        }
        self.pure_unit_token_pair("zero-argument pure Unit function")?;
        let header_form = if self.cursor.eat("->").is_some() {
            self.pure_unit_token_pair("direct Unit result")?;
            "explicit unit"
        } else {
            "implicit unit"
        };
        let header_span = actual_span(&first, &self.cursor.tokens[self.cursor.index - 1]);
        let open = self.cursor.expect("{", Some("pure Unit function"))?;
        let body_form = if self.cursor.is("}") {
            "empty block"
        } else {
            self.pure_unit_token_pair("pure Unit body")?;
            "unit expression"
        };
        if !self.cursor.is("}") {
            return Err(source_failure(
                "a helper body must be exactly an empty block or the Unit expression",
                Some(actual_span(&open, self.cursor.peek())),
            ));
        }
        let close = self.cursor.advance();
        let origin = SourceOrigin::with_type(UNIT, path, Vec::new());
        self.origins.declaration(
            &name,
            path,
            SourceDeclaration::Function {
                parameters: Vec::new(),
                result: origin.clone(),
            },
        )?;
        let body = SExpr::new(SNode::Unit, None);
        self.origins.mark(&body, origin);
        let mut proof = json!({"model":self.pure_unit_call_model,"language":"Rust","headerForm":header_form,"bodyForm":body_form,"headerSpan":header_span,"bodySpan":actual_span(&open,&close)});
        if let Some(public_token) = &self.pure_unit_public_token {
            proof["visibility"] = json!({"form":"public","span":public_token.span()});
        }
        Ok(SFn {
            name,
            params: Vec::new(),
            ret: Some(UNIT),
            body,
            span: Some(actual_span(&first, &close)),
            generated: false,
            source_signature: None,
            source_loop: None,
            source_primitive_owners: None,
            source_empty_option: None,
            source_type_namespace: None,
            source_pure_unit: Some(Box::new(proof)),
            source_local_state: None,
            source_const_function: None,
        })
    }
    pub(super) fn pure_unit_main(&mut self) -> Result<SMain> {
        let start = self.cursor.expect("fn", Some("pure Unit main"))?;
        self.cursor.expect("main", Some("pure Unit main"))?;
        self.pure_unit_token_pair("pure Unit main")?;
        if self.cursor.is("->") {
            return Err(source_failure(
                "main must retain its ordinary implicit Unit header",
                Some(actual_span(&start, self.cursor.peek())),
            ));
        }
        self.cursor.expect("{", Some("pure Unit main"))?;
        let mut effects = Vec::new();
        while !self.cursor.is("}") {
            let token = self.peek();
            let first = self.cursor.identifier(Some("owned pure Unit call"))?;
            let (target, path_form) = if first.value == "crate" {
                self.cursor.expect("::", Some("owned pure Unit call"))?;
                (
                    self.pure_unit_name("owned pure Unit call")?.value,
                    "absolute crate path",
                )
            } else {
                if !ordinary_name(&first.value) {
                    return Err(source_failure(
                        "main calls require an ordinary owned pure Unit name",
                        Some(actual_span(&token, &first)),
                    ));
                }
                (first.value, "unqualified name")
            };
            self.pure_unit_token_pair("zero-argument pure Unit call")?;
            let end = self.cursor.expect(";", Some("pure Unit call statement"))?;
            let policy = crate::translation::source_pure_unit_calls::policy();
            let base = policy["temporaryName"].as_str().unwrap();
            let mut name = base.to_owned();
            let mut suffix = 0;
            while self.reserved_binding_names.contains(&name) {
                suffix += 1;
                name = format!("{base}{suffix}");
            }
            self.reserved_binding_names.insert(name.clone());
            let site = actual_span(&token, &end);
            let value = SExpr::new(
                SNode::App {
                    func: Box::new(SExpr::new(
                        SNode::Name {
                            path: vec!["crate".to_owned(), target],
                        },
                        Some(site),
                    )),
                    args: Vec::new(),
                },
                Some(site),
            );
            effects.push(SEffect::Let{name,ty:Some(UNIT),value,constant:false,span:Some(site),source_pure_unit_call:Some(json!({"model":self.pure_unit_call_model,"language":"Rust","pathForm":path_form,"span":site}))});
        }
        let close = self.cursor.advance();
        Ok(SMain {
            effects,
            span: Some(actual_span(&start, &close)),
            sequential_async: false,
            source_runtime_worker: None,
        })
    }
    pub(super) fn finish_pure_unit_source(
        &self,
        items: &[SItem],
        main: &SMain,
    ) -> Result<Option<Value>> {
        if self.pure_unit_call_model.is_empty() {
            return Ok(None);
        }
        if !accept_rust_pure_unit_context(
            "Rust",
            items.iter().any(|item| !matches!(item, SItem::Fn(_))),
        ) {
            return Err(source_failure(
                "the selected interpretation requires a complete flat pure Unit program",
                main.span,
            ));
        }
        Ok(Some(
            json!({"model":self.pure_unit_call_model,"language":"Rust"}),
        ))
    }
}
