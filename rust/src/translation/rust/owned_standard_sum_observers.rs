//! Original source root/loans into shared &self observer decisions. Constructor
//! wildcards introduce no payload binding, move, or receiver Copy evidence.
use super::{BOOL,Result,RustParser,SExpr,SNode,SourceOrigin,Span,Token,unsupported};
use crate::translation::surface::{SPattern,SPatternNode,SRow};
use crate::translation::source_signatures::SourceOwner;
use crate::translation::rust_owned_standard_sum_observer_rules as rules;
use crate::translation::rust_standard_sum_source_rules as sum_rules;
fn fail(reason:&str,span:Option<Span>)->super::TranslationError{unsupported("Rust owned standard sum observer",reason,span)}
impl RustParser{
 pub(super) fn read_owned_standard_sum_observer(&mut self,receiver:&SExpr,method:&Token,path:&[String])->Result<Option<SExpr>>{
  if !self.ordinary_completion_context(){return Ok(None);}
  let actual=self.origins.origin(receiver,path);let Some(source)=self.origins.standard_sum_source_origin(&actual)else{return Ok(None);};let sum=source.standard_sum_source.as_deref().expect("actual standard sum");
  if rules::read_rust_owned_standard_sum_observer_arity(&sum.owner,&method.value)<0.0{return Ok(None);}
  let mut direct=actual.clone();direct.wrappers.clear();let profile=self.origins.standard_sum_ownership_facts(&direct).profile;if !rules::accept_rust_owned_standard_sum_observer_profile(&profile,actual.wrappers.len() as f64){return Ok(None);}
  let version=rules::read_rust_owned_standard_sum_observer_version_error(&sum.owner,&method.value,self.source_version_minor);if !version.is_empty(){return Err(fail(&version,Some(method.span())));}
  self.origins.check_owned_origin(&actual,receiver.span)?;
  let error=rules::read_rust_owned_standard_sum_observer_source_error(actual.proven,actual.wrappers.len() as f64,actual.wrappers.iter().all(|layer|layer=="reference"),actual.source_lifetimes.len() as f64,self.origins.track_owned_bindings(),self.origins.available_owned_binding(&actual),actual.source_owned_result_proof,actual.source_borrowed_place,actual.source_binding_identity.is_some()||!actual.source_loan_binding_identities.is_empty());
  if !error.is_empty(){return Err(fail(&error,receiver.span));}
  self.cursor.expect("(",Some("owned standard sum observer"))?;let close=self.cursor.expect(")",Some("owned standard sum observer"))?;let span=Some(Span::new(receiver.span.map_or(method.start,|span|span.start),close.end));
  let mut rows=Vec::new();for variant in sum_rules::read_rust_standard_sum_source_constructors(&sum.owner){
   let branch=rules::read_rust_owned_standard_sum_observer_branch(&sum.owner,&variant,&method.value);if branch!="true"&&branch!="false"{return Err(fail("the actual constructor has no observer branch",span));}
   let count=sum_rules::read_rust_standard_sum_source_constructor_arity(&sum.owner,&variant);let args=if count==1.0{vec![SPattern{node:SPatternNode::Wild,span}]}else{Vec::new()};
   rows.push(SRow{patterns:vec![SPattern{node:SPatternNode::Ctor{path:sum.constructor_paths[&variant].clone(),args},span}],body:SExpr::new(SNode::Bool{value:branch=="true"},span),span});
  }
  let result=SExpr::new(SNode::Match{scrutinees:vec![receiver.clone()],rows},span);let mut origin=SourceOrigin::with_type(BOOL,path,Vec::new());origin.source_owner=Some(SourceOwner{kind:"primitive".into(),path:vec!["bool".into()],arguments:Vec::new()});self.origins.mark(&result,origin);Ok(Some(result))
 }
}
