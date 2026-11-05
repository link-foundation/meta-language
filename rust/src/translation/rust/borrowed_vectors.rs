//! TYPE-only element source positions; value loans are transported separately.
use super::{Result,SourceOrigin,SourceOrigins,RustParser,Span,SExpr,SNode,SArrayItem,Type,type_error,unsupported};
use crate::translation::source_signatures::{SourceTypeSignature,SourceOwner};
use crate::translation::rust_borrowed_vector_rules as rules;

pub(super) fn argument_regions(origin:&SourceOrigin)->Vec<String>{
 let mut loans=super::copy_closure_lifetimes::value_borrows(origin);
 if origin.wrappers.first().is_some_and(|wrapper|wrapper=="reference"){
  use crate::translation::rust_owned_binding_rules::{merge_rust_owned_loan_bindings as merge,project_rust_owned_loan_bindings as project};
  loans=merge(&loans,&project(origin.source_binding_identity.as_deref().unwrap_or(""),&origin.source_loan_binding_identities));loans=merge(&loans,&project(origin.source_place_binding_identity.as_deref().unwrap_or(""),&origin.source_place_loan_binding_identities));
  if loans.is_empty(){if let Some(identity)=&origin.source_lexical_binding_identity{loans.push(identity.clone());}}
 }
 loans
}


fn result_parameters(result:&SourceOrigin,parameters:&[&SourceOrigin])->Vec<f64>{
 rules::plan_rust_borrowed_vector_result_parameters(result.array_element_position.as_deref().and_then(|position|position.lifetimes.first()).and_then(Option::as_deref).unwrap_or(""),&parameters.iter().map(|value|value.wrappers.len()==1&&value.wrappers[0]=="reference"||value.array_element_position.is_some()).collect::<Vec<_>>(),&parameters.iter().map(|value|value.array_element_position.as_deref().and_then(|position|position.lifetimes.first()).cloned().flatten().or_else(||value.source_lifetimes.first().cloned().flatten()).unwrap_or_default()).collect::<Vec<_>>())
}

