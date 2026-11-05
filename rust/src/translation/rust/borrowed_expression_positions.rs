//! Attach actual source positions to owned expression objects, never span maps.
use super::{Result,RustParser,SExpr,SourceOrigin,Span,unsupported};
use crate::translation::source_signatures::{RustSourceExpressionPosition,SourceOwner};
use crate::translation::rust_borrowed_vector_rules as rules;
impl RustParser{
 pub(super) fn prepare_borrowed_inherent_receiver_position(&self,mut origin:SourceOrigin,wrappers:Vec<String>,span:Option<Span>)->Result<SourceOrigin>{
  if !self.origins.borrowed_vector_source_enabled{origin.wrappers=wrappers;return Ok(origin);}let plan=rules::plan_rust_borrowed_inherent_position(&origin.wrappers,&wrappers);if plan.len()!=3{return Err(unsupported("Rust borrowed expression source position","the selected receiver needs its complete source layer projection",span));}
  let project=|mut owner:Option<SourceOwner>|->Result<Option<SourceOwner>>{for _ in 0..plan[0] as usize{let current=owner.take().ok_or_else(||unsupported("Rust borrowed expression source position","receiver dereference needs its actual canonical Box owner",span))?;if !crate::translation::rust_owned_recursive_datatype_rules::accept_rust_owned_box_type_node(&["box".into()],&current.kind,&current.path.join("::"),current.arguments.len() as f64){return Err(unsupported("Rust borrowed expression source position","receiver dereference needs its actual canonical Box owner",span));}owner=current.arguments.into_iter().next();}Ok(owner)};
  origin.source_owner=project(origin.source_owner.take())?;if origin.expanded_source_owner.is_some(){origin.expanded_source_owner=project(origin.expanded_source_owner.take())?;}let mut lifetimes=vec![None;plan[2] as usize];lifetimes.extend(origin.source_lifetimes.into_iter().skip(plan[1] as usize));origin.source_lifetimes=lifetimes;origin.wrappers=wrappers;Ok(origin)
 }
 pub(super) fn note_borrowed_expression_source_position(&mut self,node:&mut SExpr,path:&[String])->Result<()>{
  if !self.origins.borrowed_vector_source_enabled{return Ok(());}let origin=self.origins.origin(node,path);let relevant=!origin.wrappers.is_empty()||origin.array_element_position.is_some()||!origin.source_box_layers.is_empty();
  if origin.ty.is_none()||origin.source_owner.is_none()||!origin.proven||origin.diverging{if relevant&&!origin.diverging{self.origins.borrowed_vector_needs_final_admission=true;if !self.origins.collecting{return Err(unsupported("Rust borrowed expression source position","a qualified expression needs its actual complete source type and owner before reference erasure",node.span));}}return Ok(());}
  let cost=super::standard_sum_metadata::measure_origin_position_metadata(&origin).map_err(|message|unsupported("Rust borrowed expression source position",&message,node.span))?;
  let next=rules::read_rust_borrowed_expression_position_reservation(self.origins.source_position_materialized_nodes as f64,self.origins.source_position_materialized_units as f64,cost.nodes as f64,cost.units as f64);if next.len()!=2{return Err(unsupported("Rust borrowed expression source position","the owned expression source positions exceed their materialization budget",node.span));}self.origins.source_position_materialized_nodes=next[0] as usize;self.origins.source_position_materialized_units=next[1] as usize;
  let position=super::borrowed_vectors::source_type_position(&origin)?;let position=self.origins.standard_sum_qualified_position(&position,&origin.path,0).ok_or_else(||unsupported("Rust borrowed expression source position","the actual source position needs its complete declaration directory",node.span))?;
  node.source_position=Some(Box::new(RustSourceExpressionPosition{model:"Rust borrowed expression source position 1".into(),position:*position}));self.origins.borrowed_vector_needs_final_admission=true;Ok(())
 }
}
