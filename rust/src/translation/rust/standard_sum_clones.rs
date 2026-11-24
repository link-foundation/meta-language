//! Ordinary immutable value IR already clones non-Copy Rust variable uses.
use super::{Result,RustParser,SExpr,SNode,Span,Token,unsupported};
use crate::translation::rust_owned_standard_sum_method_rules as rules;
impl RustParser{
 pub(super) fn read_standard_sum_clone(&mut self,receiver:&SExpr,method:&Token,path:&[String])->Result<Option<SExpr>>{
  if !self.ordinary_completion_context()||method.value!="clone"{return Ok(None);}
  let actual=self.origins.origin(receiver,path);let Some(source)=self.origins.standard_sum_source_origin(&actual)else{return Ok(None);};
  self.origins.check_owned_origin(&actual,receiver.span)?;
  let role=self.origins.standard_sum_ownership_facts(&source).profile;let error=rules::read_rust_standard_sum_clone_source_error(&source.standard_sum_source.as_deref().expect("actual source sum").owner,&role,actual.wrappers.len() as f64,actual.source_borrowed_place,self.origins.track_owned_bindings(),self.origins.available_owned_binding(&actual),actual.source_owned_result_proof);
  if !error.is_empty(){return Err(unsupported("Rust standard sum Clone",&error,receiver.span));}
  let implementation_error=crate::translation::rust_nominal_ownership_rules::read_rust_nominal_clone_implementation_error(self.origins.source_clone_implementation(&source));if !implementation_error.is_empty(){return Err(unsupported("Rust standard sum Clone",&implementation_error,receiver.span));}
  self.cursor.expect("(",Some("standard sum Clone"))?;let close=self.cursor.expect(")",Some("standard sum Clone"))?;let span=Some(Span::new(receiver.span.map_or(method.start,|span|span.start),close.end));let name=self.fresh_product_binding();
  let local=SExpr::new(SNode::Name{path:vec![name.clone()]},span);self.origins.mark(&local,actual.clone());
  let body=SExpr::new(SNode::Let{name,ty:source.ty.clone().map(Into::into),value:Box::new(receiver.clone()),body:Box::new(local)},span);
  let result=super::copy_closure_lifetimes::carry_value_borrows(self.owned_standard_sum_method_output(source,true,true),&actual);self.origins.mark(&body,result);Ok(Some(body))
 }
}
