//! Actual ownership/argument order into the existing shared predicate policy.
use super::{BOOL,Options,Result,RustParser,SExpr,SNode,SourceOrigin,Span,Token,unsupported};
use crate::translation::source_signatures::SourceOwner;
use crate::translation::surface::{SPattern,SPatternNode,SRow};
use crate::translation::rust_standard_sum_predicate_rules as rules;
use crate::translation::rust_owned_standard_sum_predicate_rules as owned_rules;
use crate::translation::rust_standard_sum_source_rules as sum_rules;
struct OwnedPredicateCapture{role:String,name:String,value:SExpr,origin:SourceOrigin}
fn fail(reason:&str,span:Option<Span>)->super::TranslationError{unsupported("Rust owned standard sum predicate",reason,span)}
impl RustParser{
 fn owned_standard_sum_predicate_local(&mut self,captures:&[OwnedPredicateCapture],role:&str,span:Option<Span>)->Result<SExpr>{let capture=captures.iter().find(|capture|capture.role==role).ok_or_else(||fail("the represented source capture is absent",span))?;let value=SExpr::new(SNode::Name{path:vec![capture.name.clone()]},span);self.origins.mark(&value,capture.origin.clone());Ok(value)}
 pub(super) fn read_owned_standard_sum_predicate(&mut self,receiver:&SExpr,method:&Token,path:&[String])->Result<Option<SExpr>>{
  if !self.ordinary_completion_context(){return Ok(None);}
  let actual=self.origins.origin(receiver,path);let Some(source)=self.origins.standard_sum_source_origin(&actual)else{return Ok(None);};let sum=source.standard_sum_source.as_deref().expect("actual standard sum");let arity=rules::read_rust_standard_sum_predicate_arity(&sum.owner,&method.value);if arity<0.0{return Ok(None);}
  let mut direct=actual.clone();direct.wrappers.clear();if self.origins.standard_sum_ownership_facts(&direct).profile!="owned"{return Ok(None);}
  let version=rules::read_rust_standard_sum_predicate_version_error(&sum.owner,&method.value,self.source_version_minor);if !version.is_empty(){return Err(fail(&version,Some(method.span())));}
  let start=receiver.span.map_or(method.start,|span|span.start);self.cursor.expect("(",Some("owned standard sum predicate"))?;
  let receiver_origin=self.owned_standard_sum_method_capture(receiver,path)?;let mut captures=vec![OwnedPredicateCapture{role:"receiver".into(),name:self.fresh_product_binding(),value:receiver.clone(),origin:receiver_origin.clone()}];let mut boolean=SourceOrigin::with_type(BOOL,path,Vec::new());boolean.source_owner=Some(SourceOwner{kind:"primitive".into(),path:vec!["bool".into()],arguments:Vec::new()});let mut result_type=(arity==1.0).then(||boolean.clone());let mut owned_default=None;
  let roles=rules::read_rust_standard_sum_predicate_capture_roles(&method.value);
  for (at,role) in roles.into_iter().enumerate().skip(1){
   if at>1{self.cursor.expect(",",Some("owned standard sum predicate arguments"))?;}
   if role=="default"{
    let previous=self.standard_sum_source.expected_origin.take();let parsed=self.expr(path,Options::default());self.standard_sum_source.expected_origin=previous;let value=parsed?;let mut origin=self.origins.origin(&value,path);let profile=self.origins.standard_sum_ownership_facts(&origin).profile;
    let error=owned_rules::read_rust_owned_standard_sum_predicate_result_error(&profile,self.origins.standard_sum_source_identity(&origin).is_some());if !error.is_empty(){return Err(fail(&error,value.span));}
    if profile=="owned"{origin=self.owned_standard_sum_method_capture(&value,path)?;owned_default=Some(origin.clone());}
    result_type=Some(origin.clone());captures.push(OwnedPredicateCapture{role,name:self.fresh_product_binding(),value,origin});
   }else{
    let field=rules::read_rust_standard_sum_predicate_callback_argument(&sum.owner,&method.value,&role);if field == -2.0{return Err(fail("the source callback role is absent",Some(method.span())));}
    let parameters=if field<0.0{Vec::new()}else{vec![sum.arguments[field as usize].clone()]};let expected=if role=="predicate"{Some(&boolean)}else if role=="mapper"{result_type.as_ref()}else{None};let checked=self.read_owned_standard_sum_callback_value(path,&parameters,expected)?;
    if role=="fallback"{let error=owned_rules::read_rust_owned_standard_sum_predicate_result_error(&self.origins.standard_sum_ownership_facts(&checked.result).profile,self.origins.standard_sum_source_identity(&checked.result).is_some());if !error.is_empty(){return Err(fail(&error,checked.value.span));}result_type=Some(checked.result);}
    captures.push(OwnedPredicateCapture{role,name:self.fresh_product_binding(),value:checked.value,origin:checked.origin});
   }
  }
  self.cursor.eat(",");let close=self.cursor.expect(")",Some("owned standard sum predicate"))?;let span=Some(Span::new(start,close.end));let result_type=result_type.ok_or_else(||fail("the source operands must establish a complete actual result",span))?;
  let receiver_moved=self.origins.consume_owned_binding(&receiver_origin,receiver.span)?;let default_moved=if let Some(default)=owned_default{self.origins.consume_owned_binding(&default,span)?}else{true};let result=self.owned_standard_sum_method_output(result_type,receiver_moved,default_moved);let mut rows=Vec::new();
  for variant in sum_rules::read_rust_standard_sum_source_constructors(&sum.owner){
   let branch=rules::read_rust_standard_sum_predicate_branch(&sum.owner,&variant,&method.value);let field=sum_rules::read_rust_standard_sum_source_field_argument(&sum.owner,&variant);let count=sum_rules::read_rust_standard_sum_source_constructor_arity(&sum.owner,&variant);let callback_field=rules::read_rust_standard_sum_predicate_callback_argument(&sum.owner,&method.value,&branch);let binding=(count==1.0&&callback_field>=0.0).then(||self.fresh_product_binding());
   let payload=binding.as_ref().map(|name|{let value=SExpr::new(SNode::Name{path:vec![name.clone()]},span);self.origins.mark(&value,sum.arguments[field as usize].clone());value});
   let body=match branch.as_str(){"true"|"false"=>SExpr::new(SNode::Bool{value:branch=="true"},span),"default"=>self.owned_standard_sum_predicate_local(&captures,"default",span)?,"mapper"|"fallback"|"predicate"=>SExpr::new(SNode::App{func:Box::new(self.owned_standard_sum_predicate_local(&captures,&branch,span)?),args:if callback_field<0.0{Vec::new()}else{vec![payload.expect("actual source callback payload")]}},span),_=>return Err(fail("the actual source constructor has no method branch",span))};
   let args=if count==1.0{vec![SPattern{node:if let Some(name)=binding{SPatternNode::Bind{name}}else{SPatternNode::Wild},span}]}else{Vec::new()};rows.push(SRow{patterns:vec![SPattern{node:SPatternNode::Ctor{path:sum.constructor_paths[&variant].clone(),args},span}],body,span});
  }
  let captured=self.owned_standard_sum_predicate_local(&captures,"receiver",span)?;let mut body=SExpr::new(SNode::Match{scrutinees:vec![captured],rows},span);for capture in captures.into_iter().rev(){body=SExpr::new(SNode::Let{name:capture.name,ty:capture.origin.ty.map(Into::into),value:Box::new(capture.value),body:Box::new(body)},span);}self.origins.mark(&body,result);Ok(Some(body))
 }
}
