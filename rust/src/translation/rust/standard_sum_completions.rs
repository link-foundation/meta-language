//! Actual source terms/origins into the shared standard-sum completion policy.
use super::{Result,RustParser,SExpr,SNode,SourceOrigin,Token,Type,unsupported};
use crate::translation::surface::{SArrayItem,SCase,SCasePattern};
use crate::translation::rust_standard_sum_source_rules as rules;
use super::expression_completions::CompletionContinuation;
use std::{collections::{HashMap,HashSet},sync::Arc};

pub(super) const PRIVATE_TRY: &str = "rustStandardSumTry";
#[derive(Clone)]
pub(super) struct StandardSumTryAttempt {
    pub(super) operand_origin: SourceOrigin,
    pub(super) callable_origin: SourceOrigin,
    pub(super) payload: SExpr,
    pub(super) owned_operand_consumed: bool,
    pub(super) operand_value_origin: SourceOrigin,
    pub(super) failure_base:Option<Arc<TryBranchSnapshot>>,
    pub(super) residuals:Vec<Arc<TryResidualExit>>,
}
pub(super) struct TryBranchSnapshot{pub(super) ownership:HashMap<String,String>,pub(super) temporaries:HashSet<String>}
pub(super) struct TryResidualExit{pub(super) origin:SourceOrigin,pub(super) ownership:HashMap<String,String>}

#[cfg(test)]
mod deferred_controls{
    use super::*;
    use crate::translation::source_signatures::SourceOwner;
    fn parser()->RustParser{
        RustParser::new(crate::translation::lexer::tokenize("",crate::translation::Language::Rust).unwrap().tokens,super::super::SourceOrigins::new(None,""),super::super::SourceProfiles::default(),false,None)
    }
    fn primitive(owner:&str)->SourceOrigin{
        let mut origin=SourceOrigin::with_type(super::super::rust_fixed_type(owner).unwrap(),&[],Vec::new());origin.source_owner=Some(SourceOwner{kind:"primitive".into(),path:vec![owner.into()],arguments:Vec::new()});origin
    }
    #[test]
    fn primitive_conversion_certificates_require_the_complete_typed_identity(){
        let parser=parser();let mut origin=primitive("u8");assert_eq!(parser.standard_sum_primitive_question_error(&origin),Some("u8".into()));
        origin.fields=Some(vec![SourceOrigin::with_type(Type::Bool,&[],Vec::new())]);assert!(parser.standard_sum_primitive_question_error(&origin).is_none());
        let mut wrong=primitive("u8");wrong.source_owner.as_mut().unwrap().path=vec!["u16".into()];assert!(parser.standard_sum_primitive_question_error(&wrong).is_none());
    }
    #[test]
    fn no_span_from_projection_never_replaces_the_successful_payload_span_origin(){
        let mut parser=parser();let payload=name(vec!["payload".into()],Some(super::super::Span::new(1,4)));parser.origins.mark(&payload,primitive("u32"));
        parser.origins.scope.insert("residual".into(),primitive("u8"));let value=parser.project_primitive_from(name(vec!["residual".into()],None),"u16",None,&[]).unwrap();parser.origins.scope.remove("residual");
        assert!(value.span.is_none());assert_eq!(parser.origins.origin(&payload,&[]).ty,super::super::rust_fixed_type("u32"));
    }
}
fn name(path: Vec<String>,span: Option<super::Span>)->SExpr{SExpr::new(SNode::Name{path},span)}

