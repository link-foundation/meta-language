//! Retain only actual complete source types while lexical scopes are live.
//! No value ownership, Copy, loan, binding or producer fact is cached here.
use std::collections::HashMap;
use super::{Result,SExpr,SNode,SourceDeclaration,SourceOrigin,SourceOrigins,Span,Type,unsupported};
use super::standard_sum_metadata::{MetadataBudget,measure_expression_type};
pub(super) struct RecordedExpressionType{pub(super) ty:Type,pub(super) path:Vec<String>,pub(super) proven:bool}
#[derive(Default)]
pub(super) struct ExpressionTypeCache{types:HashMap<Span,RecordedExpressionType>,budget:MetadataBudget}
fn result(origin:&SourceOrigin)->Option<(&Type,&[String])>{if !origin.proven||origin.source_function_pointer_header||origin.standard_sum_source.as_deref().is_some_and(|sum|sum.provisional){return None;}origin.ty.as_ref().map(|ty|(ty,origin.path.as_slice()))}
fn callable(origin:&SourceOrigin)->Option<(&Type,&[String])>{if !origin.proven||origin.source_function_pointer_header{return None;}if let Some(value)=origin.callable_result.as_deref(){return result(value);}match origin.ty.as_ref()?{Type::Function{ret,..}=>Some((ret,&origin.path)),_=>None}}
fn actual<'a>(origins:&'a SourceOrigins,node:&SExpr,path:&[String])->Option<(&'a Type,&'a [String])>{
 let SNode::App{func,..}=&node.node else{return None;};
 if let SNode::Name{path:names}=&func.node{
  if names.len()==1{if let Some(origin)=origins.scope.get(&names[0]){return callable(origin);}}
  if let Some((_,entry))=origins.lookup_value_owner_ref(names,path){return match entry{SourceDeclaration::Function{result:origin,..}|SourceDeclaration::Constructor{result:origin,..}=>result(origin),_=>None};}
 }
 if let Some(origin)=origins.recorded_expression_origin(func){return callable(origin);}
 let facts=origins.recorded_expression_type(func)?;if !facts.proven{return None;}match &facts.ty{Type::Function{ret,..}=>Some((ret,&facts.path)),_=>None}
}
impl SourceOrigins{
 pub(super) fn record_expression_type(&mut self,node:&SExpr,path:&[String])->Result<()>{
  let Some(span)=node.span else{return Ok(());};if self.recorded_expression_origin(node).is_some(){self.expression_type_cache.types.remove(&span);return Ok(());}if self.collecting{return Ok(());}if self.expression_type_cache.types.contains_key(&span){return Ok(());}
  let Some((ty,scope))=actual(self,node,path)else{return Ok(());};let cost=measure_expression_type(ty,scope).map_err(|error|unsupported("Rust source expression type",&error,node.span))?;self.expression_type_cache.budget.reserve(cost).map_err(|error|unsupported("Rust source expression type",&error,node.span))?;
  let Some((ty,scope))=actual(self,node,path)else{return Ok(());};let facts=RecordedExpressionType{ty:ty.clone(),path:scope.to_vec(),proven:true};self.expression_type_cache.types.insert(span,facts);Ok(())
 }
 pub(super) fn recorded_expression_type(&self,node:&SExpr)->Option<&RecordedExpressionType>{node.span.and_then(|span|self.expression_type_cache.types.get(&span))}
 pub(super) fn recorded_expression_type_at_span(&self,span:Option<Span>)->Option<&RecordedExpressionType>{span.and_then(|span|self.expression_type_cache.types.get(&span))}
}
