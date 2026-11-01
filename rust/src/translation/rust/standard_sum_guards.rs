//! Actual owned pattern shape transport into shared deferred guard decisions.
use super::{Result,RustParser,SExpr,SPattern,SPatternNode,unsupported};
use crate::translation::rust_standard_sum_guard_rules as rules;
impl RustParser{
 pub(super) fn defer_standard_sum_guard(&self,patterns:&[SPattern],scrutinee:&SExpr,path:&[String],guarded:bool,aliases:usize)->Result<bool>{
  let origin=self.origins.origin(scrutinee,path);let owned=self.origins.is_owned_standard_sum_value(&origin);
  if rules::read_rust_standard_sum_guard_mode(owned,guarded)!="deferred guard"{return Ok(false);}
  let mut pending=patterns.iter().collect::<Vec<_>>();let mut count=aliases;
  while let Some(pattern)=pending.pop(){match &pattern.node{SPatternNode::Alias{pattern,..}=>{count+=1;pending.push(pattern);},SPatternNode::Ctor{args,..}|SPatternNode::Tuple{args,..}=>pending.extend(args),_=>{}}}
  let error=rules::read_rust_standard_sum_deferred_guard_pattern_error(patterns.len() as f64,count as f64);if !error.is_empty(){return Err(unsupported("Rust owned standard sum guard",&error,scrutinee.span));}Ok(true)
 }
}