impl RustParser {
    fn standard_sum_primitive_question_error(&self,origin:&SourceOrigin)->Option<String>{
        let ty=origin.ty.as_ref()?;let owner=origin.source_owner.as_ref()?;
        let (bits,signed)=match ty{Type::Fixed{bits,signed}=>(bits.to_string(),*signed),_=>(String::new(),false)};
        if !crate::translation::rust_primitive_from_rules::accept_rust_primitive_from_source(ty.kind(),&bits,signed,&owner.kind,&owner.path.join("::"),owner.arguments.len() as f64,origin.proven,origin.wrappers.len() as f64){return None;}
        // Independent typed structural identity: textual policy coordinates
        // alone cannot establish the complete primitive source certificate.
        let actual=self.origins.standard_sum_source_identity(origin)?;let mut canonical=SourceOrigin::with_type(ty.clone(),&origin.path,Vec::new());canonical.source_owner=Some(owner.clone());
        if self.origins.standard_sum_source_identity(&canonical).as_ref()!=Some(&actual){return None;}Some(owner.path.join("::"))
    }
    fn standard_sum_primitive_question_conversion(&self,source:&SourceOrigin,target:&SourceOrigin)->Vec<String>{
        let left=source.standard_sum_source.as_ref().expect("actual source sum");let right=target.standard_sum_source.as_ref().expect("actual callable sum");
        let from=left.arguments.get(1).and_then(|origin|self.standard_sum_primitive_question_error(origin));let to=right.arguments.get(1).and_then(|origin|self.standard_sum_primitive_question_error(origin));
        rules::read_rust_standard_sum_primitive_question_conversion_plan(&left.owner,&right.owner,self.ordinary_completion_context(),&self.origins.standard_sum_ownership_facts(source).profile,source.wrappers.len() as f64,source.direct_copy_value(),from.is_some(),to.is_some(),from.as_deref().unwrap_or(""),to.as_deref().unwrap_or(""),self.source_version_minor)
    }
    pub(super) fn standard_sum_try_expression(&mut self,operand:SExpr,path:&[String],token:&Token)->Result<SExpr>{
        let token_span=Some(super::Span::new(token.start,token.end));
        if !self.ordinary_completion_context(){return Err(unsupported("Rust question-mark completion","question-mark propagation requires the ordinary callable completion interpretation",token_span));}
        let actual=self.origins.origin(&operand,path);let origin=self.origins.standard_sum_source_origin(&actual)
            .ok_or_else(||unsupported("Rust question-mark completion","the operand needs its actual direct standard Option or Result source owner",operand.span))?;
        let callable=self.standard_sum_source.return_origin.as_ref().and_then(|origin|self.origins.standard_sum_source_origin(origin))
            .filter(|origin|origin.wrappers.is_empty()).ok_or_else(||unsupported("Rust question-mark completion","the nearest actual callable must return its own direct standard sum",token_span))?;
        let left=origin.standard_sum_source.as_deref().expect("actual operand sum");let right=callable.standard_sum_source.as_deref().expect("actual callable sum");
        let left_error=left.arguments.get(1).and_then(|value|self.origins.standard_sum_source_identity(value));let right_error=right.arguments.get(1).and_then(|value|self.origins.standard_sum_source_identity(value));
        let left_text=left_error.as_ref().map_or_else(String::new,|identity|format!("{identity:?}"));let right_text=right_error.as_ref().map_or_else(String::new,|identity|format!("{identity:?}"));
        let role=self.origins.standard_sum_ownership_facts(&origin).profile;
        let conversion=self.standard_sum_primitive_question_conversion(&origin,&callable);
        let owned=!self.header_discovery&&(left_error==right_error&&rules::accept_rust_standard_sum_owned_question_context(&left.owner,&right.owner,true,&role,origin.wrappers.len() as f64,&left_text,&right_text)||conversion.first().is_some_and(|profile|profile=="owned"));
        let identical=left_error==right_error&&rules::accept_rust_standard_sum_question_context(&left.owner,&right.owner,true,origin.direct_copy_value(),&left_text,&right_text);
        if !self.header_discovery&&!owned&&conversion.is_empty()&&!identical{return Err(unsupported("Rust question-mark completion","propagation needs identical errors or an actual standard primitive From implementation; custom Try, FromResidual and other conversions require their complete interpretation",token_span));}
        self.cursor.expect("?",Some("question-mark completion"))?;
        let place=Some(super::Span::new(operand.span.map_or(token.start,|span|span.start),token.end));
        let payload=name(vec![self.fresh_product_binding()],place);
        self.origins.check_owned_origin(&actual,operand.span)?;
        let captured=if owned{self.owned_standard_sum_method_capture(&operand,path)?}else{actual};let consumed=!owned||self.origins.consume_owned_binding(&captured,operand.span)?;
        let failure_base=if self.origins.collecting{None}else{Some(Arc::new(self.ordinary_completion_branch_snapshot()?))};
        let mut payload_origin=self.origins.project_standard_sum_pattern_field(&captured,&origin.standard_sum_source.as_ref().expect("validated actual sum").arguments[0],false);
        if owned&&self.origins.standard_sum_ownership_facts(&payload_origin).profile=="owned"{
            let produced=self.owned_standard_sum_method_output(payload_origin,consumed,true);payload_origin=self.origins.register_owned_binding(produced);
            if let Some(identity)=payload_origin.source_binding_identity.as_ref(){self.origins.source_temporary_bindings.insert(identity.clone());}
        }
        self.origins.mark(&payload,payload_origin.clone());
        let ordinal=self.standard_sum_try_attempts.len();
        self.standard_sum_try_attempts.push(StandardSumTryAttempt{operand_origin:origin,callable_origin:callable,payload,owned_operand_consumed:owned&&consumed,operand_value_origin:captured,failure_base,residuals:Vec::new()});
        let marker=SExpr::new(SNode::Math{op:PRIVATE_TRY.to_owned(),name:ordinal.to_string(),args:vec![SArrayItem::new(operand,false)]},place);
        self.origins.mark(&marker,payload_origin);Ok(marker)
    }

