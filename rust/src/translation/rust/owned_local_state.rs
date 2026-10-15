//! Source adapters around the existing Rust parser and shared owned state policy.
use super::{
    BOOL, BinaryOp, HashMap, HashSet, Options, Result, RustParser, SExpr, SItem, SMain, SNode,
    SParam, STRING, SourceOrigin, Span, Token, TokenKind, Type, span, unsupported,
};
pub(super) use crate::translation::owned_local_state::owned_local_pair as owned_pair;
use crate::translation::owned_local_state::{owned_local_position, owned_local_type, policy};
use crate::translation::source_signatures::SourceSignature;
use serde_json::{Value, json};

mod expressions;
mod lowering;
mod syntax;

fn fail(reason: &str, span: Option<Span>) -> crate::translation::diagnostics::TranslationError {
    unsupported("Rust owned local state", reason, span)
}

#[derive(Clone)]
struct LocalValue {
    name: String,
    ty: Type,
    available: bool,
    mutable: bool,
    fields: Vec<Self>,
}
impl LocalValue {
    fn new(name: String, ty: Type, mutable: bool) -> Self {
        let fields = if owned_pair(&ty) {
            vec![Self::new(format!("{name}.0"), STRING, false)]
        } else {
            Vec::new()
        };
        Self {
            name,
            ty,
            available: true,
            mutable,
            fields,
        }
    }
}
type Environment = HashMap<String, LocalValue>;
struct LocalExpression {
    value: SExpr,
    ty: Type,
}
struct LocalPattern {
    name: String,
    mutable: bool,
    span: Option<Span>,
}
enum LocalBindingPattern {
    Name(LocalPattern),
    Tuple {
        items: Vec<LocalPattern>,
        span: Option<Span>,
    },
}
enum Statement {
    Let {
        pattern: LocalBindingPattern,
        ty: Option<Type>,
        value: SExpr,
        span: Option<Span>,
    },
    Write {
        target: String,
        value: SExpr,
        span: Option<Span>,
    },
    If {
        condition: SExpr,
        then: Vec<Self>,
        otherwise: Vec<Self>,
        span: Option<Span>,
    },
    Tail(SExpr),
}
struct LocalState {
    copies: bool,
    names: HashSet<String>,
    temporary: usize,
    mutable_slot: Option<String>,
    writes: usize,
    bindings: Vec<Value>,
    reads: Vec<Value>,
    initializers: Vec<Value>,
    constructors: Vec<Value>,
}
impl LocalState {
    fn fresh(&mut self, parser: &mut RustParser) -> String {
        loop {
            let name = format!("ml_owned_local_value{}", self.temporary);
            self.temporary += 1;
            if parser.reserved_binding_names.insert(name.clone()) {
                return name;
            }
        }
    }
    fn bind(&mut self, mut value: Value) {
        value["path"] = Value::Null;
        self.bindings.push(value);
    }
    fn name(&mut self, name: &str, span: Option<Span>) -> Result<()> {
        if reserved(name) || !self.names.insert(name.to_owned()) {
            return Err(fail(
                "source bindings cannot shadow each other or standard owners",
                span,
            ));
        }
        Ok(())
    }
}
fn reserved(name: &str) -> bool {
    policy()["reservedOwners"]
        .as_array()
        .unwrap()
        .iter()
        .any(|value| value == name)
}
fn merge_availability(environment: &mut Environment, then: &Environment, otherwise: &Environment) {
    for (name, value) in environment {
        value.available &= then[name].available && otherwise[name].available;
        for (index, field) in value.fields.iter_mut().enumerate() {
            field.available &=
                then[name].fields[index].available && otherwise[name].fields[index].available;
        }
    }
}
fn binding_expression(
    name: String,
    value: LocalExpression,
    body: LocalExpression,
    span: Option<Span>,
) -> LocalExpression {
    let ty = body.ty;
    LocalExpression {
        value: SExpr::new(
            SNode::Let {
                name,
                ty: Some(value.ty),
                value: Box::new(value.value),
                body: Box::new(body.value),
            },
            span,
        ),
        ty,
    }
}
impl RustParser {
    pub(super) fn require_owned_local_indirection(&self, token: &Token) -> Result<()> {
        if !self.local_value_state_model.is_empty()
            && (token.value == "*"
                || self.local_value_state_model == policy()["models"]["copies"]
                    && ["&", "&&"].contains(&token.value.as_str()))
        {
            return Err(fail(
                "source indirection requires a separate ownership interpretation",
                Some(token.span()),
            ));
        }
        Ok(())
    }
    pub(super) fn read_owned_local_copy(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        arguments: usize,
        origin: &SourceOrigin,
        range: Option<Span>,
    ) -> Result<Option<SExpr>> {
        if self.local_value_state_model.is_empty() {
            return Ok(None);
        }
        if method.value == "as_str" {
            if arguments != 0 {
                return Err(super::type_error(
                    format!("as_str expects 0 arguments but got {arguments}"),
                    range,
                ));
            }
            return Err(fail(
                "source indirection requires a separate ownership interpretation",
                range,
            ));
        }
        if !["clone", "to_owned"].contains(&method.value.as_str()) {
            return Ok(None);
        }
        if arguments != 0 {
            return Err(super::type_error(
                format!("{} expects 0 arguments but got {arguments}", method.value),
                range,
            ));
        }
        if self.local_value_state_model != policy()["models"]["copies"]
            || method.value != "clone"
            || !matches!(receiver.node, SNode::Name { .. } | SNode::TupleIndex { .. })
            || origin.ty != Some(STRING)
            || !origin.wrappers.is_empty()
            || origin.text_source_model != Some("owned string")
        {
            return Err(fail(
                "only actual standard String clone is admitted by the canonical copy interpretation",
                range,
            ));
        }
        let mut value = receiver.clone();
        value.span = Some(Span::new(
            receiver
                .span
                .map_or(range.unwrap().start, |span| span.start),
            range.unwrap().end,
        ));
        self.local_value_copies.insert(value.span.unwrap());
        self.origins.mark(&value, origin.clone());
        Ok(Some(value))
    }
    pub(super) fn require_owned_local_program(&self, items: &[SItem], main: &SMain) -> Result<()> {
        if !self.local_value_state_model.is_empty()
            && (items.is_empty() || items.iter().any(|item| !matches!(item, SItem::Fn(function) if function.source_local_state.is_some() && !reserved(&function.name)))
                || !main.effects.is_empty())
        {
            return Err(fail(
                "only flat functions with an empty main and unshadowed standard owners are admitted",
                None,
            ));
        }
        Ok(())
    }
    pub(super) fn read_owned_local_state_body(
        &mut self,
        path: &[String],
        name: &str,
        params: &[SParam],
        ret: &Type,
        signature: Option<&SourceSignature>,
    ) -> Result<(SExpr, Value)> {
        let Some(signature) = signature else {
            return Err(fail(
                "the selected value interface needs its actual source signature",
                None,
            ));
        };
        if !path.is_empty()
            || *ret != STRING
            || !signature.lifetime_bindings.is_empty()
            || signature
                .parameters
                .iter()
                .chain(std::iter::once(&signature.result))
                .any(|position| !owned_local_position(position))
        {
            return Err(fail(
                "only direct standard String, i32 and Boolean interfaces with a String result are admitted",
                None,
            ));
        }
        let mut state = LocalState {
            copies: self.local_value_state_model == policy()["models"]["copies"],
            names: HashSet::new(),
            temporary: 0,
            mutable_slot: None,
            writes: 0,
            bindings: Vec::new(),
            reads: Vec::new(),
            initializers: Vec::new(),
            constructors: Vec::new(),
        };
        for parameter in params {
            state.name(&parameter.name, None)?;
        }
        let mut environment: Environment = params
            .iter()
            .map(|parameter| {
                (
                    parameter.name.clone(),
                    LocalValue::new(
                        parameter.name.clone(),
                        parameter.ty.clone().expect("a Rust parameter is typed"),
                        false,
                    ),
                )
            })
            .collect();
        let body = if state.copies {
            let body = self.block(path)?;
            state.expression(self, body, &mut environment, false)?
        } else {
            let syntax = self.owned_local_syntax_block(path, &mut state)?;
            state.sequence(self, &syntax, &mut environment, None)?
        };
        if !state.copies && (state.mutable_slot.is_none() || state.writes == 0) {
            return Err(fail(
                "the original move profile requires an actual tuple-owned String replacement",
                None,
            ));
        }
        if body.ty != STRING {
            return Err(fail(
                "the selected function must return its actual String value",
                None,
            ));
        }
        let proof = json!({"model": self.local_value_state_model, "language":"Rust", "sourceName": name,
            "interpretation": policy()["interpretation"], "bindings":state.bindings, "reads":state.reads,
            "initializers":state.initializers, "literalConstructors":state.constructors});
        Ok((body.value, proof))
    }
}
