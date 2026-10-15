//! Exact emitted loop and immutable write-snapshot parser adapter.
use super::{
    JavaScriptParser, Result, SExpr, SItem, SNode, SParam, SProgram, ScanEnd, Scope, Span,
    TokenKind, unsupported,
};
use crate::translation::scalar_loop_rules::accept_scalar_loop_declaration_model;
use crate::translation::source_loops::{
    ScalarLoopContinuation, ScalarLoopInterface, ScalarLoopInterfaceType, ScalarLoopState,
    ScalarLoopTransition, ScalarLoopWrite, SourceLoop, scalar_loop_policy,
    scalar_loop_snapshot_binding, validate_source_loop,
};
use std::collections::HashSet;

pub(super) fn finish_scalar_loops(program: &mut SProgram) -> Result<()> {
    fn items(values: &mut [SItem]) -> Result<()> {
        for item in values {
            if let SItem::Module(module) = item {
                items(&mut module.items)?;
            } else if let SItem::Fn(function) = item
                && function.source_loop.is_some()
            {
                let parameters = function
                    .params
                    .iter()
                    .map(|param| {
                        param
                            .ty
                            .as_ref()
                            .map(|ty| ScalarLoopInterfaceType {
                                kind: ty.kind().into(),
                            })
                            .ok_or_else(|| {
                                unsupported(
                                    "JavaScript scalar loop",
                                    "the interface needs actual inferred types",
                                    function.span,
                                )
                            })
                    })
                    .collect::<Result<Vec<_>>>()?;
                let result = function
                    .ret
                    .as_ref()
                    .map(|ty| ScalarLoopInterfaceType {
                        kind: ty.kind().into(),
                    })
                    .ok_or_else(|| {
                        unsupported(
                            "JavaScript scalar loop",
                            "the result needs its actual inferred type",
                            function.span,
                        )
                    })?;
                if let Some(proof) = function.source_loop.as_mut() {
                    proof.source_interface = Some(ScalarLoopInterface {
                        model: scalar_loop_policy()["javaScriptInterfaceModel"]
                            .as_str()
                            .unwrap()
                            .into(),
                        parameters,
                        result,
                    });
                }
                validate_source_loop(function, "JavaScript")?;
            }
        }
        Ok(())
    }
    items(&mut program.items)
}

