//! Actual source places and constructor arguments into the generated field
//! move policy. Field identities stay only in existing owner-state frames.
use super::{Options,Result,RustParser,SExpr,SNode,SourceOrigins,SourceOrigin,SourceDeclaration,Span,Type,unsupported};
use super::records::NamedRecord;
use crate::translation::rust_owned_record_update_rules as rules;
use std::collections::HashMap;
pub(super) fn field_state(snapshot:&HashMap<String,String>,identity:&str)->String{
 if let Some(state)=snapshot.get(identity){return state.clone();}let mut root=rules::read_rust_owned_record_field_root(identity);for _ in 0..crate::translation::rust_owned_record_place_rules::read_rust_owned_record_place_depth_limit() as usize{if root.is_empty(){break;}if let Some(state)=snapshot.get(&root){return rules::read_rust_owned_record_absent_field_state(state).to_owned();}root=rules::read_rust_owned_record_field_root(&root);}"absent".to_owned()
}
impl SourceOrigins{
 pub(super) fn check_owned_record_root(&self,origin:&SourceOrigin,span:Option<Span>)->Result<()>{
  let mut loans=origin.clone();loans.source_binding_identity=None;self.check_owned_origin(&loans,span)?;
  if let Some(root)=origin.source_binding_identity.as_ref().or(origin.source_place_binding_identity.as_ref()){
   let error=rules::read_rust_owned_record_field_error(self.copy_closure_lexical_state(root),"available");if !error.is_empty(){return Err(unsupported("Rust record field ownership",&error,span));}
  }Ok(())
 }
 pub(super) fn project_owned_record_field(&mut self,parent:&SourceOrigin,field:&SourceOrigin,index:usize,borrowed:bool,span:Option<Span>)->Result<SourceOrigin>{
  self.check_owned_record_root(parent,span)?;
  let root=parent.source_binding_identity.as_ref().or(parent.source_place_binding_identity.as_ref());
  let identity=root.map(|root|rules::read_rust_owned_record_field_identity(root,index as f64)).unwrap_or_default();
  if !identity.is_empty()&&!self.collecting{
   let actual=self.copy_closure_lexical_state(&identity);let error=rules::read_rust_owned_record_field_error(root.map_or("",|root|self.copy_closure_lexical_state(root)),if actual.is_empty(){"available"}else{actual});if !error.is_empty(){return Err(unsupported("Rust record field ownership",&error,span));}
  }
  let mode=rules::read_rust_owned_record_field_mode(rules::test_rust_owned_record_field_copy(field.proven,field.copy_evidence.is_some(),&field.wrappers),borrowed||parent.source_borrowed_place);
  let mut loans=root.cloned().into_iter().collect::<Vec<_>>();loans=crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&loans,&parent.source_loan_binding_identities);loans=crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&loans,&parent.source_place_loan_binding_identities);
  let mut result=self.project_nominal_pattern_field(parent,field,index,false);if !identity.is_empty()&&!self.collecting{self.ensure_owned_record_field_state(&identity);}
  result.source_binding_identity=if mode=="copied value"||identity.is_empty(){None}else{Some(identity.clone())};result.source_place_binding_identity=(!identity.is_empty()).then(||identity.clone());result.source_nominal_pattern_move=false;result.source_owned_result_proof=mode=="owned place"&&identity.is_empty()&&parent.source_owned_result_proof;result.source_borrowed_place=mode=="borrowed place";result.source_nominal_borrowed_field=mode=="borrowed place";result.source_loan_binding_identities=if mode=="copied value"{Vec::new()}else{crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&loans,&if identity.is_empty(){Vec::new()}else{vec![identity]})};result.source_place_loan_binding_identities=loans;Ok(result)
 }
}
impl RustParser{
 pub(super) fn owned_record_projection_name(&self,node:&SExpr,_path:&[String])->bool{
  let SNode::Name{path:names}=&node.node else{return false;};if names.len()!=1||!self.cursor.is(".")||self.cursor.peek_at(2).value=="("{return false;}
  let Some(origin)=self.origins.scope.get(&names[0])else{return false;};let Some(Type::Named{path,..})=origin.ty.as_ref()else{return false;};
  let Some(owner)=self.origins.lookup_type_owner(path,&origin.path)else{return false;};self.named_records.get(&owner.full_name).is_some_and(|record|record.names.contains(&self.cursor.peek_at(1).value))
 }
 pub(super) fn owned_record_update_base(&mut self,path:&[String])->Result<SExpr>{
  if let Some(place)=self.read_owned_record_place_base(path)?{return Ok(place);}
  let token=self.cursor.peek().clone();let origin=self.origins.scope.get(&token.value).cloned();
  if self.cursor.peek_at(1).value=="}"&&origin.as_ref().is_some_and(|origin|self.origins.nominal_ownership_facts(origin).profile=="owned"){
   self.cursor.advance();let span=Some(Span::new(token.start,token.end));self.origins.check_owned_record_root(origin.as_ref().expect("actual local record"),span)?;return Ok(SExpr::new(SNode::Name{path:vec![token.value]},span));
  }self.expr(path,Options::default())
 }
 pub(super) fn prepare_owned_record_entry(&mut self,record:&NamedRecord,name:&str,value:&SExpr,path:&[String])->Result<()>{
  if self.header_discovery||self.origins.collecting{return Ok(());}
  let Some(index)=record.names.iter().position(|field|field==name)else{return Ok(());};let mut names=vec!["crate".into()];names.extend(record.constructor.split('.').map(str::to_owned));
  if let Some(entry)=self.origins.lookup_value_owner(&names,&[])&&let SourceDeclaration::Constructor{fields,..}=entry.declaration&&let Some(field)=fields.get(index){self.origins.argument(&self.origins.origin(value,path),field,value.span)?;self.origins.prepare_owned_argument(value,Some(field),path)?;}Ok(())
 }
 pub(super) fn owned_record_update_origin(&mut self,record:&NamedRecord,values:&[SExpr],base:&SExpr,positions:&[Option<usize>],node:&SExpr,path:&[String],span:Option<Span>)->Result<Option<SourceOrigin>>{
  let parent=self.origins.origin(base,path);let facts=self.origins.nominal_ownership_facts(&parent);if self.header_discovery||self.origins.collecting||facts.profile!="owned"{return Ok(None);}
  let version=rules::read_rust_owned_record_update_version_error(self.source_version_minor);if !version.is_empty(){return Err(unsupported("Rust owned record update",&version,span));}
  let mut names=vec!["crate".into()];names.extend(record.constructor.split('.').map(str::to_owned));let entry=self.origins.lookup_value_owner(&names,&[]);
  let (actual,fields)=match entry{Some(entry)=>match entry.declaration{SourceDeclaration::Constructor{owner,fields,..}=>(owner==record.owner&&facts.owner==record.owner,fields),_=>(false,Vec::new())},None=>(false,Vec::new())};
  let error=rules::read_rust_owned_record_update_error(&facts.profile,actual,parent.wrappers.len() as f64,fields.len() as f64,positions.len() as f64);if !error.is_empty(){return Err(unsupported("Rust owned record update",&error,span));}
  self.origins.check_owned_record_root(&parent,span)?;if parent.source_binding_identity.is_none()&&!parent.source_owned_result_proof{return Err(unsupported("Rust owned record update","an owned record base needs its actual available root or fresh constructor producer",span));}
  let mut actuals=Vec::new();for(index,position)in positions.iter().enumerate(){actuals.push(if let Some(position)=position{self.origins.origin(&values[*position],path)}else{self.origins.project_owned_record_field(&parent,&fields[index],index,false,span)?});}
  let previous=self.origins.scope.clone();let read=(||{let mut args=Vec::new();for(index,position)in positions.iter().enumerate(){if let Some(position)=position{args.push(values[*position].clone());}else{let name=self.fresh_product_binding();self.origins.scope.insert(name.clone(),actuals[index].clone());args.push(SExpr::new(SNode::Name{path:vec![name]},None));}}self.nominal_constructor_origin(node,&args,path,span)})();self.origins.scope=previous;read
 }
}
