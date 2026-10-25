//! Actual token/term transport into the shared ordinary completion constructions.
use super::expressions::control::conditional_chains::ConditionalStages;
use super::{
    Binding, Options, Result, RustParser, SCtor, SData, SExpr, SField, SItem, SNode, SourceOrigin,
    Span, Token, TokenKind, Type, is_if_or_match, span, unsupported,
};
use crate::translation::rust_unit_value_rules as rules;
use crate::translation::surface::{SCase, SCasePattern};

struct CompletionTree {
    kind: String,
    node: CompletionNode,
}
enum CompletionNode {
    Value(SExpr, SourceOrigin),
    Return(SExpr, SourceOrigin),
    Abort(SExpr, SourceOrigin),
    Binding(Binding, Box<CompletionTree>),
    Conditional {
        condition: Option<SExpr>,
        stages: Option<ConditionalStages>,
        first: Box<CompletionTree>,
        second: Box<CompletionTree>,
        span: Option<Span>,
    },
    Sequence {
        first: Box<CompletionTree>,
        rest: Box<CompletionTree>,
        span: Option<Span>,
    },
}
impl CompletionNode {
    fn name(&self) -> &str {
        match self {
            Self::Value(..) => "value",
            Self::Return(..) => "return",
            Self::Abort(..) => "abort",
            Self::Binding(..) => "binding",
            Self::Conditional { .. } => "conditional",
            Self::Sequence { .. } => "sequence",
        }
    }
}
struct CompletionContext {
    path: Vec<String>,
    ret: Type,
    source_result: SourceOrigin,
    outputs: Vec<SourceOrigin>,
    data: Option<Vec<String>>,
}
fn name(path: Vec<String>) -> SExpr {
    SExpr::new(SNode::Name { path }, None)
}
fn binding(name: String, value: SExpr, body: SExpr, span: Option<Span>) -> SExpr {
    SExpr::new(
        SNode::Let {
            name,
            ty: None,
            value: Box::new(value),
            body: Box::new(body),
        },
        span,
    )
}
fn tree_span(tree: &CompletionTree) -> Option<Span> {
    match &tree.node {
        CompletionNode::Value(expr, _)
        | CompletionNode::Return(expr, _)
        | CompletionNode::Abort(expr, _) => expr.span,
        CompletionNode::Binding(binding, _) => binding.span,
        CompletionNode::Conditional { span, .. } | CompletionNode::Sequence { span, .. } => *span,
    }
}