pub(super) fn position_origin(position:&SourceTypeSignature,path:&[String])->SourceOrigin{
 let mut origin=SourceOrigin::value();origin.ty=Some(position.ty.clone());origin.path=path.to_vec();origin.wrappers=position.wrappers.clone();origin.source_lifetimes=position.lifetimes.clone();origin.source_owner=Some(position.owner.clone());origin.array_element_position=position.array_element_position.clone();origin.copy_evidence=position.wrappers.first().is_some_and(|wrapper|wrapper=="reference").then_some(());origin
}
fn project_box_owner(mut owner:&SourceOwner,count:usize)->Option<&SourceOwner>{for _ in 0..count{if !crate::translation::rust_owned_recursive_datatype_rules::accept_rust_owned_box_type_node(&["box".into()],&owner.kind,&owner.path.join("::"),owner.arguments.len() as f64){return None;}owner=&owner.arguments[0];}Some(owner)}
impl RustParser{
 pub(super) fn project_borrowed_iterator_element(&mut self,node:&mut SExpr,path:&[String],expected:Option<&SourceOrigin>)->Result<SourceOrigin>{self.project_borrowed_vector_element(node,path,expected)}
 pub(super) fn read_borrowed_vector_empty_source_constructor(&mut self,segments:&[String],args:&[SExpr],path:&[String],span:Option<Span>)->Result<Option<SExpr>>{if segments.is_empty(){return Ok(None);}let resolved=self.resolve(&segments[..segments.len()-1],path);let selected=self.standard_sum_source.expected_origin.as_ref().is_some_and(|o|self.origins.borrowed_vector_source_origin(o));let disposition=rules::read_rust_borrowed_vector_empty_constructor(selected,&resolved.join("::"),segments.last().map(String::as_str).unwrap_or(""),args.len() as f64);if disposition.is_empty(){return Ok(None);}self.origins.vector(&segments[..segments.len()-1],&resolved,path,span)?;if disposition=="arity"{return Err(type_error("Vec::new expects 0 arguments",span));}Ok(Some(self.create_borrowed_vector_literal(Vec::new(),path,span)?))}
 pub(super) fn borrowed_vector_signature_phase(&self,origin:&SourceOrigin)->String{if !self.origins.borrowed_vector_source_origin(origin){return String::new();}self.borrowed_vector_element_admission(&position_origin(origin.array_element_position.as_deref().expect("actual retained source position"),&origin.path))}
 pub(super) fn guard_borrowed_vector_observation(&self,node:&SExpr,path:&[String],operation:&str,span:Option<Span>)->Result<()>{
  if !self.origins.borrowed_vector_source_enabled{return Ok(());}let origin=self.origins.origin(node,path);let mut pending=vec![&origin];let mut visits=0;
  while let Some(value)=pending.pop(){visits+=1;let budget=rules::read_rust_borrowed_vector_position_budget_error(visits as f64,0.0);if !budget.is_empty(){return Err(unsupported("Rust borrowed vector source",&budget,span));}let error=rules::read_rust_borrowed_vector_observation_error(value.array_element_position.is_some(),operation);if !error.is_empty(){return Err(unsupported("Rust borrowed vector source",&error,span));}if value.array_element_position.is_some(){self.origins.check_owned_origin(value,span)?;}if let Some(fields)=value.fields.as_deref(){pending.extend(fields);}}
  Ok(())
 }
 fn project_borrowed_vector_element(&mut self,node:&mut SExpr,path:&[String],expected:Option<&SourceOrigin>)->Result<SourceOrigin>{let actual=self.origins.origin(node,path);let Some(expected)=expected else{return Ok(actual)};if actual.wrappers.first().map(String::as_str)!=Some("reference")||expected.wrappers.first().map(String::as_str)!=Some("reference")||actual.wrappers.len()<=expected.wrappers.len(){return Ok(actual);}self.origins.reserve_owned_container_copies(&[&actual,expected])?;let Some(mut projected)=self.origins.standard_sum_qualified_origin(&actual)else{return Ok(actual)};let Some(target)=self.origins.standard_sum_qualified_origin(expected)else{return Ok(actual)};let drop=projected.wrappers.len()-target.wrappers.len();let owner=projected.source_owner.as_ref().and_then(|o|project_box_owner(o,drop));let expanded=projected.expanded_source_owner.as_ref().and_then(|o|project_box_owner(o,drop));let canonical=owner.is_some()&&(projected.expanded_source_owner.is_none()||expanded.is_some());let same=owner.zip(target.source_owner.as_ref()).is_some_and(|(a,b)|a==b);if !crate::translation::rust_borrowed_source_region_rules::accept_rust_borrowed_source_box_reference(&projected.wrappers,&target.wrappers,canonical,same){return Ok(actual);}let owner=owner.cloned();let expanded=expanded.cloned();projected.wrappers=target.wrappers.clone();projected.source_owner=owner;if projected.expanded_source_owner.is_some(){projected.expanded_source_owner=expanded;}let count=drop.min(projected.source_box_layers.len());projected.source_box_layers.drain(..count);if !self.origins.same_origin(&projected,&target){return Err(type_error("a contextual Box reference needs its actual matching referent type",node.span));}self.origins.mark(node,projected.clone());self.note_borrowed_expression_source_position(node,path)?;Ok(projected)}
 pub(super) fn borrowed_vector_literal_selected(&self,args:&[SExpr],path:&[String])->bool{if !self.origins.borrowed_vector_source_enabled{return false;}let first=args.first().map(|argument|self.origins.origin(argument,path));let expected=self.standard_sum_source.expected_origin.as_ref().is_some_and(|origin|self.origins.borrowed_vector_source_origin(origin));rules::accept_rust_borrowed_vector_literal_context(true,expected,first.as_ref().is_some_and(|o|o.wrappers.first().is_some_and(|w|w=="reference")),args.len() as f64)||first.as_ref().is_some_and(|origin|self.borrowed_vector_element_admission(origin)=="admit")}
 pub(super) fn create_borrowed_vector_literal(&mut self,mut args:Vec<SExpr>,path:&[String],span:Option<Span>)->Result<SExpr>{
  let error=rules::read_rust_borrowed_vector_element_count_error(args.len() as f64);if !error.is_empty(){return Err(unsupported("Rust borrowed vector source",&error,span));}
  let expected=self.standard_sum_source.expected_origin.as_ref().filter(|origin|self.origins.borrowed_vector_source_origin(origin));let target=if let Some(expected)=expected{self.origins.reserve_owned_container_copies(&[expected])?;expected.array_element_position.as_deref().map(|p|position_origin(p,&expected.path))}else{None};
  for argument in &mut args{self.project_borrowed_vector_element(argument,path,target.as_ref())?;}
  let actual=match args.first(){Some(first)=>self.origins.origin(first,path),None=>target.ok_or_else(||type_error("a borrowed vector literal needs its actual first reference or expected element context",span))?};if self.borrowed_vector_element_admission(&actual)!="admit"{return Err(unsupported("Rust borrowed vector source","the contextual element needs its actual closed immutable reference position",span));}self.origins.reserve_owned_container_copies(&[&actual])?;let actual=self.origins.standard_sum_qualified_origin(&actual).ok_or_else(||type_error("a borrowed vector referent needs its actual qualified source owner",span))?;
  let position=source_type_position(&actual)?;let ty=actual.ty.as_ref().ok_or_else(||type_error("a borrowed vector needs its actual referent type",span))?;let owner=actual.source_owner.as_ref().ok_or_else(||type_error("a borrowed vector needs its actual referent owner",span))?;
  let mut origin=SourceOrigin::with_type(Type::Array{element:Box::new(ty.clone()),reference:false},path,Vec::new());origin.array_source_model=Some("owned vector");origin.array_element_copy_evidence=true;origin.array_element_position=Some(Box::new(position));origin.source_owner=Some(SourceOwner{kind:"library".into(),path:vec!["std".into(),"vec".into(),"Vec".into()],arguments:vec![owner.clone()]});
  let node=SExpr::new(SNode::Array{items:args.into_iter().map(|value|SArrayItem{value,spread:false}).collect(),element:None},span);self.origins.capture_borrowed_vector_constructor(&node,origin)?;Ok(node)
 }
 pub(super) fn borrowed_vector_element_admission(&self,origin:&SourceOrigin)->String{
  let reference=rules::accept_rust_borrowed_vector_element(self.origins.borrowed_vector_source_enabled,self.source_version_minor,&origin.wrappers,&origin.source_lifetimes.iter().map(|value|value.clone().unwrap_or_default()).collect::<Vec<_>>(),origin.proven,false);if !reference{return String::new();}
  // A borrowed wrapper view does not install any referent Copy/Clone proof.
  let facts=self.origins.owned_container_referent_facts(origin);
  rules::read_rust_borrowed_vector_element_phase(self.origins.borrowed_vector_source_enabled,reference,self.header_discovery,self.origins.nominal_ownership_scope_complete,facts.closed)
 }
 pub(super) fn seed_borrowed_vector_parameter_regions(&mut self,names:&[String]){
  if !self.origins.borrowed_vector_source_enabled{return;}
  for name in names{let Some(value)=self.origins.scope.get(name)else{continue;};let Some(identity)=value.source_lexical_binding_identity.as_ref()else{continue;};if !(value.wrappers.len()==1&&value.wrappers[0]=="reference"||self.origins.borrowed_vector_source_origin(value)){continue;}
   let loans=crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&super::copy_closure_lifetimes::value_borrows(value),&[identity.clone()]);if let Some(value)=self.origins.scope.get_mut(name){value.source_value_borrow_bindings=loans;}
  }
 }
 pub(super) fn validate_borrowed_vector_function_result_regions(&self,body:&SExpr,result:&SourceOrigin,names:&[String])->Result<Option<Vec<String>>>{
  if !self.origins.borrowed_vector_source_origin(result){return Ok(None);}let actual=self.origins.origin(body,&self.module_path);let parameters=names.iter().map(|name|self.origins.scope.get(name).ok_or_else(||type_error("a borrowed vector result needs its actual parameter scope",body.span))).collect::<Result<Vec<_>>>()?;let plan=result_parameters(result,&parameters);let mut allowed=Vec::new();
  for index in &plan{if *index>=0.0{allowed=crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&allowed,&argument_regions(&parameters[*index as usize]));}}
  let loans=super::copy_closure_lifetimes::value_borrows(&actual);let error=rules::read_rust_borrowed_vector_result_error(self.origins.same_borrowed_vector_type(&actual,result)&&actual.array_element_position.as_ref().zip(result.array_element_position.as_ref()).is_some_and(|(a,b)|a.owner==b.owner),actual.source_owned_result_proof||self.origins.available_owned_binding(&actual),!plan.contains(&-1.0)&&loans.iter().all(|identity|allowed.contains(identity)),self.origins.collecting);if !error.is_empty(){return Err(unsupported("Rust borrowed vector source",&error,body.span));}self.origins.validate_copy_closure_value_borrows(&actual,body.span)?;Ok(Some(loans))
 }
 pub(super) fn validate_borrowed_vector_function_body(&self,body:&SExpr,result:&SourceOrigin,names:&[String])->Result<bool>{Ok(self.validate_borrowed_vector_function_result_regions(body,result,names)?.is_some())}
}
impl SourceOrigins{
 pub(super) fn validate_borrowed_vector_call_regions(&self,args:&[SExpr],path:&[String],parameters:&[SourceOrigin],result:&SourceOrigin,span:Option<Span>)->Result<()>{
  if !self.borrowed_vector_source_origin(result)||self.collecting{return Ok(());}let plan=self.borrowed_vector_result_parameters(result,parameters);if plan.contains(&-1.0){return Err(unsupported("Rust borrowed vector source","a borrowed vector call needs its actual source parameter region plan",span));}
  for index in plan{let arg=args.get(index as usize).ok_or_else(||unsupported("Rust borrowed vector source","a borrowed vector call needs every selected actual argument",span))?;let actual=self.origin(arg,path);let regions=argument_regions(&actual);let error=rules::read_rust_borrowed_vector_call_region_error(actual.wrappers.first().is_some_and(|wrapper|wrapper=="reference"),regions.len() as f64,false);if !error.is_empty(){return Err(unsupported("Rust borrowed vector source",&error,arg.span));}self.check_owned_origin(&actual,arg.span)?;}Ok(())
 }
 pub(super) fn same_borrowed_vector_type(&self,left:&SourceOrigin,right:&SourceOrigin)->bool{
  if !self.borrowed_vector_source_origin(left)||!self.borrowed_vector_source_origin(right){return false;}let (Some(Type::Array{reference:a,..}),Some(Type::Array{reference:b,..}))=(left.ty.as_ref(),right.ty.as_ref())else{return false;};a==b&&left.array_element_position.as_deref().zip(right.array_element_position.as_deref()).is_some_and(|(a,b)|self.same_origin(&position_origin(a,&left.path),&position_origin(b,&right.path)))
 }
 pub(super) fn borrowed_vector_produced_value(&self,origin:&SourceOrigin)->bool{rules::accept_rust_borrowed_vector_produced_value(self.borrowed_vector_source_enabled,self.borrowed_vector_source_origin(origin),origin.proven,origin.source_owned_result_proof)}
 pub(super) fn borrowed_vector_source_origin(&self,origin:&SourceOrigin)->bool{
  let owner=origin.expanded_source_owner.as_ref().or(origin.source_owner.as_ref());rules::accept_rust_borrowed_vector_shape(self.borrowed_vector_source_enabled,origin.ty.as_ref().map_or("",super::Type::kind),owner.map_or("",|owner|owner.kind.as_str()),&owner.map_or_else(String::new,|owner|owner.path.join("::")),owner.map_or(-1.0,|owner|owner.arguments.len() as f64),origin.wrappers.len() as f64,origin.array_element_position.as_deref().is_some_and(|position|rules::accept_rust_borrowed_vector_position(&position.wrappers,position.lifetimes.len() as f64,position.array_element_position.is_some())))
 }
 pub(super) fn borrowed_vector_result_parameters(&self,result:&SourceOrigin,parameters:&[SourceOrigin])->Vec<f64>{
  result_parameters(result,&parameters.iter().collect::<Vec<_>>())
 }
 pub(super) fn validate_borrowed_vector_argument(&self,actual:&SourceOrigin,expected:&SourceOrigin,span:Option<Span>)->Result<()>{
  let(left,right)=(self.borrowed_vector_source_origin(actual),self.borrowed_vector_source_origin(expected));if !left&&!right{return Ok(());}let(a,b)=(actual.array_element_position.as_deref(),expected.array_element_position.as_deref());
  let error=rules::read_rust_borrowed_vector_argument_error(left,right,a.zip(b).is_some_and(|(a,b)|self.same_origin(&position_origin(a,&actual.path),&position_origin(b,&expected.path))),a.zip(b).is_some_and(|(a,b)|a.owner==b.owner),a.and_then(|value|value.lifetimes.first()).and_then(Option::as_deref).unwrap_or(""),b.and_then(|value|value.lifetimes.first()).and_then(Option::as_deref).unwrap_or(""));if !error.is_empty(){return Err(type_error(&error,span));}self.validate_copy_closure_value_borrows(actual,span)
 }
 pub(super) fn capture_borrowed_vector_constructor(&mut self,node:&SExpr,mut origin:SourceOrigin)->Result<()>{
  self.borrowed_vector_needs_final_admission=true;
  let SNode::Array{items,..}=&node.node else{return Err(unsupported("Rust borrowed vector source","a borrowed vector needs its actual array constructor",node.span));};
  let count=rules::read_rust_borrowed_vector_element_count_error(items.len() as f64);if !count.is_empty(){return Err(unsupported("Rust borrowed vector source",&count,node.span));}
  self.reserve_owned_container_copies(&[&origin])?;let position=origin.array_element_position.as_deref().ok_or_else(||type_error("a borrowed vector needs its actual element position",node.span))?;let expected=position_origin(position,&origin.path);let mut fields=Vec::new();let mut loans=Vec::new();
  for item in items{if item.spread{return Err(unsupported("Rust borrowed vector source","a borrowed vector needs individually checked immutable reference elements",item.value.span));}
   let actual=self.origin(&item.value,&origin.path);self.check_owned_origin(&actual,item.value.span)?;self.validate_copy_closure_value_borrows(&actual,item.value.span)?;self.reserve_owned_container_copies(&[&actual])?;
   let child=argument_regions(&actual);let error=rules::read_rust_borrowed_vector_production_error(self.same_origin(&actual,&expected)&&actual.source_owner==expected.source_owner,actual.wrappers.len()==1&&actual.wrappers[0]=="reference",actual.proven&&!child.is_empty(),self.collecting);if !error.is_empty(){return Err(unsupported("Rust borrowed vector source",&error,item.value.span));}
   self.argument(&actual,&expected,item.value.span)?;loans=crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&loans,&child);fields.push(child);
  }
  origin.copy_evidence=None;origin.source_lexical_binding_identity=None;origin.source_binding_identity=None;origin.source_place_binding_identity=None;origin.source_loan_binding_identities.clear();origin.source_place_loan_binding_identities.clear();origin.source_borrowed_place=false;origin.source_value_borrow_bindings=loans;origin.source_value_field_borrow_bindings=Some(fields);origin.source_owned_result_proof=true;if let Some(span)=node.span{self.fresh_vector_constructors.insert(span);}let origin=self.capture_mutable_record_value_loans(origin,node.span)?;self.mark(node,origin);Ok(())
 }
 pub(super) fn borrowed_vector_call_result(&self,node:&SExpr,path:&[String],parameters:&[SourceOrigin],mut origin:SourceOrigin)->SourceOrigin{
  if !self.borrowed_vector_source_origin(&origin){return origin;}let SNode::App{func,args}=&node.node else{return origin;};let SNode::Name{path:names}=&func.node else{return origin;};if names.len()==1&&self.scope.contains_key(&names[0])||!self.owned_standard_sum_recorded_call(node){return origin;}
  let plan=self.borrowed_vector_result_parameters(&origin,parameters);let mut loans=Vec::new();let mut lifetimes=Vec::new();for index in plan{if index<0.0{origin.proven=false;origin.uncertain=true;origin.source_owned_result_proof=false;return origin;}let Some(arg)=args.get(index as usize)else{origin.proven=false;origin.uncertain=true;origin.source_owned_result_proof=false;return origin;};let actual=self.origin(arg,path);let regions=argument_regions(&actual);let error=rules::read_rust_borrowed_vector_call_region_error(actual.wrappers.first().is_some_and(|wrapper|wrapper=="reference"),regions.len() as f64,self.collecting);if !error.is_empty(){origin.proven=false;origin.uncertain=true;origin.source_owned_result_proof=false;return origin;}loans=crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&loans,&regions);lifetimes.push(actual.array_element_position.as_deref().and_then(|position|position.lifetimes.first()).cloned().flatten().or_else(||actual.source_lifetimes.first().cloned().flatten()).unwrap_or_default());}
  if let Some(position)=origin.array_element_position.as_deref_mut(){let lifetime=rules::read_rust_borrowed_vector_call_lifetime(position.lifetimes.first().and_then(Option::as_deref).unwrap_or(""),&lifetimes);position.lifetimes=vec![(!lifetime.is_empty()).then_some(lifetime)];}
  origin.fresh_array_value=None;origin.source_lexical_binding_identity=None;origin.source_binding_identity=None;origin.source_place_binding_identity=None;origin.source_loan_binding_identities.clear();origin.source_place_loan_binding_identities.clear();origin.source_value_borrow_bindings=loans;origin.source_value_field_borrow_bindings=None;origin.source_owned_result_proof=true;origin
 }
}

