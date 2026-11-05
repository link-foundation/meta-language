//! Structural source wrapper -> provisional checked marker. Every marker must
//! pass the shared complete-program proof before the checker returns.
use super::{Checker,Env,Expr,Language,Node,Result,SArrayItem,SNode,Span,Type,unsupported};
use crate::translation::disjoint_collection_iteration_rules as rules;
impl Checker{
 pub(super) fn closed_collection_iteration(&mut self,args:&[SArrayItem],span:Option<Span>,env:&Env,path:&[String],expected:Option<&Type>,eager:bool)->Result<Expr>{
  if self.language!=Language::JavaScript||self.checked_projection{return Err(unsupported("closed collection iteration","private source operations cannot enter checked projections",span));}
  if args.len()!=1||args[0].spread{return Err(unsupported("closed collection iteration","one actual eager for-of source operand is required",span));}
  let SNode::ForOfIterable{object,..}=&args[0].value.node else{return Err(unsupported("closed collection iteration","one actual eager for-of source operand is required",span));};
  let SNode::Method{object:receiver,op, args:method_args}=&object.node else{return Err(unsupported("closed collection iteration","only a direct zero-argument standard keys, values or entries source is admitted",span));};
  if rules::read_disjoint_collection_iteration_item_form("map",op).is_empty()||!method_args.is_empty(){return Err(unsupported("closed collection iteration","only a direct zero-argument standard keys, values or entries source is admitted",span));}
  if eager&&!rules::accept_disjoint_collection_spread_receiver(receiver.node.kind()){return Err(unsupported("closed collection spread","the eager source needs an actual owned name or field receiver",span));}
  let receiver=self.expr(receiver,env,path,None,false)?;
  if rules::read_disjoint_collection_iteration_item_form(receiver.ty.kind(),op).is_empty(){return self.expr(&args[0].value,env,path,expected,false);}
  let shared=receiver.ty.is_reference();
  let mut snapshot=self.checked_collection_access(Self::reference_value(receiver),op,&[],env,path,span)?;
  if !shared{return Ok(snapshot);}
  let ordinal=self.closed_collection_iteration_markers.len() as f64;
  let marker=if eager{rules::read_disjoint_collection_spread_marker(ordinal,op)}else{rules::read_disjoint_collection_iteration_marker(ordinal,op)};self.closed_collection_iteration_markers.push((marker.clone(),op.clone()));
  let Node::CollectionItems{op,..}=&mut snapshot.node else{return Err(unsupported("closed collection iteration","the actual collection item constructor is required",span));};
  *op=marker;Ok(snapshot)
 }
}