impl RustParser {
    pub(super) fn ordinary_completion_context(&self) -> bool {
        let profiles = [
            self.completion_model.clone(),
            self.source_signature_model.clone(),
            self.scalar_loop_model.clone(),
            self.primitive_owner_model.clone(),
            self.empty_option_source_model.clone(),
            self.type_namespace_model.clone(),
            self.pure_unit_call_model.clone(),
            self.local_value_state_model.clone(),
            self.const_function_model.clone(),
            self.shared_array_handle_source_model.clone(),
            self.origins.array_result_model.clone(),
        ];
        rules::accept_rust_ordinary_completion_profiles(&profiles)
    }
    pub(super) fn ordinary_completion_selected(&self) -> bool {
        if !self.ordinary_completion_context() {
            return false;
        }
        let mut count = 64usize;
        loop {
            let values = self
                .cursor
                .tokens
                .iter()
                .skip(self.cursor.index)
                .take(count)
                .enumerate()
                .map(|(index, token)| {
                    if token.kind == TokenKind::Punct && (token.value == "{" || token.value == "}")
                    {
                        token.value.clone()
                    } else if token.kind == TokenKind::Macro && token.value == "panic" {
                        "abort".to_owned()
                    } else if self.cursor.is_at("return", index) {
                        "return".to_owned()
                    } else {
                        String::new()
                    }
                })
                .collect::<Vec<_>>();
            let presence = rules::read_rust_function_completion_presence(&values);
            if presence != -1.0 {
                return presence == 1.0;
            }
            if count >= self.cursor.tokens.len() - self.cursor.index {
                return false;
            }
            count = count.saturating_mul(2);
        }
    }
    pub(super) fn ordinary_function_completion(
        &mut self,
        path: &[String],
        ret: &Type,
        source_result: &SourceOrigin,
    ) -> Result<SExpr> {
        let tree = self.ordinary_completion_block(path)?;
        let mut context = CompletionContext {
            path: path.to_vec(),
            ret: ret.clone(),
            source_result: source_result.clone(),
            outputs: Vec::new(),
            data: None,
        };
        let expression = self.lower_ordinary_completion(tree, true, &mut context)?;
        let origin = self.origins.join(&context.outputs);
        self.origins.mark(&expression, origin);
        Ok(expression)
    }
    fn completion_leaf(&self, kind: &str, expression: SExpr, path: &[String]) -> CompletionTree {
        let origin = self.origins.origin(&expression, path);
        let (kind, node) = match kind {
            "return" => ("returned", CompletionNode::Return(expression, origin)),
            "abort" => ("aborted", CompletionNode::Abort(expression, origin)),
            _ => ("value", CompletionNode::Value(expression, origin)),
        };
        CompletionTree {
            kind: kind.to_owned(),
            node,
        }
    }
    fn ordinary_completion_block(&mut self, path: &[String]) -> Result<CompletionTree> {
        let open = self.cursor.expect("{", Some("block"))?;
        let previous = self.origins.enter(None);
        let result = self
            .ordinary_completion_sequence(path, &open)
            .and_then(|tree| {
                self.cursor.expect("}", Some("block"))?;
                Ok(tree)
            });
        self.origins.scope = previous;
        result
    }
    fn ordinary_completion_sequence(
        &mut self,
        path: &[String],
        open: &Token,
    ) -> Result<CompletionTree> {
        let mut bindings = Vec::new();
        let mut result = loop {
            if self.cursor.eat(";").is_some() {
                continue;
            }
            if self.cursor.is("let") {
                bindings.extend(self.let_statement(path)?);
                continue;
            }
            let token = self.peek();
            if self.cursor.is("}") {
                break self.completion_leaf(
                    "value",
                    SExpr::new(SNode::Unit, span(open, &token)),
                    path,
                );
            }
            if self.cursor.is("return") {
                let bare = self.cursor.is_at(";", 1) || self.cursor.is_at("}", 1);
                if !bare && !self.is_terminal_return() {
                    return Err(unsupported(
                        "early return",
                        "write the result as the tail expression of the block",
                        span(&token, &token),
                    ));
                }
                self.cursor.advance();
                let expression = if bare {
                    SExpr::new(SNode::Unit, span(&token, &token))
                } else {
                    self.expr(path, Options::default())?
                };
                if self.cursor.eat(";").is_none() && !self.cursor.is("}") {
                    return Err(Self::fail("expected ; or }", self.cursor.peek()));
                }
                while self.cursor.eat(";").is_some() {}
                if !self.cursor.is("}") {
                    return Err(unsupported(
                        "unreachable continuation",
                        "statements after a completed return are not removed by this model",
                        span(&token, self.cursor.peek()),
                    ));
                }
                break self.completion_leaf("return", expression, path);
            }
            if self.cursor.is("if") {
                let first = self.ordinary_conditional_completion(path)?;
                let terminated = self.cursor.eat(";").is_some();
                let continuation = !self.cursor.is("}");
                if rules::accept_rust_ordinary_completed_alternative(&first.kind) {
                    if continuation {
                        return Err(unsupported(
                            "unreachable continuation",
                            "statements after a completed return are not removed by this model",
                            tree_span(&first),
                        ));
                    }
                    break first;
                }
                if continuation || terminated {
                    let rest = self.ordinary_completion_sequence(path, open)?;
                    let kind =
                        rules::read_rust_ordinary_completion_sequence_kind(&first.kind, &rest.kind);
                    let range = tree_span(&first);
                    break CompletionTree {
                        kind,
                        node: CompletionNode::Sequence {
                            first: Box::new(first),
                            rest: Box::new(rest),
                            span: range,
                        },
                    };
                }
                break first;
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
            let expression = if token.kind == TokenKind::Macro
                && rules::read_rust_function_macro_disposition(&token.value) == "print"
            {
                self.function_print_value(path)?
            } else {
                self.expr(
                    path,
                    Options {
                        statement: true,
                        no_struct: false,
                    },
                )?
            };
            let terminated = self.cursor.eat(";").is_some();
            let abort = matches!(expression.node, SNode::Abort { .. });
            if terminated && !abort {
                let origin = self.origins.origin(&expression, path);
                if rules::read_rust_ordinary_statement_disposition(
                    expression.node.kind(),
                    self.origins.collecting,
                    origin.proven,
                    origin.ty.as_ref().map_or("", Type::kind),
                    !origin.wrappers.is_empty(),
                ) != "bind"
                {
                    return Err(unsupported(
                        "expression statement",
                        "statements with effects are outside the portable core in function bodies",
                        expression.span,
                    ));
                }
                bindings.push(Binding {
                    name: self.fresh_product_binding(),
                    ty: None,
                    span: expression.span,
                    value: expression,
                });
                continue;
            }
            let origin = self.origins.origin(&expression, path);
            let implicit = rules::read_rust_implicit_block_statement_disposition(
                expression.node.kind(),
                expression.block,
                !self.cursor.is("}"),
                self.origins.collecting,
                origin.proven,
                origin.ty.as_ref().map_or("", Type::kind),
                !origin.wrappers.is_empty(),
            );
            if implicit == "bind" {
                bindings.push(Binding {
                    name: self.fresh_product_binding(),
                    ty: Some(super::UNIT),
                    span: expression.span,
                    value: expression,
                });
                continue;
            }
            if !self.cursor.is("}") {
                if abort {
                    return Err(unsupported(
                        "unreachable continuation",
                        "statements after an abort are not removed by this model",
                        expression.span,
                    ));
                }
                if is_if_or_match(&expression) {
                    return Err(unsupported(
                        "expression statement",
                        "a block value must be its last expression",
                        expression.span,
                    ));
                }
                return Err(Self::fail("expected }", self.cursor.peek()));
            }
            break self.completion_leaf(if abort { "abort" } else { "value" }, expression, path);
        };
        for binding in bindings.into_iter().rev() {
            let kind = result.kind.clone();
            result = CompletionTree {
                kind,
                node: CompletionNode::Binding(binding, Box::new(result)),
            };
        }
        Ok(result)
    }
    fn ordinary_conditional_completion(&mut self, path: &[String]) -> Result<CompletionTree> {
        let token = self.cursor.advance();
        let (condition, stages, first) = if self.has_conditional_chain() {
            let previous = self.origins.enter(None);
            let parsed = (|| {
                let stages = self.conditional_chain_stages(path, &token)?;
                let first = self.ordinary_completion_block(path)?;
                Ok::<_, crate::translation::diagnostics::TranslationError>((
                    None,
                    Some(stages),
                    first,
                ))
            })();
            self.origins.scope = previous;
            parsed?
        } else {
            let condition = self.expr(
                path,
                Options {
                    statement: false,
                    no_struct: true,
                },
            )?;
            self.origins.direct(&condition, path, condition.span)?;
            (Some(condition), None, self.ordinary_completion_block(path)?)
        };
        let has_else = self.cursor.eat("else").is_some();
        let second = if has_else {
            if self.cursor.is("if") {
                self.ordinary_conditional_completion(path)?
            } else {
                self.ordinary_completion_block(path)?
            }
        } else {
            self.completion_leaf("value", SExpr::new(SNode::Unit, span(&token, &token)), path)
        };
        let kind = rules::read_rust_ordinary_completion_alternative_kind(&first.kind, &second.kind);
        Ok(CompletionTree {
            kind,
            node: CompletionNode::Conditional {
                condition,
                stages,
                first: Box::new(first),
                second: Box::new(second),
                span: span(&token, self.cursor.peek()),
            },
        })
    }
    fn ordinary_completion_data(&mut self, context: &mut CompletionContext) -> Vec<String> {
        if let Some(path) = &context.data {
            return path.clone();
        }
        let identifier = self.fresh_product_binding();
        let declaration = SData {
            name: identifier.clone(),
            generated: true,
            span: None,
            source_unit_clone_derivation: None,
            ctors: vec![
                SCtor {
                    name: "FunctionResult".to_owned(),
                    fields: vec![SField {
                        name: Some("result".to_owned()),
                        ty: context.ret.clone(),
                        rocq_type: None,
                        span: None,
                    }],
                },
                SCtor {
                    name: "StatementContinuation".to_owned(),
                    fields: Vec::new(),
                },
            ],
        };
        let mut path = context.path.clone();
        path.push(identifier);
        context.data = Some(path.clone());
        self.ordinary_completion_declarations
            .push((context.path.clone(), declaration));
        path
    }
    fn ordinary_completion_constructor(
        &mut self,
        context: &mut CompletionContext,
        variant: &str,
        args: Vec<SExpr>,
        span: Option<Span>,
    ) -> SExpr {
        let mut path = vec!["crate".to_owned()];
        path.extend(self.ordinary_completion_data(context));
        path.push(variant.to_owned());
        SExpr::new(
            SNode::App {
                func: Box::new(name(path)),
                args,
            },
            span,
        )
    }
    fn lower_ordinary_completion(
        &mut self,
        tree: CompletionTree,
        terminal: bool,
        context: &mut CompletionContext,
    ) -> Result<SExpr> {
        let form = rules::read_rust_ordinary_completion_construction(tree.node.name(), terminal);
        match tree.node {
            CompletionNode::Value(expression, origin)
            | CompletionNode::Return(expression, origin)
            | CompletionNode::Abort(expression, origin)
                if form == "value" || form == "completed" =>
            {
                self.origins
                    .argument(&origin, &context.source_result, expression.span)?;
                context.outputs.push(origin);
                Ok(if form == "completed" {
                    let span = expression.span;
                    self.ordinary_completion_constructor(
                        context,
                        "FunctionResult",
                        vec![expression],
                        span,
                    )
                } else {
                    expression
                })
            }
            CompletionNode::Value(expression, origin) if form == "continuation" => {
                if rules::read_rust_unit_statement_disposition(
                    expression.node.kind(),
                    self.origins.collecting,
                    origin.proven,
                    origin.ty.as_ref().map_or("", Type::kind),
                    !origin.wrappers.is_empty(),
                ) != "bind"
                {
                    return Err(unsupported(
                        "ordinary Rust statement completion",
                        "a falling-through branch needs an actual direct Unit result before its enclosing continuation",
                        expression.span,
                    ));
                }
                let span = expression.span;
                let binding_name = self.fresh_product_binding();
                let body = self.ordinary_completion_constructor(
                    context,
                    "StatementContinuation",
                    Vec::new(),
                    span,
                );
                Ok(binding(binding_name, expression, body, span))
            }
            CompletionNode::Binding(value, body) if form == "binding" => {
                let body = self.lower_ordinary_completion(*body, terminal, context)?;
                Ok(SExpr::new(
                    SNode::Let {
                        name: value.name,
                        ty: value.ty.map(Into::into),
                        value: Box::new(value.value),
                        body: Box::new(body),
                    },
                    value.span,
                ))
            }
            CompletionNode::Conditional {
                condition,
                stages,
                first,
                second,
                span,
            } if form == "conditional" => {
                let first = self.lower_ordinary_completion(*first, terminal, context)?;
                let second = self.lower_ordinary_completion(*second, terminal, context)?;
                if let Some(stages) = stages {
                    return self.lower_conditional_chain_stages(stages, first, second, span);
                }
                Ok(SExpr::new(
                    SNode::If {
                        cond: Box::new(condition.expect("an ordinary Boolean condition")),
                        then: Box::new(first),
                        otherwise: Box::new(second),
                    },
                    span,
                ))
            }
            CompletionNode::Sequence { first, rest, span } if form == "join" => {
                let first = self.lower_ordinary_completion(*first, false, context)?;
                let binding_name = self.fresh_product_binding();
                let result_name = self.fresh_product_binding();
                let completed = if terminal {
                    name(vec![result_name.clone()])
                } else {
                    self.ordinary_completion_constructor(
                        context,
                        "FunctionResult",
                        vec![name(vec![result_name.clone()])],
                        span,
                    )
                };
                let rest = self.lower_ordinary_completion(*rest, terminal, context)?;
                let mut owner = vec!["crate".to_owned()];
                owner.extend(self.ordinary_completion_data(context));
                let mut result_path = owner.clone();
                result_path.push("FunctionResult".to_owned());
                owner.push("StatementContinuation".to_owned());
                let body = SExpr::new(
                    SNode::Match1 {
                        scrutinee: Box::new(name(vec![binding_name.clone()])),
                        cases: vec![
                            SCase {
                                pattern: SCasePattern::Ctor {
                                    path: result_path,
                                    binds: vec![Some(result_name)],
                                },
                                body: completed,
                                span: None,
                            },
                            SCase {
                                pattern: SCasePattern::Ctor {
                                    path: owner,
                                    binds: Vec::new(),
                                },
                                body: rest,
                                span: None,
                            },
                        ],
                    },
                    None,
                );
                Ok(binding(binding_name, first, body, span))
            }
            _ => unreachable!(
                "the complete shared ordinary construction matches every private tree node"
            ),
        }
    }
    pub(super) fn install_ordinary_completion_declarations(
        &mut self,
        items: &mut Vec<SItem>,
    ) -> Result<()> {
        for (path, declaration) in std::mem::take(&mut self.ordinary_completion_declarations) {
            let mut target = &mut *items;
            for component in path {
                let module = target
                    .iter_mut()
                    .find_map(|item| match item {
                        SItem::Module(module) if module.name == component => Some(module),
                        _ => None,
                    })
                    .ok_or_else(|| {
                        unsupported(
                            "ordinary Rust completion carrier",
                            "the generated carrier needs its actual owning module",
                            None,
                        )
                    })?;
                target = &mut module.items;
            }
            target.push(SItem::Data(declaration));
        }
        Ok(())
    }
}
