//! Real owned receiver/default transitions into source-generated branch plans.
use super::{Options,Result,RustParser,SExpr,SNode,SourceDeclaration,SourceOrigin,Span,Token,unsupported};
use crate::translation::surface::{SPattern,SPatternNode,SRow};
use crate::translation::rust_owned_standard_sum_method_rules as rules;
use crate::translation::rust_standard_sum_source_rules as sum_rules;
struct OwnedMethodCapture{name:String,value:SExpr,origin:SourceOrigin}
fn fail(reason:&str,span:Option<Span>)->super::TranslationError{unsupported("Rust consuming owned standard sum method",reason,span)}
impl RustParser{
 pub(super) fn owned_standard_sum_method_capture(&mut self,value:&SExpr,path:&[String])->Result<SourceOrigin>{
  let before=self.origins.origin(value,path);let role=self.origins.standard_sum_ownership_facts(&before).profile;
  let declared=match &value.node{SNode::App{func,..}=>match &func.node{SNode::Name{path:names}=>self.origins.lookup_value_owner(names,path).and_then(|entry|match entry.declaration{SourceDeclaration::Function{result,..}=>Some(result),_=>None}),_=>None},_=>None};
  let identity=self.origins.standard_sum_source_identity(&before);let result_identity=declared.as_ref().and_then(|origin|self.origins.standard_sum_source_identity(origin));
  if before.source_binding_identity.is_none()&&rules::accept_rust_owned_standard_sum_method_call_production(&role,declared.as_ref().is_some_and(|origin|self.origins.standard_sum_ownership_facts(origin).profile=="owned"),self.origins.owned_standard_sum_recorded_call(value),identity.is_some()&&identity==result_identity,before.wrappers.len() as f64,before.source_borrowed_place){let mut production=before.clone();production.source_owned_result_proof=true;self.origins.mark(value,production);}
  let moved=self.origins.transfer_owned_result(value,path)?;
  let error=rules::read_rust_owned_standard_sum_method_capture_error(&role,before.wrappers.len() as f64,before.source_borrowed_place,self.origins.track_owned_bindings(),before.source_binding_identity.as_deref().unwrap_or(""),moved.source_binding_identity.as_deref().unwrap_or(""),self.origins.available_owned_binding(&moved),moved.source_owned_result_proof);
  if !error.is_empty(){return Err(fail(&error,value.span));}Ok(moved)
 }
 pub(super) fn owned_standard_sum_method_output(&self,mut origin:SourceOrigin,receiver:bool,argument:bool)->SourceOrigin{
  let role=self.origins.standard_sum_ownership_facts(&origin).profile;
  origin.source_binding_identity=None;origin.source_place_binding_identity=None;origin.source_lexical_binding_identity=None;origin.source_loan_binding_identities.clear();origin.source_place_loan_binding_identities.clear();origin.source_extending_temporary_bindings.clear();origin.source_borrowed_place=false;origin.source_standard_sum_borrowed_field=false;origin.source_standard_sum_pattern_move=false;
  origin.source_owned_result_proof=rules::accept_rust_owned_standard_sum_method_production(&role,receiver,argument);origin
 }
 fn owned_standard_sum_method_local(&mut self,capture:&OwnedMethodCapture,span:Option<Span>)->SExpr{
  let value=SExpr::new(SNode::Name{path:vec![capture.name.clone()]},span);self.origins.mark(&value,capture.origin.clone());value
 }
 pub(super) fn read_owned_standard_sum_method(&mut self,receiver:&SExpr,method:&Token,path:&[String])->Result<Option<SExpr>>{
  if !self.ordinary_completion_context(){return Ok(None);}
  let actual=self.origins.origin(receiver,path);let Some(source)=self.origins.standard_sum_source_origin(&actual)else{return Ok(None);};let sum=source.standard_sum_source.as_deref().expect("actual source sum");
  let arity=rules::read_rust_owned_standard_sum_method_arity(&sum.owner,&method.value);if arity<0.0{return Ok(None);}
  let mut direct=actual.clone();direct.wrappers.clear();let receiver_mode=rules::read_rust_owned_standard_sum_method_receiver_mode(&sum.owner,&method.value,&self.origins.standard_sum_ownership_facts(&direct).profile);if receiver_mode.is_empty(){return Ok(None);}
  let version=rules::read_rust_owned_standard_sum_method_version_error(&sum.owner,&method.value,self.source_version_minor);if !version.is_empty(){return Err(fail(&version,Some(method.span())));}
  let start=receiver.span.map_or(method.start,|span|span.start);self.cursor.expect("(",Some("consuming owned standard sum method"))?;
  // Moving self precedes parsing the eager argument, including its reads.
  let receiver_origin=if receiver_mode=="owned"{self.owned_standard_sum_method_capture(receiver,path)?}else{self.origins.check_owned_origin(&actual,receiver.span)?;if !actual.wrappers.is_empty()||!actual.direct_copy_value(){return Err(fail("the Copy receiver needs its actual direct immutable source proof",receiver.span));}actual.clone()};
  let mut captures=vec![OwnedMethodCapture{name:self.fresh_product_binding(),value:receiver.clone(),origin:receiver_origin.clone()}];let mut argument_origin=None;
  if arity==1.0{
   let previous=self.standard_sum_source.expected_origin.take();self.standard_sum_source.expected_origin=if method.value=="unwrap_or"{Some(sum.arguments[0].clone())}else{None};
   let parsed=self.expr(path,Options::default());self.standard_sum_source.expected_origin=previous;let value=parsed?;let mut origin=self.origins.origin(&value,path);
   if method.value=="unwrap_or"{
    self.origins.argument(&origin,&sum.arguments[0],value.span)?;let identity=self.origins.standard_sum_source_identity(&origin);
    if identity.is_none()||identity!=self.origins.standard_sum_source_identity(&sum.arguments[0]){return Err(fail("the eager fallback must preserve the complete actual payload source type",value.span));}
   }
   let role=self.origins.standard_sum_ownership_facts(&origin).profile;
   if !rules::accept_rust_owned_standard_sum_method_argument(&role)||self.origins.standard_sum_source_identity(&origin).is_none(){return Err(fail("the eager argument needs its actual complete Copy or supported owned source type",value.span));}
   if role=="owned"{origin=self.owned_standard_sum_method_capture(&value,path)?;}
   argument_origin=Some(origin.clone());captures.push(OwnedMethodCapture{name:self.fresh_product_binding(),value,origin});self.cursor.eat(",");
  }
  let close=self.cursor.expect(")",Some("consuming owned standard sum method"))?;let span=Some(Span::new(start,close.end));let policy=rules::read_rust_owned_standard_sum_method_result(&sum.owner,&method.value);
  let result_type=if policy=="payload"{sum.arguments[0].clone()}else{
   let arguments=rules::plan_rust_owned_standard_sum_method_arguments(&sum.owner,&method.value).into_iter().map(|index|if index<0.0{argument_origin.clone()}else{sum.arguments.get(index as usize).cloned()}).collect::<Option<Vec<_>>>().ok_or_else(||fail("the consuming result needs every actual source argument",span))?;
   self.instantiate_standard_sum_source(&policy,arguments,span)?
  };
  let receiver_moved=receiver_mode=="copy"||self.origins.consume_owned_binding(&receiver_origin,receiver.span)?;let mut argument_moved=true;
  if let Some(origin)=argument_origin.as_ref(){if self.origins.standard_sum_ownership_facts(origin).profile=="owned"{argument_moved=self.origins.consume_owned_binding(origin,captures[1].value.span)?;}}
  let sources=if let Some(argument)=argument_origin.as_ref(){vec![&actual,argument]}else{vec![&actual]};
  let result=super::copy_closure_lifetimes::carry_value_union(self.owned_standard_sum_method_output(result_type,receiver_moved,argument_moved),&sources);let mut rows=Vec::new();
  for variant in sum_rules::read_rust_standard_sum_source_constructors(&sum.owner){
   let plan=rules::plan_rust_owned_standard_sum_method_branch(&sum.owner,&variant,&method.value);if plan.len()!=2{return Err(fail("the actual constructor has no consuming branch",span));}
   let count=sum_rules::read_rust_standard_sum_source_constructor_arity(&sum.owner,&variant);let field=sum_rules::read_rust_standard_sum_source_field_argument(&sum.owner,&variant);let binding=(count==1.0&&plan[1]=="payload").then(||self.fresh_product_binding());
   let payload=binding.as_ref().map(|name|{let value=SExpr::new(SNode::Name{path:vec![name.clone()]},span);self.origins.mark(&value,sum.arguments[field as usize].clone());value});
   let value=match plan[1].as_str(){"payload"=>payload,"argument"=>Some(self.owned_standard_sum_method_local(&captures[1],span)),"standard default"=>Some(self.standard_default_value(&sum.arguments[0],path,span)?),_=>None};
   let body=if plan[0].is_empty(){value.ok_or_else(||fail("the consuming branch must retain its actual result value",span))?}else{
    let constructor=result.standard_sum_source.as_deref().and_then(|sum|sum.constructor_paths.get(&plan[0])).cloned().ok_or_else(||fail("the consuming result constructor has no actual datatype owner",span))?;
    let name=SExpr::new(SNode::Name{path:constructor},span);if let Some(value)=value{SExpr::new(SNode::App{func:Box::new(name),args:vec![value]},span)}else{name}
   };
   let args=if count==1.0{vec![SPattern{node:if let Some(name)=binding{SPatternNode::Bind{name}}else{SPatternNode::Wild},span}]}else{Vec::new()};
   rows.push(SRow{patterns:vec![SPattern{node:SPatternNode::Ctor{path:sum.constructor_paths[&variant].clone(),args},span}],body,span});
  }
  let captured=self.owned_standard_sum_method_local(&captures[0],span);let mut body=SExpr::new(SNode::Match{scrutinees:vec![captured],rows},span);
  for capture in captures.into_iter().rev(){body=SExpr::new(SNode::Let{name:capture.name,ty:capture.origin.ty.map(Into::into),value:Box::new(capture.value),body:Box::new(body)},span);}
  self.origins.mark(&body,result);Ok(Some(body))
 }
}
