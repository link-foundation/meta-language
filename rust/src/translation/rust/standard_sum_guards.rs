//! Actual owned pattern shape transport into shared deferred guard decisions.
use super::{Result,RustParser,SExpr,SPattern,SPatternNode,SourceOrigin,unsupported};
use crate::translation::rust_standard_sum_guard_rules as rules;
pub(super) fn standard_sum_guard_alias_count(patterns:&[SPattern])->usize{
 let mut pending=patterns.iter().collect::<Vec<_>>();let mut count=0;
 while let Some(pattern)=pending.pop(){match &pattern.node{SPatternNode::Alias{pattern,..}=>{count+=1;pending.push(pattern);},SPatternNode::Ctor{args,..}|SPatternNode::Tuple{args,..}=>pending.extend(args),_=>{}}}count
}
pub(super) fn same_standard_sum_guard_lineage(left:&SourceOrigin,right:&SourceOrigin)->bool{
 left.source_binding_identity==right.source_binding_identity&&left.source_place_binding_identity==right.source_place_binding_identity&&left.source_loan_binding_identities==right.source_loan_binding_identities&&left.source_place_loan_binding_identities==right.source_place_loan_binding_identities&&left.source_lifetimes==right.source_lifetimes&&left.source_borrowed_place==right.source_borrowed_place&&left.source_owned_result_proof==right.source_owned_result_proof&&left.source_standard_sum_pattern_move==right.source_standard_sum_pattern_move&&left.source_standard_sum_borrowed_field==right.source_standard_sum_borrowed_field&&left.source_extending_temporary_bindings==right.source_extending_temporary_bindings
}
impl RustParser{
 pub(super) fn defer_standard_sum_guard(&self,patterns:&[SPattern],scrutinee:&SExpr,path:&[String],guarded:bool,aliases:usize)->Result<bool>{
  let origin=self.origins.origin(scrutinee,path);let owned=self.origins.is_owned_standard_sum_value(&origin);
  if rules::read_rust_standard_sum_guard_mode(owned,guarded)!="deferred guard"{return Ok(false);}
  let error=rules::read_rust_standard_sum_guard_alternative_pattern_error(patterns.len() as f64,(aliases+standard_sum_guard_alias_count(patterns)) as f64);if !error.is_empty(){return Err(unsupported("Rust owned standard sum guard",&error,scrutinee.span));}Ok(true)
 }
}
