//! Actual source terms/origins into the shared standard-sum completion policy.
use super::{Result,RustParser,SExpr,SNode,SourceOrigin,Token,unsupported};
use crate::translation::surface::{SArrayItem,SCase,SCasePattern};
use crate::translation::rust_standard_sum_source_rules as rules;
use super::expression_completions::CompletionContinuation;

pub(super) const PRIVATE_TRY: &str = "rustStandardSumTry";
#[derive(Clone)]
pub(super) struct StandardSumTryAttempt {
    pub(super) operand_origin: SourceOrigin,
    pub(super) callable_origin: SourceOrigin,
    pub(super) payload: SExpr,
}
fn name(path: Vec<String>,span: Option<super::Span>)->SExpr{SExpr::new(SNode::Name{path},span)}

impl RustParser {
    pub(super) fn standard_sum_try_expression(&mut self,operand:SExpr,path:&[String],token:&Token)->Result<SExpr>{
        let token_span=Some(super::Span::new(token.start,token.end));
        if !self.ordinary_completion_context(){return Err(unsupported("Rust question-mark completion","question-mark propagation requires the ordinary callable completion interpretation",token_span));}
        let origin=self.origins.standard_sum_source_origin(&self.origins.origin(&operand,path))
            .ok_or_else(||unsupported("Rust question-mark completion","the operand needs its actual direct standard Option or Result source owner",operand.span))?;
        let callable=self.standard_sum_source.return_origin.as_ref().and_then(|origin|self.origins.standard_sum_source_origin(origin))
            .filter(|origin|origin.wrappers.is_empty()).ok_or_else(||unsupported("Rust question-mark completion","the nearest actual callable must return its own direct standard sum",token_span))?;
        self.cursor.expect("?",Some("question-mark completion"))?;
        let place=Some(super::Span::new(operand.span.map_or(token.start,|span|span.start),token.end));
        let payload=name(vec![self.fresh_product_binding()],place);
        let payload_origin=origin.standard_sum_source.as_ref().expect("validated actual sum").arguments[0].clone();
        self.origins.mark(&payload,payload_origin.clone());
        let ordinal=self.standard_sum_try_attempts.len();
        self.standard_sum_try_attempts.push(StandardSumTryAttempt{operand_origin:origin,callable_origin:callable,payload});
        let marker=SExpr::new(SNode::Math{op:PRIVATE_TRY.to_owned(),name:ordinal.to_string(),args:vec![SArrayItem::new(operand,false)]},place);
        self.origins.mark(&marker,payload_origin);Ok(marker)
    }

    pub(super) fn lower_standard_sum_try_completion(&mut self,ordinal:usize,operand:SExpr,_path:&[String],source_result:&SourceOrigin,continued:SExpr,returned:&CompletionContinuation)->Result<SExpr>{
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
        let reason="only direct standard sums of the same family and exactly identical Result error owners can propagate; custom Try, FromResidual and error conversions require their complete interpretation";
        if left_error!=right_error{return Err(unsupported("Rust question-mark completion",reason,place));}
        let left_text=left_error.as_ref().map_or_else(String::new,|identity|format!("{identity:?}"));
        let right_text=right_error.as_ref().map_or_else(String::new,|identity|format!("{identity:?}"));
        if !rules::accept_rust_standard_sum_question_context(&left.owner,&right.owner,true,source.proven&&source.copy_evidence.is_some()&&source.wrappers.is_empty(),&left_text,&right_text){return Err(unsupported("Rust question-mark completion",reason,place));}
        let captured=self.fresh_product_binding();let mut cases=Vec::new();
        let payload_name=match &attempt.payload.node{SNode::Name{path}=>path[0].clone(),_=>return Err(unsupported("Rust question-mark completion","missing actual payload binding",place))};
        for variant in rules::read_rust_standard_sum_source_constructors(&left.owner){
            let disposition=rules::read_rust_standard_sum_question_branch(&left.owner,&variant);
            let (binds,body)=if disposition=="continue field"{(vec![payload_name.clone()],continued.clone())}else{
                if disposition!="return absent"&&disposition!="return error field"{return Err(unsupported("Rust question-mark completion","unrepresented actual constructor completion",place));}
                let binds=if disposition=="return error field"{vec![self.fresh_product_binding()]}else{Vec::new()};
                let args=binds.iter().map(|binding|name(vec![binding.clone()],place)).collect();
                let value=SExpr::new(SNode::App{func:Box::new(name(right.constructor_paths[&variant].clone(),place)),args},place);
                // Residual and successful payload share a source interval but
                // have different types. Do not overwrite the span-keyed source
                // origin of the actual successful payload with this new term.
                (binds,returned.apply(&value)?)
            };
            cases.push(SCase{pattern:SCasePattern::Ctor{path:left.constructor_paths[&variant].clone(),binds:binds.into_iter().map(Some).collect()},body,span:None});
        }
        Ok(SExpr::new(SNode::Let{name:captured.clone(),ty:None,value:Box::new(operand),body:Box::new(SExpr::new(SNode::Match1{scrutinee:Box::new(name(vec![captured],place)),cases},place))},place))
    }
}
