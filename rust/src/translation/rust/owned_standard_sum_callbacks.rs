//! Actual owning receiver/capture-free callable origins into shared lazy plans.
use super::{Result,RustParser,SExpr,SNode,SourceOrigin,Span,Token,Type,unsupported};
use crate::translation::surface::{SPattern,SPatternNode,SRow};
use crate::translation::rust_owned_standard_sum_callback_rules as rules;
use crate::translation::rust_standard_sum_source_rules as sum_rules;
struct OwnedCallbackCapture{name:String,value:SExpr,origin:SourceOrigin}
fn fail(reason:&str,span:Option<Span>)->super::TranslationError{unsupported("Rust owned standard sum callback",reason,span)}
impl RustParser{
 fn owned_standard_sum_callback_local(&mut self,capture:&OwnedCallbackCapture,span:Option<Span>)->SExpr{let value=SExpr::new(SNode::Name{path:vec![capture.name.clone()]},span);self.origins.mark(&value,capture.origin.clone());value}
 pub(super) fn read_owned_standard_sum_callback(&mut self,receiver:&SExpr,method:&Token,path:&[String])->Result<Option<SExpr>>{
  if !self.ordinary_completion_context(){return Ok(None);}
  let actual=self.origins.origin(receiver,path);let Some(source)=self.origins.standard_sum_source_origin(&actual)else{return Ok(None);};let sum=source.standard_sum_source.as_deref().expect("actual source sum");
  if rules::read_rust_owned_standard_sum_callback_arity(&sum.owner,&method.value)<0.0{return Ok(None);}
  let mut direct=actual.clone();direct.wrappers.clear();if self.origins.standard_sum_ownership_facts(&direct).profile!="owned"{return Ok(None);}
  let version=rules::read_rust_owned_standard_sum_callback_version_error(&sum.owner,&method.value,self.source_version_minor);if !version.is_empty(){return Err(fail(&version,Some(method.span())));}
  let start=receiver.span.map_or(method.start,|span|span.start);self.cursor.expect("(",Some("owned standard sum callback"))?;
  let receiver_origin=self.owned_standard_sum_method_capture(receiver,path)?;let input=rules::read_rust_owned_standard_sum_callback_parameter(&sum.owner,&method.value);
  let parameters=if input<0.0{Vec::new()}else{vec![sum.arguments[input as usize].clone()]};let expected=if method.value=="unwrap_or_else"{Some(sum.arguments[0].clone())}else if method.value=="or_else"&&sum.owner=="std::option::Option"{Some(source.clone())}else{None};
  let callback=self.read_owned_callback_closure(path,&parameters,expected.as_ref())?;let callable=self.origins.origin(&callback,path);let output=callable.callable_result.as_deref();
  let count=match callable.ty.as_ref(){Some(Type::Function{params,..})=>params.len() as f64,_=>-1.0};let output_role=output.map_or_else(String::new,|origin|self.origins.standard_sum_ownership_facts(origin).profile);
  let error=rules::read_rust_owned_standard_sum_callback_source_error(callable.ty.as_ref().map_or("",Type::kind),count,parameters.len() as f64,callable.wrappers.len() as f64,callable.direct_copy_value(),&output_role);if !error.is_empty(){return Err(fail(&error,callback.span));}
  if callable.copy_closure_identity.is_some(){
   let (kind,names)=match &callback.node{SNode::Lambda{captures,..}=>("lambda",captures.len() as f64),_=>("",0.0)};let explicit_move=callback.span.and_then(|span|self.cursor.tokens.iter().find(|token|token.start==span.start)).is_some_and(|token|token.value=="move");
   let error=rules::read_rust_owned_standard_sum_callback_immediate_capture_error(callable.copy_closure_capture_count.map_or(-1.0,|count|count as f64),kind,callback.span.is_some()&&callable.copy_closure_identity==callback.span,explicit_move,names);if !error.is_empty(){return Err(fail(&error,callback.span));}
  }
  let inputs=callable.callable_parameters.as_deref().unwrap_or_default();if inputs.len()!=parameters.len(){return Err(fail("the callback must retain every actual parameter source contract",callback.span));}
  for (actual,expected) in inputs.iter().zip(&parameters){self.origins.argument(actual,expected,callback.span)?;let identity=self.origins.standard_sum_source_identity(actual);if identity.is_none()||identity!=self.origins.standard_sum_source_identity(expected){return Err(fail("the callback parameter must preserve its complete actual source identity",callback.span));}}
  let output=output.expect("shared callback source rule admitted a result");
  if let Some(expected)=expected.as_ref(){self.origins.argument(output,expected,callback.span)?;let identity=self.origins.standard_sum_source_identity(output);if identity.is_none()||identity!=self.origins.standard_sum_source_identity(expected){return Err(fail("the fallback callable must preserve the complete actual payload result contract",callback.span));}}
  self.cursor.eat(",");let close=self.cursor.expect(")",Some("owned standard sum callback"))?;let span=Some(Span::new(start,close.end));let policy=rules::read_rust_owned_standard_sum_callback_result(&sum.owner,&method.value);
  let result_type=if policy=="payload"{sum.arguments[0].clone()}else if policy=="bound sum"{
   let result=self.origins.standard_sum_source_origin(output);let bound=result.as_ref().and_then(|origin|origin.standard_sum_source.as_deref());let retained=rules::read_rust_owned_standard_sum_callback_retained_argument(&sum.owner,&method.value);
   let identity=retained<0.0||bound.is_some_and(|bound|{let actual=self.origins.standard_sum_source_identity(&sum.arguments[retained as usize]);actual.is_some()&&actual==bound.arguments.get(retained as usize).and_then(|origin|self.origins.standard_sum_source_identity(origin))});
   let error=rules::read_rust_owned_standard_sum_callback_bound_result_error(&sum.owner,bound.map_or("",|sum|sum.owner.as_str()),result.as_ref().map_or(-1.0,|origin|origin.wrappers.len() as f64),identity);if !error.is_empty(){return Err(fail(&error,callback.span));}result.expect("shared bound sum rule admitted actual source origin")
  }else{let arguments=rules::plan_rust_owned_standard_sum_callback_arguments(&sum.owner,&method.value).into_iter().map(|index|if index<0.0{output.clone()}else{sum.arguments[index as usize].clone()}).collect();self.instantiate_standard_sum_source(&policy,arguments,span)?};
  let moved=self.origins.consume_owned_binding(&receiver_origin,receiver.span)?;let result=self.owned_standard_sum_method_output(result_type,moved,true);
  let captures=[OwnedCallbackCapture{name:self.fresh_product_binding(),value:receiver.clone(),origin:receiver_origin},OwnedCallbackCapture{name:self.fresh_product_binding(),value:callback,origin:callable.clone()}];let mut rows=Vec::new();
  for variant in sum_rules::read_rust_standard_sum_source_constructors(&sum.owner){
   let plan=rules::plan_rust_owned_standard_sum_callback_branch(&sum.owner,&variant,&method.value);if plan.len()!=2{return Err(fail("the constructor has no actual owned callback branch",span));}
   let field=sum_rules::read_rust_standard_sum_source_field_argument(&sum.owner,&variant);let count=sum_rules::read_rust_standard_sum_source_constructor_arity(&sum.owner,&variant);let needs_payload=plan[1]=="payload"||plan[1]=="invoke"&&input>=0.0;let binding=(count==1.0&&needs_payload).then(||self.fresh_product_binding());
   let payload=binding.as_ref().map(|name|{let value=SExpr::new(SNode::Name{path:vec![name.clone()]},span);self.origins.mark(&value,sum.arguments[field as usize].clone());value});
   let body=match plan[1].as_str(){"payload"=>payload,"invoke"=>Some(SExpr::new(SNode::App{func:Box::new(self.owned_standard_sum_callback_local(&captures[1],span)),args:if input<0.0{Vec::new()}else{vec![payload.expect("actual callback payload")]}},span)),_=>None};
   let body=if plan[0].is_empty(){body.ok_or_else(||fail("the callback branch needs its actual result value",span))?}else{let constructor=result.standard_sum_source.as_deref().and_then(|sum|sum.constructor_paths.get(&plan[0])).cloned().ok_or_else(||fail("the callback result constructor needs its actual datatype owner",span))?;let name=SExpr::new(SNode::Name{path:constructor},span);if let Some(body)=body{SExpr::new(SNode::App{func:Box::new(name),args:vec![body]},span)}else{name}};
   let args=if count==1.0{vec![SPattern{node:if let Some(name)=binding{SPatternNode::Bind{name}}else{SPatternNode::Wild},span}]}else{Vec::new()};rows.push(SRow{patterns:vec![SPattern{node:SPatternNode::Ctor{path:sum.constructor_paths[&variant].clone(),args},span}],body,span});
  }
  let captured=self.owned_standard_sum_callback_local(&captures[0],span);let mut body=SExpr::new(SNode::Match{scrutinees:vec![captured],rows},span);for capture in captures.into_iter().rev(){body=SExpr::new(SNode::Let{name:capture.name,ty:capture.origin.ty.map(Into::into),value:Box::new(capture.value),body:Box::new(body)},span);}
  self.origins.mark(&body,result);Ok(Some(body))
 }
}