struct LoopState {
    state: Vec<String>,
    known: HashSet<String>,
    declared: Vec<String>,
    proof: SourceLoop,
}
impl JavaScriptParser {
    pub(super) fn is_scalar_loop(&self) -> bool {
        ["{", "for", "(", ";", ";", ")", "{"]
            .iter()
            .enumerate()
            .all(|(index, value)| self.cursor.is_at(value, index))
    }
    pub(super) fn read_scalar_loop(
        &mut self,
        name: &str,
        params: &[SParam],
        is_async: bool,
    ) -> Result<(SExpr, SourceLoop)> {
        if is_async || params.iter().any(|param| param.default_value.is_some()) {
            return Err(unsupported(
                "JavaScript scalar loop",
                "loop parameters are direct fixed-arity Number or Boolean values",
                Some(self.peek().span()),
            ));
        }
        self.cursor.expect("{", Some("scalar loop function"))?;
        self.cursor.expect("for", Some("scalar loop"))?;
        self.cursor.expect("(", Some("scalar loop"))?;
        self.cursor.expect(";", Some("scalar loop"))?;
        self.cursor.expect(";", Some("scalar loop"))?;
        self.cursor.expect(")", Some("scalar loop"))?;
        let state = params
            .iter()
            .map(|param| param.name.clone())
            .collect::<Vec<_>>();
        let policy = scalar_loop_policy();
        let mut context = LoopState {
            known: state.iter().cloned().collect(),
            declared: Vec::new(),
            proof: SourceLoop {
                model: self.scalar_loop_model.clone(),
                language: "JavaScript".into(),
                source_name: name.into(),
                lowering: policy["lowering"].as_str().unwrap().into(),
                state: state
                    .iter()
                    .map(|name| ScalarLoopState {
                        name: name.clone(),
                        mutable: true,
                    })
                    .collect(),
                transitions: Vec::new(),
                source_interface: None,
                continuation: accept_scalar_loop_declaration_model(&self.scalar_loop_model).then(
                    || ScalarLoopContinuation {
                        root: policy["continuationRoot"].as_str().unwrap().into(),
                        path: vec![name.into()],
                    },
                ),
            },
            state,
        };
        let body = self.scalar_loop_block(&mut context)?;
        self.cursor.expect("}", Some("scalar loop function"))?;
        Ok((body, context.proof))
    }
    fn scalar_loop_block(&mut self, context: &mut LoopState) -> Result<SExpr> {
        let start = self.cursor.expect("{", Some("scalar loop block"))?;
        let mut tdz = self.scope.tdz.clone();
        tdz.extend(self.block_declarations(self.cursor.index, ScanEnd::Brace));
        let scope = Scope {
            locals: self.scope.locals.clone(),
            tdz,
            mutable: self.scope.mutable.clone(),
        };
        let previous = std::mem::replace(&mut self.scope, scope);
        let result = (|| {
            let mut bindings = Vec::new();
            while self.cursor.eat("const").is_some() {
                let token = self.cursor.identifier(Some("scalar loop binding"))?;
                if context.known.contains(&token.value)
                    || token.value.starts_with("ml_") && !scalar_loop_snapshot_binding(&token.value)
                {
                    return Err(unsupported(
                        "JavaScript scalar loop",
                        "loop locals need distinct identities; reserved names require exact write snapshots",
                        Some(token.span()),
                    ));
                }
                context.known.insert(token.value.clone());
                context.declared.push(token.value.clone());
                self.cursor.expect("=", Some("scalar loop binding"))?;
                let value = self.expr()?;
                self.cursor.expect(";", Some("scalar loop binding"))?;
                self.declare_local(&token.value, false);
                bindings.push((token.value, value));
            }
            let value = if self.cursor.eat("return").is_some() {
                let value = self.expr()?;
                self.cursor.expect(";", Some("scalar loop return"))?;
                value
            } else if self.cursor.is("if") {
                self.scalar_loop_conditional(context)?
            } else if self.cursor.is("[")
                || self.cursor.is("continue")
                || context.proof.continuation.is_some()
                    && self.peek().kind == TokenKind::Identifier
                    && self.cursor.is_at("=", 1)
            {
                self.scalar_loop_continuation(context)?
            } else {
                return Err(unsupported(
                    "JavaScript scalar loop",
                    "every loop path explicitly returns a scalar or continues owned state",
                    Some(self.peek().span()),
                ));
            };
            let close = self.cursor.expect("}", Some("scalar loop block"))?;
            Ok(bindings
                .into_iter()
                .rev()
                .fold(value, |body, (name, value)| {
                    SExpr::new(
                        SNode::Let {
                            name,
                            ty: None,
                            value: Box::new(value),
                            body: Box::new(body),
                        },
                        Some(Span::new(start.start, close.end)),
                    )
                }))
        })();
        self.scope = previous;
        result
    }
    fn scalar_loop_conditional(&mut self, context: &mut LoopState) -> Result<SExpr> {
        let start = self.cursor.expect("if", Some("scalar loop condition"))?;
        self.cursor.expect("(", Some("scalar loop condition"))?;
        let cond = self.expr()?;
        self.cursor.expect(")", Some("scalar loop condition"))?;
        let then = self.scalar_loop_block(context)?;
        self.cursor.expect("else", Some("scalar loop branches"))?;
        let otherwise = if self.cursor.is("if") {
            self.scalar_loop_conditional(context)?
        } else {
            self.scalar_loop_block(context)?
        };
        Ok(SExpr::new(
            SNode::If {
                cond: Box::new(cond),
                then: Box::new(then),
                otherwise: Box::new(otherwise),
            },
            Some(Span::new(start.start, self.cursor.peek().end)),
        ))
    }
    fn scalar_loop_continuation(&mut self, context: &mut LoopState) -> Result<SExpr> {
        let start = self.peek();
        let mut arguments = context.state.clone();
        let mut writes = Vec::new();
        if self.cursor.eat("[").is_some() {
            for (index, target) in context.state.iter().enumerate() {
                if index > 0 {
                    self.cursor.expect(",", Some("scalar state targets"))?;
                }
                if self.cursor.identifier(Some("scalar state target"))?.value != *target {
                    return Err(unsupported(
                        "JavaScript scalar loop",
                        "state targets retain the exact header order",
                        Some(start.span()),
                    ));
                }
            }
            self.cursor.expect("]", Some("scalar state targets"))?;
            self.cursor.expect("=", Some("scalar state snapshots"))?;
            self.cursor.expect("[", Some("scalar state snapshots"))?;
            for (index, target) in context.state.iter().enumerate() {
                if index > 0 {
                    self.cursor.expect(",", Some("scalar state snapshots"))?;
                }
                let binding = self.cursor.identifier(Some("scalar state snapshot"))?.value;
                if binding != *target {
                    if !scalar_loop_snapshot_binding(&binding)
                        || !self.scope.locals.contains(&binding)
                    {
                        return Err(unsupported(
                            "JavaScript scalar loop",
                            "changed slots read actual immutable write snapshots",
                            Some(start.span()),
                        ));
                    }
                    arguments[index].clone_from(&binding);
                    writes.push(ScalarLoopWrite {
                        target: target.clone(),
                        binding,
                    });
                }
            }
            self.cursor.expect("]", Some("scalar state snapshots"))?;
            self.cursor.expect(";", Some("scalar state snapshots"))?;
            writes.sort_by_key(|write| {
                context
                    .declared
                    .iter()
                    .position(|name| name == &write.binding)
            });
        } else if context.proof.continuation.is_some()
            && self.peek().kind == TokenKind::Identifier
            && self.cursor.is_at("=", 1)
        {
            let target = self.cursor.identifier(Some("scalar state target"))?.value;
            let Some(index) = context.state.iter().position(|name| name == &target) else {
                return Err(unsupported(
                    "JavaScript scalar loop",
                    "a single write targets its actual scalar parameter slot",
                    Some(start.span()),
                ));
            };
            self.cursor
                .expect("=", Some("single scalar state snapshot"))?;
            let binding = self.cursor.identifier(Some("scalar state snapshot"))?.value;
            if binding != target {
                if !scalar_loop_snapshot_binding(&binding) || !self.scope.locals.contains(&binding)
                {
                    return Err(unsupported(
                        "JavaScript scalar loop",
                        "each changed slot reads its actual immutable write snapshot",
                        Some(start.span()),
                    ));
                }
                arguments[index].clone_from(&binding);
                writes.push(ScalarLoopWrite { target, binding });
            }
            self.cursor
                .expect(";", Some("single scalar state snapshot"))?;
        }
        self.cursor
            .expect("continue", Some("scalar continuation"))?;
        self.cursor.expect(";", Some("scalar continuation"))?;
        let name = context.proof.source_name.clone();
        context.proof.transitions.push(ScalarLoopTransition {
            writes,
            arguments: arguments.clone(),
        });
        Ok(SExpr::new(
            SNode::App {
                func: Box::new(SExpr::new(
                    SNode::Name {
                        path: context.proof.continuation.as_ref().map_or_else(
                            || vec![name],
                            |continuation| {
                                std::iter::once(continuation.root.clone())
                                    .chain(continuation.path.iter().cloned())
                                    .collect()
                            },
                        ),
                    },
                    None,
                )),
                args: arguments.iter().map(|name| SExpr::name(name)).collect(),
            },
            Some(Span::new(start.start, self.cursor.peek().end)),
        ))
    }
}