pub(super) fn source_type_position(origin:&SourceOrigin)->Result<SourceTypeSignature>{
 super::standard_sum_metadata::measure_origins(&[origin]).map_err(|message|type_error(&message,None))?;
 let ty=origin.ty.as_ref().ok_or_else(||type_error("a retained array element position needs its actual type and owner",None))?;
 let owner=origin.source_owner.as_ref().ok_or_else(||type_error("a retained array element position needs its actual type and owner",None))?;
 let mut pending=origin.array_element_position.as_deref();let mut visits=1;let mut depth=0;
 while let Some(position)=pending{
  visits+=1;depth+=1;let error=crate::translation::rust_borrowed_vector_rules::read_rust_borrowed_vector_position_budget_error(visits as f64,depth as f64);if !error.is_empty(){return Err(type_error(&error,None));}pending=position.array_element_position.as_deref();
 }
 Ok(SourceTypeSignature{ty:ty.clone(),wrappers:origin.wrappers.clone(),lifetimes:origin.source_lifetimes.clone(),owner:owner.clone(),array_element_position:origin.array_element_position.clone()})
}

#[cfg(test)]
mod deferred_position_controls{
 use super::*;
 use crate::translation::source_signatures::{SourceOwner,SourceSignature,validate_source_signature};
 use crate::translation::types::Type;
 fn signature(model:&str)->SourceSignature{
  let scalar=Type::Fixed{bits:8,signed:false};let owner=SourceOwner{kind:"primitive".into(),path:vec!["u8".into()],arguments:vec![]};
  let child=SourceTypeSignature{ty:scalar.clone(),wrappers:vec!["reference".into()],lifetimes:vec![Some("static".into())],owner:owner.clone(),array_element_position:None};
  SourceSignature{model:model.into(),language:"Rust".into(),parameters:vec![],result:SourceTypeSignature{ty:Type::Array{element:Box::new(scalar),reference:false},wrappers:vec![],lifetimes:vec![],owner:SourceOwner{kind:"library".into(),path:vec!["std".into(),"vec".into(),"Vec".into()],arguments:vec![owner]},array_element_position:Some(Box::new(child))},lifetime_bindings:vec![]}
 }
 #[test]
 fn only_the_new_source_profile_accepts_its_complete_child_position(){
  let actual=signature("Rust source function signature 3");assert!(validate_source_signature(&actual,&[],&actual.result.ty,"Rust").is_ok());
  for model in ["Rust source function signature 1","Rust source function signature 2"]{let previous=signature(model);assert!(validate_source_signature(&previous,&[],&previous.result.ty,"Rust").is_err());}
 }
 #[test]
 fn source_child_owners_are_not_inferred_from_the_erased_element(){
  let mut actual=signature("Rust source function signature 3");actual.result.array_element_position.as_mut().unwrap().owner.path=vec!["u16".into()];assert!(validate_source_signature(&actual,&[],&actual.result.ty,"Rust").is_err());
 }
}