    pub(super) fn lower_standard_sum_try_completion(&mut self,ordinal:usize,operand:SExpr,path:&[String],source_result:&SourceOrigin,continued:SExpr,returned:&CompletionContinuation)->Result<(SExpr,Vec<SourceOrigin>)>{
        let attempt=self.standard_sum_try_attempts.get(ordinal).cloned().ok_or_else(||unsupported("Rust question-mark completion","unknown actual source attempt",operand.span))?;
        let place=attempt.payload.span;
        let source=self.origins.standard_sum_source_origin(&attempt.operand_origin).ok_or_else(||unsupported("Rust question-mark completion","propagation needs both actual operand and direct enclosing callable sum owners",place))?;
        let target=self.origins.standard_sum_source_origin(source_result).filter(|origin|origin.wrappers.is_empty()).ok_or_else(||unsupported("Rust question-mark completion","propagation needs both actual operand and direct enclosing callable sum owners",place))?;
        let selected=self.origins.standard_sum_source_identity(&attempt.callable_origin);
        if selected.is_none()||selected!=self.origins.standard_sum_source_identity(&target){return Err(unsupported("Rust question-mark completion","propagation cannot cross its actual nearest callable source owner",place));}
        let left=source.standard_sum_source.as_ref().expect("validated sum");let right=target.standard_sum_source.as_ref().expect("validated sum");
        let left_error=left.arguments.get(1).and_then(|origin|self.origins.standard_sum_source_identity(origin));
        let right_error=right.arguments.get(1).and_then(|origin|self.origins.standard_sum_source_identity(origin));
        // Actual typed structural equality is checked independently of textual
        // transport into the generated string-valued policy interface.
        let reason="propagation needs identical errors or an actual standard primitive From implementation; custom Try, FromResidual and other conversions require their complete interpretation";
        let left_text=left_error.as_ref().map_or_else(String::new,|identity|format!("{identity:?}"));
        let right_text=right_error.as_ref().map_or_else(String::new,|identity|format!("{identity:?}"));
        let owned=attempt.owned_operand_consumed&&left_error==right_error&&rules::accept_rust_standard_sum_owned_question_context(&left.owner,&right.owner,true,&self.origins.standard_sum_ownership_facts(&source).profile,source.wrappers.len() as f64,&left_text,&right_text);
        let conversion=self.standard_sum_primitive_question_conversion(&source,&target);let converted=conversion.first().is_some_and(|profile|rules::accept_rust_standard_sum_primitive_question_completion(profile,attempt.owned_operand_consumed));
        let identical=left_error==right_error&&rules::accept_rust_standard_sum_question_context(&left.owner,&right.owner,true,source.proven&&source.copy_evidence.is_some()&&source.wrappers.is_empty(),&left_text,&right_text);
        if !owned&&!converted&&!identical{return Err(unsupported("Rust question-mark completion",reason,place));}
        let captured=self.fresh_product_binding();let mut cases=Vec::new();let mut residual_origins=Vec::new();
        let payload_name=match &attempt.payload.node{SNode::Name{path}=>path[0].clone(),_=>return Err(unsupported("Rust question-mark completion","missing actual payload binding",place))};
        for variant in rules::read_rust_standard_sum_source_constructors(&left.owner){
            let disposition=rules::read_rust_standard_sum_question_branch(&left.owner,&variant);
            let (binds,body)=if disposition=="continue field"{(vec![payload_name.clone()],continued.clone())}else{
                if disposition!="return absent"&&disposition!="return error field"{return Err(unsupported("Rust question-mark completion","unrepresented actual constructor completion",place));}
                let binds=if disposition=="return error field"{vec![self.fresh_product_binding()]}else{Vec::new()};
                let before=if let Some(base)=attempt.failure_base.as_ref(){let before=self.ordinary_completion_branch_snapshot()?;let restored=self.clone_ordinary_completion_snapshot(&base.ownership)?;self.origins.restore_owned_bindings(restored);self.origins.restore_owned_temporary_frame(base.temporaries.clone());Some(before)}else{None};
                let mut saved=Vec::new();
                let built=(||->Result<SExpr>{
                    let mut args=Vec::new();let mut projection=None;
                    if let Some(binding)=binds.first(){
                        let mut field=self.origins.project_standard_sum_pattern_field(&attempt.operand_value_origin,&left.arguments[1],false);
                        if attempt.owned_operand_consumed&&self.origins.standard_sum_ownership_facts(&field).profile=="owned"{let produced=self.owned_standard_sum_method_output(field,true,true);field=self.origins.register_owned_binding(produced);}
                        saved.push((binding.clone(),self.origins.scope.insert(binding.clone(),field)));
                        args.push(name(vec![binding.clone()],None));
                        if converted{
                            let projected=self.project_primitive_from(args.remove(0),&conversion[1],None,path)?;let binding=self.fresh_product_binding();
                            saved.push((binding.clone(),self.origins.scope.insert(binding.clone(),right.arguments[1].clone())));args.push(name(vec![binding.clone()],None));projection=Some((binding,projected));
                        }
                    }
                    // These terms have no source span: their error type must
                    // never replace the successful '?' payload's span origin.
                    let value=SExpr::new(SNode::App{func:Box::new(name(right.constructor_paths[&variant].clone(),None)),args},None);
                    let actual_args=match &value.node{SNode::App{args,..}=>args.as_slice(),_=>unreachable!("actual residual constructor")};
                    let produced=self.standard_sum_constructor_origin(&value,&target,actual_args,path,None)?;
                    if attempt.failure_base.is_some(){let count=crate::translation::rust_owned_record_loop_rules::read_rust_owned_record_loop_residual_count_error((self.standard_sum_try_attempts[ordinal].residuals.len()+1) as f64);if !count.is_empty(){return Err(unsupported("Rust question-mark completion",&count,place));}let ownership=self.ordinary_completion_snapshot()?;let cost=super::standard_sum_metadata::measure_origins(&[&produced]).map_err(|error|unsupported("Rust question-mark completion",&error,place))?;self.origins.standard_sum_metadata_budget.reserve(cost).map_err(|error|unsupported("Rust question-mark completion",&error,place))?;self.standard_sum_try_attempts[ordinal].residuals.push(Arc::new(TryResidualExit{origin:produced.clone(),ownership}));}
                    residual_origins.push(produced);
                    let body=returned.apply(&value)?;
                    Ok(if let Some((binding,projected))=projection{SExpr::new(SNode::Let{name:binding,ty:right.arguments[1].ty.clone().map(Into::into),value:Box::new(projected),body:Box::new(body)},None)}else{body})
                })();
                for(binding,previous)in saved.into_iter().rev(){if let Some(previous)=previous{self.origins.scope.insert(binding,previous);}else{self.origins.scope.remove(&binding);}}
                if let Some(before)=before{self.origins.restore_owned_bindings(before.ownership);self.origins.restore_owned_temporary_frame(before.temporaries);}
                (binds,built?)
            };
            cases.push(SCase{pattern:SCasePattern::Ctor{path:left.constructor_paths[&variant].clone(),binds:binds.into_iter().map(Some).collect()},body,span:None});
        }
        Ok((SExpr::new(SNode::Let{name:captured.clone(),ty:None,value:Box::new(operand),body:Box::new(SExpr::new(SNode::Match1{scrutinee:Box::new(name(vec![captured],place)),cases},place))},place),residual_origins))
    }
}
