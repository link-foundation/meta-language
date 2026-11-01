//! Actual token/term transport into the shared ordinary completion constructions.
use super::expressions::control::conditional_chains::ConditionalStages;
use super::{
    Binding, Options, Result, RustParser, SCtor, SData, SExpr, SField, SItem, SNode, SourceOrigin,
    Span, Token, TokenKind, Type, is_if_or_match, span, unsupported,
};
use crate::translation::rust_unit_value_rules as rules;
use crate::translation::surface::{SCase, SCasePattern};
use super::expression_completions::{CompletionContinuation,has_expression_completion};
use std::collections::HashMap;
mod matches;

pub(super) struct CompletionTree {
    pub(super) kind: String,
    pub(super) node: CompletionNode,
    pub(super) ownership:Option<HashMap<String,String>>,
}
pub(super) enum CompletionNode {
    Value(SExpr, SourceOrigin),
    Return(SExpr, SourceOrigin),
    Abort(SExpr, SourceOrigin),
    Break(SExpr, SourceOrigin),
    Continue(SExpr, SourceOrigin),
    Loop{expression:SExpr,exits:Vec<super::owned_record_loops::LoopPath>,continuation:Box<CompletionTree>},
    Match{scrutinee:SExpr,arms:Vec<CompletionArm>,span:Option<Span>},
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
pub(super) struct CompletionArm{pub(super) patterns:Vec<super::SPattern>,pub(super) body:Box<CompletionTree>,pub(super) span:Option<Span>}
impl CompletionNode {
    fn name(&self) -> &str {
        match self {
            Self::Value(..) => "value",
            Self::Return(..) => "return",
            Self::Abort(..) => "abort",
            Self::Break(..)=>"break",
            Self::Continue(..)=>"continue",
            Self::Loop{..}=>"loop",
            Self::Match{..}=>"match",
            Self::Binding(..) => "binding",
            Self::Conditional { .. } => "conditional",
            Self::Sequence { .. } => "sequence",
        }
    }
}
struct CompletionContext {
    loop_control:bool,
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
        | CompletionNode::Abort(expr, _)
        | CompletionNode::Break(expr,_)
        | CompletionNode::Continue(expr,_)=>expr.span,
        CompletionNode::Loop{expression,..}=>expression.span,
        CompletionNode::Match{span,..}=>*span,
        CompletionNode::Binding(binding, _) => binding.span,
        CompletionNode::Conditional { span, .. } | CompletionNode::Sequence { span, .. } => *span,
    }
}

