//! Access to private owner maps remains inside SourceOrigins.
use super::{SourceOrigins,SourceOrigin,Span,Result,unsupported};
use std::collections::{HashSet,HashMap};
use crate::translation::rust_mutable_record_loan_rules as rules;
use crate::translation::rust_owned_binding_rules as loans;
use crate::translation::rust_owned_record_update_rules as fields;
use super::super::copy_closure_lifetimes::value_borrows;
impl SourceOrigins {
 fn charge_mutable_record_loans(&mut self,nodes:usize,text:usize,span:Option<Span>)->Result<()> {
  let state=&mut self.mutable_record_loans;
  let error=rules::read_rust_mutable_record_loan_budget_error(state.nodes as f64,state.text as f64,nodes as f64,text as f64);
  if !error.is_empty(){return Err(unsupported("Rust mutable record loan",&error,span));}
  state.nodes+=nodes;state.text+=text;Ok(())
 }
 pub(in super::super) fn register_mutable_record_loan_root(&mut self,origin:&SourceOrigin,span:Option<Span>)->Result<()> {
  if self.collecting{return Ok(());}
  let Some((root,lexical))=origin.source_binding_identity.as_ref().zip(origin.source_lexical_binding_identity.as_ref()) else{return Err(unsupported("Rust mutable record loan","a loan root needs its actual available mutable nominal binding",span));};
  if !origin.proven||!origin.wrappers.is_empty()||self.nominal_ownership_facts(origin).profile!="owned"||self.source_binding_states.get(root).map(String::as_str)!=Some("available"){return Err(unsupported("Rust mutable record loan","a loan root needs its actual available mutable nominal binding",span));}
  if !self.mutable_record_loans.roots.contains_key(root){self.charge_mutable_record_loans(4,2*(root.len()+lexical.len()),span)?;}
  self.mutable_record_loans.roots.insert(root.clone(),lexical.clone());self.mutable_record_loans.lexicals.insert(lexical.clone(),root.clone());Ok(())
 }
 pub(in super::super) fn capture_mutable_record_value_loans(&mut self,mut origin:SourceOrigin,span:Option<Span>)->Result<SourceOrigin> {
  if self.collecting{return Ok(origin);}
  let contained=value_borrows(&origin);let reference=origin.wrappers.iter().any(|layer|layer=="reference");let place=origin.source_borrowed_place;let copy=origin.direct_copy_value();
  if !rules::accept_rust_mutable_record_value_loans(reference,place,copy,contained.len() as f64){return Ok(origin);}
  let mut values=contained;
  if reference||place&&!copy {values=loans::merge_rust_owned_loan_bindings(&values,&origin.source_loan_binding_identities);values=loans::merge_rust_owned_loan_bindings(&values,&origin.source_place_loan_binding_identities);values=loans::merge_rust_owned_loan_bindings(&values,&origin.source_binding_identity.iter().chain(origin.source_place_binding_identity.iter()).cloned().collect::<Vec<_>>());}
  for id in &values {if self.mutable_record_loans.loans.contains_key(id)&&self.source_binding_states.get(id).map(String::as_str)!=Some("available"){return Err(unsupported("Rust mutable record loan","a value cannot renew an unavailable source loan",span));}}
  let dependencies=values.iter().filter(|id|!self.mutable_record_loans.loans.contains_key(*id)).cloned().collect::<Vec<_>>();let mut candidates=HashSet::new();
  for id in &values {let root=if self.mutable_record_loans.roots.contains_key(id){id.clone()}else if let Some(root)=self.mutable_record_loans.lexicals.get(id){root.clone()}else if let Some(loan)=self.mutable_record_loans.loans.get(id){loan.root.clone()}else{fields::read_rust_owned_record_ultimate_root(id)};if self.mutable_record_loans.roots.contains_key(&root){candidates.insert(root);}}
  // Stable ordering preserves allocation parity independently of HashMap order.
  let mut candidates=candidates.into_iter().collect::<Vec<_>>();candidates.sort();
  for root in candidates {
   let lexical=self.mutable_record_loans.roots[&root].clone();
   if !rules::select_rust_mutable_record_loan_root(&root,&lexical,&dependencies)&&!values.iter().any(|id|self.mutable_record_loans.loans.get(id).is_some_and(|loan|loan.root==root)){continue;}
   if values.iter().any(|id|self.mutable_record_loans.loans.get(id).is_some_and(|loan|loan.root==root&&dependencies.iter().all(|id|loan.dependencies.contains(id)))){continue;}
   let error=loans::read_rust_owned_loan_read_error(&dependencies.iter().map(|id|self.source_binding_states.get(id).cloned().unwrap_or_default()).collect::<Vec<_>>());if !error.is_empty(){return Err(unsupported("Rust mutable record loan",&error,span));}
   let id=rules::read_rust_mutable_record_loan_identity(self.next_source_binding_identity as f64);if id.is_empty(){return Err(unsupported("Rust mutable record loan","the actual loan identity exceeds its source domain",span));}
   self.charge_mutable_record_loans(dependencies.len()+values.len()+2,id.len()+root.len()+dependencies.iter().map(String::len).sum::<usize>()+values.iter().map(String::len).sum::<usize>(),span)?;self.next_source_binding_identity+=1;
   self.mutable_record_loans.loans.insert(id.clone(),super::super::mutable_record_loans::ValueLoan{root:root.clone(),dependencies:dependencies.clone(),lexical:dependencies.contains(&lexical)});self.mutable_record_loans.active.entry(root).or_default().insert(id.clone());self.source_binding_states.insert(id.clone(),"available".into());self.source_temporary_bindings.insert(id.clone());values=loans::merge_rust_owned_loan_bindings(&values,&[id]);
  }
  origin.source_value_borrow_bindings=values;Ok(origin)
 }
 pub(in super::super) fn promote_mutable_record_value_loans(&mut self,origin:SourceOrigin)->SourceOrigin {if !self.collecting{for id in value_borrows(&origin){if self.mutable_record_loans.loans.contains_key(&id){self.source_temporary_bindings.remove(&id);}}}origin}
 pub(in super::super) fn mutable_record_live_loan(&self,root:&str,target:&str)->bool {self.mutable_record_loans.active.get(root).is_some_and(|active|active.iter().any(|id|{let loan=&self.mutable_record_loans.loans[id];self.source_binding_states.get(id).map(String::as_str)==Some("available")&&crate::translation::rust_owned_record_state_rules::test_rust_owned_record_state_loan_overlap(root,target,&loan.dependencies,loan.lexical)}))}
 pub(in super::super) fn check_mutable_record_loan_move(&self,origin:&SourceOrigin,span:Option<Span>)->Result<()> {let target=origin.source_binding_identity.as_deref().unwrap_or("");let root=if self.mutable_record_loans.roots.contains_key(target){target.to_owned()}else{fields::read_rust_owned_record_ultimate_root(target)};let error=rules::read_rust_mutable_record_loan_move_error(self.owned_binding_value(origin),!root.is_empty()&&self.mutable_record_live_loan(&root,target));if error.is_empty(){Ok(())}else{Err(unsupported("Rust mutable record loan",&error,span))}}
 pub(in super::super) fn mutable_record_carried_loans(&self,result:&SourceOrigin)->HashSet<String> {value_borrows(result).into_iter().filter(|id|self.mutable_record_loans.loans.contains_key(id)).collect()}
 pub(in super::super) fn mutable_record_loan_survives(&self,id:&str,before:&HashMap<String,String>,carried:&HashSet<String>)->bool {let Some(loan)=self.mutable_record_loans.loans.get(id)else{return false;};rules::accept_rust_mutable_record_loan_escape(carried.contains(id),before.contains_key(&loan.root)&&self.source_binding_states.get(&loan.root).map(String::as_str)==Some("available"),loan.dependencies.iter().all(|id|self.source_binding_states.get(id).map(String::as_str)==Some("available")))}
 pub(in super::super) fn retire_mutable_record_loan(&mut self,id:&str){if let Some(loan)=self.mutable_record_loans.loans.get(id){if let Some(active)=self.mutable_record_loans.active.get_mut(&loan.root){active.remove(id);}}}
 pub(in super::super) fn restore_mutable_record_loan_index(&mut self){self.mutable_record_loans.active.clear();for(id,state)in &self.source_binding_states{if state=="available"{if let Some(loan)=self.mutable_record_loans.loans.get(id){self.mutable_record_loans.active.entry(loan.root.clone()).or_default().insert(id.clone());}}}}
}
