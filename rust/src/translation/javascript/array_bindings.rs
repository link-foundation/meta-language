//! Token and scope transport for the complete generated array binding plans.
use std::collections::BTreeMap;
use std::sync::OnceLock;

use super::{
    HashSet, JavaScriptParser, Result, SExpr, SNode, Span, Stmt, Token, TokenKind, node,
    unsupported,
};
use crate::translation::array_binding_rules::{self as rules, ml_string::Text};
use crate::translation::frontend_rules::ml_array::number_index;

pub(super) struct ArrayBinding {
    pub(super) name: String,
    path: Vec<usize>,
    span: Span,
    rest: bool,
    default_value: Option<SExpr>,
}

fn reserved_names() -> &'static [Text] {
    static NAMES: OnceLock<Vec<Text>> = OnceLock::new();
    NAMES.get_or_init(|| {
        let policy =
            crate::read_structured_data_lino(include_str!("../../data/array-bindings.lino"))
                .expect("the generated array binding policy is valid project data");
        policy["reservedBindings"]
            .as_array()
            .expect("reserved bindings are ordered names")
            .iter()
            .map(|value| Text::from_text(value.as_str().expect("a reserved binding is text")))
            .collect()
    })
}

const fn kind(token: &Token) -> &'static str {
    match token.kind {
        TokenKind::Punct => "punct",
        TokenKind::Identifier => "identifier",
        TokenKind::Eof => "eof",
        _ => "other",
    }
}

impl JavaScriptParser {
    pub(super) fn array_binding_names(&self, start: usize) -> Vec<Token> {
        let kinds: Vec<Text> = self
            .cursor
            .tokens
            .iter()
            .map(|token| Text::from_text(kind(token)))
            .collect();
        let values: Vec<Text> = self
            .cursor
            .tokens
            .iter()
            .map(|token| Text::from_text(&token.value))
            .collect();
        let position = serde_json::Number::from(start)
            .as_f64()
            .expect("a source token position is a Number");
        let plan = rules::plan_java_script_array_binding_names(&kinds, &values, position);
        if plan.first().is_none_or(|value| *value < 0.0) {
            return Vec::new();
        }
        plan.iter()
            .skip(1)
            .map(|value| {
                self.cursor.tokens
                    [number_index(*value).expect("an actual planned binding position")]
                .clone()
            })
            .collect()
    }

    pub(super) fn read_array_binding(&mut self) -> Result<Vec<ArrayBinding>> {
        let mut names = HashSet::new();
        for token in self.array_binding_names(self.cursor.index) {
            if !rules::accept_java_script_array_binding_name(
                &Text::from_text(kind(&token)),
                &Text::from_text(&token.value),
                names.contains(&token.value),
                reserved_names(),
            ) {
                return Err(unsupported(
                    "array binding name",
                    "every binding needs a distinct admitted lexical identifier",
                    Some(token.span()),
                ));
            }
            names.insert(token.value);
        }
        let outer = self.scope.clone();
        self.scope.tdz.extend(names);
        let result = self.read_array_binding_path(&[], &mut HashSet::new());
        self.scope = outer;
        result
    }

