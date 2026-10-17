//! Thin source ownership and emitter adapters for the linked const predicate policy.
use super::{Result, RustParser, SItem, SMain, Token, span, unsupported};
use crate::decorators::{
    Decorator, DecoratorAction, DecoratorLevel, DecoratorRecord, DecoratorSet,
};
use crate::translation::ir::FnDecl;
use crate::translation::source_const_functions::{policy, policy_for_model};
use serde_json::{Value, json};
use std::sync::OnceLock;

impl RustParser {
    pub(super) fn const_function_visibility_guard(&self) -> Result<()> {
        if !self.const_function_model.is_empty()
            && self.cursor.is("pub")
            && self.cursor.is_at("(", 1)
        {
            return Err(unsupported(
                "Rust const function visibility",
                "restricted visibility requires an independent module proof",
                None,
            ));
        }
        Ok(())
    }

    pub(super) fn const_function_item_guard(&self, path: &[String], start: &Token) -> Result<()> {
        let cursor = &self.cursor;
        if !(self.const_function_model.is_empty()
            || cursor.is("const") && cursor.is_at("fn", 1)
            || path.is_empty() && cursor.is("fn") && cursor.is_at("main", 1))
        {
            return Err(unsupported(
                "Rust const function context",
                "only actual const predicate declarations and the ordinary main are admitted",
                span(start, cursor.peek()),
            ));
        }
        Ok(())
    }

    pub(super) fn require_const_function_program(
        &self,
        items: &[SItem],
        main: &SMain,
    ) -> Result<()> {
        if !self.const_function_model.is_empty() && (items.is_empty() || items.iter().any(|item| !matches!(item, SItem::Fn(function) if function.source_const_function.is_some())) || !main.effects.is_empty()) {
            return Err(unsupported("Rust const function source", "only closed flat const predicates and an empty ordinary main are admitted", None));
        }
        Ok(())
    }

    pub(super) fn create_const_function_proof(
        &self,
        name: &str,
        keyword: &Token,
        function_keyword: &Token,
        header_end: usize,
    ) -> Value {
        let body_end = self.cursor.tokens[self.cursor.index - 1].end;
        let selected = policy_for_model(&self.const_function_model);
        json!({"model":self.const_function_model,"language":"Rust","sourceName":name,"keyword":keyword.value,
            "keywordSpan":{"start":keyword.start,"end":keyword.end},
            "functionKeywordSpan":{"start":function_keyword.start,"end":function_keyword.end},
            "headerSpan":{"start":keyword.start,"end":header_end},
            "bodySpan":{"start":header_end,"end":body_end},"interpretation":selected["interpretation"]})
    }
}

#[must_use]
pub fn render_const_function_header(function: &FnDecl, source: String) -> String {
    static DECORATORS: OnceLock<DecoratorSet> = OnceLock::new();
    if function.source_const_function.is_none() {
        return source;
    }
    let decorators = DECORATORS.get_or_init(|| {
        let emission = &policy()["emission"];
        DecoratorSet::new(vec![
            Decorator::new(
                "preserve-owned-rust-const-function-keyword",
                DecoratorLevel::Emitter,
                0,
                vec![("format".into(), "Rust".into())],
                vec![DecoratorAction::Replace {
                    field: "source".into(),
                    from: emission["from"]
                        .as_str()
                        .expect("a shared header prefix")
                        .into(),
                    to: emission["to"]
                        .as_str()
                        .expect("a shared const header prefix")
                        .into(),
                }],
            )
            .expect("the shared const source header decorator is valid"),
        ])
        .expect("the shared const decorator set is valid")
    });
    decorators
        .decorate(
            DecoratorLevel::Emitter,
            &DecoratorRecord::from([("format".into(), "Rust".into()), ("source".into(), source)]),
        )
        .expect("a const header decoration retains its record")
        .remove("source")
        .expect("a decorated const header has source text")
}
