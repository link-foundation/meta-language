//! Source adapters around the existing Rust parser and shared owned state policy.
use super::owned_local_state_rules::accept_rust_owned_local_source_operation;
use super::owned_tuple_call_rules::accept_rust_owned_tuple_call_model;
use super::{
    BOOL, BinaryOp, HashMap, HashSet, Options, Result, RustParser, SExpr, SItem, SMain, SNode,
    SParam, STRING, SourceOrigin, Span, Token, TokenKind, Type, unsupported,
};
pub(super) use crate::translation::owned_local_state::owned_local_pair as owned_pair;
use crate::translation::owned_local_state::{
    owned_local_position, owned_local_type, owned_tuple_call_position, policy, tuple_call_policy,
};
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
    calls: bool,
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
            && (token.value == "*" && !accept_rust_owned_local_source_operation("dereference")
                || self.local_value_state_model == policy()["models"]["copies"]
                    && ["&", "&&"].contains(&token.value.as_str())
                    && !accept_rust_owned_local_source_operation("borrow"))
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
                .map_or_else(|| range.unwrap().start, |span| span.start),
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
        let calls = accept_rust_owned_tuple_call_model(&self.local_value_state_model);
        let helper = calls && owned_pair(ret);
        let parameter_pair = Type::Tuple {
            elements: params
                .iter()
                .filter_map(|parameter| parameter.ty.clone())
                .collect(),
        };
        if !path.is_empty()
            || (*ret != STRING && !helper)
            || !signature.lifetime_bindings.is_empty()
            || signature
                .parameters
                .iter()
                .any(|position| !owned_local_position(position))
            || if helper {
                !owned_tuple_call_position(&signature.result)
                    || params.len() != 2
                    || !owned_pair(&parameter_pair)
            } else {
                !owned_local_position(&signature.result)
            }
        {
            return Err(fail(
                "only direct standard String, i32 and Boolean interfaces with a String result are admitted",
                None,
            ));
        }
        let mut state = LocalState {
            copies: self.local_value_state_model == policy()["models"]["copies"],
            calls,
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
        let body = if state.copies || helper {
            let body = self.block(path)?;
            state.expression(self, body, &mut environment, false)?
        } else {
            let syntax = self.owned_local_syntax_block(path, &mut state)?;
            state.sequence(self, &syntax, &mut environment, None)?
        };
        if !state.copies && !helper && (state.mutable_slot.is_none() || state.writes == 0) {
            return Err(fail(
                "the original move profile requires an actual tuple-owned String replacement",
                None,
            ));
        }
        if body.ty != *ret {
            return Err(fail(
                "the selected function must return its actual String value",
                None,
            ));
        }
        if helper
            && (!matches!(body.value.node, SNode::Tuple { .. })
                || !state.bindings.is_empty()
                || !state.initializers.is_empty())
        {
            return Err(fail(
                "a tuple helper must retain one direct ordered product result without local state or calls",
                None,
            ));
        }
        let mut interpretation = policy()["interpretation"].clone();
        if calls {
            interpretation["tupleInitialization"] = tuple_call_policy()["interpretation"].clone();
        }
        let proof = json!({"model": self.local_value_state_model, "language":"Rust", "sourceName": name,
            "interpretation": interpretation, "bindings":state.bindings, "reads":state.reads,
            "initializers":state.initializers, "literalConstructors":state.constructors});
        Ok((body.value, proof))
    }
}