    fn read_array_binding_path(
        &mut self,
        path: &[usize],
        seen: &mut HashSet<String>,
    ) -> Result<Vec<ArrayBinding>> {
        self.cursor.expect("[", Some("array binding"))?;
        let mut bindings = Vec::new();
        let mut index = 0;
        while !self.cursor.is("]") {
            if self.cursor.eat(",").is_some() {
                index += 1;
                continue;
            }
            let rest = self.cursor.eat("...").is_some();
            if rest && !path.is_empty() {
                return Err(unsupported(
                    "nested array rest binding",
                    "a nested rest binding needs its product iterator proof",
                    Some(self.peek().span()),
                ));
            }
            let mut position = path.to_vec();
            position.push(index);
            if self.cursor.is("[") {
                if rest {
                    return Err(unsupported(
                        "array rest pattern",
                        "the selected rest binding needs a lexical name",
                        Some(self.peek().span()),
                    ));
                }
                bindings.extend(self.read_array_binding_path(&position, seen)?);
            } else {
                let token = self.cursor.identifier(Some("array binding"))?;
                if !rules::accept_java_script_array_binding_name(
                    &Text::from_text(kind(&token)),
                    &Text::from_text(&token.value),
                    seen.contains(&token.value),
                    reserved_names(),
                ) {
                    return Err(unsupported(
                        "array binding name",
                        "every binding needs a distinct admitted lexical identifier",
                        Some(token.span()),
                    ));
                }
                seen.insert(token.value.clone());
                let default_value = if rest {
                    let reason = rules::read_java_script_array_rest_binding_error(
                        &Text::from_text(&self.peek().value),
                    )
                    .to_string();
                    if !reason.is_empty() {
                        return Err(unsupported(
                            "array rest binding",
                            &reason,
                            Some(self.peek().span()),
                        ));
                    }
                    None
                } else if self.cursor.eat("=").is_some() {
                    if !path.is_empty() {
                        return Err(unsupported(
                            "nested array binding default",
                            "a nested default needs its product iterator proof",
                            Some(token.span()),
                        ));
                    }
                    Some(self.expr()?)
                } else {
                    None
                };
                self.declare_local(&token.value, false);
                let place = token.span();
                bindings.push(ArrayBinding {
                    name: token.value,
                    path: position,
                    span: place,
                    rest,
                    default_value,
                });
            }
            index += 1;
            if rest || self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("]", Some("array binding"))?;
        Ok(bindings)
    }

    pub(super) fn array_binding_initializer(&mut self, bindings: &[ArrayBinding]) -> Result<SExpr> {
        let outer = self.scope.clone();
        self.scope
            .tdz
            .extend(bindings.iter().map(|binding| binding.name.clone()));
        let result = self.expr();
        self.scope = outer;
        result
    }

    pub(super) fn bind_array_elements(
        &mut self,
        bindings: Vec<ArrayBinding>,
        object: &SExpr,
        mutable: bool,
        whole: Span,
    ) -> Vec<Stmt> {
        bindings.into_iter().map(|binding| {
            let mut value = object.clone();
            for &index in &binding.path {
                value = node(SNode::Index {
                    object: Box::new(value),
                    index: Box::new(node(SNode::Num { value: index.to_string(), ty: None, negative: false, unit: false }, binding.span)),
                    optional: None, proven_in_bounds: None,
                }, binding.span);
            }
            if binding.path.len() > 1 { self.nested_product_bindings.insert(binding.span); }
            let form = rules::read_java_script_array_binding_value_form(binding.rest, binding.default_value.is_some()).to_string();
            value = match form.as_str() {
                "suffix" => node(SNode::ArrayBindingRest {
                    object: Box::new(object.clone()),
                    start: serde_json::json!({"k":"num","value":binding.path[0].to_string(),"span":binding.span}),
                    extra: BTreeMap::default(),
                }, binding.span),
                "default" => node(SNode::ArrayBindingDefault {
                    value: Box::new(value), fallback: Box::new(binding.default_value.expect("the shared form selected a present default")), extra: BTreeMap::default(),
                }, binding.span),
                _ => value,
            };
            if mutable { Stmt::Let { name: binding.name, value, span: whole } }
            else { Stmt::Const { name: binding.name, value, span: whole } }
        }).collect()
    }

    pub(super) fn array_destructuring(&mut self, start: &Token) -> Result<Stmt> {
        let bindings = self.read_array_binding()?;
        if bindings.is_empty() {
            return Err(unsupported(
                "empty array binding",
                "the iterator and destructibility need a checked representation",
                Some(self.to_here(start)),
            ));
        }
        self.cursor.expect("=", Some("array binding"))?;
        let value = self.array_binding_initializer(&bindings)?;
        let whole = self.to_here(start);
        self.generated_count += 1;
        let name = format!("ml_binding{}", self.generated_count);
        let object = node(
            SNode::Name {
                path: vec![name.clone()],
            },
            whole,
        );
        for binding in &bindings {
            self.declare_local(&binding.name, false);
        }
        let mut body = vec![Stmt::Const {
            name,
            value,
            span: whole,
        }];
        body.extend(self.bind_array_elements(bindings, &object, false, whole));
        Ok(Stmt::Block {
            body,
            inline: true,
            span: whole,
        })
    }
}