impl RustParser {
    pub(super) fn clone_ordinary_completion_snapshot(&mut self,states:&HashMap<String,String>)->Result<HashMap<String,String>>{
        let entries=states.len();let text=states.iter().map(|(key,value)|key.len()+value.len()).sum::<usize>();let budget=&mut self.owned_record_state;let error=rules::read_rust_ordinary_completion_snapshot_error(budget.completion_snapshot_entries as f64,entries as f64,budget.completion_snapshot_text as f64,text as f64);if !error.is_empty(){return Err(unsupported("ordinary Rust completion ownership",&error,None));}budget.completion_snapshot_entries+=entries;budget.completion_snapshot_text+=text;Ok(states.clone())
    }
    pub(super) fn ordinary_completion_snapshot(&mut self)->Result<HashMap<String,String>>{
        let entries=self.origins.owned_binding_state_count();let text=self.origins.owned_binding_state_text_size();let state=&mut self.owned_record_state;let error=rules::read_rust_ordinary_completion_snapshot_error(state.completion_snapshot_entries as f64,entries as f64,state.completion_snapshot_text as f64,text as f64);if !error.is_empty(){return Err(unsupported("ordinary Rust completion ownership",&error,None));}state.completion_snapshot_entries+=entries;state.completion_snapshot_text+=text;Ok(self.origins.snapshot_owned_bindings())
    }
    pub(super) fn ordinary_completion_branch_snapshot(&mut self)->Result<super::standard_sum_completions::TryBranchSnapshot>{
        let ownership=self.ordinary_completion_snapshot()?;let(entries,text)=self.origins.owned_temporary_state_cost();let budget=&mut self.owned_record_state;let error=rules::read_rust_ordinary_completion_snapshot_error(budget.completion_snapshot_entries as f64,entries as f64,budget.completion_snapshot_text as f64,text as f64);if !error.is_empty(){return Err(unsupported("ordinary Rust completion ownership",&error,None));}budget.completion_snapshot_entries+=entries;budget.completion_snapshot_text+=text;Ok(super::standard_sum_completions::TryBranchSnapshot{ownership,temporaries:self.origins.owned_temporary_frame()})
    }
    fn has_let_else_completion(&self) -> bool {
        let mut count = 64usize;
        loop {
            let tokens = self
                .cursor
                .tokens
                .iter()
                .skip(self.cursor.index)
                .take(count)
                .collect::<Vec<_>>();
            let kinds = tokens
                .iter()
                .map(|token| {
                    match token.kind {
                        super::TokenKind::Punct => "punct",
                        super::TokenKind::Eof => "eof",
                        _ => "atom",
                    }
                    .to_owned()
                })
                .collect::<Vec<_>>();
            let values = tokens
                .iter()
                .map(|token| token.value.clone())
                .collect::<Vec<_>>();
            let keywords = tokens
                .iter()
                .enumerate()
                .map(|(offset, _)| self.cursor.is_at("else", offset))
                .collect::<Vec<_>>();
            let returns = tokens
                .iter()
                .enumerate()
                .map(|(offset, _)| self.cursor.is_at("return", offset))
                .collect::<Vec<_>>();
            let selected = crate::translation::let_else_completion_rules::find_rust_returning_let_else_completion(&kinds,&values,&keywords,&returns);
            if selected != -2.0 {
                return selected >= 0.0;
            }
            if tokens.len() < count {
                return false;
            }
            count = count.saturating_mul(2);
        }
    }
    fn ordinary_let_else_completion(
        &mut self,
        path: &[String],
        open: &Token,
    ) -> Result<CompletionTree> {
        let start = self.cursor.advance();
        let mut aliases = Vec::new();
        if self.cursor.is("mut") {
            return Err(unsupported(
                "mutable binding",
                "mutation is outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        let mut pattern = self.pattern(path, &mut aliases)?;
        if !aliases.is_empty() {
            return Err(unsupported(
                "Rust nominal binding alias",
                "whole-value aliases require their complete move and borrow interpretation",
                pattern.span,
            ));
        }
        self.validate_product_binding_names(
            &super::ProductBindingPattern::Nominal(pattern.clone()),
            &mut std::collections::HashSet::new(),
        )?;
        let ty = if self.cursor.eat(":").is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };
        let source_type = ty.as_ref().map(|_| self.origins.current_type.clone());
        self.cursor.expect("=", Some("let"))?;
        let first = self.cursor.index;
        let expected=self.standard_sum_source.expected_origin.take();
        self.standard_sum_source.expected_origin=source_type.clone();
        let value = self.expr(path, Options::default());
        self.standard_sum_source.expected_origin=expected;
        let value=value?;
        let tokens = &self.cursor.tokens[first..self.cursor.index];
        let kinds = tokens
            .iter()
            .map(|token| {
                if token.kind == super::TokenKind::Punct {
                    "punct".to_owned()
                } else {
                    "atom".to_owned()
                }
            })
            .collect::<Vec<_>>();
        let spellings = tokens
            .iter()
            .map(|token| token.value.clone())
            .collect::<Vec<_>>();
        let reason =
            crate::translation::rust_source_render_interfaces::read_rust_let_else_initializer_error(
                &kinds, &spellings,
            );
        if !reason.is_empty() {
            return Err(unsupported(
                "Rust let-else initializer",
                &reason,
                span(&start, self.cursor.peek()),
            ));
        }
        self.cursor.expect("else", Some("let-else"))?;
        let before_fallback=self.ordinary_completion_snapshot()?;
        let second = self.ordinary_completion_block(path)?;
        if !rules::accept_rust_ordinary_completed_alternative(&second.kind) {
            return Err(unsupported(
                "Rust let-else fallback",
                "the fallback needs a represented diverging completion",
                span(&start, self.cursor.peek()),
            ));
        }
        self.origins.restore_owned_bindings(before_fallback);
        self.cursor.expect(";", Some("let"))?;
        let place = span(&start, self.cursor.peek());
        let capture = self.fresh_product_binding();
        self.origins
            .bind(&capture, &value, path, source_type.as_ref())?;
        let scrutinee = SExpr::new(
            SNode::Name {
                path: vec![capture.clone()],
            },
            place,
        );
        let scope = self
            .origins
            .cases(std::slice::from_mut(&mut pattern), &scrutinee, path)?;
        let previous = self.origins.enter(Some(scope));
        let first = self.ordinary_completion_sequence(path, open);
        self.origins.scope = previous;
        let first = first?;
        let kind = rules::read_rust_ordinary_completion_alternative_kind(&first.kind, &second.kind);
        let conditional = CompletionTree {
            kind: kind.clone(),
            ownership:None,
            node: CompletionNode::Conditional {
                condition: None,
                stages: Some(ConditionalStages::single_pattern(pattern, scrutinee, place)),
                first: Box::new(first),
                second: Box::new(second),
                span: place,
            },
        };
        Ok(CompletionTree {
            kind,
            ownership:None,
            node: CompletionNode::Binding(
                Binding {
                    name: capture,
                    ty,
                    value,
                    span: place,
                },
                Box::new(conditional),
            ),
        })
    }
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
            || crate::translation::rust_borrowed_vector_rules::accept_rust_borrowed_vector_completion_profiles(&self.source_signature_model,&profiles)
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
                    } else if token.kind == TokenKind::Punct && token.value == "?" {
                        "try".to_owned()
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
            loop_control:false,
            path: path.to_vec(),
            ret: ret.clone(),
            source_result: source_result.clone(),
            outputs: Vec::new(),
            data: None,
        };
        let expression = self.lower_ordinary_completion(tree, true, &mut context)?;
        self.refuse_standard_sum_private_completions(&expression)?;
        let mut origin = self.origins.checked_join(&context.outputs)?;
        let transfers=context.outputs.iter().filter(|origin|!origin.diverging).map(|origin|origin.source_owned_result_proof).collect::<Vec<_>>();if self.origins.owned_binding_value(&origin)&&rules::test_rust_ordinary_completion_transferred_result(&transfers){origin.source_binding_identity=None;origin.source_place_binding_identity=None;origin.source_loan_binding_identities.clear();origin.source_place_loan_binding_identities.clear();origin.source_extending_temporary_bindings.clear();}
        self.origins.mark(&expression, origin);
        Ok(expression)
    }
    pub(super) fn ordinary_loop_completion(&mut self,path:&[String],guard:&SExpr)->Result<super::owned_record_loops::LoopCompletion>{
        let source_result=self.standard_sum_source.return_origin.clone().filter(|origin|origin.ty.is_some()).ok_or_else(||unsupported("Rust finite record loop","completed loop exits need their actual enclosing callable result type",guard.span))?;
        let callable=self.owned_record_state_callable();let previous=self.owned_record_state.effects.get_or_insert_with(Default::default).loop_scope.replace(callable);
        let tree=self.ordinary_completion_block(path);self.owned_record_state.effects.as_mut().expect("actual loop state transport").loop_scope=previous;let tree=tree?;
        let mut pending=vec![(&tree,0_usize)];let mut work=0_usize;while let Some((tree,depth))=pending.pop(){work+=1;if work>crate::translation::rust_expression_completion_rules::read_rust_expression_completion_limit("work") as usize||depth>crate::translation::rust_expression_completion_rules::read_rust_expression_completion_limit("depth") as usize{return Err(unsupported("Rust finite record loop","the complete loop completion tree exceeds its source budget",None));}match &tree.node{CompletionNode::Binding(_,body)=>pending.push((body,depth+1)),CompletionNode::Conditional{first,second,..}|CompletionNode::Sequence{first,rest:second,..}=>{pending.push((first,depth+1));pending.push((second,depth+1));},CompletionNode::Match{arms,..}=>pending.extend(arms.iter().map(|arm|(arm.body.as_ref(),depth+1))),CompletionNode::Loop{continuation,..}=>pending.push((continuation,depth+1)),_=>{}}}
        let pending=if self.origins.collecting{Vec::new()}else{super::owned_record_loops::trace_completed_loop(self,&tree,guard,path)?};
        let mut context=CompletionContext{loop_control:true,path:path.to_vec(),ret:source_result.ty.as_ref().expect("actual enclosing return type").clone(),source_result,outputs:Vec::new(),data:None};
        let expression=self.lower_ordinary_completion(tree,false,&mut context)?;let paths=self.resolve_completed_loop_paths(pending)?;let owner=self.ordinary_completion_data(&mut context);let returns=context.outputs.iter().filter(|origin|!origin.diverging).count();Ok(super::owned_record_loops::LoopCompletion{expression:Some(expression),owner,returns,paths})
    }
    fn completion_leaf(&mut self, kind: &str, expression: SExpr, path: &[String]) -> Result<CompletionTree> {
        let origin = self.origins.transfer_owned_result(&expression, path)?;
        self.origins.mark(&expression,origin.clone());
        let propagates=has_expression_completion(&expression)?;
        let loop_kind=crate::translation::rust_owned_record_loop_rules::read_rust_owned_record_loop_leaf_kind(kind);
        let (kind, node) = match kind {
            "return" => ("returned", CompletionNode::Return(expression, origin)),
            "abort" => ("aborted", CompletionNode::Abort(expression, origin)),
            "break" => (loop_kind.as_str(), CompletionNode::Break(expression,origin)),
            "continue"=>(loop_kind.as_str(),CompletionNode::Continue(expression,origin)),
            _ => ("value", CompletionNode::Value(expression, origin)),
        };
        Ok(CompletionTree {
            kind: if propagates {rules::read_rust_ordinary_completion_alternative_kind(kind,"returned")}else{kind.to_owned()},
            node,
            ownership:Some(self.ordinary_completion_snapshot()?),
        })
    }
    fn ordinary_completion_block(&mut self, path: &[String]) -> Result<CompletionTree> {
        let before=self.ordinary_completion_snapshot()?;
        let frame=self.begin_owned_record_state_block(false);
        let open = match self.cursor.expect("{", Some("block")){Ok(open)=>open,Err(error)=>{self.end_owned_record_state_block(frame)?;return Err(error);}};
        let previous = self.origins.enter(None);
        let result = self
            .ordinary_completion_sequence(path, &open)
            .and_then(|mut tree| {
                self.cursor.expect("}", Some("block"))?;
                let current=self.ordinary_completion_snapshot()?;let continuing=self.expire_ordinary_completion_paths(&mut tree,&before)?;self.origins.restore_owned_bindings(current);
                if continuing.is_empty(){self.origins.restore_owned_bindings(before);}else{let origin=self.origins.checked_join(&continuing)?;self.origins.expire_owned_bindings(&before,&origin)?;}
                Ok(tree)
            });
        self.origins.scope = previous;
        self.end_owned_record_state_block(frame)?;
        result
    }
    fn expire_ordinary_completion_paths(&mut self,tree:&mut CompletionTree,before:&HashMap<String,String>)->Result<Vec<SourceOrigin>>{
        if let Some(states)=tree.ownership.take(){let(origin,continuing)=match &tree.node{CompletionNode::Value(_,origin)=>(origin.clone(),true),CompletionNode::Return(_,origin)|CompletionNode::Abort(_,origin)|CompletionNode::Break(_,origin)|CompletionNode::Continue(_,origin)=>(origin.clone(),false),_=>return Err(unsupported("ordinary Rust completion ownership","only an actual leaf owns its exit state",None))};self.origins.restore_owned_bindings(states);self.origins.expire_owned_bindings(before,&origin)?;tree.ownership=Some(self.ordinary_completion_snapshot()?);return Ok(if continuing{vec![origin]}else{Vec::new()});}
        let mut values=Vec::new();match &mut tree.node{CompletionNode::Binding(_,body)=>values.extend(self.expire_ordinary_completion_paths(body,before)?),CompletionNode::Conditional{first,second,..}=>{values.extend(self.expire_ordinary_completion_paths(first,before)?);values.extend(self.expire_ordinary_completion_paths(second,before)?);},CompletionNode::Sequence{first,rest,..}=>{self.expire_ordinary_completion_paths(first,before)?;values.extend(self.expire_ordinary_completion_paths(rest,before)?);},CompletionNode::Match{arms,..}=>{for arm in arms{values.extend(self.expire_ordinary_completion_paths(&mut arm.body,before)?);}},CompletionNode::Loop{exits,continuation,..}=>{for exit in exits{self.origins.restore_owned_bindings(std::mem::take(&mut exit.ownership));self.origins.expire_owned_bindings(before,&exit.origin)?;exit.ownership=self.ordinary_completion_snapshot()?;}values.extend(self.expire_ordinary_completion_paths(continuation,before)?);},_=>return Err(unsupported("ordinary Rust completion ownership","an actual leaf needs its retained exit state",None))}Ok(values)
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
                if self.has_let_else_completion() {
                    break self.ordinary_let_else_completion(path, open)?;
                }
                bindings.extend(self.let_statement(path)?);
                continue;
            }
            let token = self.peek();
            if self.cursor.is("}") {
                break self.completion_leaf(
                    "value",
                    SExpr::new(SNode::Unit, span(open, &token)),
                    path,
                )?;
            }
            if token.kind==TokenKind::Identifier&&["break","continue"].contains(&token.raw.as_str()){
                if self.owned_record_state.effects.as_ref().and_then(|effects|effects.loop_scope)!=Some(self.owned_record_state_callable()){return Err(unsupported("Rust loop exit","a loop exit needs its actual enclosing callable and loop scope",span(&token,&token)));}
                self.cursor.advance();if self.cursor.eat(";").is_none()&&!self.cursor.is("}"){return Err(unsupported("Rust loop exit","labelled exits and break values need their explicit value and target interpretation",span(&token,&self.peek())));}while self.cursor.eat(";").is_some(){}
                if !self.cursor.is("}"){return Err(unsupported("unreachable continuation","statements after a completed loop exit are not removed by this model",span(&token,&self.peek())));}
                break self.completion_leaf(&token.raw,SExpr::new(SNode::Unit,span(&token,&token)),path)?;
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
                break self.completion_leaf("return", expression, path)?;
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
                        ownership:None,
                        node: CompletionNode::Sequence {
                            first: Box::new(first),
                            rest: Box::new(rest),
                            span: range,
                        },
                    };
                }
                break first;
            }
            if self.has_ordinary_match_completion()?{
                let first=self.ordinary_match_completion(path)?;let terminated=self.cursor.eat(";").is_some();let continuation=!self.cursor.is("}");if rules::accept_rust_ordinary_completed_alternative(&first.kind){if continuation{return Err(unsupported("unreachable continuation","statements after a completed return are not removed by this model",tree_span(&first)));}break first;}if continuation||terminated{let rest=self.ordinary_completion_sequence(path,open)?;let kind=rules::read_rust_ordinary_completion_sequence_kind(&first.kind,&rest.kind);let span=tree_span(&first);break CompletionTree{kind,ownership:None,node:CompletionNode::Sequence{first:Box::new(first),rest:Box::new(rest),span}};}break first;
            }
            if let Some(expression)=self.read_owned_record_finite_loop(path)?{
                let _=self.cursor.eat(";");let span=expression.span;if let Some(exits)=self.take_owned_record_loop_returns(&expression){let continuation=Box::new(self.completion_leaf("value",SExpr::new(SNode::Unit,span),path)?);let first=CompletionTree{kind:"mixed".to_owned(),ownership:None,node:CompletionNode::Loop{expression,exits,continuation}};if self.cursor.is("}"){break first;}let rest=self.ordinary_completion_sequence(path,open)?;let kind=rules::read_rust_ordinary_completion_sequence_kind(&first.kind,&rest.kind);break CompletionTree{kind,ownership:None,node:CompletionNode::Sequence{first:Box::new(first),rest:Box::new(rest),span}};}bindings.push(Binding{name:self.fresh_product_binding(),ty:Some(super::UNIT),value:expression,span});continue;
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
            let abort = super::expressions::source_completion_kind(&expression) == "abort";
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
            break self.completion_leaf(if abort { "abort" } else { "value" }, expression, path)?;
        };
        for binding in bindings.into_iter().rev() {
            let kind = if has_expression_completion(&binding.value)?{rules::read_rust_ordinary_completion_alternative_kind(&result.kind,"returned")}else{result.kind.clone()};
            result = CompletionTree {
                kind,
                ownership:None,
                node: CompletionNode::Binding(binding, Box::new(result)),
            };
        }
        Ok(result)
    }
    fn ordinary_conditional_completion(&mut self, path: &[String]) -> Result<CompletionTree> {
        let token = self.cursor.advance();
        let mut before=self.ordinary_completion_snapshot()?;
        let (condition, stages, first) = if self.has_conditional_chain() {
            let previous = self.origins.enter(None);
            let parsed = (|| {
                let expected=self.standard_sum_source.expected_origin.take();
                let stages = self.conditional_chain_stages(path, &token);
                self.standard_sum_source.expected_origin=expected;
                let mut stages=stages?;before=stages.take_ownership_failure();
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
            let expected=self.standard_sum_source.expected_origin.take();
            let condition = self.expr(
                path,
                Options {
                    statement: false,
                    no_struct: true,
                },
            );
            self.standard_sum_source.expected_origin=expected;
            let condition=condition?;
            self.origins.direct(&condition, path, condition.span)?;
            before=self.ordinary_completion_snapshot()?;
            (Some(condition), None, self.ordinary_completion_block(path)?)
        };
        let first_states=self.ordinary_completion_snapshot()?;self.origins.restore_owned_bindings(before);
        let has_else = self.cursor.eat("else").is_some();
        let second = if has_else {
            if self.cursor.is("if") {
                self.ordinary_conditional_completion(path)?
            } else {
                self.ordinary_completion_block(path)?
            }
        } else {
            self.completion_leaf("value", SExpr::new(SNode::Unit, span(&token, &token)), path)?
        };
        let second_states=self.ordinary_completion_snapshot()?;self.origins.join_owned_bindings(&first_states,&second_states,rules::test_rust_ordinary_completion_falls_through(&first.kind),rules::test_rust_ordinary_completion_falls_through(&second.kind));
        let mut kind = rules::read_rust_ordinary_completion_alternative_kind(&first.kind, &second.kind);
        if condition.as_ref().map(has_expression_completion).transpose()?.unwrap_or(false)||stages.as_ref().map(ConditionalStages::has_expression_completion).transpose()?.unwrap_or(false){kind=rules::read_rust_ordinary_completion_alternative_kind(&kind,"returned");}
        Ok(CompletionTree {
            kind,
            ownership:None,
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
        let mut declaration = SData {
            name: identifier.clone(),
            generated: true,
            span: None,
            source_unit_clone_derivation: None,
            source_field_signatures: None,
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
        if context.loop_control{for variant in ["LoopBreak","LoopContinue"]{declaration.ctors.push(SCtor{name:variant.to_owned(),fields:Vec::new()});}}
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
    fn lift_ordinary_expression(&mut self,expression:SExpr,terminal:bool,context:&mut CompletionContext,continued:CompletionContinuation)->Result<SExpr>{
        if !has_expression_completion(&expression)?{return continued.apply(&expression);}
        let hole="#rust-ordinary-return-hole".to_owned();let result=name(vec![hole.clone()]);
        let returned=if terminal{result}else{self.ordinary_completion_constructor(context,"FunctionResult",vec![result],expression.span)};
        let (expression,origins)=self.lower_expression_completions(expression,&context.path,&context.source_result,continued,CompletionContinuation{hole,expression:returned})?;
        for origin in origins{self.origins.argument(&origin,&context.source_result,expression.span)?;context.outputs.push(origin);}Ok(expression)
    }
    fn lower_ordinary_completion(
        &mut self,
        mut tree: CompletionTree,
        terminal: bool,
        context: &mut CompletionContext,
    ) -> Result<SExpr> {
        if let Some(states)=tree.ownership.take(){let before=self.ordinary_completion_snapshot()?;self.origins.restore_owned_bindings(states);let result=self.lower_ordinary_completion(tree,terminal,context);self.origins.restore_owned_bindings(before);return result;}
        let node_kind=tree.node.name().to_owned();let form = rules::read_rust_ordinary_completion_construction(&node_kind, terminal);
        match tree.node {
            CompletionNode::Break(expression,_)|CompletionNode::Continue(expression,_)if form=="loop signal"=>{
                if !context.loop_control{return Err(unsupported("Rust loop exit","a loop signal cannot escape its actual loop completion context",expression.span));}
                let variant=crate::translation::rust_owned_record_loop_rules::read_rust_owned_record_loop_signal(&node_kind);Ok(self.ordinary_completion_constructor(context,&variant,Vec::new(),expression.span))
            }
            CompletionNode::Loop{expression,exits,..}if form=="finite loop"=>{
                for exit in exits{context.outputs.push(exit.origin);}if terminal{context.outputs.push(self.origins.origin(&SExpr::new(SNode::Unit,None),&context.path));}
                let mut destination=vec!["crate".to_owned()];destination.extend(self.ordinary_completion_data(context));self.lower_owned_record_loop_completion(expression,destination,terminal,&context.source_result,&context.path)
            }
            CompletionNode::Value(expression, origin)
            | CompletionNode::Return(expression, origin)
            | CompletionNode::Abort(expression, origin)
                if form == "value" || form == "completed" =>
            {
                self.origins
                    .argument(&origin, &context.source_result, expression.span)?;
                context.outputs.push(origin);
                let hole="#rust-ordinary-value-hole".to_owned();
                let continued=if form == "completed" {
                    let span = expression.span;
                    self.ordinary_completion_constructor(
                        context,
                        "FunctionResult",
                        vec![name(vec![hole.clone()])],
                        span,
                    )
                } else {
                    name(vec![hole.clone()])
                };
                self.lift_ordinary_expression(expression,terminal,context,CompletionContinuation{hole,expression:continued})
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
                let hole="#rust-ordinary-value-hole".to_owned();let continued=binding(binding_name,name(vec![hole.clone()]),body,span);
                self.lift_ordinary_expression(expression,terminal,context,CompletionContinuation{hole,expression:continued})
            }
            CompletionNode::Binding(value, body) if form == "binding" => {
                let body = self.lower_ordinary_completion(*body, terminal, context)?;
                let hole="#rust-ordinary-value-hole".to_owned();
                let continued=SExpr::new(
                    SNode::Let {
                        name: value.name,
                        ty: value.ty.map(Into::into),
                        value: Box::new(name(vec![hole.clone()])),
                        body: Box::new(body),
                    },
                    value.span,
                );
                self.lift_ordinary_expression(value.value,terminal,context,CompletionContinuation{hole,expression:continued})
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
                    let expression=self.lower_conditional_chain_stages(stages, first, second, span)?;
                    let hole="#rust-ordinary-value-hole".to_owned();let continued=name(vec![hole.clone()]);
                    return self.lift_ordinary_expression(expression,terminal,context,CompletionContinuation{hole,expression:continued});
                }
                let hole="#rust-ordinary-value-hole".to_owned();let expression=condition.expect("an ordinary Boolean condition");
                let continued=SExpr::new(
                    SNode::If {
                        cond: Box::new(name(vec![hole.clone()])),
                        then: Box::new(first),
                        otherwise: Box::new(second),
                    },
                    span,
                );
                self.lift_ordinary_expression(expression,terminal,context,CompletionContinuation{hole,expression:continued})
            }
            CompletionNode::Match{scrutinee,arms,span}if form=="match"=>{
                let mut rows=Vec::new();for arm in arms{let body=self.lower_ordinary_completion(*arm.body,terminal,context)?;for pattern in arm.patterns{rows.push(super::SRow{patterns:vec![pattern],body:body.clone(),span:arm.span});}}
                let hole="#rust-ordinary-value-hole".to_owned();let expression=SExpr::new(SNode::Match{scrutinees:vec![name(vec![hole.clone()])],rows},span);self.lift_ordinary_expression(scrutinee,terminal,context,CompletionContinuation{hole,expression})
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
                let continuation_owner=owner.clone();owner.push("StatementContinuation".to_owned());
                let mut body = SExpr::new(
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
                if context.loop_control{if let SNode::Match1{cases,..}=&mut body.node{for variant in ["LoopBreak","LoopContinue"]{let mut path=continuation_owner.clone();path.push(variant.to_owned());cases.push(SCase{pattern:SCasePattern::Ctor{path,binds:Vec::new()},body:self.ordinary_completion_constructor(context,variant,Vec::new(),span),span:None});}}}
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
