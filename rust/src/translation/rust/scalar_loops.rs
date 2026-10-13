//! Parser adapter for one owned scalar state and its actual ordered writes.
use super::{
    Options, Result, RustParser, SExpr, SNode, SParam, SourceOrigin, Type, span, unsupported,
};
use crate::translation::source_loops::{
    ScalarLoopState, ScalarLoopTransition, ScalarLoopWrite, SourceLoop, scalar_loop_policy,
};

struct LoopState {
    name: String,
    params: Vec<SParam>,
    ret: Type,
    mutable: Vec<String>,
    proof: SourceLoop,
    next_snapshot: usize,
}
impl RustParser {
    pub(super) fn read_scalar_loop(
        &mut self,
        path: &[String],
        name: &str,
        params: &[SParam],
        ret: &Type,
        mutable: &[String],
    ) -> Result<(SExpr, SourceLoop)> {
        self.cursor.expect("{", Some("scalar loop function"))?;
        self.cursor.expect("loop", Some("scalar loop"))?;
        let policy = scalar_loop_policy();
        let mut state = LoopState {
            name: name.into(),
            params: params.to_vec(),
            ret: ret.clone(),
            mutable: mutable.to_vec(),
            next_snapshot: 0,
            proof: SourceLoop {
                model: policy["model"].as_str().unwrap().into(),
                language: "Rust".into(),
                source_name: name.into(),
                lowering: policy["lowering"].as_str().unwrap().into(),
                state: params
                    .iter()
                    .map(|param| ScalarLoopState {
                        name: param.name.clone(),
                        mutable: mutable.contains(&param.name),
                    })
                    .collect(),
                transitions: Vec::new(),
                source_interface: None,
            },
        };
        let body = self.scalar_loop_block(path, &mut state, false)?;
        self.cursor.expect("}", Some("scalar loop function"))?;
        Ok((body, state.proof))
    }
    fn scalar_loop_result(&self, value: SExpr, path: &[String]) -> Result<SExpr> {
        if !self.origins.origin(&value, path).direct_copy_value() {
            return Err(unsupported(
                "Rust scalar loop",
                "loop results require direct scalar Copy ownership",
                value.span,
            ));
        }
        Ok(value)
    }
    fn scalar_loop_block(
        &mut self,
        path: &[String],
        state: &mut LoopState,
        value_allowed: bool,
    ) -> Result<SExpr> {
        self.cursor.expect("{", Some("scalar loop block"))?;
        let previous = self.origins.enter(None);
        let result = (|| {
            let mut bindings = Vec::new();
            while self.cursor.is("let") {
                let read = self.let_statement(path)?;
                if read.len() != 1
                    || state.params.iter().any(|param| param.name == read[0].name)
                    || !self
                        .origins
                        .origin(&read[0].value, path)
                        .direct_copy_value()
                {
                    return Err(unsupported(
                        "Rust scalar loop",
                        "loop bindings require distinct immutable scalar Copy values",
                        read.first().and_then(|binding| binding.span),
                    ));
                }
                bindings.extend(read);
            }
            let value = if self.cursor.eat("return").is_some() {
                self.scalar_loop_completion(path, state)?
            } else if self.cursor.is("{") {
                self.scalar_loop_block(path, state, value_allowed)?
            } else if self.cursor.is("if") {
                self.scalar_loop_conditional(path, state, value_allowed)?
            } else if self.cursor.is("(") || self.cursor.is("continue") {
                self.scalar_loop_continuation(path, state)?
            } else if value_allowed {
                let value = self.expr(path, Options::default())?;
                self.scalar_loop_result(value, path)?
            } else {
                return Err(unsupported(
                    "Rust scalar loop",
                    "a loop path needs an explicit return or scalar continuation",
                    Some(self.peek().span()),
                ));
            };
            self.cursor.eat(";");
            self.cursor
                .expect("}", Some("scalar loop branch completion"))?;
            let result = bindings.into_iter().rev().fold(value, |body, binding| {
                SExpr::new(
                    SNode::Let {
                        name: binding.name,
                        ty: binding.ty,
                        value: Box::new(binding.value),
                        body: Box::new(body),
                    },
                    binding.span,
                )
            });
            self.origins.mark(
                &result,
                SourceOrigin::with_type(state.ret.clone(), path, Vec::new()),
            );
            Ok(result)
        })();
        self.origins.scope = previous;
        result
    }
    fn scalar_loop_completion(&mut self, path: &[String], state: &mut LoopState) -> Result<SExpr> {
        if self.cursor.is("if") {
            self.scalar_loop_conditional(path, state, true)
        } else if self.cursor.is("{") {
            self.scalar_loop_block(path, state, true)
        } else {
            let value = self.expr(path, Options::default())?;
            self.scalar_loop_result(value, path)
        }
    }
    fn scalar_loop_conditional(
        &mut self,
        path: &[String],
        state: &mut LoopState,
        value_allowed: bool,
    ) -> Result<SExpr> {
        let start = self.cursor.expect("if", Some("scalar loop condition"))?;
        let cond = self.expr(
            path,
            Options {
                no_struct: true,
                ..Options::default()
            },
        )?;
        let then = self.scalar_loop_block(path, state, value_allowed)?;
        self.cursor.expect("else", Some("scalar loop branches"))?;
        let otherwise = if self.cursor.is("if") {
            self.scalar_loop_conditional(path, state, value_allowed)?
        } else {
            self.scalar_loop_block(path, state, value_allowed)?
        };
        Ok(SExpr::new(
            SNode::If {
                cond: Box::new(cond),
                then: Box::new(then),
                otherwise: Box::new(otherwise),
            },
            span(&start, self.cursor.peek()),
        ))
    }
    fn scalar_loop_continuation(
        &mut self,
        path: &[String],
        state: &mut LoopState,
    ) -> Result<SExpr> {
        let start = self.peek();
        let mut writes = Vec::new();
        let mut values = Vec::new();
        if self.cursor.eat("(").is_some() {
            while !self.cursor.is(")") {
                let target = self.cursor.identifier(Some("scalar state slot"))?.value;
                if !state.mutable.contains(&target)
                    || writes
                        .iter()
                        .any(|write: &ScalarLoopWrite| write.target == target)
                {
                    return Err(unsupported(
                        "Rust scalar loop",
                        "parallel writes require distinct mutable parameter slots",
                        Some(start.span()),
                    ));
                }
                let binding = loop {
                    let name = format!("ml_scalar_write{}", state.next_snapshot);
                    state.next_snapshot += 1;
                    if self.reserved_binding_names.insert(name.clone()) {
                        break name;
                    }
                };
                writes.push(ScalarLoopWrite { target, binding });
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect(")", Some("parallel state writes"))?;
            self.cursor.expect("=", Some("parallel state writes"))?;
            self.cursor.expect("(", Some("parallel state values"))?;
            while !self.cursor.is(")") {
                values.push(self.expr(path, Options::default())?);
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect(")", Some("parallel state values"))?;
            self.cursor.expect(";", Some("parallel state writes"))?;
            if writes.is_empty()
                || writes.len() != values.len()
                || values
                    .iter()
                    .any(|value| !self.origins.origin(value, path).direct_copy_value())
            {
                return Err(unsupported(
                    "Rust scalar loop",
                    "every state slot needs one direct scalar Copy value",
                    span(&start, self.cursor.peek()),
                ));
            }
        }
        self.cursor
            .expect("continue", Some("scalar continuation"))?;
        self.cursor.expect(";", Some("scalar continuation"))?;
        let arguments = state
            .params
            .iter()
            .map(|param| {
                writes
                    .iter()
                    .find(|write| write.target == param.name)
                    .map_or_else(|| param.name.clone(), |write| write.binding.clone())
            })
            .collect::<Vec<_>>();
        state.proof.transitions.push(ScalarLoopTransition {
            writes: writes.clone(),
            arguments: arguments.clone(),
        });
        let mut result = SExpr::new(
            SNode::App {
                func: Box::new(SExpr::name(&state.name)),
                args: arguments.iter().map(|name| SExpr::name(name)).collect(),
            },
            span(&start, self.cursor.peek()),
        );
        for (write, value) in writes.into_iter().zip(values).rev() {
            let ty = state
                .params
                .iter()
                .find(|param| param.name == write.target)
                .and_then(|param| param.ty.clone());
            result = SExpr::new(
                SNode::Let {
                    name: write.binding,
                    ty,
                    value: Box::new(value),
                    body: Box::new(result),
                },
                span(&start, self.cursor.peek()),
            );
        }
        Ok(result)
    }
}
